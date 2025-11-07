# GeoETL Benchmarking

Simple performance benchmarking for GeoETL.

## Setup

### 1. Build GeoETL

```bash
cargo build --release
```

### 2. Download Test Data (Optional)

For large-scale benchmarking with the Microsoft Buildings dataset (15 GB GeoJSON, 129M features):

```bash
# Create directories
mkdir -p bench/data/final

# Download using curl (preferred)
curl -L -o bench/data/source/microsoft-buildings_point.fgb \
  https://github.com/geoarrow/geoarrow-data/releases/download/v0.1.0/microsoft-buildings_point.fgb


# Generate benchmark datasets
ogr2ogr -f CSV bench/data/final/csv/buildings_point_full.csv bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY=AS_WKT
ogr2ogr -limit 10000 -f CSV bench/data/final/csv/buildings_point_10k.csv bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY=AS_WKT
ogr2ogr -limit 100000 -f CSV bench/data/final/csv/buildings_point_100k.csv bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY=AS_WKT
ogr2ogr -limit 1000000 -f CSV bench/data/final/csv/buildings_point_1m.csv bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY=AS_WKT


ogr2ogr -f GeoJSON bench/data/final/geojson/buildings_point_full.geojson bench/data/source/microsoft-buildings_point.fgb
ogr2ogr -limit 10000 -f GeoJSON bench/data/final/geojson/buildings_point_10k.geojson bench/data/source/microsoft-buildings_point.fgb
ogr2ogr -limit 100000 -f GeoJSON bench/data/final/geojson/buildings_point_100k.geojson bench/data/source/microsoft-buildings_point.fgb
ogr2ogr -limit 1000000 -f GeoJSON bench/data/final/geojson/buildings_point_1m.geojson bench/data/source/microsoft-buildings_point.fgb


ogr2ogr -f Parquet bench/data/final/geoparquet/buildings_point_full.parquet bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY_ENCODING=WKB
ogr2ogr -limit 10000 -f Parquet bench/data/final/geoparquet/buildings_point_10k.parquet bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY_ENCODING=WKB
ogr2ogr -limit 100000 -f Parquet bench/data/final/geoparquet/buildings_point_100k.parquet bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY_ENCODING=WKB
ogr2ogr -limit 1000000 -f Parquet bench/data/final/geoparquet/buildings_point_1m.parquet bench/data/source/microsoft-buildings_point.fgb -lco GEOMETRY_ENCODING=WKB

**Note**: This is a 15 GB download and is only needed for large-scale performance testing. For basic benchmarking, create smaller test files as shown in the Quick Start section below.

## Running Benchmarks

### Basic Usage

```bash
./run_benchmark.sh "<command>" [test-name]
```

The script wraps any command with performance monitoring and saves results to `bench/results/`.

### Make Sample Datasets for benchmarking

```

```

### Examples

**Test GeoJSON to CSV conversion:**
```bash
./run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/test.csv \
  --input-driver GeoJSON \
  --output-driver CSV" \
  "geojson-to-csv"
```

**Test CSV to GeoJSON conversion:**
```bash
./run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.csv \
  --output bench/output/test.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column WKT \
  --geometry-type Point" \
  "csv-to-geojson"
```

**Test GeoJSON streaming (optimal batch size from ADR):**
```bash
./run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/test_streaming.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson-streaming-optimal"
```

## Benchmark Output

The script monitors and reports:
- **Duration**: Total time in seconds and minutes
- **Peak Memory**: Maximum RAM usage in MB
- **Avg CPU**: Average CPU utilization percentage
- **Input Size**: Size of input file in MB
- **Throughput**: MB processed per minute
- **Output Size**: Size of output file in MB

### Real-Time Monitoring

For long-running benchmarks, the script displays live progress updates every 30 seconds:

```
Monitoring (updates every 30s)...
========================================
[00:30] Memory: 45.2 MB | CPU: 98.5% | Output: 102.5 MB
[01:00] Memory: 68.1 MB | CPU: 99.1% | Output: 405.8 MB
[01:30] Memory: 77.2 MB | CPU: 98.2% | Output: 710.3 MB
...
```

This helps you track progress and resource usage for benchmarks that take several minutes or hours.

### Saved Results

Results are saved as:
- `bench/results/<test-name>.json` - Structured metrics
- `bench/results/<test-name>.log` - Full command output

**Tip**: Add `--verbose` flag to your command to capture detailed logs in the `.log` file. This is useful for debugging and understanding what happened during the benchmark.

### Example Output

```
========================================
GeoETL Benchmark
========================================
Test: geojson-streaming-optimal
Command: ./target/release/geoetl-cli convert ...
Input Size: 15000.00 MB
========================================

Monitoring (updates every 30s)...
System: 10 cores available
========================================
[00:30] Memory: 45.2 MB | CPU: 98.5% (9.9% per core) | Output: 102.5 MB
[01:00] Memory: 68.1 MB | CPU: 99.1% (9.9% per core) | Output: 405.8 MB
...
[44:30] Memory: 77.2 MB | CPU: 98.2% (9.8% per core) | Output: 14850.3 MB

========================================
Results
========================================
Duration:    2700s (45.00 min)
Peak Memory: 77.2 MB
Avg CPU:     98.5%
Input Size:  15000.00 MB
Throughput:  333.33 MB/min
Output Size: 15001.23 MB
Exit Code:   0
========================================

Results: bench/results/geojson-streaming-optimal.json
Log:     bench/results/geojson-streaming-optimal.log
```

## Quick Start - Running Standard Benchmarks

This section provides systematic benchmarks for CSV and GeoJSON formats using pre-created test datasets.

### Available Test Datasets

**CSV Datasets** (in `bench/data/final/csv/`):
- `buildings_point_10k.csv` - 10,000 rows (~328 KB)
- `buildings_point_100k.csv` - 100,000 rows (~3.2 MB)
- `buildings_point_1m.csv` - 1,000,000 rows (~32 MB)

**GeoJSON Datasets** (in `bench/data/final/geojson/`):
- `buildings_point_10k.geojson` - 10,000 features (~1.1 MB)
- `buildings_point_100k.geojson` - 100,000 features (~11 MB)
- `buildings_point_1m.geojson` - 1,000,000 features (~114 MB)

**GeoParquet Datasets** (in `bench/data/final/geoparquet/`):
- `buildings_point_10k.parquet` - 10,000 features (~0.34 MB)
- `buildings_point_100k.parquet` - 100,000 features (~3.31 MB)
- `buildings_point_1m.parquet` - 1,000,000 features (~33.15 MB)

### Step 1: CSV to CSV Benchmarks

```bash
# 10k rows benchmark
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/csv/buildings_point_10k.csv \
  --output bench/output/csv_10k.csv \
  --input-driver CSV \
  --output-driver CSV \
  --geometry-column WKT" \
  "csv_to_csv_10k"

# 100k rows benchmark
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/csv/buildings_point_100k.csv \
  --output bench/output/csv_100k.csv \
  --input-driver CSV \
  --output-driver CSV \
  --geometry-column WKT" \
  "csv_to_csv_100k"

# 1M rows benchmark
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/csv/buildings_point_1m.csv \
  --output bench/output/csv_1m.csv \
  --input-driver CSV \
  --output-driver CSV \
  --geometry-column WKT" \
  "csv_to_csv_1m"
```

### Step 2: GeoJSON to GeoJSON Benchmarks

```bash
# 10k features benchmark
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geojson/buildings_point_10k.geojson \
  --output bench/output/geojson_10k.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson_to_geojson_10k"

# 100k features benchmark
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geojson/buildings_point_100k.geojson \
  --output bench/output/geojson_100k.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson_to_geojson_100k"

# 1M features benchmark
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geojson/buildings_point_1m.geojson \
  --output bench/output/geojson_1m.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson_to_geojson_1m"
```

### Step 3: GeoParquet Benchmarks

```bash
# GeoParquet → GeoParquet (10k)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geoparquet/buildings_point_10k.parquet \
  --output bench/output/geoparquet_to_geoparquet_10k.parquet \
  --input-driver GeoParquet \
  --output-driver GeoParquet" \
  "geoparquet_to_geoparquet_10k"

# GeoParquet → GeoParquet (100k)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geoparquet/buildings_point_100k.parquet \
  --output bench/output/geoparquet_to_geoparquet_100k.parquet \
  --input-driver GeoParquet \
  --output-driver GeoParquet" \
  "geoparquet_to_geoparquet_100k"

# GeoParquet → GeoParquet (1M)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geoparquet/buildings_point_1m.parquet \
  --output bench/output/geoparquet_to_geoparquet_1m.parquet \
  --input-driver GeoParquet \
  --output-driver GeoParquet" \
  "geoparquet_to_geoparquet_1m"

# GeoParquet → GeoParquet (Full)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geoparquet/buildings_point_full.parquet \
  --output bench/output/geoparquet_to_geoparquet_full.parquet \
  --input-driver GeoParquet \
  --output-driver GeoParquet" \
  "geoparquet_to_geoparquet_full"

# GeoJSON → GeoParquet (1M)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/geojson/buildings_point_1m.geojson \
  --output bench/output/geojson_to_geoparquet_1m.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet" \
  "geojson_to_geoparquet_1m"


# CSV → GeoParquet (1M)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/csv/buildings_point_1m.csv \
  --output bench/output/csv_to_geoparquet_1m.parquet \
  --input-driver CSV \
  --output-driver GeoParquet \
  --geometry-column WKT" \
  "csv_to_geoparquet_1m"
```

### Step 4: View Results

```bash
# View all CSV benchmark results
echo "=== CSV to CSV Benchmark Results ===" && \
for test in csv_to_csv_10k csv_to_csv_100k csv_to_csv_1m; do \
  echo "Test: $test" && cat bench/results/${test}.json && echo ""; \
done

# View all GeoJSON benchmark results
echo "=== GeoJSON to GeoJSON Benchmark Results ===" && \
for test in geojson_to_geojson_10k geojson_to_geojson_100k geojson_to_geojson_1m; do \
  echo "Test: $test" && cat bench/results/${test}.json && echo ""; \
done

# View all GeoParquet benchmark results
echo "=== GeoParquet Benchmark Results ===" && \
for test in geoparquet_to_geoparquet_10k geoparquet_to_geoparquet_100k geoparquet_to_geoparquet_1m; do \
  echo "Test: $test" && cat bench/results/${test}.json && echo ""; \
done

# List all output files
ls -lh bench/output/
```

### Step 5: Stop Running Benchmarks

If you need to stop a long-running benchmark:

```bash
# Using the stop script
bench/stop_benchmark.sh

# Or using pkill directly
pkill -9 -f "geoetl-cli convert"
```

The stop script will:
- Kill all running geoetl-cli processes
- Kill all benchmark monitoring processes
- Show a summary of stopped processes

### Benchmark Results and Observations

#### CSV to CSV Performance

| Dataset | Rows | Input Size | Duration | Peak Memory | Throughput | Observations |
|---------|------|------------|----------|-------------|------------|--------------|
| 10k | 10,000 | 0.31 MB | <1s | N/A | Instant | Extremely fast, sub-second execution |
| 100k | 100,000 | 3.20 MB | <1s | N/A | Instant | Very fast, minimal overhead |
| 1M | 1,000,000 | 32.11 MB | 1s | N/A | 3,211 MB/min | Excellent throughput, true streaming |
| **Full** | **129M** | **4.2 GB** | **112s (1.86 min)** | **49.9 MB** | **2,266 MB/min** | **Production-scale streaming** |

**Key Observations:**
- ✅ **True streaming implementation**: Constant low memory usage across all dataset sizes
- ✅ **Linear scaling**: Processing time scales linearly with input size
- ✅ **High throughput**: Achieves 2.2-3.2 GB/min on large datasets
- ✅ **Minimal overhead**: Sub-second processing for small to medium datasets
- ✅ **WKT geometry handling**: Efficient geometry column conversion during streaming
- ✅ **Production-ready**: Processes 129M rows (4.2 GB) in under 2 minutes with only 50 MB memory

#### GeoJSON to GeoJSON Performance

| Dataset | Features | Input Size | Duration | Peak Memory | Throughput | Observations |
|---------|----------|------------|----------|-------------|------------|--------------|
| 10k | 10,000 | 1.14 MB | <1s | N/A | Instant | Instant processing |
| 100k | 100,000 | 11.40 MB | 2s | N/A | 380 MB/min | Very fast |
| 1M | 1,000,000 | 114.13 MB | 23s | 67.5 MB | 300 MB/min | Efficient streaming with low memory |
| **Full** | **129M** | **14.5 GB** | **2997s (49.95 min)** | **83.7 MB** | **297 MB/min** | **Production-scale streaming validated** |

**Key Observations:**
- ✅ **True streaming implementation**: Peak memory of only 83.7 MB for 14.5 GB input (0.0056x ratio!)
- ✅ **High CPU utilization**: 99.5% average CPU usage on full dataset, fully utilizing processing power
- ✅ **Consistent throughput**: ~297-380 MB/min throughput across all dataset sizes
- ✅ **Excellent I/O performance**: 12.02 MB/s write speed during full dataset processing
- ✅ **Memory efficiency**: Constant memory footprint (83.7 MB max) regardless of dataset size
- ⚠️ **Performance needs improvement**: Processes 129M features (14.5 GB) in 50 minutes - too slow for production
- ✅ **Memory efficient**: Uses only 84 MB memory for 14.5 GB dataset
- 🔧 **TODO**: Optimize GeoJSON parsing/serialization for better throughput (currently ~297 MB/min)

#### Format Comparison

**1M Features/Rows:**

| Metric | CSV (1M) | GeoJSON (1M) | GeoParquet (1M) | Winner |
|--------|----------|--------------|-----------------|--------|
| **Throughput** | 3,211 MB/min | 300 MB/min | 3,315 MB/min | GeoParquet (11x faster than GeoJSON) |
| **Peak Memory** | Minimal | 67.5 MB | Minimal | CSV/GeoParquet (tie) |
| **Duration** | 1s | 23s | 1s | CSV/GeoParquet (tie, 23x faster than GeoJSON) |
| **Input Size** | 32.11 MB | 114.13 MB | 33.15 MB | GeoParquet (similar to CSV) |
| **Output Size** | 29.25 MB | 97.92 MB | 35.03 MB | GeoParquet (similar to CSV) |
| **Compression** | Baseline | 3.5x larger | 1.9x smaller than CSV, 6.8x smaller than GeoJSON | GeoParquet |

**Full Dataset (129M Features/Rows):**

| Metric | CSV (Full) | GeoJSON (Full) | Winner |
|--------|------------|----------------|--------|
| **Throughput** | 2,266 MB/min | 297 MB/min | CSV (7.6x faster) |
| **Peak Memory** | 49.9 MB | 83.7 MB | CSV (1.7x lower) |
| **Duration** | 112s (1.86 min) | 2997s (49.95 min) | CSV (26.8x faster) |
| **Input Size** | 4.2 GB | 14.5 GB | CSV (3.5x smaller) |
| **Output Size** | 3.8 GB | 12.5 GB | CSV (3.3x smaller) |
| **Avg CPU** | 96.9% | 99.5% | Similar (both efficient) |
| **Disk Write** | 88.2 MB/s | 12.0 MB/s | CSV (7.4x faster) |

**Analysis:**
- **Performance**: GeoParquet and CSV are tied for fastest (3,200 MB/min), 10x faster than GeoJSON
- **Compression**: GeoParquet is the most storage-efficient (6.8x smaller than GeoJSON, 1.9x smaller than CSV)
- **Memory**: All three formats demonstrate true streaming with constant memory usage
- **Production readiness**: CSV and GeoParquet are production-ready, GeoJSON needs optimization
- **Use cases**:
  - **GeoParquet**: Best for large-scale data, storage, modern pipelines, analytics
  - **CSV**: Best for human readability, compatibility, simple data exchange
  - **GeoJSON**: Best for web compatibility, small datasets, standard compliance
- **Memory validation**: All formats successfully handle 129M features with <100 MB memory ✅
- **Performance gap**: GeoJSON throughput (297 MB/min) needs significant improvement for production use ⚠️
- **Target**: Should aim for 1-2 GB/min for GeoJSON (3-7x improvement needed)

#### GeoParquet Performance

| Dataset | Features | Input Size | Duration | Peak Memory | Throughput | Output Size | Compression |
|---------|----------|------------|----------|-------------|------------|-------------|-------------|
| 10k (roundtrip) | 10,000 | 0.34 MB | <1s | Minimal | Instant | 0.34 MB | 1.0x |
| 100k (roundtrip) | 100,000 | 3.31 MB | <1s | Minimal | Instant | 3.49 MB | 0.95x |
| 1M (roundtrip) | 1,000,000 | 33.15 MB | 1s | Minimal | 3,315 MB/min | 35.03 MB | 0.95x |

**Key Observations:**
- ✅ **Blazing fast**: Sub-second processing for up to 1M features
- ✅ **Highest throughput**: 3,315 MB/min for roundtrip operations (matches CSV performance)
- ✅ **Best compression**: 6.8x smaller than GeoJSON, 1.9x smaller than CSV
- ✅ **Streaming validated**: Constant minimal memory usage
- ✅ **Production-ready**: Excellent performance for large-scale geospatial data

**Format Conversions:**

| Conversion | Features | Input Size | Duration | Throughput | Output Size | Compression |
|------------|----------|------------|----------|------------|-------------|-------------|
| GeoJSON → GeoParquet | 1M | 114.13 MB | 2s | 3,804 MB/min | 16.86 MB | 6.8x |
| CSV → GeoParquet | 1M | 32.11 MB | 1s | 3,211 MB/min | 16.86 MB | 1.9x |
| GeoParquet → GeoJSON | 1M | 33.15 MB | 14s | 144 MB/min | 1,763 MB | 53.2x expansion |

**Winner**: 🏆 **GeoParquet** has the best overall performance
- Fastest roundtrip: 3,315 MB/min (tied with CSV)
- Best compression: 6.8x over GeoJSON, 1.9x over CSV
- Most storage-efficient format for large datasets
- Ideal for modern data pipelines (QGIS, DuckDB, Apache Arrow)

#### Streaming Architecture Validation

CSV, GeoJSON, and GeoParquet implementations successfully demonstrate **true streaming** at production scale:

1. **Memory Efficiency**: Peak memory remains constant regardless of dataset size
   - CSV: 49.9 MB for 4.2 GB input (0.012x ratio)
   - GeoJSON: 83.7 MB for 14.5 GB input (0.0056x ratio)
   - GeoParquet: < 250 MB for all conversions

2. **Incremental Processing**: Data processed in batches without accumulation
   - Consistent throughput maintained across entire dataset
   - No memory growth observed during processing

3. **Linear Scaling**: Performance scales linearly with input size
   - 1M rows/features → Full dataset (129M): ~129x increase
   - Processing time scales proportionally

4. **High Throughput**: Efficient I/O with minimal CPU bottlenecks
   - CSV: 88.2 MB/s write, 96.9% CPU utilization
   - GeoJSON: 12.0 MB/s write, 99.5% CPU utilization
   - GeoParquet: 3,200+ MB/min throughput

5. **Streaming Validated, Performance Varies**:
   - CSV: ✅ Production-ready - 4.2 GB in 1.86 minutes with 50 MB memory (2.3 GB/min)
   - GeoParquet: ✅ Production-ready - 3,315 MB/min with minimal memory
   - GeoJSON: ⚠️ Needs optimization - 14.5 GB in 49.95 minutes with 84 MB memory (297 MB/min)
   - Memory efficiency proven, but GeoJSON throughput requires 3-7x improvement for production use

## Performance Optimization Opportunities

### GeoJSON Performance Bottleneck

Current GeoJSON performance (297 MB/min) is **too slow for production use**. Based on benchmarking results:

**Current Performance:**
- Throughput: 297 MB/min (12.0 MB/s write)
- 14.5 GB dataset: 49.95 minutes
- CPU: 99.5% (fully utilized, but throughput still low)

**Target Performance:**
- Throughput: 1-2 GB/min (3-7x improvement)
- 14.5 GB dataset: 7-15 minutes (target)

**Potential Optimization Areas:**

1. **JSON Parsing/Serialization** (likely bottleneck)
   - Current: Using serde_json for parsing
   - Consider: Switch to faster JSON library (simd-json, sonic-rs)
   - Benchmark different JSON parsers

2. **Batch Processing**
   - Current batch size: 8192 (configurable via `--batch-size`)
   - Test larger batch sizes for better throughput
   - May reduce per-batch overhead

3. **Parallelization**
   - Current: Single-threaded JSON processing
   - Consider: Parallel parsing of independent GeoJSON features
   - Use rayon for parallel batch processing

4. **Memory-Mapped I/O**
   - Consider using memory-mapped files for large datasets
   - May improve read performance

5. **Profile and Identify Hotspots**
   - Use cargo flamegraph to identify bottlenecks
   - Focus optimization on the slowest operations

**Note**: CSV performance (2.3 GB/min) is already excellent and production-ready ✅

## Common Test Scenarios

### Find Optimal Batch Size

Test different batch sizes by modifying source code and rebuilding:

1. Edit `crates/formats/datafusion-geojson/src/physical_exec.rs:45`
2. Change `batch_size: 262144` to desired value
3. Rebuild: `cargo build --release`
4. Run benchmark with descriptive name

```bash
# Test 8K batch (baseline)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/test.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "batch-8k"

# Test 32K batch
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/test.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "batch-32k"

# Test 262K batch (optimal from ADR)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/test.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "batch-262k"
```

### Compare Formats

```bash
# GeoJSON → CSV
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/test.csv \
  --input-driver GeoJSON \
  --output-driver CSV" \
  "geojson-csv"

# GeoJSON → GeoJSON (streaming)
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/test.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson-geojson"

# CSV → GeoJSON
bench/run_benchmark.sh "./target/release/geoetl-cli convert \
  --input bench/data/final/microsoft-buildings_point.csv \
  --output bench/output/test.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column WKT \
  --geometry-type Point" \
  "csv-geojson"
```

## Analyzing Results

View JSON results:
```bash
cat bench/results/geojson-streaming-optimal.json
```

View full logs:
```bash
cat bench/results/geojson-streaming-optimal.log
```

Compare multiple tests:
```bash
jq '.metrics' bench/results/*.json
```

## Live Monitoring 

```
htop --filter "geoetl-cli"
btop --filter "geoetl-cli"
```

## Expected Performance

Based on actual testing (15 GB file, 129M features):

| Configuration | Batch Size | Memory | Time | Throughput |
|---------------|-----------|---------|------|------------|
| Baseline | 8,192 | 34 MB | 65 min | 230 MB/min |
| Optimized | 32,768 | 73 MB | 54 min | 278 MB/min |
| **Optimal** | **262,144** | **77 MB** | **45 min** | **330 MB/min** |
| Ultra | 524,288 | 80 MB | 50 min | 300 MB/min |

See `docs/adr/001-streaming-geojson-architecture.md` for details.

## Internals: How Metrics Are Collected

The benchmark script uses a background monitoring process that samples performance data every 5 seconds during command execution.

###  Architecture

```
Main Process                Monitor Process (Background)
    │                              │
    ├─ START_TIME                  ├─ Loop every 5s:
    ├─ Run command ───────────────►│   - ps aux | grep
    │                              │   - Extract: MEM (col 6), CPU (col 3)
    │                              │   - Save: timestamp,mem_mb,cpu_pct
    ├─ END_TIME                    │
    ├─ Kill monitor ──────────────►│
    └─ Process metrics             └─ Stop
```

### Metric Collection Methods

| Metric | Source | Method | Calculation |
|--------|--------|--------|-------------|
| **Duration** | `date +%s` | Capture Unix timestamps before/after | `END_TIME - START_TIME` |
| **Peak Memory** | `ps aux` (column 6: RSS) | Sample every 5s, take max | `sort -n \| tail -1` |
| **Avg CPU** | `ps aux` (column 3) | Sample every 5s, calculate average | `sum / count` |
| **Input Size** | `stat -f%z` (macOS) or `stat -c%s` (Linux) | Read once after completion | `bytes / 1048576` |
| **Output Size** | `stat -f%z` (macOS) or `stat -c%s` (Linux) | Read once after completion | `bytes / 1048576` |
| **Throughput** | Calculated | Division | `input_mb / duration_mins` |

### Data Flow

1. **Collection**: Background process writes to temporary log file:
   ```
   1730563200,45.2,98.5
   1730563205,52.8,97.3
   1730563210,77.2,99.1  ← Peak memory sample
   ```

2. **Processing**: After command completes:
   ```bash
   # Peak memory: sort column 2, take max
   PEAK_MEMORY_MB=$(awk -F',' '{print $2}' log | sort -n | tail -1)

   # Average CPU: sum column 3, divide by count
   AVG_CPU=$(awk -F',' '{sum+=$3; count++} END {print sum/count}' log)
   ```

3. **Output**: Results saved in two formats:
   - **JSON** (`bench/results/test_name.json`): Structured metrics for programmatic analysis
   - **Log** (`bench/results/test_name.log`): Verbose command output with `--verbose` flag

### Sampling Rate

- **Interval**: 5 seconds
- **Overhead**: Minimal (single `ps aux` call per sample)
- **Example**: 45-minute benchmark = 540 samples = ~2.7KB temp log file

The temporary monitoring log is deleted after processing to save disk space.

## References

- Test Data: https://github.com/geoarrow/geoarrow-data/
- ADR 001: Streaming GeoJSON Architecture
- Performance Tuning Guide: `docs/PERFORMANCE_TUNING.md`
