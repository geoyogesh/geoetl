---
sidebar_position: 1
---

# Supported Drivers

Complete reference of all GeoETL format drivers, their capabilities, and planned support.

## Quick Summary

**Currently Supported**: 3 drivers (GeoJSON, CSV, GeoParquet)
**Planned**: 65+ additional drivers via GDAL integration

## Currently Supported Drivers

These drivers are fully implemented and production-ready:

### GeoJSON

**Status**: ✅ Fully Supported (v0.1.0+)
**Capabilities**: Info ✓ | Read ✓ | Write ✓

**Format**: JSON-based geographic data structure (RFC 7946)

**Use cases**:
- Web mapping applications
- JavaScript/TypeScript projects
- API responses and data exchange
- Small to medium datasets (&lt;100k features)

**Example**:
```bash
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV
```

**Learn more**: [Working with GeoJSON Tutorial](../tutorial-basics/working-with-geojson)

---

### CSV (Comma Separated Value)

**Status**: ✅ Fully Supported (v0.1.0+)
**Capabilities**: Info ✓ | Read ✓ | Write ✓

**Format**: CSV with WKT (Well-Known Text) geometries

**Use cases**:
- Excel compatibility
- Simple tabular data with geometries
- Data analysis and visualization
- Legacy system integration

**Example**:
```bash
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt
```

**Learn more**: [Working with CSV Tutorial](../tutorial-basics/working-with-csv)

---

### GeoParquet

**Status**: ✅ Fully Supported (v0.3.0+)
**Capabilities**: Info ✓ | Read ✓ | Write ✓

**Format**: Apache Parquet with WKB-encoded geometries and GeoArrow types

**Performance**:
- 🏆 **Best compression**: 6.8x smaller than GeoJSON
- ⚡ **Best throughput**: 3,315 MB/min (11x faster than GeoJSON)
- 💾 **Memory efficient**: &lt;250 MB peak memory
- 🚀 **Production-ready**: Handles 100M+ features

**Use cases**:
- Large-scale datasets (1M+ features)
- Cloud storage optimization
- Analytics pipelines (DuckDB, Spark)
- Data archival
- High-performance processing

**Example**:
```bash
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet
```

**Learn more**: [Working with GeoParquet Tutorial](../tutorial-basics/working-with-geoparquet)

---

## Planned Drivers

These drivers are planned for future releases:

### High Priority (Q1-Q2 2026)

#### FlatGeobuf (FGB)
**Status**: 🚧 Planned (v0.4.0)

**Use cases**:
- Cloud-optimized streaming
- HTTP range requests
- Spatial indexing
- Large dataset handling

---

#### GeoPackage (GPKG)
**Status**: 🚧 Planned (Q1 2026)

**Use cases**:
- SQLite-based geospatial database
- OGC standard format
- Mobile and offline applications
- Multiple layers per file

---

#### ESRI Shapefile
**Status**: 🚧 Planned (Q1 2026)

**Use cases**:
- Industry standard format
- GIS software compatibility
- Legacy data conversion
- Wide tool support

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

## Format Comparison

### File Size (1M features, Microsoft Buildings dataset)

| Format | File Size | Compression vs GeoJSON |
|--------|-----------|----------------------|
| GeoJSON | 114.13 MB | Baseline |
| CSV | 32.11 MB | 3.5x smaller |
| **GeoParquet** | **16.86 MB** | **6.8x smaller** 🏆 |

### Processing Speed (1M features)

| Conversion | Throughput | Duration | Performance |
|------------|-----------|----------|-------------|
| GeoJSON → GeoJSON | 300 MB/min | 23s | Baseline |
| CSV → CSV | 3,211 MB/min | 1s | 10.7x faster |
| **GeoParquet → GeoParquet** | **3,315 MB/min** | **1s** | **11x faster** 🏆 |
| GeoJSON → GeoParquet | 3,804 MB/min | 2s | 12.7x faster |

### Memory Usage

All conversions use **&lt;250 MB** peak memory regardless of dataset size, confirming GeoETL's streaming architecture.

---

## Choosing the Right Driver

### Decision Tree

**For web applications** → **GeoJSON**
- JavaScript-friendly
- Human-readable
- Wide browser support

**For data analysis** → **GeoParquet** (best) or **CSV** (simple)
- Columnar format for fast queries
- Efficient compression
- Modern analytics tools

**For large datasets (1M+ features)** → **GeoParquet** 🏆
- 6.8x smaller than GeoJSON
- 3,315 MB/min throughput
- Minimal memory usage
- Production-ready at scale

**For GIS software** → **Shapefile** or **GeoPackage** (coming soon)
- Industry standard
- Universal support
- Metadata preservation

**For cloud/big data** → **GeoParquet** 🏆
- Columnar storage
- Best compression
- Cloud-optimized
- Modern data stacks

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
# GeoJSON to GeoParquet (6.8x compression!)
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
# CSV to GeoParquet (1.9x compression)
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

### WKT Examples

```
POINT(x y)
LINESTRING(x1 y1, x2 y2, x3 y3)
POLYGON((x1 y1, x2 y2, x3 y3, x1 y1))
MULTIPOINT((x1 y1), (x2 y2))
MULTILINESTRING((...), (...))
MULTIPOLYGON((...), (...))
```

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

---

**Last Updated**: v0.3.0 (2025-11-04)
