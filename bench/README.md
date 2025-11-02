# GeoETL Benchmarking

Simple performance benchmarking for GeoETL.

## Setup

### 1. Download Test Data

```bash
cd bench
./data_download.sh
```

Downloads Microsoft Buildings dataset (15 GB GeoJSON, 129M features) from https://github.com/geoarrow/geoarrow-data/

### 2. Build GeoETL

```bash
cd ..
cargo build --release
```

## Running Benchmarks

### Basic Usage

```bash
./run_benchmark.sh "<command>" [test-name]
```

The script wraps any command with performance monitoring and saves results to `bench/results/`.

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

### Step 1: Create Small Test Data

```bash
# Create small test file (3 features)
cat > bench/data/final/test_small.geojson << 'EOF'
{"type":"FeatureCollection","features":[
  {"type":"Feature","geometry":{"type":"Point","coordinates":[-122.4,37.8]},"properties":{"id":1,"name":"Point A"}},
  {"type":"Feature","geometry":{"type":"Point","coordinates":[-122.5,37.9]},"properties":{"id":2,"name":"Point B"}},
  {"type":"Feature","geometry":{"type":"Point","coordinates":[-122.6,38.0]},"properties":{"id":3,"name":"Point C"}}
]}
EOF
```

### Step 2: Create Medium Test Data

```bash
# Create medium test file (10,000 features) using Python
python3 << 'EOF'
import json

features = []
for i in range(10000):
    features.append({
        "type": "Feature",
        "geometry": {
            "type": "Point",
            "coordinates": [-122.4 + (i * 0.0001), 37.8 + (i * 0.0001)]
        },
        "properties": {
            "id": i,
            "name": f"Point {i}",
            "value": i * 10
        }
    })

geojson = {
    "type": "FeatureCollection",
    "features": features
}

with open('bench/data/final/test_medium.geojson', 'w') as f:
    json.dump(geojson, f)

print(f"Created medium test file with {len(features)} features")
EOF
```

### Step 3: Run Standard Benchmarks

```bash
# Small dataset benchmark (3 features)
bench/run_benchmark.sh "./target/release/geoetl-cli convert --verbose \
  --input bench/data/final/test_small.geojson \
  --output bench/output/test_small.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson_small"

# Medium dataset benchmark (10,000 features)
bench/run_benchmark.sh "./target/release/geoetl-cli convert --verbose \
  --input bench/data/final/test_medium.geojson \
  --output bench/output/test_medium.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson_medium"

# Large dataset benchmark (129M features, 15 GB - takes ~45 minutes)
bench/run_benchmark.sh "./target/release/geoetl-cli convert --verbose \
  --input bench/data/final/microsoft-buildings_point.geojson \
  --output bench/output/microsoft-buildings_streaming.geojson \
  --input-driver GeoJSON \
  --output-driver GeoJSON" \
  "geojson_large"
```

### Step 4: View Results

```bash
# View all benchmark results
echo "=== Benchmark Results Summary ===" && echo "" && \
for test in geojson_small geojson_medium geojson_large; do \
  echo "Test: $test" && cat bench/results/${test}.json && echo ""; \
done && \
ls -lh bench/output/

# View individual results
cat bench/results/geojson_small.json
cat bench/results/geojson_medium.json
cat bench/results/geojson_large.json

# View logs
cat bench/results/geojson_small.log
cat bench/results/geojson_medium.log
cat bench/results/geojson_large.log
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
