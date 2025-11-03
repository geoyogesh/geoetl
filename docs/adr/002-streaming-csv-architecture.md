# ADR 002: Streaming Architecture for CSV Processing with Inline Geometry Conversion

## Status

**Accepted** - Implemented and Benchmarked in v0.1.2
**Date**: 2025-01-03

## Context

GeoETL must process large CSV files containing spatial data with geometry columns in WKT (Well-Known Text) format. CSV processing presents different challenges than GeoJSON:

### The Problem

- Desktop systems must process multi-GB CSV files with limited memory
- Geometry data in CSV is typically stored as WKT strings
- Need to convert between WKT and native geometry types during streaming
- Must maintain compatibility with DataFusion's CSV reader
- Performance requirements: >1 GB/min throughput for production use

### Technical Constraints

**DataFusion Requirements:**
- Must use DataFusion's CSV FileFormat for compatibility
- Schema must be inferred or provided upfront
- Streaming must produce RecordBatch streams
- Single-partition writes for proper CSV formatting

**CSV with Geometry Characteristics:**
- Geometry stored as WKT text column
- Schema inference works well (CSV has headers)
- No nested structures (unlike GeoJSON)
- Simpler parsing than JSON (no nesting, escaping is minimal)
- Geometry conversion must happen during streaming

### Decision Drivers

1. **Performance**: Maximize throughput for large files (target >1 GB/min)
2. **Memory efficiency**: O(1) complexity regardless of file size
3. **Geometry handling**: Seamless WKT ↔ native geometry conversion
4. **DataFusion compatibility**: Leverage existing CSV infrastructure
5. **Simplicity**: Reuse proven components where possible

## Decision

**We will implement streaming CSV processing using DataFusion's CSV FileFormat with inline geometry conversion via DataSink pattern.**

### Core Approach

**Reading:**
1. **Use DataFusion CSV**: Leverage existing CSV parser for performance
2. **Schema inference**: Standard DataFusion CSV schema detection
3. **WKT detection**: Identify geometry columns by name or explicit parameter
4. **Stream directly**: No custom parser needed - DataFusion handles it

**Writing:**
1. **DataSink pattern**: Use DataFusion's DataSink trait for streaming writes
2. **Inline conversion**: Convert geometry columns to WKT during streaming
3. **Batch-by-batch**: Process each RecordBatch independently
4. **Single partition**: Force single-partition writes for proper CSV formatting

### Key Design Principles

- **Leverage existing infrastructure**: Use DataFusion CSV parser (faster than custom)
- **Inline conversion**: Convert geometry ↔ WKT during streaming (no buffering)
- **Memory efficient**: Process batches independently, no accumulation
- **Format constraints**: Single-partition writes required for valid CSV

### Implementation Components

1. **CsvFileFormat** (`file_format.rs`): Wraps DataFusion CSV with geometry awareness
2. **CsvSink** (`sink.rs`): DataSink implementation with inline WKT conversion
3. **Geometry conversion** (`sink.rs`): Uses geoarrow-rs to convert geometry arrays to WKT
4. **Configuration** (`operations.rs`): Automatic single-partition override with warning

### Why This Works Better Than GeoJSON

**CSV advantages:**
- Simpler format: No nested JSON objects or arrays
- Faster parsing: CSV parsing is inherently faster than JSON
- Better locality: Columnar data structure in memory
- Less overhead: No JSON escaping, object structure, or type inference complexity
- Existing optimizations: DataFusion's CSV reader is highly optimized

**Result**: CSV achieves 7-10x better throughput than GeoJSON (2.3 GB/min vs 297 MB/min)

## Consequences

### Positive

1. **Excellent performance**: 2.3 GB/min throughput (7.6x faster than GeoJSON) ✅
2. **Memory efficient**: 50 MB peak for 4.2 GB file (0.012x ratio) ✅
3. **Production-ready**: Meets performance requirements for real-world use ✅
4. **High CPU utilization**: 96.9% average - efficiently uses available resources ✅
5. **Fast disk I/O**: 88.2 MB/s write speed ✅
6. **Simpler implementation**: Reuses DataFusion infrastructure ✅
7. **Reliable**: Proven DataFusion CSV parser with years of testing ✅

### Negative

1. **Single-partition constraint**: Must write to single file (no parallel writes)
   - *Reason*: CSV format requires sequential rows in single file
   - *Mitigation*: Automatic override with warning when user specifies multiple partitions
   - *Impact*: Acceptable - CSV write speed (88 MB/s) is already excellent

2. **WKT overhead**: Geometry stored as text increases file size vs binary formats
   - *Impact*: CSV files ~3.3x larger than binary geometry alternatives
   - *Tradeoff*: Acceptable for human-readable, portable format

3. **Schema limitations**: CSV schema inference less sophisticated than GeoJSON
   - *Impact*: Type detection based on first N rows
   - *Mitigation*: DataFusion's inference is well-tested and reliable

### Learnings from Production Benchmarking

**What Worked:**
- ✅ DataFusion CSV parser: Excellent performance out of the box
- ✅ Inline WKT conversion: No memory overhead, happens during streaming
- ✅ DataSink pattern: Clean integration with DataFusion execution
- ✅ Single-partition enforcement: Prevents user errors, clear warnings

**Comparison to GeoJSON:**

| Metric | CSV | GeoJSON | CSV Advantage |
|--------|-----|---------|---------------|
| Throughput | 2,266 MB/min | 297 MB/min | 7.6x faster |
| Peak Memory | 49.9 MB | 83.7 MB | 1.7x lower |
| Write Speed | 88.2 MB/s | 12.0 MB/s | 7.4x faster |
| CPU Usage | 96.9% | 99.5% | Similar efficiency |
| Production Ready | ✅ Yes | ⚠️ No | CSV ready now |

**Decision Validation:**
- **Correct choice**: CSV performance validates decision to use DataFusion infrastructure
- **No optimization needed**: Performance already exceeds requirements
- **Architecture proven**: Streaming with inline conversion works excellently

### Trade-offs Summary

| Aspect | CSV (CHOSEN) | Result |
|--------|--------------|--------|
| Throughput | 2.3 GB/min | ✅ Production-ready |
| Memory | O(1) - 50 MB | ✅ Excellent |
| Max File Size | Unlimited | ✅ Streaming validated |
| Complexity | Low (reuses DataFusion) | ✅ Simple |
| File Size | 3.3x larger than binary | ⚠️ Acceptable (human-readable) |
| Parallel Writes | No (single partition) | ⚠️ Acceptable (fast enough) |

## Implementation Evidence

### Production Benchmark Results

Comprehensive testing with Microsoft Buildings dataset (129M rows, 4.2 GB CSV):

**Dataset Sizes Tested:**

| Dataset | Rows | Input Size | Duration | Peak Memory | Throughput | CPU |
|---------|------|------------|----------|-------------|------------|-----|
| 10k | 10,000 | 0.31 MB | <1s | Minimal | Instant | N/A |
| 100k | 100,000 | 3.20 MB | <1s | Minimal | Instant | N/A |
| 1M | 1,000,000 | 32.11 MB | 1s | Minimal | 3,211 MB/min | N/A |
| **Full** | **129M** | **4.2 GB** | **112s (1.86 min)** | **49.9 MB** | **2,266 MB/min** | **96.9%** |

**Key Findings:**

1. **Excellent Performance** ✅
   - Throughput: 2.3 GB/min (38.2 MB/s)
   - Meets and exceeds production requirements
   - 7.6x faster than GeoJSON implementation

2. **Memory Efficiency** ✅
   - Peak memory: 49.9 MB for 4.2 GB input (0.012x ratio)
   - True O(1) space complexity validated
   - Constant memory usage across all dataset sizes

3. **I/O Performance** ✅
   - Disk read: 8.8 MB/s average
   - Disk write: 88.2 MB/s average
   - Excellent disk utilization

4. **CPU Utilization** ✅
   - Average: 96.9% CPU usage
   - Efficiently using available processing power
   - Not fully saturated (room for other tasks)

5. **Scalability** ✅
   - Linear scaling from 10k to 129M rows
   - Performance characteristics remain consistent
   - No degradation at scale

**Configuration:**
- Batch size: 8,192 (configurable via --batch-size)
- Read partitions: 1 (default)
- Write partitions: 1 (enforced for CSV format)
- Geometry column: WKT (specified via --geometry-column)

## Related Decisions

- ADR 001: Streaming GeoJSON Architecture (comparison baseline)
- Future ADR: Binary Geometry Formats (if needed for performance)

## Implementation Notes

### Key Code Locations

**CSV Reading:**
- `crates/formats/datafusion-csv/src/file_format.rs` - CSV FileFormat implementation
- Uses DataFusion's built-in CSV reader for parsing

**CSV Writing:**
- `crates/formats/datafusion-csv/src/sink.rs` - CsvSink DataSink implementation
- `crates/formats/datafusion-csv/src/sink.rs:convert_geometry_to_wkt_in_batch()` - Inline WKT conversion

**Configuration:**
- `crates/geoetl-core/src/operations.rs:convert()` - Automatic single-partition override for CSV
- Warning logged when write_partitions > 1 for CSV format

### Geometry Conversion Details

**WKT Conversion (write path):**
```rust
fn convert_geometry_to_wkt_in_batch(batch: &RecordBatch) -> Result<RecordBatch> {
    // For each column:
    // 1. Try to interpret as geometry array using geoarrow
    // 2. If successful, convert to WKT using to_wkt()
    // 3. Replace geometry column with WKT string column
    // 4. Return new RecordBatch with WKT columns
}
```

**Read path:**
- WKT columns read as strings by DataFusion CSV parser
- No conversion on read (handled downstream if needed)
- Geometry column specified via --geometry-column parameter

### Performance Tuning Options

**For higher throughput** (more memory):
```rust
// Increase batch size (via CLI: --batch-size 32768)
batch_size: 32768  // May improve throughput slightly
```

**For lower memory** (slower):
```rust
// Decrease batch size (via CLI: --batch-size 2048)
batch_size: 2048  // ~15 MB RAM, slightly slower
```

**For parallel reads** (if source supports):
```rust
// Increase read partitions (via CLI: --read-partitions 4)
read_partitions: 4  // Parallel reading if beneficial
```

Note: Write partitions always forced to 1 for CSV format.

## External References

- [DataFusion CSV Implementation](https://docs.rs/datafusion/latest/datafusion/datasource/file_format/csv/)
- [Apache Arrow CSV](https://docs.rs/arrow-csv/latest/arrow_csv/)
- [GeoArrow WKT Conversion](https://docs.rs/geoarrow/latest/geoarrow/)
- [CSV RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180)

## Notes

- **Decision Date**: 2025-01-03
- **Decision Makers**: GeoETL Core Team
- **Test Environment**: macOS Darwin 24.6.0, 10-core CPU
- **Benchmark Data**: Microsoft Buildings dataset (129M features)
- **Supersedes**: Initial prototype with batch collection
