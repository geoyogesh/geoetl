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

**See also**: [Supported Drivers Reference](../drivers/supported-drivers) - Complete driver documentation with examples and comparisons

### Support Status

Each capability has one of three statuses:

| Status | Meaning | Available? |
|--------|---------|------------|
| **Supported** | Fully implemented and working | ✅ Yes |
| **Planned** | Will be implemented in future | 🚧 Soon |
| **Not Supported** | Not planned for implementation | ❌ No |

## Currently Working Drivers

### CSV - Comma Separated Value

**Status**: ✅ Fully Supported

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

### GeoJSON - Geographic JSON

**Status**: ✅ Fully Supported

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

### GeoParquet - Columnar Geospatial Format

**Status**: ✅ Fully Supported 

**Use cases**:
- Large-scale geospatial data (100M+ features)
- Cloud storage optimization
- Modern data pipelines (DuckDB, Apache Arrow, Spark)
- High-performance analytics (columnar format)
- Efficient archival storage

**Geometry format**: WKB (Well-Known Binary) with GeoArrow types

**Performance highlights**:
- High throughput
- Efficient compression
- Fast processing: Handles large datasets quickly
- Memory efficient: Minimal memory usage with streaming architecture
- Production-ready: Handles 100M+ features efficiently

**Example**:
```bash
# GeoJSON to GeoParquet
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# CSV to GeoParquet
geoetl-cli convert -i data.csv -o data.parquet \
  --input-driver CSV --output-driver GeoParquet \
  --geometry-column WKT

# GeoParquet roundtrip
geoetl-cli convert -i input.parquet -o output.parquet \
  --input-driver GeoParquet --output-driver GeoParquet
```

**Learn more**:
- 📚 [Working with GeoParquet Tutorial](./working-with-geoparquet) - Complete guide
- 📊 [Benchmark Results](https://github.com/geoyogesh/geoetl/blob/main/bench/README.md#geoparquet-performance) - Performance data
- 🏗️ [Architecture ADR](https://github.com/geoyogesh/geoetl/blob/main/docs/adr/004-streaming-geoparquet-architecture.md) - Technical deep dive
- 🌐 [GeoParquet Specification](https://geoparquet.org/) - Format specification

## Planned Drivers (Coming Soon)

### High Priority

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

**Core Formats**:
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

## Future Driver Features

Coming in future releases:

### Phase 2
- Driver auto-detection from file extensions
- More format drivers (10-15 total)
- Enhanced error messages

### Phase 3
- Advanced driver options
- Custom driver plugins
- Format-specific optimizations

### Phase 4
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

## Next Steps

Continue learning:

**Next: [Working with GeoJSON](./working-with-geojson)** - Web-standard format

Or explore:
- [Working with CSV](./working-with-csv) - CSV operations
- [Working with GeoParquet](./working-with-geoparquet) - Modern columnar format

## Need Help?

- **Command help**: `geoetl-cli drivers`
- **GitHub Issues**: [Report problems](https://github.com/geoyogesh/geoetl/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/geoyogesh/geoetl/discussions)
