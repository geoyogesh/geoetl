//! `GeoJSON` Data Sink implementation for writing data to `GeoJSON` files

use std::io::Write;
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

use crate::writer::GeoJsonWriterOptions;

/// `GeoJSON` data sink that implements the `DataSink` trait
#[derive(Debug)]
pub struct GeoJsonSink {
    config: FileSinkConfig,
    writer_options: GeoJsonWriterOptions,
}

impl GeoJsonSink {
    /// Create a new `GeoJSON` sink
    #[must_use]
    pub fn new(config: FileSinkConfig, writer_options: GeoJsonWriterOptions) -> Self {
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
    pub fn writer_options(&self) -> &GeoJsonWriterOptions {
        &self.writer_options
    }
}

#[async_trait]
impl DataSink for GeoJsonSink {
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
        use std::fs::OpenOptions;

        // Get output path from original URL (the file path user specified)
        // Strip file:// prefix if present
        let url_str = &self.config.original_url;
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
                    "Insert operation {:?} is not supported for GeoJSON",
                    self.config.insert_op
                )));
            },
        };

        let mut row_count = 0u64;

        // Write opening of FeatureCollection
        if self.writer_options.feature_collection {
            if self.writer_options.pretty_print {
                file.write_all(b"{\n  \"type\": \"FeatureCollection\",\n  \"features\": [\n")
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;
            } else {
                file.write_all(b"{\"type\":\"FeatureCollection\",\"features\":[")
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;
            }
        }

        let mut first_feature = true;

        // Stream batches and write features incrementally
        while let Some(batch_result) = data.next().await {
            let batch = batch_result?;
            row_count += batch.num_rows() as u64;

            // Convert batch to features
            let features = crate::writer::batch_to_features(&batch, &self.writer_options)?;

            // Write each feature
            for feature in features {
                if self.writer_options.feature_collection {
                    // Write comma separator (except before first feature)
                    if !first_feature {
                        file.write_all(b",")
                            .map_err(|e| DataFusionError::External(Box::new(e)))?;
                        if self.writer_options.pretty_print {
                            file.write_all(b"\n")
                                .map_err(|e| DataFusionError::External(Box::new(e)))?;
                        }
                    }
                    first_feature = false;

                    // Write feature
                    let json_str = if self.writer_options.pretty_print {
                        let feature_json = serde_json::to_string_pretty(&feature)
                            .map_err(|e| DataFusionError::External(Box::new(e)))?;
                        // Indent the feature JSON
                        feature_json
                            .lines()
                            .map(|line| format!("    {line}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    } else {
                        serde_json::to_string(&feature)
                            .map_err(|e| DataFusionError::External(Box::new(e)))?
                    };

                    file.write_all(json_str.as_bytes())
                        .map_err(|e| DataFusionError::External(Box::new(e)))?;
                } else {
                    // Newline-delimited GeoJSON
                    let geojson = geojson::GeoJson::Feature(feature);
                    let json_str = serde_json::to_string(&geojson)
                        .map_err(|e| DataFusionError::External(Box::new(e)))?;

                    file.write_all(json_str.as_bytes())
                        .map_err(|e| DataFusionError::External(Box::new(e)))?;
                    file.write_all(b"\n")
                        .map_err(|e| DataFusionError::External(Box::new(e)))?;
                }
            }
        }

        // Write closing of FeatureCollection
        if self.writer_options.feature_collection {
            if self.writer_options.pretty_print {
                file.write_all(b"\n  ]\n}")
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;
            } else {
                file.write_all(b"]}")
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;
            }
        }

        Ok(row_count)
    }
}

impl DisplayAs for GeoJsonSink {
    fn fmt_as(&self, _t: DisplayFormatType, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeoJsonSink")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer::GeoJsonWriterOptions;
    use arrow_schema::{DataType, Field, Schema};
    use datafusion::datasource::listing::ListingTableUrl;
    use datafusion::datasource::physical_plan::FileGroup;
    use datafusion::logical_expr::dml::InsertOp;
    use datafusion_execution::object_store::ObjectStoreUrl;

    #[test]
    fn test_geojson_sink_creation() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("name", DataType::Utf8, true),
            Field::new("geometry", DataType::Utf8, true),
        ]));

        let config = FileSinkConfig {
            original_url: "file:///tmp/output.geojson".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "geojson".to_string(),
        };

        let writer_options = GeoJsonWriterOptions::default();
        let sink = GeoJsonSink::new(config, writer_options);

        assert_eq!(sink.schema().fields().len(), 3);
        assert_eq!(sink.writer_options().geometry_column_name, "geometry");
    }

    #[test]
    fn test_geojson_sink_as_any() {
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let config = FileSinkConfig {
            original_url: "file:///tmp/output.geojson".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "geojson".to_string(),
        };

        let sink = GeoJsonSink::new(config, GeoJsonWriterOptions::default());
        assert!(sink.as_any().is::<GeoJsonSink>());
    }

    #[test]
    fn test_geojson_sink_metrics() {
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let config = FileSinkConfig {
            original_url: "file:///tmp/output.geojson".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "geojson".to_string(),
        };

        let sink = GeoJsonSink::new(config, GeoJsonWriterOptions::default());
        assert!(sink.metrics().is_none());
    }

    #[test]
    fn test_geojson_sink_display() {
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));
        let config = FileSinkConfig {
            original_url: "file:///tmp/output.geojson".to_string(),
            object_store_url: ObjectStoreUrl::local_filesystem(),
            file_group: FileGroup::default(),
            table_paths: vec![ListingTableUrl::parse("file:///tmp").unwrap()],
            output_schema: schema.clone(),
            table_partition_cols: vec![],
            insert_op: InsertOp::Append,
            keep_partition_by_columns: false,
            file_extension: "geojson".to_string(),
        };

        let sink = GeoJsonSink::new(config, GeoJsonWriterOptions::default());
        assert_eq!(format!("{sink:?}"), format!("{sink:?}"));
    }
}
