---
sidebar_position: 4
---

# Working with GeoJSON

Learn how to use GeoJSON with GeoETL.

## What is GeoJSON?

GeoJSON is a JSON-based format for encoding geographic data structures.

For detailed information, see the [GeoJSON Driver Reference](../drivers/vector/geojson).

## Quick Start

### Reading GeoJSON

**Inspect a GeoJSON file**:
```bash
geoetl-cli info data.geojson --driver GeoJSON
```

**Example output**:
```
Dataset: /path/to/data.geojson
Driver: GeoJSON

=== Geometry Columns ===
+----------+-------------------+-----+
| Column   | Extension         | CRS |
+----------+-------------------+-----+
| geometry | geoarrow.geometry | N/A |
+----------+-------------------+-----+

=== Fields ===
+------------+--------+----------+
| Field      | Type   | Nullable |
+------------+--------+----------+
| name       | String | Yes      |
| population | Int64  | Yes      |
+------------+--------+----------+
```

### Converting from GeoJSON

**To CSV**:
```bash
geoetl-cli convert \
  --input data.geojson \
  --output data.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

**To GeoParquet**:
```bash
geoetl-cli convert \
  --input data.geojson \
  --output data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

### Converting to GeoJSON

**From CSV**:
```bash
geoetl-cli convert \
  --input data.csv \
  --output data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt
```

**From GeoParquet**:
```bash
geoetl-cli convert \
  --input data.parquet \
  --output data.geojson \
  --input-driver GeoParquet \
  --output-driver GeoJSON
```

## GeoJSON Structure

### Feature Collection

The most common GeoJSON structure:

```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {
        "name": "San Francisco",
        "population": 873965,
        "state": "California"
      },
      "geometry": {
        "type": "Point",
        "coordinates": [-122.4194, 37.7749]
      }
    },
    {
      "type": "Feature",
      "properties": {
        "name": "New York",
        "population": 8336817,
        "state": "New York"
      },
      "geometry": {
        "type": "Point",
        "coordinates": [-74.006, 40.7128]
      }
    }
  ]
}
```

### Geometry Types

GeoJSON supports these geometry types:

**Point**:
```json
{
  "type": "Point",
  "coordinates": [-122.4194, 37.7749]
}
```

**LineString**:
```json
{
  "type": "LineString",
  "coordinates": [
    [-122.4194, 37.7749],
    [-74.006, 40.7128]
  ]
}
```

**Polygon**:
```json
{
  "type": "Polygon",
  "coordinates": [
    [
      [-122.4, 37.8],
      [-122.4, 37.7],
      [-122.5, 37.7],
      [-122.5, 37.8],
      [-122.4, 37.8]
    ]
  ]
}
```

**MultiPoint**:
```json
{
  "type": "MultiPoint",
  "coordinates": [
    [-122.4194, 37.7749],
    [-74.006, 40.7128]
  ]
}
```

**MultiLineString**, **MultiPolygon**, **GeometryCollection** are also supported.

## Troubleshooting

### Invalid JSON

**Error**: `Parse error in GeoJSON at line 15`

**Fix**: Validate JSON syntax
```bash
# Use jq to validate
cat data.geojson | jq . > /dev/null

# Or Python
python3 -m json.tool data.geojson > /dev/null
```

### Wrong Coordinate Order

**Error**: Points appear in wrong location

**Fix**: Ensure `[longitude, latitude]` order
```json
// Correct: longitude first
"coordinates": [-122.4194, 37.7749]
```

### Missing Properties

**Error**: Empty properties object

**Fix**: Ensure each feature has properties
```json
{
  "type": "Feature",
  "properties": {},  // ❌ Empty
  "geometry": { ... }
}

{
  "type": "Feature",
  "properties": { "name": "value" },  // ✅ Good
  "geometry": { ... }
}
```

## Quick Reference

### Essential Commands

```bash
# Inspect GeoJSON
geoetl-cli info data.geojson -f GeoJSON

# GeoJSON to CSV
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV

# GeoJSON to GeoParquet
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# CSV to GeoJSON
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt

# Validate GeoJSON
geoetl-cli convert -i data.geojson -o validated.geojson \
  --input-driver GeoJSON --output-driver GeoJSON
```

## References

- [GeoJSON Specification (RFC 7946)](https://datatracker.ietf.org/doc/html/rfc7946)
- [geojson.io](http://geojson.io) - Online editor and validator

## See Also

- [Working with CSV](./working-with-csv)
- [Working with GeoParquet](./working-with-geoparquet)
