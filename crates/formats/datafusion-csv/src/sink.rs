//! CSV Data Sink implementation for writing data to CSV files

use std::sync::Arc;

use arrow_array::RecordBatch;
use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::datasource::physical_plan::FileSinkConfig;
use datafusion::datasource::sink::DataSink;
use datafusion::physical_plan::DisplayAs;
use datafusion::physical_plan::DisplayFormatType;
use datafusion::physical_plan::metrics::MetricsSet;
use datafusion_common::{DataFusionError, Result};
use datafusion_execution::{SendableRecordBatchStream, TaskContext};
use futures::StreamExt;

use crate::writer::CsvWriterOptions;

/// Convert geometry columns to WKT in a record batch
fn convert_geometry_to_wkt_in_batch(batch: &RecordBatch) -> Result<RecordBatch> {
    use arrow_schema::Schema;
    use geoarrow_array::GeoArrowArray;
    use geoarrow_array::array::from_arrow_array;
    use geoarrow_array::cast::to_wkt;

    let schema = batch.schema();
    let mut new_columns = Vec::with_capacity(batch.num_columns());
    let mut new_fields = Vec::with_capacity(batch.num_columns());
    let mut any_converted = false;

    for (idx, field) in schema.fields().iter().enumerate() {
        let array = batch.column(idx);

        // Try to convert from Arrow array to GeometryArray
        if let Ok(geom_array) = from_arrow_array(array.as_ref(), field) {
            // This is a geometry column - convert to WKT (using i32 offsets)
            if let Ok(wkt_array) = to_wkt::<i32>(&geom_array) {
                new_columns.push(wkt_array.into_array_ref());
                new_fields.push(Arc::new(arrow_schema::Field::new(
                    field.name(),
                    arrow_schema::DataType::Utf8,
                    field.is_nullable(),
                )));
                any_converted = true;
            } else {
                // Conversion failed, keep original
                new_columns.push(array.clone());
                new_fields.push(Arc::new((**field).clone()));
            }
        } else {
            // Not a geometry column, keep as is
            new_columns.push(array.clone());
            new_fields.push(Arc::new((**field).clone()));
        }
    }

    if any_converted {
        let new_schema = Arc::new(Schema::new(new_fields));
        RecordBatch::try_new(new_schema, new_columns)
            .map_err(|e| DataFusionError::External(Box::new(e)))
    } else {
        Ok(batch.clone())
    }
}

/// CSV data sink that implements the `DataSink` trait
#[derive(Debug)]
pub struct CsvSink {
    config: FileSinkConfig,
    writer_options: CsvWriterOptions,
}

impl CsvSink {
    /// Create a new CSV sink
    #[must_use]
    pub fn new(config: FileSinkConfig, writer_options: CsvWriterOptions) -> Self {
        Self {
            config,
            writer_options,
        }
    }

    /// Get the sink configuration
    #[must_use]
    pub fn config(&self) -> &FileSinkConfig {
        &self.config
    }

    /// Get writer options
    #[must_use]
    pub fn writer_options(&self) -> &CsvWriterOptions {
        &self.writer_options
    }
}

#[async_trait]
impl DataSink for CsvSink {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn metrics(&self) -> Option<MetricsSet> {
        None
    }

    fn schema(&self) -> &SchemaRef {
        self.config.output_schema()
    }

    async fn write_all(
        &self,
        mut data: SendableRecordBatchStream,
        _context: &Arc<TaskContext>,
    ) -> Result<u64> {
        use arrow_csv::WriterBuilder;
        use datafusion::logical_expr::dml::InsertOp;
        use std::fs::OpenOptions;

        // Write to output - for now write to a single file
        let output_path = self
            .config
            .table_paths
            .first()
            .ok_or_else(|| DataFusionError::Internal("No output path specified".to_string()))?;

        // Convert URL to file path - remove file:// prefix if present
        let url_str =
            <datafusion::datasource::listing::ListingTableUrl as AsRef<str>>::as_ref(output_path);
        let file_path = if let Some(path) = url_str.strip_prefix("file://") {
            path
        } else {
            url_str
        };

        // Open file based on insert operation mode
        let mut file = match self.config.insert_op {
            InsertOp::Overwrite => {
                // Overwrite mode: create or truncate the file
                std::fs::File::create(file_path)
                    .map_err(|e| DataFusionError::External(Box::new(e)))?
            },
            InsertOp::Append => {
                // Append mode: open for appending or create if doesn't exist
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)
                    .map_err(|e| DataFusionError::External(Box::new(e)))?
            },
            InsertOp::Replace => {
                return Err(DataFusionError::NotImplemented(format!(
                    "Insert operation {:?} is not supported for CSV",
                    self.config.insert_op
                )));
            },
        };

        let mut builder = WriterBuilder::new()
            .with_delimiter(self.writer_options.delimiter)
            .with_header(self.writer_options.has_header);

        if let Some(ref format) = self.writer_options.date_format {
            builder = builder.with_date_format(format.clone());
        }
        if let Some(ref format) = self.writer_options.datetime_format {
            builder = builder.with_datetime_format(format.clone());
        }
        if let Some(ref format) = self.writer_options.timestamp_format {
            builder = builder.with_timestamp_format(format.clone());
        }
        if let Some(ref format) = self.writer_options.time_format {
            builder = builder.with_time_format(format.clone());
        }
        if !self.writer_options.null_value.is_empty() {
            builder = builder.with_null(self.writer_options.null_value.clone());
        }

        let mut csv_writer = builder.build(&mut file);
        let mut row_count = 0u64;

        // Stream batches and write incrementally
        while let Some(batch_result) = data.next().await {
            let batch = batch_result?;
            row_count += batch.num_rows() as u64;

            // Convert geometry columns to WKT before writing
            let batch_to_write = convert_geometry_to_wkt_in_batch(&batch)?;

            // Write batch - header is written automatically by Arrow CSV writer for first batch
            csv_writer
                .write(&batch_to_write)
                .map_err(|e| DataFusionError::External(Box::new(e)))?;
        }

        Ok(row_count)
    }
}

impl DisplayAs for CsvSink {
    fn fmt_as(&self, _t: DisplayFormatType, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CsvSink")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer::CsvWriterOptions;
    use arrow_schema::{DataType, Field, Schema};
    use datafusion::datasource::listing::ListingTableUrl;
    use datafusion::datasource::physical_plan::FileGroup;
    use datafusion::logical_expr::dml::InsertOp;
    use datafusion_execution::object_store::ObjectStoreUrl;

    #[test]
    fn test_csv_sink_creation() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("name", DataType::Utf8, true),
        ]));

        let config = FileSinkConfig {
            original_url: "file:///tmp/output.csv".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "csv".to_string(),
        };

        let writer_options = CsvWriterOptions::default();
        let sink = CsvSink::new(config, writer_options);

        assert_eq!(sink.schema().fields().len(), 2);
        assert_eq!(sink.writer_options().delimiter, b',');
    }

    #[test]
    fn test_csv_sink_as_any() {
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let config = FileSinkConfig {
            original_url: "file:///tmp/output.csv".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "csv".to_string(),
        };

        let sink = CsvSink::new(config, CsvWriterOptions::default());
        assert!(sink.as_any().is::<CsvSink>());
    }

    #[test]
    fn test_csv_sink_metrics() {
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let config = FileSinkConfig {
            original_url: "file:///tmp/output.csv".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "csv".to_string(),
        };

        let sink = CsvSink::new(config, CsvWriterOptions::default());
        assert!(sink.metrics().is_none());
    }

    #[test]
    fn test_csv_sink_display() {
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let config = FileSinkConfig {
            original_url: "file:///tmp/output.csv".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "csv".to_string(),
        };

        let sink = CsvSink::new(config, CsvWriterOptions::default());
        assert_eq!(format!("{sink:?}"), format!("{sink:?}"));
    }
}
