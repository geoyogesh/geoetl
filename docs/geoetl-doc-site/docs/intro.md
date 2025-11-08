---
sidebar_position: 1
---

# Welcome to GeoETL

**GeoETL** is a CLI tool for geospatial data conversion and processing, built with Rust and Apache DataFusion.

## What is GeoETL?

GeoETL is a geospatial ETL tool with the following features:

- **Self-Contained**: Single binary with no external dependencies
- **Memory Safety**: Built with Rust
- **Modern Architecture**: Leverages Apache DataFusion and Apache Arrow
- **Multiple Format Drivers**: [See all supported drivers](./drivers/supported-drivers)

## What You'll Learn

This tutorial will teach you:

- **Installation** - How to install and build GeoETL
- **Basic Operations** - Converting data between formats
- **Working with Data** - Understanding drivers and formats
- **Advanced Features** - Performance tips and best practices

## What You'll Need

### System Requirements

- **A computer** running Linux, macOS, or Windows
- **Command-line terminal** - Terminal, PowerShell, or Command Prompt
- That's it! The pre-built binary is self-contained.

### Recommended

- Basic command-line knowledge
- Familiarity with geospatial data formats (helpful but not required)

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

You should see a table of supported format drivers. See the complete [Supported Drivers Reference](./drivers/supported-drivers) for details!

**Example conversion**:

```bash
./geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet
```

See the [Installation Guide](./tutorial-basics/installation) for detailed step-by-step instructions.

## Tutorial Structure

Get started with these beginner-friendly tutorials:

1. **[Installation Guide](./tutorial-basics/installation)** - Get GeoETL up and running
2. **[Your First Conversion](./tutorial-basics/first-conversion)** - Convert a GeoJSON file
3. **[Understanding Drivers](./tutorial-basics/understanding-drivers)** - Learn about format support
4. **[Working with GeoJSON](./tutorial-basics/working-with-geojson)** - Web-standard format
5. **[Working with CSV](./tutorial-basics/working-with-csv)** - CSV and WKT geometries
6. **[Working with GeoParquet](./tutorial-basics/working-with-geoparquet)** - High-performance columnar format
7. **[Error Handling & Troubleshooting](./tutorial-basics/troubleshooting)** - Debug and resolve issues

## Getting Help

Need assistance?

- **Documentation**: Browse these tutorials and guides
- **GitHub Issues**: [Report bugs or request features](https://github.com/geoyogesh/geoetl/issues)
- **GitHub Discussions**: [Ask questions and share ideas](https://github.com/geoyogesh/geoetl/discussions)
- **Command Help**: Run `geoetl-cli --help` or `geoetl-cli <command> --help`

## Next Steps

Ready to dive in?

**[Start with the Installation Guide](./tutorial-basics/installation)**

Or jump to:
- [Supported Drivers Reference](./drivers/supported-drivers) - Complete driver documentation
- [Your First Conversion](./tutorial-basics/first-conversion) - Quick hands-on tutorial
- [Understanding Drivers](./tutorial-basics/understanding-drivers) - Learn format support
- [Working with GeoJSON](./tutorial-basics/working-with-geojson) - Web-standard format
- [Working with GeoParquet](./tutorial-basics/working-with-geoparquet) - Modern columnar format

---

Ready to begin? **[Start with the Installation Guide](./tutorial-basics/installation)**
