//! Integration tests for `GeoJSON` writer functionality

use std::fs;
use std::sync::Arc;

use arrow_array::{ArrayRef, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use datafusion::datasource::listing::ListingTableUrl;
use datafusion::datasource::physical_plan::{FileGroup, FileSinkConfig};
use datafusion::datasource::sink::DataSink;
use datafusion::logical_expr::dml::InsertOp;
use datafusion::physical_plan::stream::RecordBatchStreamAdapter;
use datafusion_execution::object_store::ObjectStoreUrl;
use datafusion_execution::{SendableRecordBatchStream, TaskContext};
use datafusion_geojson::{GeoJsonSink, GeoJsonWriterOptions, write_geojson_to_bytes};
use futures::stream;
use tempfile::TempDir;

#[ignore = "Requires proper object store integration"]
#[tokio::test]
async fn test_geojson_sink_write_all() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().to_str().unwrap();

    // Pre-create the directory since sink expects it to exist
    fs::create_dir_all(output_path).unwrap();

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("geometry", DataType::Utf8, true),
    ]));

    // Create test data
    let id_array: ArrayRef = Arc::new(Int64Array::from(vec![1, 2, 3]));
    let name_array: ArrayRef = Arc::new(StringArray::from(vec![
        Some("Alice"),
        Some("Bob"),
        Some("Charlie"),
    ]));
    let geom_array: ArrayRef = Arc::new(StringArray::from(vec![
        Some("POINT(0 0)"),
        Some("POINT(1 1)"),
        Some("POINT(2 2)"),
    ]));

    let batch =
        RecordBatch::try_new(schema.clone(), vec![id_array, name_array, geom_array]).unwrap();

    // Create file sink config
    let config = FileSinkConfig {
        original_url: format!("file://{output_path}/output.geojson"),
        object_store_url: ObjectStoreUrl::local_filesystem(),
        file_group: FileGroup::default(),
        table_paths: vec![ListingTableUrl::parse(format!("file://{output_path}")).unwrap()],
        output_schema: schema.clone(),
        table_partition_cols: vec![],
        insert_op: InsertOp::Append,
        keep_partition_by_columns: false,
        file_extension: "geojson".to_string(),
    };

    let writer_options = GeoJsonWriterOptions::default();
    let sink = GeoJsonSink::new(config, writer_options);

    // Create a stream from the batch
    let stream: SendableRecordBatchStream = Box::pin(RecordBatchStreamAdapter::new(
        schema.clone(),
        stream::iter(vec![Ok(batch)]),
    ));

    // Write data
    let context = Arc::new(TaskContext::default());
    let row_count = sink.write_all(stream, &context).await.unwrap();

    assert_eq!(row_count, 3);

    // Verify file was created and contains data
    let file_path = format!("{output_path}/data.geojson");
    assert!(fs::metadata(&file_path).is_ok());

    let contents = fs::read_to_string(&file_path).unwrap();
    assert!(contents.contains("\"type\":\"FeatureCollection\""));
    assert!(contents.contains("\"features\""));
}

#[test]
fn test_geojson_writer_options_builder() {
    let options = GeoJsonWriterOptions::new()
        .with_geometry_column("geom")
        .with_feature_collection(false)
        .with_pretty_print(true);

    assert_eq!(options.geometry_column_name, "geom");
    assert!(!options.feature_collection);
    assert!(options.pretty_print);
}

#[test]
fn test_write_geojson_feature_collection() {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("value", DataType::Int64, false),
        Field::new("geometry", DataType::Utf8, true),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int64Array::from(vec![1, 2])) as ArrayRef,
            Arc::new(Int64Array::from(vec![100, 200])) as ArrayRef,
            Arc::new(StringArray::from(vec![Some("geom1"), Some("geom2")])) as ArrayRef,
        ],
    )
    .unwrap();

    let options = GeoJsonWriterOptions::default();
    let result = write_geojson_to_bytes(&[batch], &options).unwrap();
    let json_str = String::from_utf8(result).unwrap();

    assert!(json_str.contains("\"type\":\"FeatureCollection\""));
    assert!(json_str.contains("\"features\""));
    assert!(json_str.contains("\"id\":1"));
    assert!(json_str.contains("\"value\":100"));
}

#[test]
fn test_write_geojson_newline_delimited() {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("geometry", DataType::Utf8, true),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int64Array::from(vec![1, 2])) as ArrayRef,
            Arc::new(StringArray::from(vec![Some("geom1"), Some("geom2")])) as ArrayRef,
        ],
    )
    .unwrap();

    let options = GeoJsonWriterOptions::default().with_feature_collection(false);
    let result = write_geojson_to_bytes(&[batch], &options).unwrap();
    let json_str = String::from_utf8(result).unwrap();

    let lines: Vec<&str> = json_str.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("\"type\":\"Feature\""));
    assert!(lines[1].contains("\"type\":\"Feature\""));
}

#[test]
fn test_geojson_sink_config_accessors() {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("geometry", DataType::Utf8, true),
    ]));

    let config = FileSinkConfig {
        original_url: "file:///tmp/test.geojson".to_string(),
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
    let sink = GeoJsonSink::new(config.clone(), writer_options.clone());

    assert_eq!(sink.config().original_url, config.original_url);
    assert_eq!(
        sink.writer_options().geometry_column_name,
        writer_options.geometry_column_name
    );
}

#[test]
fn test_write_geojson_pretty_print() {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("geometry", DataType::Utf8, true),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int64Array::from(vec![1])) as ArrayRef,
            Arc::new(StringArray::from(vec![Some("geom")])) as ArrayRef,
        ],
    )
    .unwrap();

    let options = GeoJsonWriterOptions::default().with_pretty_print(true);
    let result = write_geojson_to_bytes(&[batch], &options).unwrap();
    let json_str = String::from_utf8(result).unwrap();

    // Pretty printed JSON should have newlines and indentation
    assert!(json_str.contains('\n'));
    assert!(json_str.contains("  ")); // Indentation
}

#[tokio::test]
async fn test_geojson_sink_overwrite_mode() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("output.geojson");
    let output_path = output_file.to_str().unwrap();

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("geometry", DataType::Utf8, true),
    ]));

    // First write: create initial file with data
    let id_array: ArrayRef = Arc::new(Int64Array::from(vec![1, 2]));
    let name_array: ArrayRef = Arc::new(StringArray::from(vec![Some("first"), Some("second")]));
    let geom_array: ArrayRef = Arc::new(StringArray::from(vec![
        Some("POINT(0 0)"),
        Some("POINT(1 1)"),
    ]));
    let batch1 =
        RecordBatch::try_new(schema.clone(), vec![id_array, name_array, geom_array]).unwrap();

    let config_overwrite = FileSinkConfig {
        original_url: output_path.to_string(),
        object_store_url: ObjectStoreUrl::local_filesystem(),
        file_group: FileGroup::default(),
        table_paths: vec![ListingTableUrl::parse(format!("file://{output_path}")).unwrap()],
        output_schema: schema.clone(),
        table_partition_cols: vec![],
        insert_op: InsertOp::Overwrite,
        keep_partition_by_columns: false,
        file_extension: "geojson".to_string(),
    };

    let writer_options = GeoJsonWriterOptions::default().with_feature_collection(true);
    let sink1 = GeoJsonSink::new(config_overwrite.clone(), writer_options.clone());
    let stream1: SendableRecordBatchStream = Box::pin(RecordBatchStreamAdapter::new(
        schema.clone(),
        stream::iter(vec![Ok(batch1)]),
    ));

    let context = Arc::new(TaskContext::default());
    let row_count1 = sink1.write_all(stream1, &context).await.unwrap();
    assert_eq!(row_count1, 2);

    // Verify initial content
    let contents1 = fs::read_to_string(output_path).unwrap();
    assert!(contents1.contains("\"type\":\"FeatureCollection\""));
    assert!(contents1.contains("\"id\":1"));
    assert!(contents1.contains("\"name\":\"first\""));
    assert!(contents1.contains("\"id\":2"));
    assert!(contents1.contains("\"name\":\"second\""));

    // Second write: overwrite with different data
    let id_array2: ArrayRef = Arc::new(Int64Array::from(vec![10, 20, 30]));
    let name_array2: ArrayRef = Arc::new(StringArray::from(vec![
        Some("new1"),
        Some("new2"),
        Some("new3"),
    ]));
    let geom_array2: ArrayRef = Arc::new(StringArray::from(vec![
        Some("POINT(10 10)"),
        Some("POINT(20 20)"),
        Some("POINT(30 30)"),
    ]));
    let batch2 =
        RecordBatch::try_new(schema.clone(), vec![id_array2, name_array2, geom_array2]).unwrap();

    let sink2 = GeoJsonSink::new(config_overwrite, writer_options);
    let stream2: SendableRecordBatchStream = Box::pin(RecordBatchStreamAdapter::new(
        schema.clone(),
        stream::iter(vec![Ok(batch2)]),
    ));

    let row_count2 = sink2.write_all(stream2, &context).await.unwrap();
    assert_eq!(row_count2, 3);

    // Verify file was overwritten (not appended)
    let contents2 = fs::read_to_string(output_path).unwrap();
    assert!(contents2.contains("\"type\":\"FeatureCollection\""));
    assert!(contents2.contains("\"id\":10"));
    assert!(contents2.contains("\"name\":\"new1\""));
    assert!(contents2.contains("\"id\":20"));
    assert!(contents2.contains("\"name\":\"new2\""));
    assert!(contents2.contains("\"id\":30"));
    assert!(contents2.contains("\"name\":\"new3\""));

    // Old data should NOT be present (be specific to avoid substring matches)
    assert!(
        !contents2.contains("\"id\":1,"),
        "Old data id:1 should not be present"
    );
    assert!(
        !contents2.contains("\"id\":1}"),
        "Old data id:1 should not be present"
    );
    assert!(
        !contents2.contains("\"name\":\"first\""),
        "Old data name:first should not be present"
    );
    assert!(
        !contents2.contains("\"id\":2,"),
        "Old data id:2 should not be present"
    );
    assert!(
        !contents2.contains("\"id\":2}"),
        "Old data id:2 should not be present"
    );
    assert!(
        !contents2.contains("\"name\":\"second\""),
        "Old data name:second should not be present"
    );
}

#[tokio::test]
async fn test_geojson_sink_append_mode() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("output_append.geojson");
    let output_path = output_file.to_str().unwrap();

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("geometry", DataType::Utf8, true),
    ]));

    // First write: create initial file
    let id_array: ArrayRef = Arc::new(Int64Array::from(vec![1, 2]));
    let name_array: ArrayRef = Arc::new(StringArray::from(vec![Some("first"), Some("second")]));
    let geom_array: ArrayRef = Arc::new(StringArray::from(vec![
        Some("POINT(0 0)"),
        Some("POINT(1 1)"),
    ]));
    let batch1 =
        RecordBatch::try_new(schema.clone(), vec![id_array, name_array, geom_array]).unwrap();

    let config_append = FileSinkConfig {
        original_url: output_path.to_string(),
        object_store_url: ObjectStoreUrl::local_filesystem(),
        file_group: FileGroup::default(),
        table_paths: vec![ListingTableUrl::parse(format!("file://{output_path}")).unwrap()],
        output_schema: schema.clone(),
        table_partition_cols: vec![],
        insert_op: InsertOp::Append,
        keep_partition_by_columns: false,
        file_extension: "geojson".to_string(),
    };

    let writer_options = GeoJsonWriterOptions::default().with_feature_collection(false); // Use newline-delimited for append
    let sink1 = GeoJsonSink::new(config_append.clone(), writer_options.clone());
    let stream1: SendableRecordBatchStream = Box::pin(RecordBatchStreamAdapter::new(
        schema.clone(),
        stream::iter(vec![Ok(batch1)]),
    ));

    let context = Arc::new(TaskContext::default());
    let row_count1 = sink1.write_all(stream1, &context).await.unwrap();
    assert_eq!(row_count1, 2);

    // Second write: append more data
    let id_array2: ArrayRef = Arc::new(Int64Array::from(vec![3, 4]));
    let name_array2: ArrayRef = Arc::new(StringArray::from(vec![Some("third"), Some("fourth")]));
    let geom_array2: ArrayRef = Arc::new(StringArray::from(vec![
        Some("POINT(2 2)"),
        Some("POINT(3 3)"),
    ]));
    let batch2 =
        RecordBatch::try_new(schema.clone(), vec![id_array2, name_array2, geom_array2]).unwrap();

    let sink2 = GeoJsonSink::new(config_append, writer_options);
    let stream2: SendableRecordBatchStream = Box::pin(RecordBatchStreamAdapter::new(
        schema.clone(),
        stream::iter(vec![Ok(batch2)]),
    ));

    let row_count2 = sink2.write_all(stream2, &context).await.unwrap();
    assert_eq!(row_count2, 2);

    // Verify both sets of data are present (appended, not overwritten)
    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("\"id\":1"));
    assert!(contents.contains("\"name\":\"first\""));
    assert!(contents.contains("\"id\":2"));
    assert!(contents.contains("\"name\":\"second\""));
    assert!(contents.contains("\"id\":3"));
    assert!(contents.contains("\"name\":\"third\""));
    assert!(contents.contains("\"id\":4"));
    assert!(contents.contains("\"name\":\"fourth\""));

    // Should have 4 lines (newline-delimited format)
    let line_count = contents.lines().count();
    assert_eq!(line_count, 4);
}
