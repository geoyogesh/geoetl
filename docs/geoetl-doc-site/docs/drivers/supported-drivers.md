---
sidebar_position: 1
---

# Supported Drivers

Complete reference of all GeoETL format drivers, their capabilities, and planned support.

## Quick Summary

**Currently Supported**: GeoJSON, CSV, GeoParquet
**Planned**: 65+ additional drivers via GDAL integration

## Currently Supported Drivers

These drivers are fully implemented and production-ready:

### GeoJSON

**Status**: ✅ Fully Supported
**Capabilities**: Info ✓ | Read ✓ | Write ✓

**Format**: JSON-based geographic data structure (RFC 7946)

**Example**:
```bash
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV
```

**Learn more**: [Working with GeoJSON Tutorial](../tutorial-basics/working-with-geojson)

---

### CSV (Comma Separated Value)

**Status**: ✅ Fully Supported
**Capabilities**: Info ✓ | Read ✓ | Write ✓

**Format**: CSV with WKT (Well-Known Text) geometries

**Example**:
```bash
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt
```

**Learn more**: [Working with CSV Tutorial](../tutorial-basics/working-with-csv)

---

### GeoParquet

**Status**: ✅ Fully Supported
**Capabilities**: Info ✓ | Read ✓ | Write ✓

**Format**: Apache Parquet with WKB-encoded geometries and GeoArrow types

**Performance**:
- Efficient compression
- Fast throughput
- Memory efficient: Low peak memory usage
- Production-ready: Handles 100M+ features

**Example**:
```bash
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet
```

**Learn more**: [Working with GeoParquet Tutorial](../tutorial-basics/working-with-geoparquet)

---

## Planned Drivers

These drivers are planned for future releases:

### High Priority

#### FlatGeobuf (FGB)
**Status**: 🚧 Planned

---

#### GeoPackage (GPKG)
**Status**: 🚧 Planned

---

#### ESRI Shapefile
**Status**: 🚧 Planned

---

### Additional Planned Formats

**Vector Formats**:
- GeoJSONSeq (GeoJSON newline-delimited)
- (Geo)Arrow IPC
- KML (Keyhole Markup Language)
- GML (Geography Markup Language)
- GPX (GPS Exchange Format)

**Database Formats**:
- PostgreSQL/PostGIS
- SQLite/Spatialite
- MySQL
- Microsoft SQL Server
- MongoDB
- Oracle Spatial

**CAD & Engineering**:
- AutoCAD DXF
- AutoCAD DWG
- Microstation DGN
- ESRI File Geodatabase

**Web Services**:
- OGC WFS (Web Feature Service)
- OGC API - Features
- Elasticsearch
- Carto

**Cloud & Big Data**:
- Cloud-optimized GeoTIFF (COG)
- Zarr
- TileDB

See the full roadmap: [VISION.md](https://github.com/geoyogesh/geoetl/blob/main/docs/VISION.md)

---

## Using Drivers in Commands

### Basic Syntax

```bash
geoetl-cli convert \
  --input <file> \
  --output <file> \
  --input-driver <DRIVER> \
  --output-driver <DRIVER>
```

### List All Drivers

```bash
geoetl-cli drivers
```

Output shows:
- **Short Name**: Driver identifier (use this in commands)
- **Long Name**: Full description
- **Info/Read/Write**: Capability status

### Driver Name Rules

1. **Case-sensitive**: Use exact capitalization
   ```bash
   # ✅ Correct
   --input-driver GeoJSON

   # ❌ Wrong
   --input-driver geojson
   ```

2. **Use Short Name**: From the drivers table
   ```bash
   # ✅ Correct
   --input-driver CSV

   # ❌ Wrong
   --input-driver "Comma Separated Value (.csv)"
   ```

3. **Check availability**: Use `geoetl-cli drivers` to verify

---

## Common Conversions

### GeoJSON ↔ GeoParquet (Recommended)

```bash
# GeoJSON to GeoParquet (excellent compression!)
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# GeoParquet to GeoJSON (for web use)
geoetl-cli convert -i data.parquet -o data.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

### CSV ↔ GeoJSON

```bash
# CSV to GeoJSON
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt

# GeoJSON to CSV
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV
```

### CSV ↔ GeoParquet

```bash
# CSV to GeoParquet (good compression)
geoetl-cli convert -i data.csv -o data.parquet \
  --input-driver CSV --output-driver GeoParquet \
  --geometry-column wkt

# GeoParquet to CSV (may need workaround for bbox columns)
# See: Working with GeoParquet tutorial
```

---

## Geometry Format Support

Different drivers handle geometries differently:

| Driver | Geometry Format | Example |
|--------|----------------|---------|
| GeoJSON | Native JSON | `{"type": "Point", "coordinates": [x, y]}` |
| CSV | WKT (Well-Known Text) | `"POINT(x y)"` |
| GeoParquet | WKB (Well-Known Binary) | Binary columnar format |

---

## Quick Reference

```bash
# List all drivers
geoetl-cli drivers

# Search for specific driver
geoetl-cli drivers | grep -i "parquet"

# Get dataset info
geoetl-cli info data.geojson -f GeoJSON
geoetl-cli info data.csv -f CSV --geometry-column wkt
geoetl-cli info data.parquet -f GeoParquet

# Convert formats
geoetl-cli convert -i input.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV

# Get help
geoetl-cli convert --help
geoetl-cli drivers --help
```

---

## See Also

**Tutorials**:
- [Understanding Drivers](../tutorial-basics/understanding-drivers) - Driver system overview
- [Working with GeoJSON](../tutorial-basics/working-with-geojson) - GeoJSON guide
- [Working with CSV](../tutorial-basics/working-with-csv) - CSV operations
- [Working with GeoParquet](../tutorial-basics/working-with-geoparquet) - GeoParquet guide

**Benchmarks**:
- [Performance Benchmarks](https://github.com/geoyogesh/geoetl/blob/main/bench/README.md) - Detailed benchmark results

**Roadmap**:
- [VISION.md](https://github.com/geoyogesh/geoetl/blob/main/docs/VISION.md) - Complete roadmap and planned features
