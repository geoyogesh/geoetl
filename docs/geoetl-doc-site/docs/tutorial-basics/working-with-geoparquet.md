---
sidebar_position: 6
---

# Working with GeoParquet

Learn how to use GeoParquet with GeoETL.

## What is GeoParquet?

GeoParquet is a columnar storage format for geospatial data that combines Apache Parquet's efficient columnar storage with WKB-encoded geometries and GeoArrow types.

For detailed information, see the [GeoParquet Driver Reference](../drivers/vector/geoparquet).

## Quick Start

### Converting to GeoParquet

**From GeoJSON**:
```bash
geoetl-cli convert \
  --input data.geojson \
  --output data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

**From CSV**:
```bash
geoetl-cli convert \
  --input data.csv \
  --output data.parquet \
  --input-driver CSV \
  --output-driver GeoParquet \
  --geometry-column WKT
```

### Converting from GeoParquet

**To GeoJSON**:
```bash
geoetl-cli convert \
  --input data.parquet \
  --output data.geojson \
  --input-driver GeoParquet \
  --output-driver GeoJSON
```

## Troubleshooting

### CSV Export with bbox Columns

GeoParquet files from ogr2ogr may have bbox struct columns that CSV cannot represent. Workaround using roundtrip via GeoJSON:

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

## References

- [GeoParquet Specification](https://geoparquet.org/)
- [Apache Parquet Documentation](https://parquet.apache.org/)
- [GeoArrow Specification](https://geoarrow.org/)

## See Also

- [Working with GeoJSON](./working-with-geojson)
- [Working with CSV](./working-with-csv)
