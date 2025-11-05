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
use futures::stream::{BoxStream, StreamExt, TryStreamExt};
use geoarrow_schema::CoordType;
use geoparquet::metadata::GeoParquetMetadata;
use geoparquet::reader::{GeoParquetReaderBuilder, GeoParquetRecordBatchStream};
use object_store::{ObjectStore, path::Path as ObjectPath};
use parquet::arrow::async_reader::{ParquetObjectReader, ParquetRecordBatchStreamBuilder};

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
            let object_path = ObjectPath::from(location.as_ref());

            // 1. Use ParquetObjectReader to create an adapter for streaming reads
            let reader = ParquetObjectReader::new(Arc::clone(&object_store), object_path);

            // 2. Build a ParquetRecordBatchStream for async reading
            // This reads metadata without reading actual data
            let mut builder = ParquetRecordBatchStreamBuilder::new(reader)
                .await
                .map_err(|e| DataFusionError::External(Box::new(e)))?;

            // Set batch size
            builder = builder.with_batch_size(options.batch_size);

            // Apply projection if provided
            if let Some(ref proj) = projection {
                let projection_mask = parquet::arrow::ProjectionMask::roots(
                    builder.parquet_schema(),
                    proj.iter().copied(),
                );
                builder = builder.with_projection(projection_mask);
            }

            // 3. Check for GeoParquet metadata to determine if this is a GeoParquet file
            let maybe_geo_metadata = builder.geoparquet_metadata();

            // 4. Build the core data stream and handle GeoParquet decoding
            let stream: BoxStream<'static, Result<RecordBatch>> = if let Some(Ok(gp_meta)) =
                maybe_geo_metadata
            {
                // 5. Create a decoding stream for GeoParquet with native geometry types
                // Get the GeoArrow schema (what we want after decoding)
                let geo_schema = builder
                    .geoarrow_schema(
                        &gp_meta,
                        true, // parse_to_native: convert WKB to native geometry types
                        CoordType::Interleaved,
                    )
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;

                // Build the parquet stream
                let parquet_stream = builder
                    .build()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;

                // Wrap with GeoParquetRecordBatchStream to decode WKB on the fly
                let geo_stream = GeoParquetRecordBatchStream::try_new(parquet_stream, geo_schema)
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;

                geo_stream
                    .map(|result| result.map_err(|e| DataFusionError::External(Box::new(e))))
                    .boxed()
            } else {
                // 6. Handle plain Parquet - just pass through without decoding
                let parquet_stream = builder
                    .build()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;

                parquet_stream
                    .map(|result| result.map_err(|e| DataFusionError::External(Box::new(e))))
                    .boxed()
            };

            // 7. Return the final streaming result
            Ok(stream)
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
