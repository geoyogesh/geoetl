//! `GeoParquet` Data Sink implementation for writing data to `GeoParquet` files

use std::sync::Arc;

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::datasource::physical_plan::FileSinkConfig;
use datafusion::datasource::sink::DataSink;
use datafusion::physical_plan::metrics::MetricsSet;
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
        mut data: SendableRecordBatchStream,
        _context: &Arc<TaskContext>,
    ) -> Result<u64> {
        use datafusion::logical_expr::dml::InsertOp;

        // Get output path from original URL (the file path user specified)
        // Strip file:// prefix if present
        let url_str = &self.config.original_url;
        let file_path = if let Some(path) = url_str.strip_prefix("file://") {
            path
        } else {
            url_str
        };

        // For GeoParquet, we only support Overwrite mode
        // Append doesn't make sense for Parquet files
        match self.config.insert_op {
            InsertOp::Overwrite => {},
            InsertOp::Append | InsertOp::Replace => {
                return Err(DataFusionError::NotImplemented(format!(
                    "Insert operation {:?} is not supported for GeoParquet. Only Overwrite is supported.",
                    self.config.insert_op
                )));
            },
        }

        // Collect all batches
        let mut batches = Vec::new();
        let mut row_count = 0u64;

        while let Some(batch_result) = data.next().await {
            let batch = batch_result?;
            row_count += batch.num_rows() as u64;
            batches.push(batch);
        }

        // Write all batches to file
        let file =
            std::fs::File::create(file_path).map_err(|e| DataFusionError::External(Box::new(e)))?;

        let mut writer = file;
        crate::writer::write_geoparquet(&mut writer, &batches, &self.writer_options)?;

        Ok(row_count)
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
