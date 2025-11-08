---
slug: shell-completions-v0-3-1
title: "GeoETL v0.3.1: Shell Completions & Future Format Support"
authors: [geoyogesh]
tags: [release, shell-completions, developer-experience, v0.3.1]
date: 2025-11-06
---

**TL;DR**: GeoETL v0.3.1 enhances developer experience with shell completions for 5 shells (bash, zsh, fish, powershell, elvish) and lays groundwork for 4 major geospatial formats coming soon.

<!--truncate-->

## Why This Release Matters

After the major performance improvements in v0.2.0 and GeoParquet support in v0.3.0, we're focusing on two areas:

1. **Better Developer Experience** - Shell completions make GeoETL faster and easier to use
2. **Future-Proofing** - Scaffolding for the next wave of format drivers

This is a quality-of-life release that sets us up for rapid format expansion in the coming months.

## Headline Features

### 🎯 Shell Completions Support

**Problem**: Typing long commands with exact parameter names is tedious and error-prone. Users had to constantly reference documentation to remember command syntax.

**Solution**: We've added native shell completion support via a new `completions` subcommand. Press `Tab` to autocomplete commands, subcommands, and options.

**Value**: **Faster, error-free command entry**. Discover available options without leaving your terminal.

**Supported Shells**:
- ✅ **bash** - Most common Linux/macOS shell
- ✅ **zsh** - macOS default, popular on Linux
- ✅ **fish** - Modern shell with great UX
- ✅ **powershell** - Windows PowerShell
- ✅ **elvish** - Modern shell with innovative features

**Installation Examples**:

```bash
# For bash (add to ~/.bashrc)
eval "$(geoetl-cli completions bash)"

# For zsh (add to ~/.zshrc)
eval "$(geoetl-cli completions zsh)"

# For fish (add to ~/.config/fish/config.fish)
geoetl-cli completions fish | source

# For powershell (add to your profile)
geoetl-cli completions powershell | Out-String | Invoke-Expression

# For elvish (add to ~/.elvish/rc.elv)
eval (geoetl-cli completions elvish | slurp)
```

**Demo - Tab Completion in Action**:

```bash
$ geoetl-cli <TAB>
completions  convert  drivers  help  info

$ geoetl-cli convert --<TAB>
--batch-size       --input-driver      --output-driver
--debug            --input             --output
--geometry-column  --geometry-type     --read-partitions
--help             --write-partitions  --verbose

$ geoetl-cli convert --input-driver <TAB>
CSV  GeoJSON  GeoParquet
```

**Documentation**: See updated [README.md](https://github.com/geoyogesh/geoetl/blob/main/README.md#shell-completions) and [QUICKREF.md](https://github.com/geoyogesh/geoetl/blob/main/QUICKREF.md#shell-completions) for complete installation instructions.

### 🏗️ New Geospatial Format Scaffolding

**Problem**: Implementing a new format driver involves significant boilerplate code and careful integration with DataFusion's FileFormat trait.

**Solution**: We've created module scaffolding for 4 major geospatial formats with proper structure and documentation placeholders.

**Value**: **Faster format implementation**. Contributors can focus on format-specific logic rather than boilerplate.

**New Format Modules** (scaffolding only, implementation coming):

1. **Arrow IPC** (`datafusion-arrow-ipc`)
   - Zero-copy data exchange format
   - Binary columnar format
   - Use case: Fast inter-process communication

2. **GeoPackage** (`datafusion-geopackage`)
   - SQLite-based vector data storage
   - OGC standard format
   - Use case: Mobile apps, offline GIS

3. **OpenStreetMap** (`datafusion-osm`)
   - OSM PBF (Protobuf Binary Format)
   - OSM XML support
   - Use case: OpenStreetMap data processing

4. **Shapefile** (`datafusion-shapefile`)
   - Legacy ESRI Shapefile format
   - Still widely used in GIS
   - Use case: Compatibility with legacy systems

**Timeline**: These formats will be progressively implemented in v0.4.0 and beyond. Contributors are welcome to help implement any of these formats!

### ⚡ GeoParquet Streaming I/O Enhancements

**Problem**: GeoParquet I/O could benefit from statistics inference for better query optimization.

**Solution**: Implemented automatic statistics inference and enhanced streaming capabilities.

**Value**: Improved performance and reduced memory usage when processing large GeoParquet files.

**Technical Details**:
- Statistics inference for min/max values, null counts
- Enhanced streaming I/O reduces memory pressure
- Better integration with DataFusion's query optimizer

## Other Improvements & Fixes

### Changed

- **GeoParquet ADR refactored**: Updated [ADR 004](https://github.com/geoyogesh/geoetl/blob/main/docs/adr/004-streaming-geoparquet-architecture.md) to follow Michael Nygard's ADR template for consistency
- **Documentation updates**: Added shell completions to README, QUICKREF, and doc site
- **Removed version annotations**: Cleaned up version-specific notes from documentation

### Dependencies

- ⬆️ **Upgraded geoarrow**: v0.5.0 → v0.6.2 (latest version with bug fixes)
- ➕ **Added clap_complete**: v4.5.50 (enables shell completion generation)

### Removed

- 🗑️ **Performance tests**: Removed from GeoParquet E2E tests (moved to dedicated benchmark suite)

## ⚠️ Breaking Changes

None - this release is fully backward compatible with v0.3.0.

## Community & Contributors

Thank you to everyone who:
- Requested shell completion support
- Contributed to the geoarrow ecosystem
- Provided feedback on v0.3.0

We're grateful for the community's engagement and support!

## The Future: What's Next?

We have an exciting roadmap ahead:

**v0.4.0 (Target: Q1 2026)**:
- 🎯 **FlatGeobuf format support** - Cloud-optimized geospatial format
- 🎯 **Shapefile read support** - For legacy data compatibility
- 🎯 **GeoJSON performance optimization** - Target 3-7x speedup
- 🎯 **Format auto-detection** - Automatic driver inference from file extensions

**v0.5.0 and beyond**:
- 🚀 **GeoPackage support** - SQLite-based vector data
- 🚀 **Arrow IPC support** - Zero-copy data exchange
- 🚀 **OSM support** - OpenStreetMap data processing
- 🚀 **Spatial operations** - Buffer, intersection, union

See our full [Roadmap](https://github.com/geoyogesh/geoetl/blob/main/docs/VISION.md) for details.

## How to Upgrade

### Installation

**From source**:
```bash
git clone https://github.com/geoyogesh/geoetl.git
cd geoetl
git checkout v0.3.1
cargo build --release

# Binary at: target/release/geoetl-cli
```

### Verify Installation

```bash
$ geoetl-cli --version
geoetl-cli 0.3.1

$ geoetl-cli completions --help
# Should show completion generation help
```

### Enable Shell Completions

**Choose your shell and add to your shell's config file**:

```bash
# Bash (~/.bashrc)
eval "$(geoetl-cli completions bash)"

# Zsh (~/.zshrc)
eval "$(geoetl-cli completions zsh)"

# Fish (~/.config/fish/config.fish)
geoetl-cli completions fish | source
```

Restart your shell or source the config file:
```bash
source ~/.bashrc  # or ~/.zshrc
```

## Get Started Today

**Try shell completions**:

```bash
# After enabling completions, try:
geoetl-cli con<TAB>      # Autocompletes to 'convert'
geoetl-cli convert --in<TAB>  # Shows --input and --input-driver
```

**Convert data with the latest version**:

```bash
# GeoJSON to GeoParquet with tab completion
geoetl-cli convert \
  --input data.geojson \
  --output data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

## Documentation

- 📖 [Shell Completions Guide](https://github.com/geoyogesh/geoetl/blob/main/README.md#shell-completions)
- 📖 [Quick Reference](https://github.com/geoyogesh/geoetl/blob/main/QUICKREF.md)
- 📖 [GeoParquet ADR 004](https://github.com/geoyogesh/geoetl/blob/main/docs/adr/004-streaming-geoparquet-architecture.md)
- 📖 [Full Changelog](https://github.com/geoyogesh/geoetl/blob/main/CHANGELOG.md#031---2025-11-06)

## Get Involved

We need your help to implement the new format drivers!

**High-Priority Contributions**:
- 🎯 **FlatGeobuf driver** - Implementation help needed
- 🎯 **Shapefile reader** - Legacy format support
- 🎯 **GeoJSON optimization** - Performance improvements
- 🎯 **GeoPackage driver** - SQLite-based format

**How to Contribute**:
- ⭐ **Star us on GitHub**: [github.com/geoyogesh/geoetl](https://github.com/geoyogesh/geoetl)
- 🐛 **Report bugs**: [Open an issue](https://github.com/geoyogesh/geoetl/issues)
- 💬 **Discuss features**: [GitHub Discussions](https://github.com/geoyogesh/geoetl/discussions)
- 🔧 **Contribute code**: Check [DEVELOPMENT.md](https://github.com/geoyogesh/geoetl/blob/main/docs/DEVELOPMENT.md)
- 📣 **Spread the word**: Share your experience with GeoETL

---

**Download**: [GeoETL v0.3.1](https://github.com/geoyogesh/geoetl/releases/tag/v0.3.1)

*Questions or feedback? Join the conversation on [GitHub Discussions](https://github.com/geoyogesh/geoetl/discussions)!*
