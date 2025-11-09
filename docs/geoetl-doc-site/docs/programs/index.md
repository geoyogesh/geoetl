---
sidebar_position: 1
title: Programs Overview
description: GeoETL command-line programs reference
---

# Programs

Complete reference for all GeoETL command-line programs and their options.

## geoetl-cli

The main GeoETL command-line interface provides commands for converting geospatial data between formats, inspecting datasets, and managing drivers.

### Available Commands

| Command | Description |
|---------|-------------|
| [convert](./convert.md) | Convert data between formats |
| [info](./info.md) | Display dataset information |
| [drivers](./drivers.md) | List available format drivers |
| [completions](./completions.md) | Generate shell completions |

### Global Options

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Print help information |
| `--version` | `-V` | Print version information |
| `--verbose` | `-v` | Enable verbose logging |

### Basic Usage

```bash
# Get help
geoetl-cli --help

# Get version
geoetl-cli --version

# List available drivers
geoetl-cli drivers

# Convert a file
geoetl-cli convert -i input.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV

# Get dataset information
geoetl-cli info data.geojson --driver GeoJSON
```

### Verbose Logging

Enable verbose output with `-v` flag:

```bash
geoetl-cli -v convert -i input.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV
```

For detailed debug logging, use `RUST_LOG`:

```bash
RUST_LOG=debug geoetl-cli convert ...
RUST_LOG=info geoetl-cli convert ...
```

## Command Reference

Browse the detailed reference for each command:

- **[convert](./convert.md)** - Convert data between formats
- **[info](./info.md)** - Display dataset information
- **[drivers](./drivers.md)** - List available format drivers
- **[completions](./completions.md)** - Generate shell completions

## Common Options

### Input/Output Specification

Most commands accept input and output files:

```bash
--input <PATH>      # or -i <PATH>
--output <PATH>     # or -o <PATH>
```

### Driver Specification

Specify format drivers for input and output:

```bash
--input-driver <DRIVER>
--output-driver <DRIVER>
```

**Available drivers**: `GeoJSON`, `CSV`, `GeoParquet`

See [Supported Drivers](../drivers/supported-drivers.md) for details.

### CSV-Specific Options

When working with CSV files:

```bash
--geometry-column <COLUMN>  # Required for CSV input
--geometry-type <TYPE>      # Optional geometry type hint
```

## Exit Codes

GeoETL uses standard Unix exit codes:

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |

## Environment Variables

### RUST_LOG

Control logging level:

```bash
export RUST_LOG=info     # Informational messages
export RUST_LOG=debug    # Debug messages
export RUST_LOG=trace    # Trace messages
```

Examples:
```bash
RUST_LOG=info geoetl-cli convert ...
RUST_LOG=debug geoetl-cli info ...
```

## Examples

### Basic Conversion

```bash
geoetl-cli convert \
  -i cities.geojson \
  -o cities.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

### Verbose Conversion

```bash
geoetl-cli -v convert \
  -i large_dataset.geojson \
  -o large_dataset.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

### CSV with Custom Geometry Column

```bash
geoetl-cli convert \
  -i data.csv \
  -o data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt
```

### Dataset Inspection

```bash
geoetl-cli info cities.geojson --driver GeoJSON
geoetl-cli info data.csv --driver CSV --geometry-column geometry
geoetl-cli info data.parquet --driver GeoParquet
```

## See Also

- [Getting Started](../getting-started/installation.md) - Installation guide
- [Your First Conversion](../getting-started/first-conversion.md) - Quick start tutorial
- [Supported Drivers](../drivers/supported-drivers.md) - Format driver reference
- [FAQ](../faq.md) - Frequently asked questions

---

**Need help?** Run `geoetl-cli --help` or `geoetl-cli <command> --help`
