---
sidebar_position: 1
---

# Welcome to GeoETL

**GeoETL** is a modern, high-performance CLI tool for geospatial data conversion and processing, built with Rust and Apache DataFusion.

Let's get you started with **GeoETL in less than 5 minutes**.

## What is GeoETL?

GeoETL is designed to be a next-generation alternative to traditional geospatial ETL tools, offering:

- **High Performance**: 5-10x faster processing through vectorized execution
- **Memory Safety**: Built with Rust for guaranteed memory safety
- **Modern Architecture**: Leverages Apache DataFusion and Apache Arrow
- **Scalability**: From single-machine to distributed processing (coming soon)
- **3 Format Drivers**: GeoJSON, CSV, and GeoParquet ([see all supported drivers](./reference/supported-drivers))

## What You'll Learn

This tutorial will teach you:

✅ **Installation** - How to install and build GeoETL
✅ **Basic Operations** - Converting data between formats
✅ **Working with Data** - Understanding drivers and formats
✅ **Advanced Features** - Performance tips and best practices

## What You'll Need

### System Requirements

- **A computer** running Linux, macOS, or Windows
- **Command-line terminal** - Terminal, PowerShell, or Command Prompt
- That's it! The pre-built binary is self-contained.

:::note Building from Source
If you want to build from source instead, you'll need Rust 1.90.0+ and Git.
See [Installation Guide](./tutorial-basics/installation) for details.
:::

### Recommended

- Basic command-line knowledge
- Familiarity with geospatial data formats (GeoJSON, CSV, etc.)
- A text editor for viewing data files

## Quick Start

Get the latest release from GitHub:

```bash
# 1. Download from GitHub Releases
# https://github.com/geoyogesh/geoetl/releases

# 2. Extract the archive
tar -xzf geoetl-cli-*.tar.gz  # Linux/macOS
# or extract the .zip on Windows

# 3. Run your first command
./geoetl-cli drivers
```

You should see a table of supported format drivers (currently 3: GeoJSON, CSV, GeoParquet). See the complete [Supported Drivers Reference](./reference/supported-drivers) for details!

**Try GeoParquet**:

```bash
# Convert GeoJSON to efficient GeoParquet (6.8x smaller!)
./geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet
```

**→ See the [Installation Guide](./tutorial-basics/installation) for detailed step-by-step instructions.**

## Tutorial Structure

Get started with these beginner-friendly tutorials:

1. **[Installation Guide](./tutorial-basics/installation)** - Get GeoETL up and running
2. **[Your First Conversion](./tutorial-basics/first-conversion)** - Convert a GeoJSON file
3. **[Understanding Drivers](./tutorial-basics/understanding-drivers)** - Learn about format support
4. **[Working with GeoJSON](./tutorial-basics/working-with-geojson)** - Web-standard format
5. **[Working with CSV](./tutorial-basics/working-with-csv)** - CSV and WKT geometries
6. **[Working with GeoParquet](./tutorial-basics/working-with-geoparquet)** - High-performance columnar format
7. **[Error Handling & Troubleshooting](./tutorial-basics/troubleshooting)** - Debug and resolve issues

## Current Status

GeoETL is in **Phase 1 (Complete)**. Here's what works today:

✅ **Working Now**:
- CSV format (read/write with WKT geometries)
- GeoJSON format (full read/write support)
- GeoParquet format (full read/write with WKB geometries)
- Convert command (CSV ↔ GeoJSON ↔ GeoParquet conversions)
- Info command (dataset schema inspection)
- Driver registry and capability checking
- Shell completions (bash, zsh, fish, powershell, elvish)
- Comprehensive error messages with helpful examples

🚧 **Coming Soon** (Q1-Q2 2026):
- GeoPackage, Shapefile, FlatGeobuf drivers
- Spatial operations (buffer, intersection, union)
- CRS transformations

See our [Roadmap](https://github.com/geoyogesh/geoetl/blob/main/docs/VISION.md) for complete details.

## Getting Help

Need assistance?

- **Documentation**: Browse these tutorials and guides
- **GitHub Issues**: [Report bugs or request features](https://github.com/geoyogesh/geoetl/issues)
- **GitHub Discussions**: [Ask questions and share ideas](https://github.com/geoyogesh/geoetl/discussions)
- **Command Help**: Run `geoetl-cli --help` or `geoetl-cli <command> --help`

## Next Steps

Ready to dive in?

👉 **[Start with the Installation Guide →](./tutorial-basics/installation)**

Or jump to:
- [Supported Drivers Reference](./reference/supported-drivers) - Complete driver documentation
- [Your First Conversion](./tutorial-basics/first-conversion) - Quick hands-on tutorial
- [Understanding Drivers](./tutorial-basics/understanding-drivers) - Learn format support
- [Working with GeoJSON](./tutorial-basics/working-with-geojson) - Web-standard format
- [Working with GeoParquet](./tutorial-basics/working-with-geoparquet) - Modern columnar format

---

**Let's get started!** 🚀
