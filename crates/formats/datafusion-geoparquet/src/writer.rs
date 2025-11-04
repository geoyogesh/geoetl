//! `GeoParquet` writer implementation for converting Arrow record batches to `GeoParquet` format

use std::io::Write as IoWrite;

use arrow_array::RecordBatch;
use datafusion_common::{DataFusionError, Result};
use geoparquet::writer::GeoParquetWriterOptions as GpWriterOptions;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

/// Options for `GeoParquet` writing
#[derive(Debug, Clone)]
pub struct GeoParquetWriterOptions {
    /// Name of the geometry column (default: "geometry")
    pub geometry_column_name: String,
    /// Compression codec to use
    pub compression: parquet::basic::Compression,
    /// Row group size
    pub row_group_size: usize,
}

impl Default for GeoParquetWriterOptions {
    fn default() -> Self {
        Self {
            geometry_column_name: "geometry".to_string(),
            compression: parquet::basic::Compression::SNAPPY,
            row_group_size: 8192,
        }
    }
}

impl GeoParquetWriterOptions {
    /// Create new writer options with defaults
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set geometry column name
    #[must_use]
    pub fn with_geometry_column(mut self, name: impl Into<String>) -> Self {
        self.geometry_column_name = name.into();
        self
    }

    /// Set compression codec
    #[must_use]
    pub fn with_compression(mut self, compression: parquet::basic::Compression) -> Self {
        self.compression = compression;
        self
    }

    /// Set row group size
    #[must_use]
    pub fn with_row_group_size(mut self, size: usize) -> Self {
        self.row_group_size = size;
        self
    }
}

/// Write record batches to `GeoParquet` format
///
/// # Errors
///
/// Returns an error if writing fails or if the geometry column is not found
pub fn write_geoparquet<W: IoWrite + Send>(
    writer: &mut W,
    batches: &[RecordBatch],
    options: &GeoParquetWriterOptions,
) -> Result<()> {
    if batches.is_empty() {
        return Ok(());
    }

    let schema = batches[0].schema();

    // Find geometry column index
    let geom_idx = schema
        .fields()
        .iter()
        .position(|f| f.name() == &options.geometry_column_name);

    if geom_idx.is_none() {
        return Err(DataFusionError::Plan(format!(
            "Geometry column '{}' not found in schema",
            options.geometry_column_name
        )));
    }

    // Create writer options for geoparquet
    let gp_opts = GpWriterOptions::default();

    // Create encoder to convert GeoArrow batches to Parquet-compatible format
    let mut encoder =
        geoparquet::writer::GeoParquetRecordBatchEncoder::try_new(schema.as_ref(), &gp_opts)
            .map_err(|e| DataFusionError::External(Box::new(e)))?;

    // Get the target schema for Parquet (WKB-encoded geometries)
    let target_schema = encoder.target_schema();

    // Set up Parquet writer properties
    let props = WriterProperties::builder()
        .set_compression(options.compression)
        .set_max_row_group_size(options.row_group_size)
        .build();

    // Encode all batches first (since into_keyvalue consumes encoder)
    let mut encoded_batches = Vec::new();
    for batch in batches {
        let encoded_batch = encoder
            .encode_record_batch(batch)
            .map_err(|e| DataFusionError::External(Box::new(e)))?;
        encoded_batches.push(encoded_batch);
    }

    // Create Parquet writer
    let mut arrow_writer = ArrowWriter::try_new(writer, target_schema.clone(), Some(props))
        .map_err(|e| DataFusionError::External(Box::new(e)))?;

    // Write metadata (consumes encoder)
    let metadata_kv = encoder
        .into_keyvalue()
        .map_err(|e| DataFusionError::External(Box::new(e)))?;
    arrow_writer.append_key_value_metadata(metadata_kv);

    // Write all encoded batches
    for encoded_batch in &encoded_batches {
        arrow_writer
            .write(encoded_batch)
            .map_err(|e| DataFusionError::External(Box::new(e)))?;
    }

    // Finish writing
    arrow_writer
        .finish()
        .map_err(|e| DataFusionError::External(Box::new(e)))?;

    Ok(())
}

/// Write record batches to `GeoParquet` format in memory
///
/// # Errors
///
/// Returns an error if writing fails or if the geometry column is not found
pub fn write_geoparquet_to_bytes(
    batches: &[RecordBatch],
    options: &GeoParquetWriterOptions,
) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    write_geoparquet(&mut buffer, batches, options)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_array::{Float64Array, Int64Array};
    use arrow_schema::{DataType, Field, Schema};
    use std::sync::Arc;

    #[test]
    fn test_write_geoparquet_empty() {
        let batches: Vec<RecordBatch> = vec![];
        let options = GeoParquetWriterOptions::default();
        let mut buffer = Vec::new();

        let result = write_geoparquet(&mut buffer, &batches, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_geoparquet_to_bytes() {
        // Create a simple schema with a geometry field
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("value", DataType::Float64, true),
        ]));

        // Create a batch
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(Int64Array::from(vec![1, 2, 3])),
                Arc::new(Float64Array::from(vec![Some(1.0), None, Some(3.0)])),
            ],
        )
        .unwrap();

        let options = GeoParquetWriterOptions::default().with_geometry_column("geometry");

        // This will fail because we don't have a geometry column, but tests the API
        let result = write_geoparquet_to_bytes(&[batch], &options);
        assert!(result.is_err());
    }
}
