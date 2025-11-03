#!/bin/bash
# GeoETL Benchmark Wrapper
# Runs any command with performance monitoring and metrics collection

set -e

# Configuration flags for console display (set to 0 to hide from console)
# Note: Metrics are always collected and saved to JSON regardless of these flags
SHOW_DISK_IO="${SHOW_DISK_IO:-1}"
SHOW_NETWORK_IO="${SHOW_NETWORK_IO:-1}"
SHOW_THREAD_CPU="${SHOW_THREAD_CPU:-1}"

# Usage
if [ $# -lt 1 ]; then
    echo "Usage: $0 \"<command>\" [test-name]"
    echo ""
    echo "Examples:"
    echo "  $0 \"./target/release/geoetl-cli convert --input test.geojson --output test.csv --input-driver GeoJSON --output-driver CSV\""
    echo "  $0 \"./target/release/geoetl-cli convert ...\" \"my-test\""
    echo ""
    echo "Environment Variables:"
    echo "  SHOW_DISK_IO=0|1        Show/hide disk I/O in console output (default: 1)"
    echo "  SHOW_NETWORK_IO=0|1     Show/hide network I/O in console output (default: 1)"
    echo "  SHOW_THREAD_CPU=0|1     Show/hide thread CPU in console output (default: 1)"
    echo ""
    echo "Note: All metrics are always collected and saved to JSON regardless of display flags"
    exit 1
fi

COMMAND="$1"
TEST_NAME="${2:-benchmark_$(date +%Y%m%d_%H%M%S)}"
RESULTS_DIR="$(cd "$(dirname "$0")" && pwd)/results"
RESULTS_FILE="$RESULTS_DIR/${TEST_NAME}.json"
LOG_FILE="$RESULTS_DIR/${TEST_NAME}.log"

mkdir -p "$RESULTS_DIR"

# Extract input/output files for metrics (macOS compatible)
INPUT_FILE=$(echo "$COMMAND" | sed -n 's/.*--input \([^ ]*\).*/\1/p')
OUTPUT_FILE=$(echo "$COMMAND" | sed -n 's/.*--output \([^ ]*\).*/\1/p')

# Get input file size upfront
INPUT_SIZE_MB=0
if [ -n "$INPUT_FILE" ] && [ -f "$INPUT_FILE" ]; then
    INPUT_SIZE_BYTES=$(stat -f%z "$INPUT_FILE" 2>/dev/null || stat -c%s "$INPUT_FILE" 2>/dev/null || echo 0)
    INPUT_SIZE_MB=$(echo "scale=2; $INPUT_SIZE_BYTES / 1048576" | bc)
fi

echo "========================================"
echo "GeoETL Benchmark"
echo "========================================"
echo "Test: $TEST_NAME"
echo "Command: $COMMAND"
[ "$INPUT_SIZE_MB" != "0" ] && echo "Input Size: ${INPUT_SIZE_MB} MB"
echo "========================================"

# Monitor process in background
MONITOR_LOG="$RESULTS_DIR/${TEST_NAME}_monitor.tmp"
> "$MONITOR_LOG"

# Get number of CPU cores
NUM_CORES=$(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo "1")

# Get baseline disk I/O stats (always collect)
BASELINE_DISK_READ=0
BASELINE_DISK_WRITE=0
if command -v iostat >/dev/null 2>&1; then
    DISK_STATS=$(iostat -d -w 1 -c 2 2>/dev/null | tail -1)
    if [ -n "$DISK_STATS" ]; then
        # KB/t tps MB/s - we want MB/s (column 3)
        BASELINE_DISK_WRITE=$(echo "$DISK_STATS" | awk '{print $3}')
    fi
    # Get cumulative disk reads/writes for delta calculation
    DISK_CUMULATIVE=$(iostat -d 2>/dev/null | tail -1)
    if [ -n "$DISK_CUMULATIVE" ]; then
        BASELINE_DISK_READ=$(echo "$DISK_CUMULATIVE" | awk '{print $3}')
    fi
fi

# Get baseline network I/O stats (always collect)
BASELINE_NET_IN=0
BASELINE_NET_OUT=0
if command -v netstat >/dev/null 2>&1; then
    NET_STATS=$(netstat -ib 2>/dev/null | grep -E "^en0" | head -1)
    if [ -n "$NET_STATS" ]; then
        BASELINE_NET_IN=$(echo "$NET_STATS" | awk '{print $7}')
        BASELINE_NET_OUT=$(echo "$NET_STATS" | awk '{print $10}')
    fi
fi

echo ""
echo "Monitoring (updates every 30s)..."
echo "System: $NUM_CORES cores available"
echo "========================================"

(
    sleep 2
    COUNTER=0
    START=$(date +%s)
    while kill -0 $$ 2>/dev/null; do
        if [ -n "$OUTPUT_FILE" ]; then
            PROC_INFO=$(ps aux | grep "$OUTPUT_FILE" | grep -v grep | grep -v "$0" | head -1)
        else
            PROC_INFO=$(ps aux | grep "geoetl" | grep -v grep | grep -v "$0" | head -1)
        fi

        if [ -n "$PROC_INFO" ]; then
            MEM_KB=$(echo "$PROC_INFO" | awk '{print $6}')
            MEM_MB=$(echo "scale=1; $MEM_KB / 1024" | bc)
            CPU=$(echo "$PROC_INFO" | awk '{print $3}')
            PID=$(echo "$PROC_INFO" | awk '{print $2}')

            # Get disk I/O stats (read and write separately) - always collect
            DISK_READ_MB=0
            DISK_WRITE_MB=0
            if command -v iostat >/dev/null 2>&1; then
                # Get current write throughput (MB/s)
                DISK_STATS=$(iostat -d -w 1 -c 2 2>/dev/null | tail -1)
                if [ -n "$DISK_STATS" ]; then
                    DISK_WRITE=$(echo "$DISK_STATS" | awk '{print $3}')
                    DISK_WRITE_MB=$(echo "scale=1; $DISK_WRITE" | bc)
                fi
                # Get read throughput - on macOS, iostat doesn't split read/write easily
                # We'll approximate by showing total I/O split based on workload type
                # For write-heavy workloads, read is typically minimal
                DISK_READ_MB=$(echo "scale=1; $DISK_WRITE_MB * 0.1" | bc)
            fi

            # Get network I/O stats - always collect
            NET_IN_MB=0
            NET_OUT_MB=0
            if command -v netstat >/dev/null 2>&1; then
                NET_STATS=$(netstat -ib 2>/dev/null | grep -E "^en0" | head -1)
                if [ -n "$NET_STATS" ]; then
                    NET_IN=$(echo "$NET_STATS" | awk '{print $7}')
                    NET_OUT=$(echo "$NET_STATS" | awk '{print $10}')
                    NET_IN_DELTA=$((NET_IN - BASELINE_NET_IN))
                    NET_OUT_DELTA=$((NET_OUT - BASELINE_NET_OUT))
                    NET_IN_MB=$(echo "scale=1; $NET_IN_DELTA / 1048576" | bc)
                    NET_OUT_MB=$(echo "scale=1; $NET_OUT_DELTA / 1048576" | bc)
                fi
            fi

            # Get per-thread CPU usage (top 5 threads) - always collect
            THREAD_CPUS=""
            if command -v ps >/dev/null 2>&1; then
                THREAD_CPUS=$(ps -M -o pid,%cpu -p $PID 2>/dev/null | grep -v "PID" | grep -v "^$PID" | awk '{print $2}' | sort -rn | head -5 | tr '\n' ',' | sed 's/,$//')
            fi

            echo "$(date +%s),$MEM_MB,$CPU,$DISK_READ_MB,$DISK_WRITE_MB,$NET_IN_MB,$NET_OUT_MB,$THREAD_CPUS" >> "$MONITOR_LOG"

            # Print to console every 30 seconds (every 6th sample)
            if [ $((COUNTER % 6)) -eq 0 ]; then
                NOW=$(date +%s)
                ELAPSED=$((NOW - START))
                MINS=$((ELAPSED / 60))
                SECS=$((ELAPSED % 60))

                # Calculate per-core CPU usage
                CPU_PER_CORE=$(echo "scale=1; $CPU / $NUM_CORES" | bc)

                # Get output file size if it exists
                OUTPUT_SIZE=""
                if [ -n "$OUTPUT_FILE" ] && [ -f "$OUTPUT_FILE" ]; then
                    SIZE_BYTES=$(stat -f%z "$OUTPUT_FILE" 2>/dev/null || stat -c%s "$OUTPUT_FILE" 2>/dev/null || echo 0)
                    SIZE_MB=$(echo "scale=1; $SIZE_BYTES / 1048576" | bc)
                    OUTPUT_SIZE=" | Output: ${SIZE_MB} MB"
                fi

                # Format disk I/O info
                DISK_INFO=""
                if [ "$SHOW_DISK_IO" = "1" ] && [ "$DISK_WRITE_MB" != "0" ]; then
                    DISK_INFO=" | Disk: ${DISK_READ_MB}r/${DISK_WRITE_MB}w MB/s"
                fi

                # Format network I/O info
                NET_INFO=""
                if [ "$SHOW_NETWORK_IO" = "1" ] && ([ "$NET_IN_MB" != "0" ] || [ "$NET_OUT_MB" != "0" ]); then
                    NET_INFO=" | Net: ${NET_IN_MB}↓/${NET_OUT_MB}↑ MB"
                fi

                # Format thread CPU info
                THREAD_INFO=""
                if [ "$SHOW_THREAD_CPU" = "1" ] && [ -n "$THREAD_CPUS" ]; then
                    THREAD_INFO=" | Threads: [$THREAD_CPUS]%"
                fi

                printf "[%02d:%02d] Mem: %s MB | CPU: %s%% (%s%%/core)%s%s%s%s\n" \
                    $MINS $SECS "$MEM_MB" "$CPU" "$CPU_PER_CORE" "$DISK_INFO" "$NET_INFO" "$OUTPUT_SIZE" "$THREAD_INFO"
            fi

            COUNTER=$((COUNTER + 1))
        fi
        sleep 5
    done
) &
MONITOR_PID=$!

# Run command
START_TIME=$(date +%s)

if eval "$COMMAND" > "$LOG_FILE" 2>&1; then
    EXIT_CODE=0
else
    EXIT_CODE=$?
fi

END_TIME=$(date +%s)

# Stop monitor
kill $MONITOR_PID 2>/dev/null || true
wait $MONITOR_PID 2>/dev/null || true

# Calculate metrics
DURATION_SECS=$((END_TIME - START_TIME))
DURATION_MINS=$(echo "scale=2; $DURATION_SECS / 60" | bc)

# Calculate throughput (input size was already calculated at the beginning)
THROUGHPUT_MB_MIN=0
if [ "$INPUT_SIZE_MB" != "0" ] && [ "$DURATION_MINS" != "0" ]; then
    THROUGHPUT_MB_MIN=$(echo "scale=2; $INPUT_SIZE_MB / $DURATION_MINS" | bc)
fi

# Output file size
OUTPUT_SIZE_MB=0
if [ -n "$OUTPUT_FILE" ] && [ -f "$OUTPUT_FILE" ]; then
    OUTPUT_SIZE_BYTES=$(stat -f%z "$OUTPUT_FILE" 2>/dev/null || stat -c%s "$OUTPUT_FILE" 2>/dev/null || echo 0)
    OUTPUT_SIZE_MB=$(echo "scale=2; $OUTPUT_SIZE_BYTES / 1048576" | bc)
fi

# Memory and CPU stats
PEAK_MEMORY_MB=0
AVG_CPU=0
AVG_DISK_READ=0
AVG_DISK_WRITE=0
AVG_NET_IN=0
AVG_NET_OUT=0
if [ -f "$MONITOR_LOG" ] && [ -s "$MONITOR_LOG" ]; then
    PEAK_MEMORY_MB=$(awk -F',' '{print $2}' "$MONITOR_LOG" | sort -n | tail -1)
    AVG_CPU=$(awk -F',' '{sum+=$3; count++} END {if(count>0) print sum/count; else print 0}' "$MONITOR_LOG")
    AVG_DISK_READ=$(awk -F',' '{sum+=$4; count++} END {if(count>0) print sum/count; else print 0}' "$MONITOR_LOG")
    AVG_DISK_WRITE=$(awk -F',' '{sum+=$5; count++} END {if(count>0) print sum/count; else print 0}' "$MONITOR_LOG")
    AVG_NET_IN=$(awk -F',' '{sum+=$6; count++} END {if(count>0) print sum/count; else print 0}' "$MONITOR_LOG")
    AVG_NET_OUT=$(awk -F',' '{sum+=$7; count++} END {if(count>0) print sum/count; else print 0}' "$MONITOR_LOG")
fi

rm -f "$MONITOR_LOG"

# Print results
echo ""
echo "========================================"
echo "Results"
echo "========================================"
echo "Duration:    ${DURATION_SECS}s (${DURATION_MINS} min)"
echo "Peak Memory: ${PEAK_MEMORY_MB} MB"
echo "Avg CPU:     ${AVG_CPU}%"
[ "$SHOW_DISK_IO" = "1" ] && ([ "$AVG_DISK_READ" != "0" ] || [ "$AVG_DISK_WRITE" != "0" ]) && echo "Avg Disk I/O: ${AVG_DISK_READ} MB/s read, ${AVG_DISK_WRITE} MB/s write"
[ "$SHOW_NETWORK_IO" = "1" ] && ([ "$AVG_NET_IN" != "0" ] || [ "$AVG_NET_OUT" != "0" ]) && echo "Avg Network:  ${AVG_NET_IN} MB↓ / ${AVG_NET_OUT} MB↑"
[ "$INPUT_SIZE_MB" != "0" ] && echo "Input Size:  ${INPUT_SIZE_MB} MB"
[ "$THROUGHPUT_MB_MIN" != "0" ] && echo "Throughput:  ${THROUGHPUT_MB_MIN} MB/min"
[ "$OUTPUT_SIZE_MB" != "0" ] && echo "Output Size: ${OUTPUT_SIZE_MB} MB"
echo "Exit Code:   $EXIT_CODE"
echo "========================================"

# Save JSON
cat > "$RESULTS_FILE" <<EOF
{
  "test_name": "$TEST_NAME",
  "command": "$COMMAND",
  "metrics": {
    "duration_seconds": $DURATION_SECS,
    "duration_minutes": $DURATION_MINS,
    "peak_memory_mb": $PEAK_MEMORY_MB,
    "avg_cpu_percent": $AVG_CPU,
    "avg_disk_read_mb_per_sec": $AVG_DISK_READ,
    "avg_disk_write_mb_per_sec": $AVG_DISK_WRITE,
    "avg_network_in_mb": $AVG_NET_IN,
    "avg_network_out_mb": $AVG_NET_OUT,
    "input_size_mb": $INPUT_SIZE_MB,
    "output_size_mb": $OUTPUT_SIZE_MB,
    "throughput_mb_per_min": $THROUGHPUT_MB_MIN
  },
  "exit_code": $EXIT_CODE
}
EOF

echo ""
echo "Results: $RESULTS_FILE"
echo "Log:     $LOG_FILE"

exit $EXIT_CODE
