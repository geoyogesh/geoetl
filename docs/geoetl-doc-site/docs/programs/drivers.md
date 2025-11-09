---
sidebar_position: 4
title: geoetl-cli drivers
description: List available format drivers
---

# geoetl-cli drivers

List all available format drivers and their capabilities.

## Synopsis

```bash
geoetl-cli drivers
```

## Description

The `drivers` command displays a table of all supported format drivers, showing their capabilities (Info, Read, Write support) and status.

## Examples

### List All Drivers

```bash
geoetl-cli drivers
```

**Output**:
```
Available Drivers:

┌─────────────────────┬──────────────────────────────┬────────────┬────────────┬────────────┐
│ Short Name          │ Long Name                    │ Info       │ Read       │ Write      │
├─────────────────────┼──────────────────────────────┼────────────┼────────────┼────────────┤
│ CSV                 │ Comma Separated Value (.csv) │ Supported  │ Supported  │ Supported  │
│ GeoJSON             │ GeoJSON                      │ Supported  │ Supported  │ Supported  │
│ GeoParquet          │ GeoParquet                   │ Supported  │ Supported  │ Supported  │
└─────────────────────┴──────────────────────────────┴────────────┴────────────┴────────────┘
```

## See Also

- [Supported Drivers](../drivers/supported-drivers.md) - Complete driver documentation
- [convert](./convert.md) - Convert data between formats
