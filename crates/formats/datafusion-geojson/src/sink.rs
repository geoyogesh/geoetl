//! `GeoJSON` Data Sink implementation for writing data to `GeoJSON` files

use std::io::Write;
use std::sync::Arc;

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::datasource::physical_plan::FileSinkConfig;
use datafusion::datasource::sink::DataSink;
use datafusion::physical_plan::metrics::MetricsSet;
use datafusion::physical_plan::{DisplayAs, DisplayFormatType, ExecutionPlan};
use datafusion_common::{DataFusionError, Result};
use datafusion_execution::{SendableRecordBatchStream, TaskContext};
use datafusion_physical_expr::LexRequirement;
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

/// `GeoJSON` writer physical execution plan
#[derive(Debug)]
pub struct GeoJsonWriterExec {
    input: Arc<dyn ExecutionPlan>,
    sink: Arc<GeoJsonSink>,
    _order_requirements: Option<LexRequirement>,
}

impl GeoJsonWriterExec {
    /// Create a new `GeoJSON` writer execution plan
    pub fn new(
        input: Arc<dyn ExecutionPlan>,
        sink: Arc<GeoJsonSink>,
        order_requirements: Option<LexRequirement>,
    ) -> Self {
        Self {
            input,
            sink,
            _order_requirements: order_requirements,
        }
    }
}

impl DisplayAs for GeoJsonWriterExec {
    fn fmt_as(&self, _t: DisplayFormatType, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeoJsonWriterExec")
    }
}

impl std::fmt::Display for GeoJsonWriterExec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeoJsonWriterExec")
    }
}

impl ExecutionPlan for GeoJsonWriterExec {
    fn name(&self) -> &'static str {
        "GeoJsonWriterExec"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn properties(&self) -> &datafusion::physical_plan::PlanProperties {
        self.input.properties()
    }

    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![&self.input]
    }

    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        if children.len() != 1 {
            return Err(DataFusionError::Internal(
                "GeoJsonWriterExec requires exactly one child".to_string(),
            ));
        }

        #[allow(clippy::used_underscore_binding)]
        Ok(Arc::new(Self {
            input: Arc::clone(&children[0]),
            sink: Arc::clone(&self.sink),
            _order_requirements: self._order_requirements.clone(),
        }))
    }

    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        if partition != 0 {
            return Err(DataFusionError::Internal(
                "GeoJsonWriterExec only supports single partition".to_string(),
            ));
        }

        // Execute input and get stream
        let input_stream = self.input.execute(partition, Arc::clone(&context))?;

        // For now, we'll return the input stream
        Ok(input_stream)
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

    #[test]
    fn test_geojson_writer_exec_creation() {
        use datafusion::physical_plan::empty::EmptyExec;

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

        let sink = Arc::new(GeoJsonSink::new(config, GeoJsonWriterOptions::default()));
        let input = Arc::new(EmptyExec::new(schema.clone())) as Arc<dyn ExecutionPlan>;
        let exec = GeoJsonWriterExec::new(input, sink, None);

        assert_eq!(exec.name(), "GeoJsonWriterExec");
    }

    #[test]
    fn test_geojson_writer_exec_as_any() {
        use datafusion::physical_plan::empty::EmptyExec;

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

        let sink = Arc::new(GeoJsonSink::new(config, GeoJsonWriterOptions::default()));
        let input = Arc::new(EmptyExec::new(schema.clone())) as Arc<dyn ExecutionPlan>;
        let exec = GeoJsonWriterExec::new(input, sink, None);

        assert!(exec.as_any().is::<GeoJsonWriterExec>());
    }

    #[test]
    fn test_geojson_writer_exec_children() {
        use datafusion::physical_plan::empty::EmptyExec;

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

        let sink = Arc::new(GeoJsonSink::new(config, GeoJsonWriterOptions::default()));
        let input = Arc::new(EmptyExec::new(schema.clone())) as Arc<dyn ExecutionPlan>;
        let exec = GeoJsonWriterExec::new(input, sink, None);

        let children = exec.children();
        assert_eq!(children.len(), 1);
    }

    #[test]
    fn test_geojson_writer_exec_with_new_children() {
        use datafusion::physical_plan::empty::EmptyExec;

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

        let sink = Arc::new(GeoJsonSink::new(config, GeoJsonWriterOptions::default()));
        let input = Arc::new(EmptyExec::new(schema.clone())) as Arc<dyn ExecutionPlan>;
        let exec = Arc::new(GeoJsonWriterExec::new(input.clone(), sink, None));

        // Test with one child
        let new_exec = exec.clone().with_new_children(vec![input.clone()]).unwrap();
        assert_eq!(new_exec.children().len(), 1);
    }

    #[test]
    fn test_geojson_writer_exec_with_new_children_error() {
        use datafusion::physical_plan::empty::EmptyExec;

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

        let sink = Arc::new(GeoJsonSink::new(config, GeoJsonWriterOptions::default()));
        let input = Arc::new(EmptyExec::new(schema.clone())) as Arc<dyn ExecutionPlan>;
        let exec = Arc::new(GeoJsonWriterExec::new(input.clone(), sink, None));

        // Test with wrong number of children
        let result = exec.clone().with_new_children(vec![]);
        assert!(result.is_err());

        let result = exec
            .clone()
            .with_new_children(vec![input.clone(), input.clone()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_geojson_writer_exec_execute_error() {
        use datafusion::physical_plan::empty::EmptyExec;

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

        let sink = Arc::new(GeoJsonSink::new(config, GeoJsonWriterOptions::default()));
        let input = Arc::new(EmptyExec::new(schema.clone())) as Arc<dyn ExecutionPlan>;
        let exec = GeoJsonWriterExec::new(input, sink, None);

        let context = Arc::new(TaskContext::default());

        // Test with invalid partition
        let result = exec.execute(1, context);
        assert!(result.is_err());
    }

    #[test]
    fn test_geojson_writer_exec_display() {
        use datafusion::physical_plan::empty::EmptyExec;

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

        let sink = Arc::new(GeoJsonSink::new(config, GeoJsonWriterOptions::default()));
        let input = Arc::new(EmptyExec::new(schema.clone())) as Arc<dyn ExecutionPlan>;
        let exec = GeoJsonWriterExec::new(input, sink, None);

        assert_eq!(format!("{exec}"), "GeoJsonWriterExec");
    }
}
