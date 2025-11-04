---
sidebar_position: 3
---

# Understanding Drivers

Learn about GeoETL's driver system and supported geospatial formats.

## What are Drivers?

**Drivers** are modules that enable GeoETL to read from and write to different geospatial file formats. Think of them as translators that understand specific file formats.

Each driver has three capabilities:
- **Info**: Read metadata about a dataset
- **Read**: Load data from a file
- **Write**: Save data to a file

## Listing Available Drivers

View all supported drivers:

```bash
geoetl-cli drivers
```

This shows a table with:
- **Short Name**: The driver identifier you use in commands
- **Long Name**: Full descriptive name
- **Info**: Metadata support status
- **Read**: Read capability status
- **Write**: Write capability status

**See also**: [Supported Drivers Reference](../reference/supported-drivers) - Complete driver documentation with examples and comparisons

### Support Status

Each capability has one of three statuses:

| Status | Meaning | Available? |
|--------|---------|------------|
| **Supported** | Fully implemented and working | ✅ Yes |
| **Planned** | Will be implemented in future | 🚧 Soon |
| **Not Supported** | Not planned for implementation | ❌ No |

## Currently Working Drivers

### CSV - Comma Separated Value

**Status**: ✅ Fully Supported (v0.1.0+)

**Use cases**:
- Simple tabular data with geometries
- Excel-compatible format
- Data analysis and visualization

**Geometry format**: WKT (Well-Known Text)

**Example**:
```bash
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt
```

**Sample CSV with WKT**:
```csv
id,name,population,wkt
1,San Francisco,873965,"POINT(-122.4194 37.7749)"
2,New York,8336817,"POINT(-74.006 40.7128)"
```

### GeoJSON - Geographic JSON

**Status**: ✅ Fully Supported (v0.1.0+)

**Use cases**:
- Web mapping applications
- JavaScript/web development
- Human-readable format
- Version control friendly

**Format**: JSON with geometry objects

**Example**:
```bash
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV
```

**Sample GeoJSON**:
```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {"name": "San Francisco"},
      "geometry": {
        "type": "Point",
        "coordinates": [-122.4194, 37.7749]
      }
    }
  ]
}
```

### GeoParquet - Columnar Geospatial Format

**Status**: ✅ Fully Supported (New in v0.3.0!)

**Use cases**:
- Large-scale geospatial data (100M+ features)
- Cloud storage optimization (6.8x smaller than GeoJSON)
- Modern data pipelines (DuckDB, Apache Arrow, Spark)
- High-performance analytics (columnar format)
- Efficient archival storage

**Geometry format**: WKB (Well-Known Binary) with GeoArrow types

**Performance highlights**:
- 🏆 **Best overall performance**: 3,315 MB/min throughput (11x faster than GeoJSON!)
- 📦 **Best compression**: 6.8x smaller than GeoJSON, 1.9x smaller than CSV
- ⚡ **Sub-second processing**: 100k features in &lt;1 second
- 💾 **Memory efficient**: Minimal memory usage with streaming architecture
- 🚀 **Production-ready**: Handles 100M+ features efficiently

**Example**:
```bash
# GeoJSON to GeoParquet (6.8x compression!)
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# CSV to GeoParquet (1.9x compression)
geoetl-cli convert -i data.csv -o data.parquet \
  --input-driver CSV --output-driver GeoParquet \
  --geometry-column WKT

# GeoParquet roundtrip (ultra-fast)
geoetl-cli convert -i input.parquet -o output.parquet \
  --input-driver GeoParquet --output-driver GeoParquet
```

**When to use GeoParquet**:
- ✅ Working with large datasets (1M+ features)
- ✅ Storage efficiency is critical
- ✅ Query performance matters (columnar format enables fast filtering)
- ✅ Integration with modern tools (QGIS, DuckDB, Python/GeoPandas)
- ✅ Cloud storage (smaller files = lower costs)
- ✅ Data archival (best compression + standard format)

**Sample GeoParquet inspection**:
```bash
# GeoParquet is binary format - use tools to inspect:

# parquet-tools (install: pip install parquet-tools)
parquet-tools schema data.parquet
parquet-tools head data.parquet

# DuckDB (SQL queries)
duckdb -c "SELECT * FROM 'data.parquet' LIMIT 10"

# Convert to GeoJSON for viewing
geoetl-cli convert -i data.parquet -o temp.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

**Learn more**:
- 📚 [Working with GeoParquet Tutorial](./working-with-geoparquet) - Complete guide
- 📊 [Benchmark Results](https://github.com/geoyogesh/geoetl/blob/main/bench/README.md#geoparquet-performance) - Performance data
- 🏗️ [Architecture ADR](https://github.com/geoyogesh/geoetl/blob/main/docs/adr/004-streaming-geoparquet-architecture.md) - Technical deep dive
- 🌐 [GeoParquet Specification](https://geoparquet.org/) - Format specification

## Planned Drivers (Coming Soon)

### High Priority (Q1-Q2 2026)

#### GeoPackage (GPKG)
**Status**: 🚧 Planned

**Use cases**:
- SQLite-based geospatial database
- OGC standard format
- Mobile and offline applications
- Large datasets

**Will support**:
- Multiple layers per file
- Spatial indexing
- Attributes and metadata

#### ESRI Shapefile
**Status**: 🚧 Planned

**Use cases**:
- Industry standard format
- GIS software compatibility
- Legacy data conversion
- Wide tool support

**Will support**:
- .shp, .shx, .dbf, .prj files
- Attribute data
- Coordinate systems


#### FlatGeobuf (FGB)
**Status**: 🚧 Planned

**Use cases**:
- Streaming data
- Web mapping
- Large dataset handling
- HTTP range requests

**Will support**:
- Spatial indexing
- Cloud-optimized
- Partial reads

## Driver Catalog by Category

### Vector Formats

**Core Formats** (3 working, more planned):
- ✅ GeoJSON, GeoJSONSeq
- ✅ CSV (with WKT)
- ✅ GeoParquet
- 🚧 ESRI Shapefile
- 🚧 GeoPackage (GPKG)
- 🚧 FlatGeobuf
- 🚧 (Geo)Arrow IPC

### Database Formats

**Planned**:
- 🚧 PostgreSQL/PostGIS
- 🚧 MySQL
- 🚧 SQLite/Spatialite
- 🚧 Oracle Spatial
- 🚧 Microsoft SQL Server
- 🚧 MongoDB

### CAD & Engineering

**Planned**:
- 🚧 AutoCAD DXF
- 🚧 AutoCAD DWG
- 🚧 Microstation DGN
- 🚧 ESRI File Geodatabase

### Web Services

**Planned**:
- 🚧 OGC WFS (Web Feature Service)
- 🚧 OGC API - Features
- 🚧 Carto
- 🚧 Elasticsearch
- 🚧 Google Earth Engine

## Choosing the Right Driver

Use this guide to select the best format:

### For Web Applications
→ **GeoJSON**
- JavaScript-friendly
- Human-readable
- Wide browser support

### For Data Analysis
→ **GeoParquet** (best) or **CSV** (simple)
- Columnar format for fast queries
- Efficient compression
- Modern analytics tools (DuckDB, Arrow, Spark)

### For Large Datasets (1M+ features)
→ **GeoParquet** 🏆
- 6.8x smaller than GeoJSON
- 3,315 MB/min throughput
- Minimal memory usage
- Production-ready at scale

### For GIS Software Compatibility
→ **Shapefile** or **GeoPackage** (coming soon)
- Industry standard
- Universal support
- Metadata preservation

### For Cloud/Big Data
→ **GeoParquet** 🏆
- Columnar storage
- Best compression (6.8x over GeoJSON)
- Cloud-optimized
- Works with modern data stacks

## Using Drivers in Commands

### Basic Syntax

```bash
geoetl-cli convert \
  --input <file> \
  --output <file> \
  --input-driver <DRIVER> \
  --output-driver <DRIVER>
```

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

## Driver Capabilities

### Checking Read Support

Before using a driver as input, verify it supports reading:

```bash
geoetl-cli drivers | grep -i "shapefile"
```

Look for "Supported" in the "Read" column.

### Checking Write Support

Before using a driver as output, verify it supports writing:

```bash
geoetl-cli drivers | grep -i "geojson"
```

Look for "Supported" in the "Write" column.

### Error Handling

If you try to use an unsupported operation:

```bash
# This will fail because GML doesn't support read yet
geoetl-cli convert -i data.gml -o data.geojson \
  --input-driver GML --output-driver GeoJSON
```

Error message:
```
Error: Input driver 'GML' does not support reading.
```

## Geometry Format Support

Different drivers handle geometries differently:

| Driver | Geometry Format | Example |
|--------|----------------|---------|
| GeoJSON | Native GeoJSON | `{"type": "Point", "coordinates": [x, y]}` |
| CSV | WKT (Well-Known Text) | `"POINT(x y)"` |
| **GeoParquet** | **WKB (Well-Known Binary)** | **(columnar binary)** |
| Shapefile | Binary format | (not human-readable) |
| GeoPackage | Binary format | (not human-readable) |

**WKT Examples**:
```
POINT(x y)
LINESTRING(x1 y1, x2 y2, x3 y3)
POLYGON((x1 y1, x2 y2, x3 y3, x1 y1))
MULTIPOINT((x1 y1), (x2 y2))
```

## Future Driver Features

Coming in future releases:

### Phase 2 (Q2 2026)
- Driver auto-detection from file extensions
- More format drivers (10-15 total)
- Enhanced error messages

### Phase 3 (Q3 2026)
- Advanced driver options
- Custom driver plugins
- Format-specific optimizations

### Phase 4 (Q4 2026)
- Cloud storage drivers (S3, Azure, GCS)
- Database connection pooling
- Streaming data sources

## Driver Development

Want to contribute a driver? See:

- [Contributing Guide](https://github.com/geoyogesh/geoetl/blob/main/docs/DEVELOPMENT.md)
- [DataFusion Integration Guide](https://github.com/geoyogesh/geoetl/blob/main/docs/DATAFUSION_GEOSPATIAL_FORMAT_INTEGRATION_GUIDE.md)
- [Driver Implementation ADR](https://github.com/geoyogesh/geoetl/blob/main/docs/adr/0001-high-level-architecture.md)

## Quick Reference

```bash
# List all drivers
geoetl-cli drivers

# Search for specific driver
geoetl-cli drivers | grep -i "parquet"

# Check driver capabilities
geoetl-cli drivers | grep -i "csv"

# Get command help
geoetl-cli convert --help
```

## Key Takeaways

🎯 **What you learned**:
- What drivers are and how they work
- Which drivers are currently available
- How to check driver capabilities
- How to use drivers in commands
- Planned future drivers

🚀 **Skills unlocked**:
- Choosing the right format for your use case
- Understanding driver limitations
- Planning data workflows

## Next Steps

Continue learning:

👉 **Next: [Working with GeoJSON](./working-with-geojson)** - Web-standard format

Or explore:
- [Working with CSV](./working-with-csv) - CSV operations
- [Working with GeoParquet](./working-with-geoparquet) - Modern columnar format

## Need Help?

- **Command help**: `geoetl-cli drivers`
- **GitHub Issues**: [Report problems](https://github.com/geoyogesh/geoetl/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/geoyogesh/geoetl/discussions)
