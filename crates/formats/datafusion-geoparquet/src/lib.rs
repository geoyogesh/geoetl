//! `DataFusion` `GeoParquet` Integration
//!
//! This crate provides `GeoParquet` file format support for Apache `DataFusion`.
//! It allows reading and querying `GeoParquet` files using `DataFusion`'s SQL and `DataFrame` APIs.
//!
//! `GeoParquet` is a columnar storage format for geospatial data based on Apache Parquet,
//! optimized for efficient spatial data queries and analytics.
//!
//! # Architecture
//!
//! The crate is organized into several modules:
//! - `file_format` - `GeoParquet` format configuration and options
//! - `file_source` - `GeoParquet` source builders and table providers
//! - `physical_exec` - Physical execution plan for reading `GeoParquet`
//! - `writer` - Writer implementation for creating `GeoParquet` files
//! - `sink` - `DataSink` implementation for streaming writes
//! - `factory` - Format factory for integration with driver registry
//!
//! # Example
//!
//! ```no_run
//! use datafusion::prelude::*;
//! use datafusion_geoparquet::SessionContextGeoParquetExt;
//!
//! #[tokio::main]
//! async fn main() -> datafusion::error::Result<()> {
//!     let ctx = SessionContext::new();
//!
//!     // Register a GeoParquet file as a table
//!     ctx.register_geoparquet_file("my_table", "path/to/file.parquet").await?;
//!
//!     // Query the GeoParquet file
//!     let df = ctx.sql("SELECT * FROM my_table").await?;
//!     df.show().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod factory;
mod file_format;
mod file_source;
mod physical_exec;
mod sink;
mod writer;

// Re-export public types
pub use factory::register_geoparquet_format;
pub use file_format::GeoParquetFormatOptions;
pub use file_source::GeoParquetSourceBuilder;
pub use sink::GeoParquetSink;
pub use writer::{GeoParquetWriterOptions, write_geoparquet, write_geoparquet_to_bytes};

use datafusion::prelude::*;
use datafusion_common::Result;

/// Extension trait for `SessionContext` to add convenient `GeoParquet` registration methods
#[allow(async_fn_in_trait)]
pub trait SessionContextGeoParquetExt {
    /// Register a `GeoParquet` file as a table with default options
    ///
    /// # Example
    ///
    /// ```no_run
    /// use datafusion::prelude::*;
    /// use datafusion_geoparquet::SessionContextGeoParquetExt;
    ///
    /// # async fn example() -> datafusion_common::Result<()> {
    /// let ctx = SessionContext::new();
    /// ctx.register_geoparquet_file("buildings", "data/buildings.parquet").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn register_geoparquet_file(&self, name: &str, path: &str) -> Result<()>;

    /// Register a `GeoParquet` file with custom options
    ///
    /// # Example
    ///
    /// ```no_run
    /// use datafusion::prelude::*;
    /// use datafusion_geoparquet::{SessionContextGeoParquetExt, GeoParquetFormatOptions};
    ///
    /// # async fn example() -> datafusion_common::Result<()> {
    /// let ctx = SessionContext::new();
    /// let options = GeoParquetFormatOptions::new()
    ///     .with_batch_size(16384);
    ///
    /// ctx.register_geoparquet_with_options("data", "file.parquet", options).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn register_geoparquet_with_options(
        &self,
        name: &str,
        path: &str,
        options: GeoParquetFormatOptions,
    ) -> Result<()>;

    /// Read a `GeoParquet` file into a `DataFrame` with default options
    ///
    /// # Example
    ///
    /// ```no_run
    /// use datafusion::prelude::*;
    /// use datafusion_geoparquet::SessionContextGeoParquetExt;
    ///
    /// # async fn example() -> datafusion_common::Result<()> {
    /// let ctx = SessionContext::new();
    /// let df = ctx.read_geoparquet_file("data/buildings.parquet").await?;
    /// df.show().await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn read_geoparquet_file(&self, path: &str) -> Result<DataFrame>;

    /// Read a `GeoParquet` file into a `DataFrame` with custom options
    ///
    /// # Example
    ///
    /// ```no_run
    /// use datafusion::prelude::*;
    /// use datafusion_geoparquet::{SessionContextGeoParquetExt, GeoParquetFormatOptions};
    ///
    /// # async fn example() -> datafusion_common::Result<()> {
    /// let ctx = SessionContext::new();
    /// let options = GeoParquetFormatOptions::new().with_batch_size(8192);
    /// let df = ctx.read_geoparquet_with_options("data.parquet", options).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn read_geoparquet_with_options(
        &self,
        path: &str,
        options: GeoParquetFormatOptions,
    ) -> Result<DataFrame>;
}

impl SessionContextGeoParquetExt for SessionContext {
    async fn register_geoparquet_file(&self, name: &str, path: &str) -> Result<()> {
        let options = GeoParquetFormatOptions::default();
        self.register_geoparquet_with_options(name, path, options)
            .await
    }

    async fn register_geoparquet_with_options(
        &self,
        name: &str,
        path: &str,
        options: GeoParquetFormatOptions,
    ) -> Result<()> {
        let table =
            file_source::create_geoparquet_table_provider(&self.state(), path, options).await?;
        self.register_table(name, table)?;
        Ok(())
    }

    async fn read_geoparquet_file(&self, path: &str) -> Result<DataFrame> {
        let options = GeoParquetFormatOptions::default();
        self.read_geoparquet_with_options(path, options).await
    }

    async fn read_geoparquet_with_options(
        &self,
        path: &str,
        options: GeoParquetFormatOptions,
    ) -> Result<DataFrame> {
        let table =
            file_source::create_geoparquet_table_provider(&self.state(), path, options).await?;
        self.read_table(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_geoparquet_file() -> Result<()> {
        // This is a placeholder test - actual GeoParquet file creation
        // requires the writer implementation
        Ok(())
    }
}
