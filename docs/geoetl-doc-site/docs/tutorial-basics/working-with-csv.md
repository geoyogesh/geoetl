---
sidebar_position: 5
---

# Working with CSV

Learn how to use CSV files with WKT geometries in GeoETL.

## CSV + Geospatial Data

CSV files store geospatial data by including geometry in WKT (Well-Known Text) format.

## WKT Geometry Format

WKT represents geometries as text strings. Coordinates in WKT are `longitude latitude` (x y), not latitude longitude.

## Creating CSV with Geometries

Example CSV file with WKT geometries:

```csv
name,population,geometry
Tokyo,13960000,"POINT(139.6917 35.6895)"
Delhi,32941000,"POINT(77.2090 28.6139)"
Shanghai,27796000,"POINT(121.4737 31.2304)"
```

## Converting CSV to GeoJSON

### Basic Conversion

```bash
geoetl-cli convert \
  -i cities.csv \
  -o cities.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column geometry
```

:::danger IMPORTANT
The `--geometry-column` parameter is **REQUIRED** for all CSV operations in GeoETL.

Unlike GeoJSON (which has a standard geometry structure), CSV files can have any column name for geometries. You must explicitly tell GeoETL which column contains your WKT geometries.

**If you forget this parameter, you'll get a clear error message with an example.**
:::

By default, most CSV files use "geometry" as the column name, but you must explicitly specify it.

### Custom Geometry Column

If your geometry column has a different name:

```csv
name,population,wkt
Tokyo,13960000,"POINT(139.6917 35.6895)"
Delhi,32941000,"POINT(77.2090 28.6139)"
```

Use `--geometry-column`:

```bash
geoetl-cli convert \
  -i cities.csv \
  -o cities.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt
```

### Specifying Geometry Type

For optimization, you can specify the geometry type:

```bash
geoetl-cli convert \
  -i points.csv \
  -o points.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt \
  --geometry-type Point
```

Supported types:
- `Point`
- `LineString`
- `Polygon`
- `MultiPoint`
- `MultiLineString`
- `MultiPolygon`
- `Geometry` (mixed types - default)

## Converting GeoJSON to CSV

### Basic Conversion

```bash
geoetl-cli convert \
  -i cities.geojson \
  -o cities.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

This creates a CSV with:
- All properties as columns
- Geometry column with WKT strings

### Custom Geometry Column Name

Currently, the geometry column is always named "geometry". In future versions, you'll be able to customize this.


## Troubleshooting CSV Issues

### Issue: Invalid WKT

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

**Solutions**:
```csv
# ✅ Correct format
"POINT(-122.4194 37.7749)"
```

### Issue: Geometry Column Not Found

**Error**: `Geometry column 'geometry' not found`

**Solution**: Specify the correct column name
```bash
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt  # or whatever your column is named
```

### Issue: Encoding Problems

**Symptoms**: Special characters appear as `�` or weird symbols

**Solution**: Ensure UTF-8 encoding
```bash
# Convert to UTF-8 (Linux/macOS)
iconv -f ISO-8859-1 -t UTF-8 input.csv > output.csv

# Then convert with GeoETL
geoetl-cli convert -i output.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON
```

### Issue: Empty Output

**Symptoms**: Conversion succeeds but output is empty

**Possible causes**:
1. Input CSV is empty
2. No geometry column found
3. All WKT strings are invalid

**Debug with verbose mode**:
```bash
geoetl-cli -v convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry
```

## Large CSV Files

For large files, use verbose mode to monitor progress:

```bash
geoetl-cli -v convert -i large_data.csv -o large_data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry
```

## Quick Reference

```bash
# Basic CSV to GeoJSON (--geometry-column REQUIRED for CSV)
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry

# Custom geometry column name
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt

# Specify geometry type for optimization
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt \
  --geometry-type Point

# GeoJSON to CSV (no geometry-column needed for GeoJSON input)
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV

# Verbose output
geoetl-cli -v convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry
```

## References

- **WKT Specification**: [Wikipedia - Well-known text](https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry)

## See Also

- [Working with GeoJSON](./working-with-geojson)
- [Working with GeoParquet](./working-with-geoparquet)
