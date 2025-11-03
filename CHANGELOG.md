# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-11-03

### Added

- **Streaming CSV Architecture** ([ADR 002](docs/adr/002-streaming-csv-architecture.md))
  - Implemented production-ready CSV format support with inline WKT geometry conversion
  - O(1) memory complexity: processes 4.2 GB files in 49.9 MB constant memory
  - Production-ready throughput: 2,266 MB/min (38.2 MB/s)
  - Automatic single-partition write enforcement for proper CSV formatting
  - Inline geometry ↔ WKT conversion during streaming (no buffering)

- **Configurable Batch Size** ([#156](https://github.com/geoyogesh/geoetl/issues/156))
  - Added `--batch-size` CLI parameter for performance tuning
  - Default: 8,192 features (conservative, memory-efficient)
  - Optimal for large files: 262,144 features (1.43x faster based on benchmarking)
  - Users can tune memory/speed tradeoff for their workload

- **Configurable Partitioning Parameters**
  - Added `--read-partitions` CLI parameter to control parallel reading
  - Added `--write-partitions` CLI parameter to control parallel writing
  - CSV and GeoJSON formats automatically override write partitions to 1 with warning
  - Enables future parallel processing optimizations for other formats

- **GeoJSON Incremental Decoder** ([ADR 001](docs/adr/001-streaming-geojson-architecture.md))
  - Implemented state machine-based incremental JSON parsing
  - Handles incomplete JSON chunks across byte stream boundaries
  - Eliminates OOM errors on large files (processes 14.5 GB in 83.7 MB memory)
  - Supports FeatureCollection and newline-delimited GeoJSON formats

- **Comprehensive Benchmarking Infrastructure** ([bench/](bench/))
  - Real-time monitoring script with CPU, memory, disk I/O tracking
  - Automated benchmark suite with JSON result output
  - Data download scripts for Microsoft Buildings dataset (129M features)
  - Systematic testing across multiple dataset sizes (10k, 100k, 1M, Full)
  - Performance regression testing framework

- **Architecture Decision Records (ADRs)**
  - [ADR 001: Streaming GeoJSON Architecture](docs/adr/001-streaming-geojson-architecture.md)
    - Documents streaming implementation with actual benchmark results
    - Memory efficiency: 83.7 MB constant for 14.5 GB files
    - Identifies performance bottleneck: JSON parsing needs 3-7x improvement
  - [ADR 002: Streaming CSV Architecture](docs/adr/002-streaming-csv-architecture.md)
    - Documents production-ready CSV implementation
    - Performance: 7.6x faster than GeoJSON (2,266 MB/min vs 297 MB/min)
    - Validates streaming architecture works excellently for simpler formats
  - [ADR 003: GeoJSON Performance Optimization Strategy](docs/adr/003-geojson-performance-optimization.md)
    - Phased optimization roadmap (Profile → Quick Wins → Structural → Advanced)
    - Target: 1-2 GB/min throughput (3-7x improvement)
    - Evaluation of faster JSON libraries (simd-json, sonic-rs)

- **Factory Pattern for Writers**
  - Implemented `WriterFactory` trait for consistent writer creation
  - Added factory methods to CSV and GeoJSON formats
  - Increased test coverage for writer initialization

### Changed

- **GeoJSON Performance Optimization** (262,144 batch size)
  - Updated default batch_size from 8,192 to 262,144 for optimal performance
  - Throughput increased from 230 MB/min to 297 MB/min (1.29x improvement)
  - Peak memory remains constant at 83.7 MB (O(1) complexity maintained)
  - Applied to both SessionConfig and physical execution plan

- **CSV Format Production-Ready**
  - Throughput: 2,266 MB/min (exceeds production requirements)
  - Memory: 49.9 MB peak for 4.2 GB input (0.012x ratio)
  - CPU: 96.9% average utilization (efficient, not saturated)
  - Status: Recommended for performance-critical workloads

- **Documentation Restructuring**
  - Removed 7 outdated implementation docs (superseded by ADRs)
  - Updated blog post: "Performance Benchmarking Infrastructure" with actual results
  - Enhanced DataFusion integration guide with optimal batch_size recommendations
  - Clear separation: ADRs (architecture), bench/README (procedures), blog (public)

- **Honest Performance Assessment**
  - CSV: ✅ Production-ready (2.3 GB/min)
  - GeoJSON: ⚠️ Memory-efficient but needs 3-7x speed improvement
  - Format comparison documented with evidence-based recommendations

### Performance

**CSV Benchmarks** (Microsoft Buildings: 129M features, 4.2 GB):
- Duration: 1.86 minutes (112 seconds)
- Memory: 49.9 MB peak (constant)
- Throughput: 2,266 MB/min (38.2 MB/s)
- CPU: 96.9% average
- Disk write: 88.2 MB/s
- Status: ✅ Production-ready

**GeoJSON Benchmarks** (Microsoft Buildings: 129M features, 14.5 GB):
- Duration: 49.95 minutes (2,997 seconds)
- Memory: 83.7 MB peak (constant)
- Throughput: 297 MB/min (12.0 MB/s)
- CPU: 99.5% saturated (parsing bottleneck)
- Disk write: 12.0 MB/s
- Status: ⚠️ Needs optimization (target: 1-2 GB/min)

**Key Finding**: 7.6x performance gap between CSV and GeoJSON due to JSON parsing complexity.

### Fixed

- **GeoJSON Schema Inference** - Reduced memory usage from scanning entire file to sampling first 10 MB
- **GeoJSON Reader OOM** - Fixed out-of-memory errors on large files (15+ GB) by implementing streaming decoder
- **CSV Write Partitioning** - Fixed invalid CSV output when write_partitions > 1 (now enforced to 1 with warning)

### Deprecated

- None

### Removed

- **Outdated Documentation** (7 files superseded by ADRs):
  - `docs/GEOJSON_ARCHITECTURE.md` → ADR 001
  - `docs/GEOJSON_STREAMING_FIX.md` → ADR 001
  - `docs/PERFORMANCE_BENCHMARKING.md` → bench/README.md
  - `docs/PERFORMANCE_TUNING.md` → ADRs 001/002
  - `docs/STREAMING_IMPLEMENTATION_SUMMARY.md` → ADR 001
  - `docs/STREAMING_READER_IMPLEMENTATION.md` → ADR 001
  - `docs/streaming-implementation.md` → ADR 001

### Security

- None

## [0.1.2] - 2025-11-01

### Added

- **Custom Error Types**: Implemented comprehensive error handling system with `GeoEtlError` enum
  - Added specialized error types for IO, driver, format, conversion, validation, configuration, data processing, and geometry operations
  - Integrated error types across CLI, core, and operations crates
  - All error handling tests passing
- **Automated Documentation Deployment**: Integrated Cloudflare Pages deployment into release workflow
  - Documentation automatically deploys to production after GitHub release creation
  - Deployed to https://geoetl-web-circleci.pages.dev on every release tag
  - Uses CircleCI with Wrangler CLI for deployment

### Changed

- **Documentation Reorganization**:
  - Removed redundant `docs/USERGUIDE.md` (content already on website at https://geoetl.com)
  - Updated all references in README.md, QUICKREF.md, DEVELOPMENT.md to point to website
  - Moved format-specific documentation to package directories:
    - `docs/formats/csv-*.md` → `crates/formats/datafusion-csv/docs/`
    - `docs/formats/geojson-*.md` → `crates/formats/datafusion-geojson/docs/`
  - Updated `DATAFUSION_GEOSPATIAL_FORMAT_INTEGRATION_GUIDE.md` with new documentation paths

### Removed

- `docs/USERGUIDE.md` - Superseded by documentation website
- `docs/formats/` directory - Documentation moved to respective package directories

## [0.1.0] - 2025-10-31

### Added

- **Initial Project Structure**: Created a new Rust workspace with the initial crates:
  - `geoetl-cli`: The command-line interface for user interaction.
  - `geoetl-core`: The core library containing business logic and driver management.
  - `geoetl-operations`: Crate for handling specific operations like `convert`.
  - Placeholders for `formats` crates (`datafusion-csv`, `datafusion-geojson`, etc.).
- **CLI Framework**:
  - Implemented a robust CLI using `clap` for argument parsing.
  - Added global flags for logging control: `--verbose` for INFO and `--debug` for DEBUG levels.
  - Set up `tracing` for structured logging and `tracing_log` to bridge standard `log` messages.
- **Core Commands**:
  - **`drivers`**: A fully functional command that lists all 68+ available vector format drivers and their capabilities (read, write, info) in a formatted table.
  - **`convert`**: Initial implementation of the format conversion command. It includes argument parsing for input/output paths, driver names, geometry column, and geometry type. It validates that the specified drivers exist and have the required read/write support.
  - **`info`**: A placeholder for a future command to display dataset metadata, with `--detailed` and `--stats` flags.
- **Driver and Operations Logic**:
  - `geoetl-core` now includes a driver registry for managing available format drivers.
  - `geoetl-operations` contains the initial `convert` function, which is called by the CLI.
- **Unit Tests**:
  - Added initial unit tests for the `convert` command handler to ensure correct validation of input/output drivers and their capabilities.
- **Documentation**:
  - Created extensive initial documentation including:
    - `README.md` with project overview, features, and quick start.
    - `VISION.md` outlining the project's long-term goals.
    - `DEVELOPMENT.md` for contributor guidelines.
    - `adr/0001-high-level-architecture.md` detailing the technical architecture.
- **CI/CD**:
  - Set up a basic CircleCI configuration (`config-ci.yml`) for continuous integration.
  - Included a `Makefile` for common development tasks.
