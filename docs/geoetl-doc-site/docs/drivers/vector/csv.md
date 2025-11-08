---
sidebar_position: 2
title: CSV
description: Comma Separated Value with WKT geometries
---

# CSV (Comma Separated Value)

CSV format with Well-Known Text (WKT) geometries for simple tabular geospatial data.

## Driver Metadata

| Property | Value |
|----------|-------|
| **Short Name** | CSV |
| **Long Name** | Comma Separated Value (.csv) |
| **Supported Since** | v0.1.0 |
| **Status** | ✅ Stable |
| **File Extension** | `.csv` |

## Driver Capabilities

- ✅ **Read Support**: Full
- ✅ **Write Support**: Full
- ✅ **Info Support**: Yes
- ✅ **Geometry Format**: WKT (Well-Known Text)
- ✅ **Geometry Types**: Point, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon

## Format Description

CSV (Comma Separated Value) files with WKT (Well-Known Text) geometries stored in a column.

## Reading Data

### Get Information

```bash
geoetl-cli info cities.csv \
  --driver CSV \
  --geometry-column geometry
```

:::danger IMPORTANT
The `--geometry-column` parameter is **REQUIRED** for all CSV operations.

Unlike GeoJSON (which has a standard geometry structure), CSV files can have any column name for geometries. You must explicitly tell GeoETL which column contains your WKT geometries.
:::

### Convert from CSV

```bash
geoetl-cli convert \
  --input cities.csv \
  --output cities.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column geometry
```

### Read Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--geometry-column` | string | **Yes** | Name of column containing WKT geometries |
| `--geometry-type` | string | No | Geometry type hint (Point, LineString, Polygon, etc.) |

## Writing Data

### Convert to CSV

```bash
geoetl-cli convert \
  --input data.geojson \
  --output data.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```


### Write Behavior

- Geometry column is always named `geometry`
- All properties become CSV columns
- WKT strings are quoted if they contain commas
- UTF-8 encoding by default

## Examples

### Example 1: Convert GeoJSON to CSV

```bash
geoetl-cli convert \
  -i cities.geojson \
  -o cities.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

### Example 2: Convert CSV to GeoJSON

```bash
geoetl-cli convert \
  -i data.csv \
  -o data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt
```

### Example 3: Convert CSV to GeoParquet

```bash
geoetl-cli convert \
  -i data.csv \
  -o data.parquet \
  --input-driver CSV \
  --output-driver GeoParquet \
  --geometry-column geometry
```

## Troubleshooting

### Error: Geometry Column Not Found

**Error**: `Geometry column 'geometry' not found`

**Cause**: The CSV file uses a different column name (e.g., `wkt`, `geom`, `the_geom`)

**Solution**: Specify the correct column name:
```bash
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt
```

### Error: Invalid WKT

**Error**: `Failed to parse WKT`

**Common causes**:
```csv
# ❌ Missing coordinates
"POINT()"

# ❌ Wrong format
"POINT: -122, 37"

# ❌ Extra spaces
"POINT ( -122  37 )"
```

**Solution**:
```csv
# ✅ Correct format
"POINT(-122.4194 37.7749)"
```

### Error: Encoding Problems

**Symptoms**: Special characters appear as `�` or weird symbols

**Solution**: Ensure UTF-8 encoding
```bash
# Convert to UTF-8 (Linux/macOS)
iconv -f ISO-8859-1 -t UTF-8 input.csv > output.csv

# Then convert with GeoETL
geoetl-cli convert -i output.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry
```

## See Also

**Tutorials**:
- [Working with CSV](../../tutorial-basics/working-with-csv.md) - Complete tutorial
- [Your First Conversion](../../getting-started/first-conversion.md) - Quick start

**Other Drivers**:
- [GeoJSON Driver](./geojson.md) - Web-standard JSON format
- [GeoParquet Driver](./geoparquet.md) - High-performance columnar format

**Reference**:
- [Supported Drivers](../supported-drivers.md) - All drivers comparison
- [WKT Reference](https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry)

## References

- [Well-Known Text on Wikipedia](https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry)
- [Simple Features for SQL Specification](https://www.ogc.org/standards/sfa)
