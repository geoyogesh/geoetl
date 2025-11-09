---
sidebar_position: 10
title: FAQ
description: Frequently Asked Questions about GeoETL
---

# Frequently Asked Questions

Common questions about GeoETL and their answers.

## General Questions

### What is GeoETL?

GeoETL is a modern, high-performance CLI tool for geospatial data conversion and processing, built with Rust and Apache DataFusion. It's designed as a next-generation alternative to traditional geospatial ETL tools like ogr2ogr.

###  How do you pronounce GeoETL?

"Geo-E-T-L" (separate letters) or "Geo-Ettle" - both are acceptable!

### What does ETL stand for?

ETL stands for **Extract, Transform, Load** - the process of moving data between different systems and formats.

### Is GeoETL free and open source?

Yes! GeoETL is free and open source software released under the MIT license. You can use it for any purpose, including commercial projects.

### What platforms does GeoETL support?

GeoETL runs on:
- **Linux** (x86_64)
- **macOS** (x86_64, Apple Silicon via Rosetta)
- **Windows** (x86_64)

Both 32-bit and 64-bit architectures are supported where available.

## Installation

### How do I install GeoETL?

Download the latest release from [GitHub Releases](https://github.com/geoyogesh/geoetl/releases), extract the archive, and run the binary. No additional dependencies required!

See the [Installation Guide](./getting-started/installation.md) for detailed instructions.

### Do I need to install dependencies?

No! The GeoETL binary is self-contained. You don't need to install Rust, GDAL, or any other dependencies.

### Can I install GeoETL system-wide?

Yes! You can move the `geoetl-cli` binary to `/usr/local/bin/` (Linux/macOS) or add it to your PATH on Windows.

See [Installation Guide - Add to PATH](./getting-started/installation.md#step-4-add-to-path-optional).

### How do I update GeoETL?

Download the latest release and replace your existing binary. Check the [CHANGELOG](./community/changelog.md) for what's new.

## Features & Capabilities

### What formats does GeoETL support?

GeoETL supports multiple geospatial formats with full read/write capabilities. See [Supported Drivers](./drivers/supported-drivers.md) for the complete list and roadmap.

### Will GeoETL support Shapefile/GeoPackage/other formats?

Yes! Support for additional vector formats is planned. See the [Roadmap](./community/roadmap.md) for details.

### Can GeoETL transform coordinate systems?

Not yet. CRS transformations are planned for future releases. See the [Roadmap](./community/roadmap.md) for details.

### Does GeoETL support spatial operations?

Not yet. Spatial operations (buffer, intersection, union, etc.) are planned for future releases. See the [Roadmap](./community/roadmap.md) for details.

## Performance

### How fast is GeoETL?

GeoETL is built for high performance using Rust, Apache DataFusion, and Apache Arrow. It handles large datasets efficiently with constant memory usage through streaming architecture. See [Performance Benchmarks](./reference/benchmarks.md) for detailed measurements.

## Usage

### How do I convert between formats?

Use the `convert` command:

```bash
geoetl-cli convert \
  --input input.geojson \
  --output output.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

See [Your First Conversion](./getting-started/first-conversion.md).

### Do I need to specify the driver?

Yes, currently you must specify both `--input-driver` and `--output-driver`. Future versions may support automatic format detection.

### Why is `--geometry-column` required for CSV?

CSV files can have any column name for geometries (`geometry`, `wkt`, `geom`, etc.). Unlike GeoJSON which has a standard structure, GeoETL needs to know which column contains the WKT geometries.

```bash
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt
```

### How do I see what's happening during conversion?

Use the `-v` or `--verbose` flag:

```bash
geoetl-cli -v convert -i input.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV
```

For more details, use `RUST_LOG=debug`:

```bash
RUST_LOG=debug geoetl-cli convert ...
```

### Can I batch process multiple files?

Yes! Use a shell loop:

```bash
for file in data/*.geojson; do
  output="output/$(basename "$file" .geojson).csv"
  geoetl-cli convert -i "$file" -o "$output" \
    --input-driver GeoJSON --output-driver CSV
done
```

## Troubleshooting

### Why do I get "Driver not found"?

Driver names are case-sensitive. Use the exact name from `geoetl-cli drivers`:

```bash
# ❌ Wrong
--input-driver geojson

# ✅ Correct
--input-driver GeoJSON
```

### Why do I get "Permission denied"?

On Linux/macOS, you may need to make the binary executable:

```bash
chmod +x geoetl-cli
```

Or check your write permissions for the output directory.

### My conversion failed. How do I debug it?

1. **Enable verbose logging**:
   ```bash
   geoetl-cli -v convert ...
   ```

2. **Try a small sample first**:
   ```bash
   head -n 100 input.geojson > sample.geojson
   geoetl-cli convert -i sample.geojson -o test.csv \
     --input-driver GeoJSON --output-driver CSV
   ```

3. **Check file format**:
   ```bash
   geoetl-cli info input.geojson --driver GeoJSON
   ```

4. **Report the issue**: [GitHub Issues](https://github.com/geoyogesh/geoetl/issues)

### Where can I get help?

- **Documentation**: Browse these docs
- **GitHub Discussions**: [Ask questions](https://github.com/geoyogesh/geoetl/discussions)
- **GitHub Issues**: [Report bugs](https://github.com/geoyogesh/geoetl/issues)
- **Command Help**: `geoetl-cli --help`

## Development & Contributing

### Is GeoETL actively maintained?

Yes! GeoETL is actively developed with regular releases. Check the [CHANGELOG](./community/changelog.md) for recent updates.

### Can I contribute to GeoETL?

Absolutely! Contributions are welcome:
- **Code**: Submit PRs for bug fixes or features
- **Documentation**: Improve docs, add examples
- **Testing**: Report bugs, test new features
- **Discussion**: Share ideas, help others

See [Contributing Guide](./community/contributing.md).

### How do I report a bug?

[Open an issue on GitHub](https://github.com/geoyogesh/geoetl/issues/new) with:
- GeoETL version (`geoetl-cli --version`)
- Operating system
- Command you ran
- Expected vs actual behavior
- Sample data (if possible)

### How do I request a feature?

[Open a discussion on GitHub](https://github.com/geoyogesh/geoetl/discussions) describing:
- The use case
- Why existing features don't work
- Proposed solution (optional)

## Still Have Questions?

- 📚 **Browse Documentation**: Check the guides and tutorials
- 💬 **GitHub Discussions**: [Ask the community](https://github.com/geoyogesh/geoetl/discussions)
- 🐛 **GitHub Issues**: [Report bugs](https://github.com/geoyogesh/geoetl/issues)
- 📖 **Glossary**: See [Glossary](./glossary.md) for term definitions

---

**Didn't find your question?** [Start a discussion on GitHub](https://github.com/geoyogesh/geoetl/discussions) →
