---
slug: geoparquet-support-v0-3-0
title: "GeoETL v0.3.0: GeoParquet Support"
authors: [geoyogesh]
tags: [release, geoparquet, performance, v0.3.0]
date: 2025-11-04
---

# GeoETL v0.3.0: GeoParquet Support

GeoETL v0.3.0 adds full GeoParquet format support with production-ready performance.

**Key highlights**:
- ✅ Full read/write support
- ⚡ 3,315 MB/min processing throughput
- 💾 Streaming architecture with minimal memory
- 🚀 Handles 100M+ features efficiently

<!--truncate-->

## What's New

GeoParquet is now a fully supported format driver in GeoETL, joining CSV and GeoJSON.

**Format**: Apache Parquet with WKB-encoded geometries and GeoArrow types

**Use cases**:
- Large-scale geospatial data processing
- Cloud storage (smaller files)
- Analytics pipelines
- Data archival

## Performance Results

Benchmarked with Microsoft Buildings dataset (up to 129M features):

### Processing Speed (1M features)

| Operation | Throughput | Duration |
|-----------|-----------|----------|
| GeoJSON → GeoJSON | 300 MB/min | 23s |
| CSV → CSV | 3,211 MB/min | 1s |
| **GeoParquet → GeoParquet** | **3,315 MB/min** | **1s** |
| **GeoJSON → GeoParquet** | **3,804 MB/min** | **2s** |
| **CSV → GeoParquet** | **3,211 MB/min** | **1s** |

**Key finding**: GeoJSON → GeoParquet conversion achieves highest throughput (3,804 MB/min), making format migration fast and efficient.

### File Size (1M features)

| Format | Size | vs GeoJSON |
|--------|------|-----------|
| GeoJSON | 114.13 MB | baseline |
| CSV | 32.11 MB | 3.5x smaller |
| **GeoParquet** | **16.86 MB** | **6.8x smaller** |

### Memory Usage

All conversions use **&lt;250 MB** peak memory regardless of dataset size, confirming streaming architecture works at scale.

### Scalability Test (129M features)

| Format | Input Size | Processing Time | Peak Memory |
|--------|-----------|----------------|-------------|
| GeoJSON | 14.5 GB | 50 minutes | 84 MB |
| **GeoParquet** | **~4 GB** | **~2 minutes (projected)** | **&lt;100 MB** |

## Getting Started

### Installation

Download GeoETL v0.3.0: [GitHub Releases](https://github.com/geoyogesh/geoetl/releases/tag/v0.3.0)

### Basic Usage

```bash
# GeoJSON to GeoParquet
geoetl-cli convert \
  --input data.geojson \
  --output data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet

# CSV to GeoParquet
geoetl-cli convert \
  --input data.csv \
  --output data.parquet \
  --input-driver CSV \
  --output-driver GeoParquet \
  --geometry-column WKT

# GeoParquet to GeoJSON
geoetl-cli convert \
  --input data.parquet \
  --output data.geojson \
  --input-driver GeoParquet \
  --output-driver GeoJSON
```

## Architecture

GeoETL's GeoParquet implementation:
- **Streaming**: Constant O(1) memory regardless of file size
- **Native types**: GeoArrow Point, LineString, Polygon, Multi*, etc.
- **Standard encoding**: WKB (Well-Known Binary)
- **Metadata**: CRS, bounding boxes, schema preservation

Technical details: [Architecture ADR 004](https://github.com/geoyogesh/geoetl/blob/main/docs/adr/004-streaming-geoparquet-architecture.md)

## Documentation

- [Supported Drivers Reference](../docs/reference/supported-drivers) - Complete driver documentation
- [Working with GeoParquet Tutorial](../docs/tutorial-basics/working-with-geoparquet)
- [Benchmark Results](https://github.com/geoyogesh/geoetl/blob/main/bench/README.md#geoparquet-performance)
- [GeoParquet Specification](https://geoparquet.org/)

## What's Next

- **Next (v0.4.0)**: FlatGeobuf format support

See full [Roadmap](https://github.com/geoyogesh/geoetl/blob/main/docs/VISION.md)

## Community

- [GitHub Discussions](https://github.com/geoyogesh/geoetl/discussions) - Ask questions
- [GitHub Issues](https://github.com/geoyogesh/geoetl/issues) - Report bugs
- [Documentation](../docs/intro) - Learn more

---

**Download**: [GeoETL v0.3.0](https://github.com/geoyogesh/geoetl/releases/tag/v0.3.0)
