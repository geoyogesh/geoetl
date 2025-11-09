//! End-to-end integration tests for the convert operation
//!
//! These tests verify the complete conversion workflow from file I/O
//! through the driver system to the final output.

use geoetl_core::drivers::{Driver, SupportStatus, find_driver};
use geoetl_core::operations::convert;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

/// Helper to create a sample CSV file with spatial data
fn create_spatial_csv(path: &std::path::Path) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "id,name,wkt,category,value")?;
    writeln!(file, "1,Location A,\"POINT(-74.0060 40.7128)\",retail,100")?;
    writeln!(
        file,
        "2,Location B,\"POINT(-118.2437 34.0522)\",warehouse,250"
    )?;
    writeln!(file, "3,Location C,\"POINT(-87.6298 41.8781)\",office,175")?;
    writeln!(file, "4,Location D,\"POINT(-95.3698 29.7604)\",retail,320")?;
    writeln!(file, "5,Location E,\"POINT(-112.0740 33.4484)\",office,280")?;
    Ok(())
}

/// Helper to create a sample `GeoJSON` file
fn create_sample_geojson(path: &std::path::Path) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(
        file,
        r#"{{
  "type": "FeatureCollection",
  "features": [
    {{
      "type": "Feature",
      "geometry": {{
        "type": "Point",
        "coordinates": [-122.4194, 37.7749]
      }},
      "properties": {{
        "city": "San Francisco",
        "state": "CA",
        "population": 883305,
        "established": 1776
      }}
    }},
    {{
      "type": "Feature",
      "geometry": {{
        "type": "Point",
        "coordinates": [-87.6298, 41.8781]
      }},
      "properties": {{
        "city": "Chicago",
        "state": "IL",
        "population": 2746388,
        "established": 1837
      }}
    }},
    {{
      "type": "Feature",
      "geometry": {{
        "type": "Point",
        "coordinates": [-74.0060, 40.7128]
      }},
      "properties": {{
        "city": "New York",
        "state": "NY",
        "population": 8336817,
        "established": 1624
      }}
    }}
  ]
}}"#
    )?;
    Ok(())
}

#[tokio::test]
async fn test_e2e_csv_to_csv_conversion() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input_data.csv");
    let output_path = temp_dir.path().join("output_data.csv");

    // Create input data
    create_spatial_csv(&input_path).unwrap();

    // Get drivers
    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Perform conversion
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        None, // sql_query
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
    assert!(output_path.exists(), "Output file was not created");

    // Verify output content
    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("id,name,wkt,category,value"));
    assert!(output.contains("Location A"));
    assert!(output.contains("Location B"));
    assert!(output.contains("Location C"));
    assert!(output.contains("POINT(-74.006 40.7128)"));

    // Verify row count
    let line_count = output.lines().count();
    assert_eq!(line_count, 6); // Header + 5 data rows
}

#[tokio::test]
async fn test_e2e_geojson_to_geojson_conversion() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("cities.geojson");
    let output_path = temp_dir.path().join("cities_output.geojson");

    // Create input data
    create_sample_geojson(&input_path).unwrap();

    // Get drivers
    let geojson_driver = find_driver("GeoJSON").expect("GeoJSON driver should exist");

    // Perform conversion
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &geojson_driver,
        &geojson_driver,
        "geometry",
        None,
        None, // sql_query
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
    assert!(output_path.exists(), "Output file was not created");

    // Verify output is valid GeoJSON
    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("FeatureCollection"));
    assert!(output.contains("San Francisco"));
    assert!(output.contains("Chicago"));
    assert!(output.contains("New York"));

    // Verify it has the expected structure
    assert!(output.contains("\"type\""));
    assert!(output.contains("\"features\""));
}

#[tokio::test]
async fn test_e2e_large_csv_conversion() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("large_data.csv");
    let output_path = temp_dir.path().join("large_output.csv");

    // Create a larger CSV file with 1000 rows
    let mut file = File::create(&input_path).unwrap();
    writeln!(file, "id,value,category,wkt").unwrap();
    for i in 1..=1000 {
        let wkt = format!("POINT({i} {i})");
        writeln!(file, "{},{},category_{},\"{}\"", i, i * 10, i % 5, wkt).unwrap();
    }

    // Get drivers
    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Perform conversion
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        None, // sql_query
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
    assert!(output_path.exists(), "Output file was not created");

    // Verify output
    let output = std::fs::read_to_string(&output_path).unwrap();
    let line_count = output.lines().count();
    assert_eq!(line_count, 1001); // Header + 1000 data rows

    // Verify some sample data
    assert!(output.contains("500,5000,category_0"));
    assert!(output.contains("1000,10000,category_0"));
}

#[tokio::test]
async fn test_e2e_driver_validation() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("test.csv");
    let output_path = temp_dir.path().join("output.shp");

    // Create input
    create_spatial_csv(&input_path).unwrap();

    // Create unsupported driver
    let input_driver = Driver::new(
        "CSV",
        "CSV",
        SupportStatus::Supported,
        SupportStatus::Supported,
        SupportStatus::Supported,
    );
    let output_driver = Driver::new(
        "ESRI Shapefile",
        "ESRI Shapefile",
        SupportStatus::NotSupported,
        SupportStatus::NotSupported,
        SupportStatus::NotSupported,
    );

    // Attempt conversion
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &input_driver,
        &output_driver,
        "wkt",
        None,
        None, // sql_query
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    // After factory refactoring, unregistered drivers produce a "not registered" error
    // OR "is not yet implemented" for the write path which still uses a switch statement
    assert!(
        error_msg.contains("not registered") || error_msg.contains("is not yet implemented"),
        "Unexpected error message: {error_msg}"
    );
}

#[tokio::test]
async fn test_e2e_csv_with_special_characters() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("special.csv");
    let output_path = temp_dir.path().join("special_output.csv");

    // Create CSV with special characters and proper quoting
    let mut file = File::create(&input_path).unwrap();
    writeln!(file, "id,name,description,wkt").unwrap();
    writeln!(file, "1,O'Brien,Simple name,\"POINT(1 1)\"").unwrap();
    writeln!(file, "2,Smith,Another name,\"POINT(2 2)\"").unwrap();
    writeln!(file, "3,Müller,Unicode name,\"POINT(3 3)\"").unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        None, // sql_query
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());
    assert!(output_path.exists(), "Output file was not created");

    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("O'Brien"));
    assert!(output.contains("Smith"));
    assert!(output.contains("Müller"));
}

#[tokio::test]
async fn test_e2e_multiple_conversions_same_session() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();

    // First conversion
    let input1 = temp_dir.path().join("input1.csv");
    let output1 = temp_dir.path().join("output1.csv");
    create_spatial_csv(&input1).unwrap();

    // Second conversion
    let input2 = temp_dir.path().join("input2.geojson");
    let output2 = temp_dir.path().join("output2.geojson");
    create_sample_geojson(&input2).unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");
    let geojson_driver = find_driver("GeoJSON").expect("GeoJSON driver should exist");

    // First conversion
    let result1 = convert(
        input1.to_str().unwrap(),
        output1.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        None, // sql_query
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    // Second conversion
    let result2 = convert(
        input2.to_str().unwrap(),
        output2.to_str().unwrap(),
        &geojson_driver,
        &geojson_driver,
        "geometry",
        None,
        None, // sql_query
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(output1.exists());
    assert!(output2.exists());
}

#[tokio::test]
async fn test_e2e_sql_filter_conversion() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("cities.csv");
    let output_path = temp_dir.path().join("filtered_cities.csv");

    // Create input data with multiple records
    let mut file = File::create(&input_path).unwrap();
    writeln!(file, "id,name,population,wkt").unwrap();
    writeln!(file, "1,New York,8336817,\"POINT(-74.0060 40.7128)\"").unwrap();
    writeln!(file, "2,Los Angeles,3979576,\"POINT(-118.2437 34.0522)\"").unwrap();
    writeln!(file, "3,Chicago,2746388,\"POINT(-87.6298 41.8781)\"").unwrap();
    writeln!(file, "4,Houston,2304580,\"POINT(-95.3698 29.7604)\"").unwrap();
    writeln!(file, "5,Phoenix,1608139,\"POINT(-112.0740 33.4484)\"").unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Filter cities with population > 2,000,000
    // Note: table name "cities" is inferred from input filename "cities.csv"
    let sql_query = "SELECT * FROM cities WHERE population > 2000000";
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        Some(sql_query),
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(
        result.is_ok(),
        "Conversion with SQL filter failed: {:?}",
        result.err()
    );
    assert!(output_path.exists(), "Output file was not created");

    // Verify only filtered records are in output
    let output = std::fs::read_to_string(&output_path).unwrap();

    // Should contain the large cities (population > 2M)
    assert!(output.contains("New York")); // 8,336,817
    assert!(output.contains("Los Angeles")); // 3,979,576
    assert!(output.contains("Chicago")); // 2,746,388

    // Houston has 2,304,580 which is > 2,000,000, so it should be included!
    assert!(output.contains("Houston")); // 2,304,580

    // Phoenix should NOT be included (population < 2M)
    assert!(!output.contains("Phoenix")); // 1,608,139

    // Verify line count: header + 4 cities with population > 2M
    let line_count = output.lines().count();
    assert_eq!(line_count, 5); // Header + 4 cities
}

#[tokio::test]
async fn test_e2e_sql_column_selection() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("full_data.csv");
    let output_path = temp_dir.path().join("selected_columns.csv");

    // Create input with many columns
    create_spatial_csv(&input_path).unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Select only specific columns
    // Note: table name "full_data" is inferred from input filename
    let sql_query = "SELECT id, name, wkt FROM full_data";
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        Some(sql_query),
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(
        result.is_ok(),
        "Conversion with column selection failed: {:?}",
        result.err()
    );
    assert!(output_path.exists(), "Output file was not created");

    // Verify only selected columns are in output
    let output = std::fs::read_to_string(&output_path).unwrap();
    let header_line = output.lines().next().unwrap();

    // Should have only the selected columns
    assert!(header_line.contains("id"));
    assert!(header_line.contains("name"));
    assert!(header_line.contains("wkt"));

    // Should NOT have the other columns
    assert!(!header_line.contains("category"));
    assert!(!header_line.contains("value"));
}

#[tokio::test]
async fn test_e2e_sql_aggregation() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("data_to_aggregate.csv");
    let output_path = temp_dir.path().join("aggregated.csv");

    // Create input data
    create_spatial_csv(&input_path).unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Aggregate by category
    // Note: table name "data_to_aggregate" is inferred from input filename
    let sql_query = "SELECT category, COUNT(*) as count, SUM(value) as total_value FROM data_to_aggregate GROUP BY category";
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        Some(sql_query),
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(
        result.is_ok(),
        "Conversion with aggregation failed: {:?}",
        result.err()
    );
    assert!(output_path.exists(), "Output file was not created");

    // Verify aggregated results
    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("category"));
    assert!(output.contains("count"));
    assert!(output.contains("total_value"));

    // Should have aggregated rows
    assert!(output.contains("retail"));
    assert!(output.contains("office"));
}

#[tokio::test]
async fn test_e2e_sql_order_by() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("unsorted.csv");
    let output_path = temp_dir.path().join("sorted.csv");

    // Create input data
    create_spatial_csv(&input_path).unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Order by value descending
    // Note: table name "unsorted" is inferred from input filename
    let sql_query = "SELECT * FROM unsorted ORDER BY value DESC";
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        Some(sql_query),
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(
        result.is_ok(),
        "Conversion with ORDER BY failed: {:?}",
        result.err()
    );
    assert!(output_path.exists(), "Output file was not created");

    // Verify sorted output - highest value should be first data row
    let output = std::fs::read_to_string(&output_path).unwrap();
    let lines: Vec<&str> = output.lines().collect();

    // First data row (after header) should have value 320 (Location D)
    assert!(lines.len() >= 2);
    assert!(lines[1].contains("Location D") || lines[1].contains("320"));
}

#[tokio::test]
async fn test_e2e_sql_invalid_query() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("test.csv");
    let output_path = temp_dir.path().join("output.csv");

    create_spatial_csv(&input_path).unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Invalid SQL query (non-existent column)
    // Note: table name "test" is inferred from input filename
    let sql_query = "SELECT nonexistent_column FROM test";
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        Some(sql_query),
        None, // table_name_override
        None, // batch_size
        None, // read_partitions
        None, // write_partitions
    )
    .await;

    assert!(result.is_err(), "Should fail with invalid SQL query");
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("SQL query") || error_msg.contains("nonexistent_column"));
}

#[tokio::test]
async fn test_e2e_custom_table_name() {
    // Initialize format drivers
    geoetl_core::init::initialize();

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("my_complex_filename_2024.csv");
    let output_path = temp_dir.path().join("output.csv");

    // Create input data
    create_spatial_csv(&input_path).unwrap();

    let csv_driver = find_driver("CSV").expect("CSV driver should exist");

    // Use custom table name "data" instead of inferred "my_complex_filename_2024"
    let sql_query = "SELECT id, name FROM data WHERE value > 200";
    let result = convert(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &csv_driver,
        &csv_driver,
        "wkt",
        None,
        Some(sql_query),
        Some("data"), // Custom table name override
        None,         // batch_size
        None,         // read_partitions
        None,         // write_partitions
    )
    .await;

    assert!(
        result.is_ok(),
        "Conversion with custom table name failed: {:?}",
        result.err()
    );
    assert!(output_path.exists(), "Output file was not created");

    // Verify output
    let output = std::fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("id,name"));

    // Should have filtered records where value > 200 (Locations B, D, E from create_spatial_csv)
    let line_count = output.lines().count();
    assert!(line_count >= 3); // Header + at least 2-3 filtered records
}
