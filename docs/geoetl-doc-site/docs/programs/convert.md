---
sidebar_position: 2
title: geoetl-cli convert
description: Convert data between geospatial formats
---

# geoetl-cli convert

Convert geospatial data between different formats.

## Synopsis

```bash
geoetl-cli convert [OPTIONS] \
  --input <PATH> \
  --output <PATH> \
  --input-driver <DRIVER> \
  --output-driver <DRIVER>
```

## Description

The `convert` command transforms geospatial data from one format to another. It supports converting between GeoJSON, CSV (with WKT geometries), and GeoParquet formats.

GeoETL uses a streaming architecture for constant memory usage regardless of dataset size, making it suitable for large-scale data conversion.

## Required Options

| Option | Short | Type | Description |
|--------|-------|------|-------------|
| `--input` | `-i` | path | Input file path |
| `--output` | `-o` | path | Output file path |
| `--input-driver` | | string | Input format driver (see [supported drivers](../drivers/supported-drivers)) |
| `--output-driver` | | string | Output format driver (see [supported drivers](../drivers/supported-drivers)) |

## Optional Options

### CSV-Specific Options

| Option | Type | Required For | Description |
|--------|------|--------------|-------------|
| `--geometry-column` | string | CSV input | Name of column containing WKT geometries |
| `--geometry-type` | string | CSV input (optional) | Geometry type hint (Point, LineString, Polygon, etc.) |

### Global Options

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Print help information |
| `--verbose` | `-v` | Enable verbose output |

## Examples

### Example 1: GeoJSON to CSV

```bash
geoetl-cli convert \
  --input cities.geojson \
  --output cities.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```


### Example 2: CSV to GeoJSON

```bash
geoetl-cli convert \
  --input data.csv \
  --output data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column geometry
```

:::warning REQUIRED
The `--geometry-column` parameter is **required** for CSV input to specify which column contains WKT geometries.
:::

### Example 3: GeoJSON to GeoParquet

```bash
geoetl-cli convert \
  --input large_dataset.geojson \
  --output large_dataset.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```


### Example 4: Short Form

```bash
geoetl-cli convert \
  -i input.geojson \
  -o output.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

### Example 5: Verbose Output

```bash
geoetl-cli -v convert \
  -i large_file.geojson \
  -o large_file.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

**Output**:
```
INFO geoetl_cli: Converting large_file.geojson to large_file.parquet
INFO geoetl_core::operations: Starting conversion:
INFO geoetl_core::operations: Input: large_file.geojson (Driver: GeoJSON)
INFO geoetl_core::operations: Output: large_file.parquet (Driver: GeoParquet)
INFO geoetl_core::operations: Read 1000 record batch(es)
INFO geoetl_core::operations: Total rows: 1000000
INFO geoetl_core::operations: Conversion completed successfully
```

### Example 6: Custom Geometry Column

```bash
geoetl-cli convert \
  -i database_export.csv \
  -o output.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt_geom
```

**Use case**: When CSV uses custom column name like `wkt_geom`, `geom`, or `the_geom`.

## Common Workflows

### Workflow 1: Optimize for Cloud Storage

```bash
# Convert to GeoParquet for compression
geoetl-cli convert \
  -i source.geojson \
  -o optimized.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

## Driver Compatibility

### Supported Conversions

| From ↓ / To → | GeoJSON | CSV | GeoParquet |
|---------------|---------|-----|------------|
| **GeoJSON** | ✅ | ✅ | ✅ |
| **CSV** | ✅ | ✅ | ✅ |
| **GeoParquet** | ✅ | ✅ | ✅ |

All format combinations are supported.

## Error Messages

### Error: Driver not found

**Message**: `Input driver 'geojson' not found`

**Cause**: Driver name is case-sensitive or misspelled

**Solution**: Use exact driver name from `geoetl-cli drivers`:
```bash
# ❌ Wrong
--input-driver geojson

# ✅ Correct
--input-driver GeoJSON
```

### Error: File not found

**Message**: `No such file or directory`

**Cause**: Input file doesn't exist or path is incorrect

**Solution**:
```bash
# Check file exists
ls -l input.geojson

# Use absolute path
geoetl-cli convert -i /full/path/to/input.geojson ...
```

### Error: Missing geometry column

**Message**: `Missing required option: geometry-column`

**Cause**: CSV input requires `--geometry-column` parameter

**Solution**:
```bash
geoetl-cli convert \
  -i data.csv \
  -o data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column geometry  # Add this
```

### Error: Permission denied

**Message**: `Permission denied`

**Cause**: No write permission for output directory

**Solution**:
```bash
# Check permissions
ls -ld output/

# Use different directory
geoetl-cli convert -i input.geojson -o /tmp/output.csv ...
```

## Debugging

### Enable Verbose Logging

```bash
geoetl-cli -v convert ...
```

### Enable Debug Logging

```bash
RUST_LOG=debug geoetl-cli convert ...
```

### Test with Small Sample

```bash
# Extract first 100 features
head -n 100 large.geojson > sample.geojson

# Test conversion
geoetl-cli convert -i sample.geojson -o test.csv \
  --input-driver GeoJSON --output-driver CSV
```

## See Also

**Commands**:
- [info](./info.md) - Display dataset information
- [drivers](./drivers.md) - List available drivers

**Tutorials**:
- [Your First Conversion](../getting-started/first-conversion.md) - Quick start
- [Working with CSV](../tutorial-basics/working-with-csv.md) - CSV tutorial
- [Working with GeoParquet](../tutorial-basics/working-with-geoparquet.md) - GeoParquet tutorial

**Drivers**:
- [GeoJSON Driver](../drivers/vector/geojson.md)
- [CSV Driver](../drivers/vector/csv.md)
- [GeoParquet Driver](../drivers/vector/geoparquet.md)

**Reference**:
- [Supported Drivers](../drivers/supported-drivers.md) - All drivers
- [Performance Benchmarks](../reference/benchmarks.md) - Speed comparisons

---

**Need help?** Run `geoetl-cli convert --help`
