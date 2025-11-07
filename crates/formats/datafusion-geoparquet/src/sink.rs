//! `GeoParquet` Data Sink implementation for writing data to `GeoParquet` files

use std::sync::Arc;

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::datasource::physical_plan::FileSinkConfig;
use datafusion::datasource::sink::DataSink;
use datafusion::physical_plan::metrics::MetricsSet;
use datafusion::physical_plan::stream::RecordBatchStreamAdapter;
use datafusion::physical_plan::{DisplayAs, DisplayFormatType};
use datafusion_common::{DataFusionError, Result};
use datafusion_execution::{SendableRecordBatchStream, TaskContext};
use futures::StreamExt;

use crate::writer::GeoParquetWriterOptions;

/// `GeoParquet` data sink that implements the `DataSink` trait
#[derive(Debug)]
pub struct GeoParquetSink {
    config: FileSinkConfig,
    writer_options: GeoParquetWriterOptions,
}

impl GeoParquetSink {
    /// Create a new `GeoParquet` sink
    #[must_use]
    pub fn new(config: FileSinkConfig, writer_options: GeoParquetWriterOptions) -> Self {
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
    pub fn writer_options(&self) -> &GeoParquetWriterOptions {
        &self.writer_options
    }
}

#[async_trait]
impl DataSink for GeoParquetSink {
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
        data: SendableRecordBatchStream,
        _context: &Arc<TaskContext>,
    ) -> Result<u64> {
        use std::sync::atomic::{AtomicU64, Ordering};

        use datafusion::logical_expr::dml::InsertOp;

        // Get output path from original URL (the file path user specified)
        // Strip file:// prefix if present
        let url_str = &self.config.original_url;
        let file_path = if let Some(path) = url_str.strip_prefix("file://") {
            path
        } else {
            url_str
        };

        // For GeoParquet, we support Overwrite and Replace modes
        // Both modes overwrite the entire file since Parquet files are immutable
        // Append doesn't make sense for Parquet files
        match self.config.insert_op {
            InsertOp::Overwrite | InsertOp::Replace => {
                // Both modes result in overwriting the file
            },
            InsertOp::Append => {
                return Err(DataFusionError::NotImplemented(
                    "Insert operation Append is not supported for GeoParquet. \
                     Parquet files are immutable and cannot be appended to. \
                     Use Overwrite or Replace instead."
                        .to_string(),
                ));
            },
        }

        // Create file for writing
        let file =
            std::fs::File::create(file_path).map_err(|e| DataFusionError::External(Box::new(e)))?;

        let mut writer = file;

        // Use streaming writer to process batches without buffering all in memory
        // We need to count rows, so we'll wrap the stream to track row count
        let row_count = Arc::new(AtomicU64::new(0));
        let row_count_clone = row_count.clone();

        // Get schema before wrapping the stream
        let schema = self.config.output_schema().clone();

        // Create a stream that counts rows as batches pass through
        let counting_stream = data.inspect(move |result| {
            if let Ok(batch) = result {
                row_count_clone.fetch_add(batch.num_rows() as u64, Ordering::SeqCst);
            }
        });

        // Wrap in RecordBatchStreamAdapter to make it a proper RecordBatchStream
        let adapted_stream = RecordBatchStreamAdapter::new(schema, counting_stream);

        crate::writer::write_geoparquet_stream(
            &mut writer,
            Box::pin(adapted_stream),
            &self.writer_options,
        )
        .await?;

        Ok(row_count.load(Ordering::SeqCst))
    }
}

impl DisplayAs for GeoParquetSink {
    fn fmt_as(&self, t: DisplayFormatType, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match t {
            DisplayFormatType::Default | DisplayFormatType::Verbose => {
                write!(f, "GeoParquetSink({})", self.config.original_url)
            },
            DisplayFormatType::TreeRender => {
                write!(f, "GeoParquetSink")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_options_default() {
        let options = GeoParquetWriterOptions::default();
        assert_eq!(options.geometry_column_name, "geometry");
    }

    #[test]
    fn test_writer_options_with_geometry_column() {
        let options = GeoParquetWriterOptions::default().with_geometry_column("wkt");
        assert_eq!(options.geometry_column_name, "wkt");
    }

    #[test]
    fn test_writer_options_with_row_group_size() {
        let options = GeoParquetWriterOptions::default().with_row_group_size(1024);
        assert_eq!(options.row_group_size, 1024);
    }

    #[test]
    fn test_writer_options_builder_pattern() {
        use parquet::basic::ZstdLevel;

        let options = GeoParquetWriterOptions::new()
            .with_geometry_column("location")
            .with_row_group_size(4096)
            .with_compression(parquet::basic::Compression::ZSTD(ZstdLevel::default()));

        assert_eq!(options.geometry_column_name, "location");
        assert_eq!(options.row_group_size, 4096);
        assert!(matches!(
            options.compression,
            parquet::basic::Compression::ZSTD(_)
        ));
    }
}
