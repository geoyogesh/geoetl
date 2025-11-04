//! Factory implementation for `GeoParquet` format support.
//!
//! This module implements the `FormatFactory` trait to integrate `GeoParquet`
//! with the dynamic driver registry system.

use anyhow::Result;
use async_trait::async_trait;
use datafusion::datasource::TableProvider;
use datafusion::execution::context::SessionState;
use datafusion::physical_plan::ExecutionPlan;
use geoetl_core_common::{
    DataReader, DataWriter, Driver, FormatFactory, FormatOptions, SupportStatus,
};
use std::sync::Arc;

use crate::{GeoParquetFormatOptions, file_source};

/// `GeoParquet` format options wrapper for the factory system.
impl FormatOptions for GeoParquetFormatOptions {
    fn as_any(&self) -> Box<dyn std::any::Any + Send> {
        Box::new(self.clone())
    }
}

/// Reader implementation for `GeoParquet` format.
struct GeoParquetReader;

#[async_trait]
impl DataReader for GeoParquetReader {
    async fn create_table_provider(
        &self,
        state: &SessionState,
        path: &str,
        options: Box<dyn std::any::Any + Send>,
    ) -> Result<Arc<dyn TableProvider>> {
        let geoparquet_options = options
            .downcast::<GeoParquetFormatOptions>()
            .map_err(|_| anyhow::anyhow!("Invalid options type for GeoParquet reader"))?;

        let table =
            file_source::create_geoparquet_table_provider(state, path, *geoparquet_options).await?;
        Ok(table)
    }
}

/// Writer implementation for `GeoParquet` format.
struct GeoParquetWriter;

#[async_trait]
impl DataWriter for GeoParquetWriter {
    async fn create_writer_plan(
        &self,
        _input: Arc<dyn ExecutionPlan>,
        _path: &str,
        _options: Box<dyn std::any::Any + Send>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        // TODO: Implement writer plan creation
        // This requires creating a GeoParquetSink with FileSinkConfig
        Err(anyhow::anyhow!(
            "GeoParquet writer not yet implemented in factory"
        ))
    }

    fn create_writer_options(&self, geometry_column: &str) -> Box<dyn std::any::Any + Send> {
        // GeoParquet writer options with configured geometry column
        let options =
            crate::writer::GeoParquetWriterOptions::default().with_geometry_column(geometry_column);
        Box::new(options)
    }

    fn write_batches(
        &self,
        path: &str,
        batches: &[datafusion::arrow::array::RecordBatch],
        options: Box<dyn std::any::Any + Send>,
    ) -> Result<()> {
        use crate::writer::write_geoparquet;

        // Downcast options to GeoParquetWriterOptions
        let boxed_options = options
            .downcast::<crate::writer::GeoParquetWriterOptions>()
            .map_err(|_| anyhow::anyhow!("Invalid options type for GeoParquet writer"))?;
        let writer_options = *boxed_options;

        // Write to file
        let mut output_file = std::fs::File::create(path)
            .map_err(|e| anyhow::anyhow!("Failed to create output file: {e}"))?;

        write_geoparquet(&mut output_file, batches, &writer_options)?;

        Ok(())
    }
}

/// Factory for creating `GeoParquet` readers and writers.
pub struct GeoParquetFormatFactory;

impl FormatFactory for GeoParquetFormatFactory {
    fn driver(&self) -> Driver {
        Driver::new(
            "GeoParquet",
            "GeoParquet",
            SupportStatus::Supported,
            SupportStatus::Supported,
            SupportStatus::Supported,
        )
    }

    fn create_reader(&self) -> Option<Arc<dyn DataReader>> {
        Some(Arc::new(GeoParquetReader))
    }

    fn create_writer(&self) -> Option<Arc<dyn DataWriter>> {
        Some(Arc::new(GeoParquetWriter))
    }

    fn create_file_format(
        &self,
        geometry_column: &str,
    ) -> Option<Arc<dyn datafusion::datasource::file_format::FileFormat>> {
        // Create GeoParquet file format for streaming execution
        let options = crate::file_format::GeoParquetFormatOptions::default()
            .with_geometry_column_name(geometry_column);
        let format = crate::file_format::GeoParquetFormat::new(options);
        Some(Arc::new(format))
    }
}

/// Registers the `GeoParquet` format with the global driver registry.
///
/// This is called by `geoetl-core` during initialization.
pub fn register_geoparquet_format() {
    let registry = geoetl_core_common::driver_registry();
    registry.register(Arc::new(GeoParquetFormatFactory));
}
