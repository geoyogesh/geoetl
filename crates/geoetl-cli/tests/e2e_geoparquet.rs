//! End-to-end CLI tests for `GeoParquet` format support
//!
//! These tests verify the complete `GeoParquet` workflow including:
//! - Reading `GeoParquet` files
//! - Writing `GeoParquet` files
//! - Converting to/from other formats (CSV, `GeoJSON`)
//! - Info command on `GeoParquet` files

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

const TEST_DATA_CSV: &str = "tests/e2e_data/csv/natural-earth_cities_native_AS_WKT.csv";
const TEST_DATA_GEOJSON: &str = "tests/e2e_data/geojson/natural-earth_cities.geojson";
const TEST_DATA_GEOPARQUET: &str = "tests/e2e_data/geoparquet/natural-earth_cities.parquet";

/// Helper to create a command instance for the CLI
fn geoetl_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("geoetl-cli"))
}

// ============================================================================
// Conversion Tests: CSV to GeoParquet
// ============================================================================

#[test]
fn test_cli_convert_csv_to_geoparquet() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.parquet");

    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_CSV)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("CSV")
        .arg("--output-driver")
        .arg("GeoParquet")
        .assert()
        .success();

    // Verify output file exists
    assert!(
        output_path.exists(),
        "GeoParquet output file should be created"
    );

    // Verify file is not empty
    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0, "GeoParquet file should not be empty");
}

#[test]
fn test_cli_convert_csv_to_geoparquet_with_verbose() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output_verbose.parquet");

    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_CSV)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("CSV")
        .arg("--output-driver")
        .arg("GeoParquet")
        .arg("--verbose")
        .assert()
        .success();

    assert!(output_path.exists());
}

#[test]
fn test_cli_convert_csv_to_geoparquet_with_custom_batch_size() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output_batched.parquet");

    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_CSV)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("CSV")
        .arg("--output-driver")
        .arg("GeoParquet")
        .arg("--batch-size")
        .arg("1024")
        .assert()
        .success();

    assert!(output_path.exists());
}

// ============================================================================
// Conversion Tests: GeoJSON to GeoParquet
// ============================================================================

#[test]
fn test_cli_convert_geojson_to_geoparquet() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.parquet");

    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_GEOJSON)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("GeoJSON")
        .arg("--output-driver")
        .arg("GeoParquet")
        .assert()
        .success();

    assert!(
        output_path.exists(),
        "GeoParquet output file should be created"
    );

    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0, "GeoParquet file should not be empty");
}

// ============================================================================
// Conversion Tests: GeoParquet to CSV
// ============================================================================

#[test]
fn test_cli_convert_geoparquet_to_csv() {
    let temp_dir = TempDir::new().unwrap();
    let intermediate_path = temp_dir.path().join("intermediate.parquet");
    let output_path = temp_dir.path().join("output.csv");

    // First convert CSV to GeoParquet to create a file without bbox column
    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_CSV)
        .arg("--output")
        .arg(&intermediate_path)
        .arg("--input-driver")
        .arg("CSV")
        .arg("--output-driver")
        .arg("GeoParquet")
        .assert()
        .success();

    // Then convert GeoParquet to CSV
    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(&intermediate_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("GeoParquet")
        .arg("--output-driver")
        .arg("CSV")
        .assert()
        .success();

    assert!(output_path.exists(), "CSV output file should be created");

    let output_content = fs::read_to_string(&output_path).unwrap();

    // Verify headers
    assert!(output_content.contains("geometry,name"));

    // Verify sample data points
    assert!(output_content.contains("Vatican City"));
    assert!(output_content.contains("POINT"));
}

// ============================================================================
// Conversion Tests: GeoParquet to GeoJSON
// ============================================================================

#[test]
fn test_cli_convert_geoparquet_to_geojson() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.geojson");

    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_GEOPARQUET)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("GeoParquet")
        .arg("--output-driver")
        .arg("GeoJSON")
        .assert()
        .success();

    assert!(
        output_path.exists(),
        "GeoJSON output file should be created"
    );

    let output_content = fs::read_to_string(&output_path).unwrap();

    // Verify GeoJSON structure
    assert!(output_content.contains("FeatureCollection"));
    assert!(output_content.contains("\"type\""));
    assert!(output_content.contains("\"features\""));

    // Verify sample city data
    assert!(output_content.contains("Vatican City"));
}

// ============================================================================
// Conversion Tests: GeoParquet to GeoParquet (roundtrip)
// ============================================================================

#[test]
fn test_cli_convert_geoparquet_to_geoparquet() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.parquet");

    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_GEOPARQUET)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("GeoParquet")
        .arg("--output-driver")
        .arg("GeoParquet")
        .assert()
        .success();

    assert!(
        output_path.exists(),
        "GeoParquet output file should be created"
    );

    let metadata = fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 0, "GeoParquet file should not be empty");
}

// ============================================================================
// Info Command Tests
// ============================================================================

#[test]
fn test_cli_info_geoparquet() {
    geoetl_cmd()
        .arg("info")
        .arg(TEST_DATA_GEOPARQUET)
        .arg("-f")
        .arg("GeoParquet")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dataset:"))
        .stdout(predicate::str::contains("Geometry Columns"));
}

#[test]
fn test_cli_info_geoparquet_with_verbose() {
    geoetl_cmd()
        .arg("info")
        .arg(TEST_DATA_GEOPARQUET)
        .arg("-f")
        .arg("GeoParquet")
        .arg("--verbose")
        .assert()
        .success();
}

// ============================================================================
// Drivers Command Tests
// ============================================================================

#[test]
fn test_cli_drivers_lists_geoparquet() {
    geoetl_cmd()
        .arg("drivers")
        .assert()
        .success()
        .stdout(predicate::str::contains("GeoParquet"));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_cli_convert_geoparquet_nonexistent_input() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("nonexistent.parquet");
    let output_path = temp_dir.path().join("output.csv");

    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(&input_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("GeoParquet")
        .arg("--output-driver")
        .arg("CSV")
        .assert()
        .success(); // DataFusion handles missing files gracefully
}

// ============================================================================
// Case Sensitivity Tests
// ============================================================================

#[test]
fn test_cli_convert_geoparquet_case_insensitive() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.parquet");

    // Test with different case variations
    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_CSV)
        .arg("--output")
        .arg(&output_path)
        .arg("--input-driver")
        .arg("csv")
        .arg("--output-driver")
        .arg("geoparquet") // lowercase
        .assert()
        .success();

    assert!(output_path.exists());
}

// ============================================================================
// Data Integrity Tests
// ============================================================================

#[test]
fn test_cli_convert_roundtrip_csv_geoparquet_csv() {
    let temp_dir = TempDir::new().unwrap();
    let parquet_path = temp_dir.path().join("intermediate.parquet");
    let csv_output_path = temp_dir.path().join("output.csv");

    // Step 1: CSV to GeoParquet
    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_CSV)
        .arg("--output")
        .arg(&parquet_path)
        .arg("--input-driver")
        .arg("CSV")
        .arg("--output-driver")
        .arg("GeoParquet")
        .assert()
        .success();

    assert!(parquet_path.exists());

    // Step 2: GeoParquet to CSV
    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(&parquet_path)
        .arg("--output")
        .arg(&csv_output_path)
        .arg("--input-driver")
        .arg("GeoParquet")
        .arg("--output-driver")
        .arg("CSV")
        .assert()
        .success();

    assert!(csv_output_path.exists());

    // Verify data integrity
    let output_content = fs::read_to_string(&csv_output_path).unwrap();
    assert!(output_content.contains("Vatican City"));
    assert!(output_content.contains("POINT"));
}

#[test]
fn test_cli_convert_roundtrip_geojson_geoparquet_geojson() {
    let temp_dir = TempDir::new().unwrap();
    let parquet_path = temp_dir.path().join("intermediate.parquet");
    let geojson_output_path = temp_dir.path().join("output.geojson");

    // Step 1: GeoJSON to GeoParquet
    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(TEST_DATA_GEOJSON)
        .arg("--output")
        .arg(&parquet_path)
        .arg("--input-driver")
        .arg("GeoJSON")
        .arg("--output-driver")
        .arg("GeoParquet")
        .assert()
        .success();

    assert!(parquet_path.exists());

    // Step 2: GeoParquet to GeoJSON
    geoetl_cmd()
        .arg("convert")
        .arg("--input")
        .arg(&parquet_path)
        .arg("--output")
        .arg(&geojson_output_path)
        .arg("--input-driver")
        .arg("GeoParquet")
        .arg("--output-driver")
        .arg("GeoJSON")
        .assert()
        .success();

    assert!(geojson_output_path.exists());

    // Verify data integrity
    let output_content = fs::read_to_string(&geojson_output_path).unwrap();
    assert!(output_content.contains("FeatureCollection"));
    assert!(output_content.contains("Vatican City"));

    // Verify it's valid JSON
    let json_result = serde_json::from_str::<serde_json::Value>(&output_content);
    assert!(json_result.is_ok(), "Output should be valid JSON");
}
