---
sidebar_position: 3
title: geoetl-cli info
description: Display dataset information
---

# geoetl-cli info

Display information about a geospatial dataset.

## Synopsis

```bash
geoetl-cli info [OPTIONS] <PATH> --driver <DRIVER>
```

## Description

The `info` command displays metadata and schema information about a geospatial dataset without converting it. Use this command to inspect file contents, verify format, and understand data structure.

## Options

| Option | Short | Type | Required | Description |
|--------|-------|------|----------|-------------|
| `<PATH>` | | path | Yes | Input file path |
| `--driver` | `-f` | string | Yes | Format driver (see [supported drivers](../drivers/supported-drivers)) |
| `--geometry-column` | | string | For CSV | Column containing WKT geometries |
| `--help` | `-h` | flag | No | Print help information |

## Examples

### Example 1: Inspect GeoJSON

```bash
geoetl-cli info cities.geojson --driver GeoJSON
```

### Example 2: Inspect CSV

```bash
geoetl-cli info data.csv --driver CSV --geometry-column geometry
```

### Example 3: Inspect GeoParquet

```bash
geoetl-cli info data.parquet --driver GeoParquet
```

## See Also

- [convert](./convert.md) - Convert data between formats
- [drivers](./drivers.md) - List available drivers
