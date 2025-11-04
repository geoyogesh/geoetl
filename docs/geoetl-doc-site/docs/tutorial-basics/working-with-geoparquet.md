---
sidebar_position: 6
---

# Working with GeoParquet

Learn how to use GeoParquet, the modern columnar format for geospatial data.

## What is GeoParquet?

**GeoParquet** is a columnar storage format for geospatial data that combines:
- Apache Parquet's efficient columnar storage
- WKB-encoded geometries
- GeoArrow types for native geometry representation
- Rich metadata (CRS, bounding boxes)

**Why use GeoParquet?**
- 🏆 **Best performance**: 3,315 MB/min (11x faster than GeoJSON)
- 📦 **Best compression**: 6.8x smaller than GeoJSON
- 💾 **Memory efficient**: Minimal memory usage
- 🚀 **Production-ready**: Handles 100M+ features
- 🔧 **Modern ecosystem**: Works with DuckDB, QGIS, Arrow

## Quick Start

### Converting to GeoParquet

**From GeoJSON** (most common):
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

**To GeoJSON** (for web use):
```bash
geoetl-cli convert \
  --input data.parquet \
  --output data.geojson \
  --input-driver GeoParquet \
  --output-driver GeoJSON
```

## Performance Benefits

### Compression Comparison (1M features)

| Format | File Size | Compression vs GeoJSON |
|--------|-----------|----------------------|
| GeoJSON | 114.13 MB | Baseline |
| CSV | 32.11 MB | 3.5x smaller |
| **GeoParquet** | **16.86 MB** | **6.8x smaller** 🏆 |

**Storage savings example**:
- 1M features: Save 97 MB (GeoJSON → GeoParquet)
- 100M features: Save 9.7 GB
- 1B features: Save 97 GB

### Speed Comparison (1M features)

| Conversion | Throughput | Duration |
|------------|-----------|----------|
| GeoJSON → GeoJSON | 300 MB/min | 23s |
| CSV → CSV | 3,211 MB/min | 1s |
| **GeoParquet → GeoParquet** | **3,315 MB/min** | **1s** 🏆 |
| GeoJSON → GeoParquet | 3,804 MB/min | 2s |

## Use Cases

### 1. Storage Optimization

**Problem**: Large GeoJSON files consuming cloud storage

**Solution**: Convert to GeoParquet for 6.8x space savings

```bash
# Before: 1 GB GeoJSON
# After: 147 MB GeoParquet (6.8x smaller)

geoetl-cli convert -i large.geojson -o compressed.parquet \
  --input-driver GeoJSON --output-driver GeoParquet
```

**Benefits**:
- Lower storage costs
- Faster uploads/downloads
- Reduced egress charges

### 2. Analytics Pipeline

**Problem**: Need to query geospatial data with DuckDB/SQL

**Solution**: Use columnar GeoParquet format

```bash
# Convert to GeoParquet
geoetl-cli convert -i cities.geojson -o cities.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# Query with DuckDB
duckdb -c "
  SELECT name, population
  FROM 'cities.parquet'
  WHERE population &gt; 1000000
  ORDER BY population DESC
"
```

**Benefits**:
- Fast column-oriented queries
- Efficient filtering (predicate pushdown)
- No need to load entire dataset

### 3. Large Dataset Processing

**Problem**: Processing 100M+ features efficiently

**Solution**: GeoParquet's streaming architecture

```bash
# Process 129M features with minimal memory
geoetl-cli convert \
  --input buildings_129m.geojson \
  --output buildings_129m.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet

# Expected: ~4 minutes, &lt;100 MB memory
```

**Benefits**:
- Constant memory usage
- Linear scaling
- Production-ready performance

### 4. Data Archival

**Problem**: Long-term storage of geospatial datasets

**Solution**: GeoParquet's efficient compression

**Benefits**:
- 6.8x space savings
- Standard format (Apache Parquet)
- Schema preservation
- Metadata included (CRS, bbox)

## Working with Other Tools

### QGIS

GeoParquet files can be opened directly in QGIS:

1. **Layer → Add Layer → Add Vector Layer**
2. Select your `.parquet` file
3. QGIS automatically reads GeoParquet metadata

### DuckDB

Query GeoParquet files with SQL:

```sql
-- Install spatial extension
INSTALL spatial;
LOAD spatial;

-- Query GeoParquet
SELECT * FROM 'data.parquet' LIMIT 10;

-- Spatial query
SELECT name, ST_AsText(geometry)
FROM 'data.parquet'
WHERE ST_Within(geometry, ST_MakeEnvelope(-180, -90, 180, 90));
```

### Python (GeoPandas)

```python
import geopandas as gpd

# Read GeoParquet
gdf = gpd.read_parquet('data.parquet')

# Process
filtered = gdf[gdf['population'] &gt; 100000]

# Write back
filtered.to_parquet('filtered.parquet')
```

### parquet-tools

Inspect GeoParquet files:

```bash
# Install parquet-tools
pip install parquet-tools

# View schema
parquet-tools schema data.parquet

# View first 10 rows
parquet-tools head data.parquet

# Get metadata
parquet-tools meta data.parquet
```

## Common Patterns

### Web + Storage Workflow

**Pattern**: Serve GeoJSON for web, store as GeoParquet

```bash
# 1. Store original as GeoParquet (efficient)
geoetl-cli convert -i source.geojson -o archive.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# 2. Generate web version when needed
geoetl-cli convert -i archive.parquet -o web_version.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

**Benefits**:
- Save 6.8x storage space
- Generate web formats on-demand
- Single source of truth

### Batch Processing

**Pattern**: Process multiple files efficiently

```bash
# Convert directory of GeoJSON to GeoParquet
for file in data/*.geojson; do
  output="output/$(basename "$file" .geojson).parquet"
  geoetl-cli convert -i "$file" -o "$output" \
    --input-driver GeoJSON --output-driver GeoParquet
done
```

## Limitations & Workarounds

### 1. Not Human-Readable

**Limitation**: GeoParquet is binary format

**Workaround**: Use tools for inspection
```bash
# parquet-tools
parquet-tools head data.parquet

# DuckDB
duckdb -c "SELECT * FROM 'data.parquet' LIMIT 10"

# Convert to GeoJSON for viewing
geoetl-cli convert -i data.parquet -o temp.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

### 2. CSV Export with bbox Columns

**Limitation**: GeoParquet files from ogr2ogr may have bbox struct columns that CSV cannot represent

**Workaround**: Roundtrip via GeoJSON
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

## Performance Tips

### 1. Use GeoParquet for Large Datasets

**Rule of thumb**:
- **&lt; 10k features**: Any format works
- **10k - 1M features**: GeoParquet recommended
- **&gt; 1M features**: GeoParquet strongly recommended

### 2. Convert Once, Use Many Times

Store in GeoParquet, convert to other formats as needed:

```bash
# Store once
geoetl-cli convert -i source.geojson -o master.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# Generate formats on-demand
geoetl-cli convert -i master.parquet -o web.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

### 3. Leverage Columnar Format

For analytics, GeoParquet enables:
- Column pruning (read only needed columns)
- Predicate pushdown (filter before loading)
- Compression per column

## Benchmarks

Real-world performance with Microsoft Buildings dataset:

| Dataset | Features | GeoJSON Size | GeoParquet Size | Conversion Time |
|---------|----------|--------------|-----------------|----------------|
| Small | 10,000 | 1.14 MB | 0.16 MB (7.1x) | &lt;1s |
| Medium | 100,000 | 11.40 MB | 1.68 MB (6.8x) | &lt;1s |
| Large | 1,000,000 | 114.13 MB | 16.86 MB (6.8x) | 2s |
| **Full** | **129,000,000** | **~15 GB** | **~2.2 GB (6.8x)** | **~4 min** |

**Memory usage**: &lt;100 MB for all conversions (constant)

**See full benchmarks**: [bench/README.md](https://github.com/geoyogesh/geoetl/blob/main/bench/README.md#geoparquet-performance)

## Key Takeaways

🎯 **What you learned**:
- GeoParquet is the best format for large datasets
- 6.8x compression over GeoJSON, 1.9x over CSV
- 3,315 MB/min throughput (production-ready)
- Works with modern data tools (DuckDB, QGIS, Arrow)

🚀 **Skills unlocked**:
- Converting to/from GeoParquet
- Optimizing storage with columnar format
- Building modern geospatial data pipelines
- Leveraging GeoParquet in analytics workflows

## Next Steps

Continue learning:

👉 **Next: [Error Handling & Troubleshooting](./troubleshooting)** - Debug and resolve issues

Or explore:
- [Understanding Drivers](./understanding-drivers) - Driver capabilities
- [Performance Benchmarking Blog](https://github.com/geoyogesh/geoetl/blob/main/bench/README.md) - Deep dive

## References

- [GeoParquet Specification](https://geoparquet.org/)
- [Apache Parquet Documentation](https://parquet.apache.org/)
- [GeoArrow Specification](https://geoarrow.org/)
- [Architecture ADR 004](https://github.com/geoyogesh/geoetl/blob/main/docs/adr/004-streaming-geoparquet-architecture.md)

## Need Help?

- **Command help**: `geoetl-cli convert --help`
- **GitHub Issues**: [Report problems](https://github.com/geoyogesh/geoetl/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/geoyogesh/geoetl/discussions)
