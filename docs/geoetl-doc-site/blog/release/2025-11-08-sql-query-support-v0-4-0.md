---
slug: sql-query-support-v0-4-0
title: "GeoETL v0.4.0: SQL Query Support for Data Transformation"
authors: [geoyogesh]
tags: [release, sql, datafusion, data-transformation, v0.4.0]
date: 2025-11-08
---

**TL;DR**: GeoETL v0.4.0 adds powerful SQL query support via the `--sql` flag, enabling filtering, column selection, aggregations, and transformations during format conversion—no intermediate files needed.

<!--truncate-->

## Why This Release Matters

After establishing production-ready streaming performance (v0.2.0) and comprehensive format support with GeoParquet (v0.3.0), we're adding a critical capability: **in-flight data transformation**.

Previously, users had to:
1. Convert the entire dataset to a new format
2. Load it into a database or GIS tool
3. Filter/transform the data
4. Export back to the desired format

Now you can **filter, select, aggregate, and transform data during conversion** with standard SQL queries. One command, one step, no intermediate files.

This release also includes a major documentation overhaul establishing our documentation website as the single source of truth.

## Headline Features

### 🎯 SQL Query Support

**Problem**: Converting large datasets and then filtering/transforming them wastes disk space, I/O, and time. Users often only need a subset of features or specific columns, but had to convert everything first.

**Solution**: GeoETL now integrates DataFusion's SQL engine directly into the conversion pipeline via the `--sql` flag. Apply WHERE filters, SELECT columns, use GROUP BY aggregations, ORDER BY sorting, and LIMIT results—all during conversion.

**Value**: **Single-step data transformation**. Process only what you need, exactly when you need it.

**Key Capabilities**:
- ✅ **Filtering** - Extract features matching specific criteria with WHERE clauses
- ✅ **Column Selection** - Reduce file size by selecting only needed columns
- ✅ **Aggregations** - Generate summary statistics with GROUP BY
- ✅ **Sorting** - Order results with ORDER BY
- ✅ **Limiting** - Extract top N records with LIMIT
- ✅ **Full SQL Support** - All DataFusion SQL capabilities (JOIN, window functions, etc.)

**How It Works**:

GeoETL automatically infers a table name from your input filename, registers it with DataFusion's SQL engine, and executes your query during the streaming conversion process.

**Example 1: Filter Large Cities**

```bash
geoetl-cli convert \
  -i world_cities.csv \
  -o large_cities.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt \
  --sql "SELECT * FROM world_cities WHERE population > 1000000"
```

**Table name inference**: `world_cities.csv` → table name `"world_cities"`

**Result**: Only cities with population > 1,000,000 are written to the output file.

**Example 2: Select Specific Columns**

```bash
geoetl-cli convert \
  -i full_dataset.geojson \
  -o simplified.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON \
  --sql "SELECT name, population, geometry FROM full_dataset"
```

**Use case**: Reduce a 500 MB GeoJSON file to 50 MB by excluding unnecessary attribute columns.

**Example 3: Aggregate Statistics**

```bash
geoetl-cli convert \
  -i parcels.csv \
  -o zone_summary.csv \
  --input-driver CSV \
  --output-driver CSV \
  --geometry-column wkt \
  --sql "SELECT zone_type, COUNT(*) as count, SUM(area) as total_area
         FROM parcels
         GROUP BY zone_type"
```

**Use case**: Generate zoning summary statistics from raw parcel data.

**Example 4: Top N with ORDER BY + LIMIT**

```bash
geoetl-cli convert \
  -i buildings.geojson \
  -o top_10_tallest.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON \
  --sql "SELECT * FROM buildings ORDER BY height DESC LIMIT 10"
```

**Use case**: Extract the 10 tallest buildings for a visualization.

**Example 5: Custom Table Names**

```bash
geoetl-cli convert \
  -i very_long_complex_filename_2024_v2.csv \
  -o output.csv \
  --input-driver CSV \
  --output-driver CSV \
  --geometry-column wkt \
  --table-name data \
  --sql "SELECT * FROM data WHERE category = 'residential'"
```

**Why**: When dealing with complex filenames, use `--table-name` to override the inferred table name for simpler SQL queries.

**Common Workflows**:

**Workflow 1: Filter and Transform Pipeline**

```bash
# Extract North American cities, select columns, sort by population
geoetl-cli convert \
  -i global_cities.csv \
  -o north_america_cities.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt \
  --sql "SELECT name, country, population, wkt
         FROM global_cities
         WHERE country IN ('USA', 'Canada', 'Mexico')
         ORDER BY population DESC"
```

**Workflow 2: Data Reduction for Web Visualization**

```bash
# Simplify large datasets for web maps
geoetl-cli convert \
  -i all_buildings.geojson \
  -o significant_buildings.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON \
  --sql "SELECT name, type, geometry
         FROM all_buildings
         WHERE floor_count > 5 OR landmark = true"
```

**Technical Implementation**:

- **Table Name Inference**: Automatically extracts table name from input filename stem (e.g., `cities.csv` → `"cities"`)
- **Custom Override**: Use `--table-name` flag to override inferred table name
- **Streaming Execution**: SQL query executes during streaming conversion (constant memory usage)
- **DataFusion Integration**: Full SQL capabilities via Apache Arrow DataFusion
- **Backward Compatible**: All existing commands work unchanged (SQL is optional)

**Documentation**: Complete SQL reference with 5 examples and 3 workflows at [convert command docs](https://geoetl.dev/docs/programs/convert#example-7-filter-data-with-sql-query).

### 📚 Documentation Restructure

**Problem**: Documentation was scattered across README, multiple markdown files, and the website, leading to duplication and inconsistency.

**Solution**: Established the documentation website as the single source of truth with comprehensive reorganization.

**Value**: **Easier discovery, better learning experience, no duplication**.

**Major Changes** ([commit 8526f07](https://github.com/geoyogesh/geoetl/commit/8526f07)):

- ✅ **Single Source of Truth** - All documentation consolidated to website
- ✅ **Community Section** - Added changelog, contributing guide, roadmap
- ✅ **Driver Documentation** - Dedicated pages for GeoJSON, CSV, GeoParquet
- ✅ **Getting Started Guides** - Installation → First Conversion → Format Tutorials
- ✅ **Programs Reference** - Detailed command documentation (convert, drivers, info)
- ✅ **FAQ & Glossary** - Improved discoverability of common questions
- ✅ **Troubleshooting Guide** - Enhanced error resolution resources

**Impact**: 4,442 additions, 1,628 deletions across 38 files

### 📝 Historical Release Blog Posts

**Problem**: Earlier releases (v0.1.2, v0.2.0, v0.3.1) didn't have corresponding blog posts, making it harder for users to understand the project's evolution.

**Solution**: Added missing release blog posts following our blog post guidelines ([commit 6aa66f6](https://github.com/geoyogesh/geoetl/commit/6aa66f6)).

**Value**: **Complete project history** for new users and contributors.

## Other Improvements & Fixes

### Changed

- `convert` operation now accepts optional `sql_query` and `table_name_override` parameters
- `initialize_context` function now returns both `SessionContext` and inferred/custom table name
- All existing tests updated to include new optional SQL parameters
- Documentation accuracy: Updated driver count to reflect accurate "3 drivers (GeoJSON, CSV, GeoParquet)" with note about 68+ planned drivers via GDAL integration

### Testing

Added comprehensive test coverage:
- ✅ 7 integration tests for SQL functionality
  - SQL filtering with WHERE clauses
  - Column selection with SELECT
  - Aggregations with GROUP BY
  - Sorting with ORDER BY
  - Custom table name overrides
  - Invalid SQL query error handling
  - Multi-step filter and transform workflows
- ✅ Unit test for custom catalog name parameter
- ✅ All existing tests updated for backward compatibility

## ⚠️ Breaking Changes

None - all new parameters are optional and backward compatible.

Existing commands continue to work without modification:

```bash
# This still works exactly as before
geoetl-cli convert \
  -i input.geojson \
  -o output.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

## Community & Contributors

Thank you to everyone who:
- Requested SQL query support and data transformation capabilities
- Provided feedback on documentation organization
- Contributed to the broader DataFusion and Arrow ecosystem

We're grateful for the community's engagement and support!

## The Future: What's Next?

We have an exciting roadmap ahead:

**v0.5.0 (Target: Q1 2026)**:
- 🎯 **FlatGeobuf format support** - Cloud-optimized geospatial format with spatial indexing
- 🎯 **Shapefile read support** - For legacy data compatibility
- 🎯 **GeoJSON performance optimization** - Target 3-7x speedup for large files
- 🎯 **Format auto-detection** - Automatic driver inference from file extensions

**v0.6.0 and beyond**:
- 🚀 **GeoPackage support** - SQLite-based vector data
- 🚀 **Arrow IPC support** - Zero-copy data exchange
- 🚀 **OSM support** - OpenStreetMap data processing
- 🚀 **Spatial operations** - Buffer, intersection, union via SQL functions

See our full [Roadmap](https://geoetl.dev/docs/community/roadmap) for details.

## How to Upgrade

### Installation

**From source**:
```bash
git clone https://github.com/geoyogesh/geoetl.git
cd geoetl
git checkout v0.4.0
cargo build --release

# Binary at: target/release/geoetl-cli
```

### Verify Installation

```bash
$ geoetl-cli --version
geoetl-cli 0.4.0

$ geoetl-cli convert --help
# Should show new --sql and --table-name options
```

## Get Started Today

### Try SQL Queries

**Filter large datasets**:

```bash
# Extract only major cities from a global dataset
geoetl-cli convert \
  -i world_cities.csv \
  -o major_cities.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt \
  --sql "SELECT * FROM world_cities WHERE population > 500000"
```

**Select specific columns**:

```bash
# Reduce file size by selecting only needed fields
geoetl-cli convert \
  -i full_data.geojson \
  -o minimal_data.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON \
  --sql "SELECT id, name, geometry FROM full_data"
```

**Generate summary statistics**:

```bash
# Aggregate data during conversion
geoetl-cli convert \
  -i parcels.csv \
  -o summary.csv \
  --input-driver CSV \
  --output-driver CSV \
  --geometry-column wkt \
  --sql "SELECT zone_type, COUNT(*) as count, AVG(area) as avg_area
         FROM parcels
         GROUP BY zone_type"
```

## Documentation

- 📖 [SQL Query Examples](https://geoetl.dev/docs/programs/convert#example-7-filter-data-with-sql-query)
- 📖 [Convert Command Reference](https://geoetl.dev/docs/programs/convert)
- 📖 [Getting Started Guide](https://geoetl.dev/docs/getting-started/first-conversion)
- 📖 [Full Changelog](https://geoetl.dev/docs/community/changelog#040---2025-11-08)
- 📖 [Contributing Guide](https://geoetl.dev/docs/community/contributing)

## Get Involved

We need your help to build the future of GeoETL!

**High-Priority Contributions**:
- 🎯 **FlatGeobuf driver** - Cloud-optimized format implementation
- 🎯 **Shapefile reader** - Legacy format support
- 🎯 **GeoJSON optimization** - Performance improvements
- 🎯 **Spatial SQL functions** - ST_Buffer, ST_Intersection, etc.

**How to Contribute**:
- ⭐ **Star us on GitHub**: [github.com/geoyogesh/geoetl](https://github.com/geoyogesh/geoetl)
- 🐛 **Report bugs**: [Open an issue](https://github.com/geoyogesh/geoetl/issues)
- 💬 **Discuss features**: [GitHub Discussions](https://github.com/geoyogesh/geoetl/discussions)
- 🔧 **Contribute code**: Check [DEVELOPMENT.md](https://github.com/geoyogesh/geoetl/blob/main/docs/DEVELOPMENT.md)
- 📣 **Spread the word**: Share your SQL query workflows with GeoETL

---

**Download**: [GeoETL v0.4.0](https://github.com/geoyogesh/geoetl/releases/tag/v0.4.0)

*Questions or feedback? Join the conversation on [GitHub Discussions](https://github.com/geoyogesh/geoetl/discussions)!*
