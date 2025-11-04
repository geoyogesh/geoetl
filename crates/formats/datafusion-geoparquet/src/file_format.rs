//! `GeoParquet` file format configuration and `DataFusion` integration.
#![allow(clippy::result_large_err)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::single_match_else)]
#![allow(unused_imports)]

use std::sync::Arc;

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::datasource::file_format::FileFormat;
use datafusion::datasource::file_format::file_compression_type::FileCompressionType;
use datafusion::datasource::physical_plan::{FileScanConfig, FileSource};
use datafusion::error::Result;
use datafusion::physical_plan::ExecutionPlan;
use datafusion_common::Statistics;
use datafusion_session::Session;
use object_store::{ObjectMeta, ObjectStore};

use crate::file_source::GeoParquetFileSource;
use crate::physical_exec::GeoParquetExec;

/// Options controlling `GeoParquet` reading behaviour.
#[derive(Debug, Clone)]
pub struct GeoParquetFormatOptions {
    /// Target batch size when producing record batches.
    pub batch_size: usize,
    /// File extension to look for when listing datasets.
    pub file_extension: String,
    /// Name of the geometry column in the output schema.
    pub geometry_column_name: String,
}

impl Default for GeoParquetFormatOptions {
    fn default() -> Self {
        Self {
            batch_size: 8192,
            file_extension: ".parquet".to_string(),
            geometry_column_name: "geometry".to_string(),
        }
    }
}

impl GeoParquetFormatOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    #[must_use]
    pub fn with_file_extension(mut self, extension: impl Into<String>) -> Self {
        self.file_extension = extension.into();
        self
    }

    #[must_use]
    pub fn with_geometry_column_name(mut self, name: impl Into<String>) -> Self {
        self.geometry_column_name = name.into();
        self
    }

    pub(crate) fn file_extension_with_dot(&self) -> String {
        if self.file_extension.starts_with('.') {
            self.file_extension.clone()
        } else {
            format!(".{}", self.file_extension)
        }
    }
}

/// `GeoParquet` [`FileFormat`] implementation for `DataFusion`.
#[derive(Debug, Clone)]
pub struct GeoParquetFormat {
    options: GeoParquetFormatOptions,
}

impl GeoParquetFormat {
    pub fn new(options: GeoParquetFormatOptions) -> Self {
        Self { options }
    }
}

impl Default for GeoParquetFormat {
    fn default() -> Self {
        Self::new(GeoParquetFormatOptions::default())
    }
}

#[async_trait]
impl FileFormat for GeoParquetFormat {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ext(&self) -> String {
        self.options.file_extension_with_dot()
    }

    fn get_ext_with_compression(&self, _c: &FileCompressionType) -> Result<String> {
        Ok(self.get_ext())
    }

    fn compression_type(&self) -> Option<FileCompressionType> {
        None
    }

    async fn infer_schema(
        &self,
        _state: &dyn Session,
        store: &Arc<dyn ObjectStore>,
        objects: &[ObjectMeta],
    ) -> Result<SchemaRef> {
        if objects.is_empty() {
            return Ok(Arc::new(arrow_schema::Schema::empty()));
        }

        let object = &objects[0];

        // Use geoparquet crate to read the schema
        let get_result = store.get(&object.location).await?;
        let bytes = get_result.bytes().await?;

        // Use the parquet reader builder to get schema
        use geoarrow_schema::CoordType;
        use geoparquet::reader::GeoParquetReaderBuilder;
        use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

        let parquet_builder = ParquetRecordBatchReaderBuilder::try_new(bytes)
            .map_err(|e| datafusion::error::DataFusionError::External(Box::new(e)))?;

        // Try to get GeoParquet metadata and infer GeoArrow schema
        match parquet_builder.geoparquet_metadata() {
            Some(Ok(geoparquet_meta)) => {
                // Successfully got GeoParquet metadata, infer GeoArrow schema
                let geo_schema = parquet_builder
                    .geoarrow_schema(
                        &geoparquet_meta,
                        true, // parse_to_native: convert WKB to native geometry types
                        CoordType::Interleaved,
                    )
                    .map_err(|e| datafusion::error::DataFusionError::External(Box::new(e)))?;

                Ok(geo_schema)
            },
            _ => {
                // Fall back to plain Arrow schema if GeoParquet metadata is not found
                let arrow_schema = parquet_builder.schema();
                Ok(Arc::new(arrow_schema.as_ref().clone()))
            },
        }
    }

    async fn infer_stats(
        &self,
        _state: &dyn Session,
        _store: &Arc<dyn ObjectStore>,
        table_schema: SchemaRef,
        _object: &ObjectMeta,
    ) -> Result<Statistics> {
        Ok(Statistics::new_unknown(&table_schema))
    }

    async fn create_physical_plan(
        &self,
        _state: &dyn Session,
        conf: FileScanConfig,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        let exec = GeoParquetExec::new(conf, self.options.clone());
        Ok(Arc::new(exec))
    }

    fn file_source(&self) -> Arc<dyn FileSource> {
        Arc::new(GeoParquetFileSource::new(self.options.clone()))
    }

    async fn create_writer_physical_plan(
        &self,
        input: Arc<dyn ExecutionPlan>,
        _state: &dyn Session,
        conf: datafusion::datasource::physical_plan::FileSinkConfig,
        order_requirements: Option<datafusion_physical_expr::LexRequirement>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        use datafusion_datasource::sink::DataSinkExec;

        // Create writer options from format options
        let writer_options = crate::writer::GeoParquetWriterOptions::default()
            .with_geometry_column(self.options.geometry_column_name.clone());

        // Create the sink
        let sink = Arc::new(crate::sink::GeoParquetSink::new(conf, writer_options));

        Ok(Arc::new(DataSinkExec::new(input, sink, order_requirements)))
    }
}

/// Helper to detect file extensions from a provided path.
#[allow(dead_code)]
pub(crate) fn detect_file_extension(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_helpers() {
        let options = GeoParquetFormatOptions::new()
            .with_batch_size(256)
            .with_file_extension("geoparquet")
            .with_geometry_column_name("geom");

        assert_eq!(options.batch_size, 256);
        assert_eq!(options.file_extension_with_dot(), ".geoparquet");
        assert_eq!(options.geometry_column_name, "geom");
    }
}
