#!/bin/bash
# Download Microsoft Buildings dataset for GeoETL benchmarking
# Dataset: 15 GB GeoJSON, 129M features
# Source: https://github.com/geoarrow/geoarrow-data/

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DATA_DIR="$SCRIPT_DIR/data"
FINAL_DIR="$DATA_DIR/final"

echo "========================================"
echo "GeoETL Benchmark Data Download"
echo "========================================"
echo "Dataset: Microsoft Buildings (US)"
echo "Size: ~15 GB GeoJSON"
echo "Features: 129.7M points"
echo "Source: geoarrow-data repository"
echo "========================================"

mkdir -p "$FINAL_DIR"

# Download FlatGeobuf file
FGB_FILE="$DATA_DIR/microsoft-buildings_point.fgb"
if [ ! -f "$FGB_FILE" ]; then
    echo "Downloading FlatGeobuf file (~7 GB)..."
    curl -L -o "$FGB_FILE" \
        "https://github.com/geoarrow/geoarrow-data/releases/download/v0.1.0/microsoft-buildings_point.fgb"
    echo "Download complete: $FGB_FILE"
else
    echo "FlatGeobuf file already exists: $FGB_FILE"
fi

# Convert to GeoJSON using ogr2ogr (if available)
GEOJSON_FILE="$FINAL_DIR/microsoft-buildings_point.geojson"
if [ ! -f "$GEOJSON_FILE" ]; then
    if command -v ogr2ogr &> /dev/null; then
        echo "Converting FlatGeobuf to GeoJSON (~15 GB)..."
        ogr2ogr -f GeoJSON "$GEOJSON_FILE" "$FGB_FILE"
        echo "GeoJSON created: $GEOJSON_FILE"
    else
        echo "Warning: ogr2ogr not found. Install GDAL to convert FGB to GeoJSON."
        echo "  macOS: brew install gdal"
        echo "  Ubuntu: sudo apt-get install gdal-bin"
        echo ""
        echo "Or use GeoETL to convert:"
        echo "  ./target/release/geoetl-cli convert \\"
        echo "    --input $FGB_FILE \\"
        echo "    --output $GEOJSON_FILE \\"
        echo "    --input-driver FlatGeobuf \\"
        echo "    --output-driver GeoJSON"
    fi
else
    echo "GeoJSON file already exists: $GEOJSON_FILE"
fi

# Convert to CSV using ogr2ogr (if available)
CSV_FILE="$FINAL_DIR/microsoft-buildings_point.csv"
if [ ! -f "$CSV_FILE" ]; then
    if command -v ogr2ogr &> /dev/null && [ -f "$GEOJSON_FILE" ]; then
        echo "Converting GeoJSON to CSV..."
        ogr2ogr -f CSV "$CSV_FILE" "$GEOJSON_FILE" \
            -lco GEOMETRY=AS_WKT
        echo "CSV created: $CSV_FILE"
    else
        echo "Skipping CSV conversion (requires ogr2ogr and GeoJSON file)"
    fi
else
    echo "CSV file already exists: $CSV_FILE"
fi

echo ""
echo "========================================"
echo "Download Complete"
echo "========================================"
ls -lh "$DATA_DIR" 2>/dev/null || true
ls -lh "$FINAL_DIR" 2>/dev/null || true
echo ""
echo "Ready for benchmarking!"
echo "See README.md for usage examples."
echo "========================================"
