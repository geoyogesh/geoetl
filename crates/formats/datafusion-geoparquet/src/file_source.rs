//! `GeoParquet` file source and table provider implementation.
#![allow(clippy::items_after_statements)]
#![allow(clippy::unnecessary_literal_bound)]
#![allow(unused_imports)]

use std::sync::Arc;

use arrow_array::RecordBatch;
use arrow_schema::SchemaRef;
use datafusion::catalog::TableProvider;
use datafusion::datasource::file_format::FileFormat;
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::listing::{ListingTable, ListingTableConfig, ListingTableUrl};
use datafusion::datasource::physical_plan::{FileMeta, FileOpenFuture, FileOpener};
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::context::SessionState;
use futures::StreamExt;
use futures::stream::BoxStream;
use object_store::ObjectStore;

use crate::file_format::{GeoParquetFormat, GeoParquetFormatOptions};

/// Builder for creating `GeoParquet` data sources.
pub struct GeoParquetSourceBuilder {
    options: GeoParquetFormatOptions,
}

impl GeoParquetSourceBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            options: GeoParquetFormatOptions::default(),
        }
    }

    #[must_use]
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.options.batch_size = batch_size;
        self
    }

    #[must_use]
    pub fn with_geometry_column_name(mut self, name: impl Into<String>) -> Self {
        self.options.geometry_column_name = name.into();
        self
    }

    #[must_use]
    pub fn build(self) -> GeoParquetFormatOptions {
        self.options
    }
}

impl Default for GeoParquetSourceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a `GeoParquet` table provider for the given path and options.
pub async fn create_geoparquet_table_provider(
    state: &SessionState,
    path: &str,
    options: GeoParquetFormatOptions,
) -> Result<Arc<dyn TableProvider>> {
    let table_url = ListingTableUrl::parse(path)?;
    let format = Arc::new(GeoParquetFormat::new(options));
    let file_ext = format.get_ext();

    let config = ListingTableConfig::new(table_url)
        .with_listing_options(
            datafusion::datasource::listing::ListingOptions::new(format)
                .with_file_extension(file_ext),
        )
        .infer_schema(state)
        .await?;

    let table = ListingTable::try_new(config)?;
    Ok(Arc::new(table))
}

/// `GeoParquet` file opener that produces record batches using the geoparquet crate.
#[derive(Clone)]
pub struct GeoParquetOpener {
    options: GeoParquetFormatOptions,
    #[allow(dead_code)]
    schema: SchemaRef,
    projection: Option<Vec<usize>>,
    object_store: Arc<dyn ObjectStore>,
}

impl GeoParquetOpener {
    pub fn new(
        options: GeoParquetFormatOptions,
        schema: SchemaRef,
        projection: Option<Vec<usize>>,
        object_store: Arc<dyn ObjectStore>,
    ) -> Self {
        Self {
            options,
            schema,
            projection,
            object_store,
        }
    }
}

impl FileOpener for GeoParquetOpener {
    fn open(&self, file_meta: FileMeta, _file: PartitionedFile) -> Result<FileOpenFuture> {
        let options = self.options.clone();
        let projection = self.projection.clone();
        let object_store = Arc::clone(&self.object_store);

        Ok(Box::pin(async move {
            let location = file_meta.location();

            // Read the entire file (GeoParquet files are typically compressed and efficient)
            let get_result = object_store.get(location).await?;
            let bytes = get_result.bytes().await?;

            // Use the geoparquet reader API
            use geoarrow_schema::CoordType;
            use geoparquet::reader::{GeoParquetReaderBuilder, GeoParquetRecordBatchReader};
            use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

            // Create a Parquet reader builder from bytes
            let parquet_builder = ParquetRecordBatchReaderBuilder::try_new(bytes)
                .map_err(|e| DataFusionError::External(Box::new(e)))?;

            // Check if GeoParquet metadata exists before consuming the builder
            let maybe_geo_metadata = parquet_builder.geoparquet_metadata();

            // Build the parquet reader (consumes builder)
            let parquet_reader = parquet_builder
                .with_batch_size(options.batch_size)
                .build()
                .map_err(|e| DataFusionError::External(Box::new(e)))?;

            // Convert metadata result and handle appropriately
            let reader: Box<dyn Iterator<Item = Result<RecordBatch>>> = match maybe_geo_metadata {
                Some(Ok(geoparquet_meta)) => {
                    // Rebuild the builder to get the schema (we can't use the consumed one)
                    let get_result2 = object_store.get(location).await?;
                    let bytes2 = get_result2.bytes().await?;
                    let parquet_builder2 = ParquetRecordBatchReaderBuilder::try_new(bytes2)
                        .map_err(|e| DataFusionError::External(Box::new(e)))?;

                    // Get GeoArrow schema
                    let geo_schema = parquet_builder2
                        .geoarrow_schema(
                            &geoparquet_meta,
                            true, // parse_to_native: convert WKB to native geometry types
                            CoordType::Interleaved,
                        )
                        .map_err(|e| DataFusionError::External(Box::new(e)))?;

                    // Wrap with GeoParquet reader and convert errors
                    let geo_reader =
                        GeoParquetRecordBatchReader::try_new(parquet_reader, geo_schema)
                            .map_err(|e| DataFusionError::External(Box::new(e)))?;
                    Box::new(
                        geo_reader.map(|r| r.map_err(|e| DataFusionError::External(Box::new(e)))),
                    )
                },
                _ => {
                    // Fall back to plain parquet reader
                    Box::new(
                        parquet_reader
                            .map(|r| r.map_err(|e| DataFusionError::External(Box::new(e)))),
                    )
                },
            };

            // Read all record batches
            let mut batches = Vec::new();
            for maybe_batch in reader {
                let batch = maybe_batch.map_err(|e| DataFusionError::External(Box::new(e)))?;

                // Apply projection if needed
                let projected_batch = if let Some(ref proj) = projection {
                    let columns: Vec<_> = proj.iter().map(|&i| batch.column(i).clone()).collect();
                    let projected_schema = Arc::new(arrow_schema::Schema::new(
                        proj.iter()
                            .map(|&i| batch.schema().field(i).clone())
                            .collect::<Vec<_>>(),
                    ));
                    RecordBatch::try_new(projected_schema, columns)?
                } else {
                    batch
                };

                batches.push(projected_batch);
            }

            // Create a stream from the batches
            let stream = futures::stream::iter(batches.into_iter().map(Ok));
            Ok(stream.boxed() as BoxStream<'static, Result<RecordBatch>>)
        }))
    }
}

/// `GeoParquet` file source for `DataFusion`'s `FileSource` trait.
#[derive(Debug, Clone)]
pub struct GeoParquetFileSource {
    options: GeoParquetFormatOptions,
}

impl GeoParquetFileSource {
    pub fn new(options: GeoParquetFormatOptions) -> Self {
        Self { options }
    }
}

impl datafusion::datasource::physical_plan::FileSource for GeoParquetFileSource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn create_file_opener(
        &self,
        _store: Arc<dyn ObjectStore>,
        _config: &datafusion::datasource::physical_plan::FileScanConfig,
        _partition: usize,
    ) -> Arc<dyn FileOpener> {
        // Not implemented - we use GeoParquetOpener directly
        unimplemented!("Use GeoParquetOpener directly instead")
    }

    fn with_batch_size(
        &self,
        batch_size: usize,
    ) -> Arc<dyn datafusion::datasource::physical_plan::FileSource> {
        let mut new_options = self.options.clone();
        new_options.batch_size = batch_size;
        Arc::new(Self::new(new_options))
    }

    fn with_schema(
        &self,
        _schema: SchemaRef,
    ) -> Arc<dyn datafusion::datasource::physical_plan::FileSource> {
        Arc::new(self.clone())
    }

    fn with_projection(
        &self,
        _config: &datafusion::datasource::physical_plan::FileScanConfig,
    ) -> Arc<dyn datafusion::datasource::physical_plan::FileSource> {
        Arc::new(self.clone())
    }

    fn with_statistics(
        &self,
        _stats: datafusion_common::Statistics,
    ) -> Arc<dyn datafusion::datasource::physical_plan::FileSource> {
        Arc::new(self.clone())
    }

    fn metrics(&self) -> &datafusion::physical_plan::metrics::ExecutionPlanMetricsSet {
        unimplemented!("Metrics not implemented for GeoParquetFileSource")
    }

    fn statistics(&self) -> Result<datafusion_common::Statistics> {
        Ok(datafusion_common::Statistics::new_unknown(&Arc::new(
            arrow_schema::Schema::empty(),
        )))
    }

    fn file_type(&self) -> &str {
        "GEOPARQUET"
    }
}
