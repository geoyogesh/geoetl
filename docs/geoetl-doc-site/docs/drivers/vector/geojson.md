---
sidebar_position: 1
title: GeoJSON
description: Geographic JSON format for web mapping
---

# GeoJSON

GeoJSON is a JSON-based format for encoding geographic data structures, widely used in web mapping applications.

## Driver Metadata

| Property | Value |
|----------|-------|
| **Short Name** | GeoJSON |
| **Long Name** | GeoJSON (RFC 7946) |
| **Supported Since** | v0.1.0 |
| **Status** | ✅ Stable |
| **File Extension** | `.geojson`, `.json` |

## Driver Capabilities

- ✅ **Read Support**: Full
- ✅ **Write Support**: Full
- ✅ **Info Support**: Yes
- ✅ **Geometry Types**: Point, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon, GeometryCollection
- ✅ **Coordinate Systems**: WGS84 (EPSG:4326) by default

## Format Description

GeoJSON is a JSON-based format for encoding geographic data structures defined by RFC 7946.

## Reading Data

### Get Information

```bash
geoetl-cli info cities.geojson --driver GeoJSON
```

**Output**:
```
Dataset: cities.geojson
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
| state      | String | Yes      |
+------------+--------+----------+
```

### Convert from GeoJSON

```bash
geoetl-cli convert \
  --input data.geojson \
  --output data.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

## Writing Data

### Convert to GeoJSON

```bash
geoetl-cli convert \
  --input data.csv \
  --output data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column geometry
```

### Supported Write Options

Currently, GeoETL writes standard GeoJSON without additional options. Future versions may support:

- Coordinate precision
- Pretty printing
- Bbox inclusion
- CRS specification (RFC 7946 recommends WGS84 only)

## Examples

### Example 1: Convert GeoJSON to CSV

```bash
geoetl-cli convert \
  -i cities.geojson \
  -o cities.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

### Example 2: Convert GeoJSON to GeoParquet

```bash
geoetl-cli convert \
  -i data.geojson \
  -o data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

## Troubleshooting

### Invalid JSON Syntax

**Error**: `Parse error in GeoJSON at line 15`

**Cause**: Malformed JSON (missing comma, bracket, quote)

**Solution**:
```bash
# Validate JSON syntax
cat data.geojson | jq . > /dev/null

# Or use Python
python3 -m json.tool data.geojson > /dev/null
```

### Wrong Coordinate Order

**Problem**: Points appear in wrong location

**Cause**: Coordinates in `[latitude, longitude]` instead of `[longitude, latitude]`

**Solution**: Verify coordinate order in source file. GeoJSON specification requires `[longitude, latitude]` order.

## Format Specification

- **RFC 7946**: [GeoJSON Specification](https://datatracker.ietf.org/doc/html/rfc7946)
- **Website**: [https://geojson.org/](https://geojson.org/)

## See Also

**Tutorials**:
- [Working with GeoJSON](../../tutorial-basics/working-with-geojson.md) - Complete tutorial
- [Your First Conversion](../../getting-started/first-conversion.md) - Quick start

**Other Drivers**:
- [CSV Driver](./csv.md) - CSV with WKT geometries
- [GeoParquet Driver](./geoparquet.md) - High-performance columnar format

**Reference**:
- [Supported Drivers](../supported-drivers.md) - All drivers comparison
- [Driver Matrix](../../reference/driver-matrix.md) - Capability comparison

## References

- [GeoJSON Specification (RFC 7946)](https://datatracker.ietf.org/doc/html/rfc7946)
- [geojson.io](http://geojson.io) - Online editor and validator
- [More than you ever wanted to know about GeoJSON](https://macwright.com/2015/03/23/geojson-second-bite.html)
