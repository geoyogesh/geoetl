#!/usr/bin/env bash

# Stop running benchmark processes
# This script kills all geoetl-cli processes and benchmark monitoring processes

echo "========================================="
echo "Stopping GeoETL Benchmarks"
echo "========================================="
echo ""

# Find and kill geoetl-cli processes
GEOETL_PIDS=$(ps aux | grep "geoetl-cli convert" | grep -v grep | awk '{print $2}')

if [ -n "$GEOETL_PIDS" ]; then
    echo "Stopping geoetl-cli processes:"
    echo "$GEOETL_PIDS" | while read pid; do
        echo "  Killing process $pid"
        kill -9 "$pid" 2>/dev/null || true
    done
else
    echo "No geoetl-cli processes found"
fi

echo ""

# Find and kill benchmark script processes
BENCH_PIDS=$(ps aux | grep "run_benchmark.sh" | grep -v grep | awk '{print $2}')

if [ -n "$BENCH_PIDS" ]; then
    echo "Stopping benchmark script processes:"
    echo "$BENCH_PIDS" | while read pid; do
        echo "  Killing process $pid"
        kill -9 "$pid" 2>/dev/null || true
    done
else
    echo "No benchmark script processes found"
fi

echo ""

# Find and kill monitoring processes (ps aux loops)
MONITOR_PIDS=$(ps aux | grep "ps aux" | grep "geoetl\|benchmark" | grep -v grep | awk '{print $2}')

if [ -n "$MONITOR_PIDS" ]; then
    echo "Stopping monitoring processes:"
    echo "$MONITOR_PIDS" | while read pid; do
        echo "  Killing process $pid"
        kill -9 "$pid" 2>/dev/null || true
    done
fi

echo ""
echo "========================================="
echo "All benchmark processes stopped"
echo "========================================="
echo ""

# Show any remaining related processes
REMAINING=$(ps aux | grep -E "geoetl|benchmark" | grep -v grep | grep -v "stop_benchmark")

if [ -n "$REMAINING" ]; then
    echo "Warning: Some processes may still be running:"
    echo "$REMAINING"
else
    echo "All processes successfully stopped"
fi
