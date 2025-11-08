---
sidebar_position: 3
title: GeoParquet
description: High-performance columnar geospatial format
---

# GeoParquet

Modern columnar storage format combining Apache Parquet efficiency with geospatial capabilities.

## Driver Metadata

| Property | Value |
|----------|-------|
| **Short Name** | GeoParquet |
| **Long Name** | GeoParquet (Parquet + GeoArrow) |
| **Supported Since** | v0.3.0 |
| **Status** | ✅ Stable |
| **File Extension** | `.parquet`, `.geoparquet` |

## Driver Capabilities

- ✅ **Read Support**: Full
- ✅ **Write Support**: Full
- ✅ **Info Support**: Yes
- ✅ **Geometry Format**: WKB (Well-Known Binary) with GeoArrow types
- ✅ **Geometry Types**: All standard types with WKB encoding
- ✅ **Metadata**: CRS, bounding boxes, encoding information

## Format Description

GeoParquet combines Apache Parquet's columnar storage with WKB-encoded geometries and GeoArrow types for geospatial data.

## Reading Data

### Get Information

```bash
geoetl-cli info data.parquet --driver GeoParquet
```

**Output**:
```
Dataset: data.parquet
Driver: GeoParquet

=== Geometry Columns ===
+----------+------------------+-----+
| Column   | Extension        | CRS |
+----------+------------------+-----+
| geometry | geoarrow.wkb     | N/A |
+----------+------------------+-----+

=== Fields ===
+------------+--------+----------+
| Field      | Type   | Nullable |
+------------+--------+----------+
| name       | String | Yes      |
| population | Int64  | Yes      |
+------------+--------+----------+
```

### Convert from GeoParquet

```bash
# To GeoJSON
geoetl-cli convert \
  --input data.parquet \
  --output data.geojson \
  --input-driver GeoParquet \
  --output-driver GeoJSON
```

```bash
# To CSV
geoetl-cli convert \
  --input data.parquet \
  --output data.csv \
  --input-driver GeoParquet \
  --output-driver CSV
```

## Writing Data

### Convert to GeoParquet

```bash
# From GeoJSON
geoetl-cli convert \
  --input data.geojson \
  --output data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

```bash
# From CSV
geoetl-cli convert \
  --input data.csv \
  --output data.parquet \
  --input-driver CSV \
  --output-driver GeoParquet \
  --geometry-column geometry
```

### Write Options

Currently, GeoETL writes GeoParquet with optimal defaults. Future versions may support:
- Compression codec selection (SNAPPY, GZIP, ZSTD)
- Row group size
- Column encoding
- Metadata customization

## Examples

### Convert GeoJSON to GeoParquet

```bash
geoetl-cli convert \
  -i data.geojson \
  -o data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

### Convert CSV to GeoParquet

```bash
geoetl-cli convert \
  -i data.csv \
  -o data.parquet \
  --input-driver CSV \
  --output-driver GeoParquet \
  --geometry-column geometry
```

### Convert GeoParquet to GeoJSON

```bash
geoetl-cli convert \
  -i data.parquet \
  -o data.geojson \
  --input-driver GeoParquet \
  --output-driver GeoJSON
```

## Troubleshooting

### CSV Export with bbox Columns

**Problem**: Error when exporting GeoParquet (created by ogr2ogr) to CSV

**Cause**: ogr2ogr GeoParquet files may have bbox struct columns that CSV cannot represent

**Solution**: Roundtrip via GeoJSON
```bash
# This may fail
geoetl-cli convert -i ogr_created.parquet -o output.csv \
  --input-driver GeoParquet --output-driver CSV

# Use roundtrip instead
geoetl-cli convert -i ogr_created.parquet -o temp.geojson \
  --input-driver GeoParquet --output-driver GeoJSON

geoetl-cli convert -i temp.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV
```

### Large File Performance

**Problem**: Slow processing or high memory usage

**Solution**: GeoParquet uses streaming architecture - ensure sufficient disk I/O:
```bash
# Use verbose mode to monitor progress
geoetl-cli -v convert \
  -i large.geojson \
  -o large.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```


## Format Specification

- **GeoParquet Spec**: [https://geoparquet.org/](https://geoparquet.org/)
- **Apache Parquet**: [https://parquet.apache.org/](https://parquet.apache.org/)
- **GeoArrow Spec**: [https://geoarrow.org/](https://geoarrow.org/)

## See Also

- [Working with GeoParquet](../../tutorial-basics/working-with-geoparquet.md)
- [GeoJSON Driver](./geojson.md)
- [CSV Driver](./csv.md)
