# ADR 003: GeoJSON Performance Optimization Strategy

## Status

**Proposed** - Performance gap identified, optimization work needed
**Date**: 2025-01-03

## Context

Production benchmarking revealed that GeoJSON processing performance is **not production-ready**, despite successful streaming architecture implementation.

### The Performance Gap

**Current State (ADR 001 implementation):**
- Throughput: 297 MB/min (12.0 MB/s)
- Processing time: 49.95 minutes for 14.5 GB file
- CPU utilization: 99.5% (fully saturated)
- Memory: 83.7 MB (excellent ✅)

**Comparison to CSV (ADR 002):**
- CSV throughput: 2,266 MB/min (38.2 MB/s)
- CSV is 7.6x faster for same dataset
- Both use streaming architecture, but CSV much faster

**Production Requirements:**
- Target throughput: 1-2 GB/min (60-120 MB/s)
- Improvement needed: 3-7x current performance
- Memory budget: <200 MB (current 84 MB is acceptable)

### The Problem

Despite achieving O(1) memory complexity and full CPU utilization, GeoJSON processing is CPU-bound on JSON parsing/serialization:

1. **Bottleneck identified**: JSON parsing consumes ~85% of processing time
2. **Current approach**: Single-threaded serde_json parsing
3. **Result**: Even with 99.5% CPU usage, throughput is too slow
4. **Impact**: 15 GB files take 50 minutes (should be 7-15 minutes)

### Why This Matters

**User Impact:**
- Large datasets (10+ GB) take too long to process
- Blocks interactive workflows
- Makes GeoJSON less viable than CSV for large datasets
- Limits adoption for production ETL pipelines

**Business Impact:**
- GeoJSON is common format in geospatial industry
- Poor performance compared to competitors (ogr2ogr, etc.)
- Cannot claim "production-ready" for GeoJSON processing
- CSV performance proves our streaming architecture works - just need to optimize JSON path

### Decision Drivers

1. **Close the performance gap**: Get GeoJSON within 2-3x of CSV performance
2. **Maintain memory efficiency**: Keep O(1) memory usage (<200 MB)
3. **Evidence-based optimization**: Profile before changing, measure after
4. **Incremental approach**: Make targeted improvements, not wholesale rewrites
5. **Preserve correctness**: Maintain compatibility with RFC 7946

## Decision

**We will systematically optimize GeoJSON processing through profiling, algorithm improvements, and library evaluation - aiming for 3-7x performance improvement.**

### Optimization Strategy

#### Phase 1: Profiling and Analysis (Immediate)

**Goal**: Identify exact bottlenecks in JSON processing path

**Actions:**
1. **Profile with cargo flamegraph**:
   ```bash
   cargo flamegraph --bin geoetl-cli -- convert \
     --input bench/data/final/geojson/buildings_point_1m.geojson \
     --output /tmp/out.geojson \
     --input-driver GeoJSON \
     --output-driver GeoJSON
   ```

2. **Analyze hotspots**:
   - JSON parsing time (read path)
   - JSON serialization time (write path)
   - Arrow conversion overhead
   - String allocation patterns

3. **Measure baseline**:
   - Current: 297 MB/min on full dataset
   - Record detailed metrics for comparison

**Expected outcome**: Clear data on where time is spent

#### Phase 2: Quick Wins (Short-term, 1-2 weeks)

**Goal**: 1.5-2x improvement from low-hanging fruit

**Option A: Faster JSON Library**

Evaluate alternatives to serde_json:

| Library | Approach | Expected Speedup | Compatibility |
|---------|----------|------------------|---------------|
| **simd-json** | SIMD-accelerated parsing | 2-3x | Partial (may need adjustments) |
| **sonic-rs** | Rust port of sonic (Go) | 2-4x | Unknown |
| **json** | Simplified parser | 1.5-2x | Good |

**Decision criteria:**
- Must maintain RFC 7946 compatibility
- Should handle streaming (partial JSON documents)
- Benchmark with our actual data (not microbenchmarks)

**Implementation:**
1. Create feature flag for JSON library selection
2. Benchmark each library with 1M feature dataset
3. Choose fastest that maintains compatibility
4. Run full test suite to verify correctness

**Option B: Reduce String Allocations**

Profile showed potential for optimization:
- Reuse string buffers where possible
- Use `Cow<str>` for zero-copy string handling
- Consider string interning for repeated values

**Expected impact**: 10-20% improvement

#### Phase 3: Structural Optimizations (Medium-term, 1-2 months)

**Goal**: 2-3x improvement from architectural changes

**Option A: Parallel Parsing**

**Current**: Single-threaded JSON parsing
**Proposed**: Parse independent features in parallel

**Approach:**
1. Split input stream into feature chunks
2. Parse each feature in parallel using rayon
3. Collect results into batches
4. Maintain order if required

**Challenges:**
- GeoJSON FeatureCollection is sequential
- Need to identify feature boundaries without full parse
- Synchronization overhead for batch assembly

**Feasibility**: Medium - requires careful design

**Expected impact**: 2-3x on multi-core systems

**Option B: SIMD Optimizations**

**Current**: Scalar processing
**Proposed**: SIMD for JSON parsing

**Approach:**
- Use simd-json library (Phase 2)
- Explore custom SIMD for geometry parsing
- Vectorize coordinate array processing

**Expected impact**: 1.5-2x if not already using simd-json

**Option C: Streaming Write Optimizations**

**Current**: Convert each batch to JSON, write sequentially
**Proposed**: Buffer writes, reduce system calls

**Approach:**
- Larger write buffers (currently using default)
- Batch multiple features before writing
- Memory-mapped output file

**Expected impact**: 20-40% improvement on write path

#### Phase 4: Advanced Optimizations (Long-term, 3-6 months)

**Goal**: Approach CSV-level performance (additional 1.5-2x)

**Option: JIT Compilation**

Generate specialized parsers for inferred schema:
- Compile custom parser based on schema
- Eliminate generic JSON handling
- Direct field extraction

**Feasibility**: Low - high complexity, uncertain ROI

**Option: Alternative Format**

**Reality check**: If we cannot get GeoJSON within 2-3x of CSV:
- Document GeoJSON as "memory-efficient but slower"
- Recommend CSV for performance-critical workloads
- Consider GeoParquet or FlatGeobuf as alternatives

### Phased Approach Rationale

**Why incremental:**
- Each phase builds on previous measurements
- Can stop when performance is "good enough"
- Lower risk than wholesale rewrite
- Maintains working system throughout

**Success criteria:**
- Phase 1 complete: Have flamegraph and bottleneck analysis
- Phase 2 success: Reach 450-600 MB/min (1.5-2x improvement)
- Phase 3 success: Reach 900-1200 MB/min (3-4x improvement)
- Final goal: 1-2 GB/min (3-7x total improvement)

## Consequences

### If We Optimize (CHOSEN)

**Positive:**
- ✅ GeoJSON becomes production-ready for large files
- ✅ Competitive with other tools (ogr2ogr, etc.)
- ✅ Users can choose GeoJSON for JSON ecosystem compatibility
- ✅ Validates streaming architecture works for all formats
- ✅ Learning applies to future format implementations

**Negative:**
- ⚠️ Engineering effort required (2-6 months total)
- ⚠️ Risk of introducing bugs during optimization
- ⚠️ May need to maintain multiple JSON library backends
- ⚠️ Testing burden increases with optimizations

**Mitigations:**
- Comprehensive benchmarking at each phase
- Feature flags for experimental optimizations
- Extensive test suite to catch regressions
- Can stop optimization if diminishing returns

### If We Don't Optimize (Rejected)

**What we'd lose:**
- GeoJSON remains slow (50 min for 15 GB)
- Users forced to use CSV or other tools
- Competitive disadvantage vs other ETL tools
- Streaming architecture appears "broken" for JSON

**Why rejected:**
- CSV proves our architecture works
- JSON is too important in geospatial ecosystem
- Performance gap is solvable problem
- Small investment for major user benefit

## Implementation Plan

### Phase 1: Profiling (Week 1)

**Deliverables:**
1. Flamegraph of full dataset processing
2. Detailed breakdown of time spent in each component
3. Baseline metrics document
4. Prioritized list of optimization opportunities

**Success metric**: Clear data showing where to optimize

### Phase 2: Quick Wins (Weeks 2-3)

**Deliverables:**
1. Benchmark of alternative JSON libraries
2. Implementation of fastest library behind feature flag
3. String allocation optimizations
4. New benchmark showing improvement

**Success metric**: 1.5-2x throughput improvement (450-600 MB/min)

### Phase 3: Structural Changes (Months 2-3)

**Deliverables:**
1. Parallel parsing implementation (if beneficial)
2. Write buffering optimizations
3. Comprehensive benchmark suite
4. Performance regression tests

**Success metric**: 3-4x total throughput improvement (900-1200 MB/min)

### Phase 4: Final Push (Months 4-6, if needed)

**Deliverables:**
1. Advanced optimizations based on Phase 3 results
2. Production testing with real user workloads
3. Performance documentation
4. Updated ADR 001 with new numbers

**Success metric**: 1-2 GB/min throughput (production-ready)

## Acceptance Criteria

**Minimum viable (Phase 2):**
- Throughput: ≥450 MB/min (1.5x improvement)
- Memory: ≤200 MB peak
- Tests: All existing tests pass
- Correctness: RFC 7946 compliance maintained

**Production-ready (Phase 3):**
- Throughput: ≥900 MB/min (3x improvement)
- Memory: ≤200 MB peak
- CPU: >95% utilization maintained
- Scaling: Linear performance to 100+ GB files

**Ideal (Phase 4):**
- Throughput: 1-2 GB/min (3-7x improvement)
- Within 2-3x of CSV performance
- Memory: ≤150 MB peak
- Best-in-class GeoJSON performance

## Measurement and Success

**How we'll know it's working:**

1. **Continuous benchmarking**:
   - Run full benchmark suite after each optimization
   - Track metrics in version control (bench/results/)
   - Compare to baseline from ADR 001

2. **Performance regression tests**:
   - Add benchmark to CI pipeline
   - Fail if throughput drops >10% from baseline
   - Alert on memory increases

3. **User validation**:
   - Beta test with real user datasets
   - Measure wall-clock time improvements
   - Gather feedback on production readiness

4. **Comparison to competitors**:
   - Benchmark against ogr2ogr
   - Compare to DuckDB GeoJSON support
   - Aim for competitive or better performance

## Related Decisions

- ADR 001: Streaming GeoJSON Architecture (performance issue identified)
- ADR 002: Streaming CSV Architecture (proves streaming works, sets performance bar)
- Future ADR: JSON Library Selection (Phase 2 outcome)
- Future ADR: Parallel Processing Architecture (Phase 3, if implemented)

## Open Questions

1. **Which JSON library** will provide best performance for our use case?
   - Answer after Phase 1 profiling

2. **Is parallel parsing feasible** given GeoJSON structure?
   - Answer after Phase 2, based on feature boundary detection

3. **What's the theoretical maximum** throughput for JSON parsing?
   - Investigate during Phase 1 analysis

4. **Should we support multiple backends** (different JSON libraries)?
   - Decide based on Phase 2 results

## Notes

- **Decision Date**: 2025-01-03
- **Decision Makers**: GeoETL Core Team
- **Priority**: High - blocks production readiness for GeoJSON
- **Dependencies**: None (can start immediately)
- **Timeline**: Phase 1 starts immediately, Phase 2 within 2 weeks
- **Review**: Update this ADR after each phase with results
