---
sidebar_position: 11
title: Glossary
description: Glossary of geospatial and GeoETL terms
---

# Glossary

Definitions of terms and acronyms used throughout the GeoETL documentation.

## A

### Apache Arrow

A cross-language development platform for in-memory data with a columnar memory format. GeoETL uses Arrow internally for efficient data processing.

### Apache DataFusion

A pluggable and extensible query execution framework that provides SQL and DataFrame APIs. GeoETL leverages DataFusion for vectorized execution and its extensible architecture for adding geospatial capabilities.

### Apache Parquet

A columnar storage file format optimized for use with big data processing frameworks. GeoParquet extends Parquet with geospatial capabilities.

## C

### CLI

Command-Line Interface. A text-based interface for running programs by typing commands. GeoETL is a CLI tool.

### Columnar Format

A file format that stores data by column rather than by row. Examples include Parquet and Arrow. Columnar formats enable efficient compression and fast column-oriented queries.

### Convert

The primary GeoETL command for converting data between formats:
```bash
geoetl-cli convert -i input.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV
```

### Coordinate Reference System (CRS)

A coordinate-based system used to locate geographical entities. Defines how coordinates relate to real-world locations. Common example: WGS84 (EPSG:4326).

### CSV

Comma Separated Value. A simple text file format that stores tabular data. In GeoETL, CSV files contain geometries in WKT format.

## D

### Dataset

A collection of geographic features and their associated data. In GeoETL, a dataset may be a single file or a folder (e.g., Shapefile uses multiple files, GeoParquet can be partitioned).

### Driver

A software component that enables GeoETL to read from and write to specific file formats. See [Supported Drivers](./drivers/supported-drivers) for the complete list.

## E

### ETL

Extract, Transform, Load. The process of extracting data from sources, transforming it to fit operational needs, and loading it into a target system.

### EPSG

European Petroleum Survey Group. Known for creating the EPSG Geodetic Parameter Dataset, a database of coordinate systems. Example: EPSG:4326 is WGS84.

## F

### Feature

A geographic entity with properties and geometry. In GeoJSON:
```json
{
  "type": "Feature",
  "properties": {"name": "San Francisco"},
  "geometry": {"type": "Point", "coordinates": [-122.4194, 37.7749]}
}
```

### FeatureCollection

A collection of features. The standard top-level GeoJSON structure:
```json
{
  "type": "FeatureCollection",
  "features": [...]
}
```

## G

### GeoArrow

A specification for storing geospatial data in Apache Arrow format. GeoETL uses GeoArrow types internally.

### GeoJSON

A JSON-based format for encoding geographic data structures (RFC 7946). The web-standard format for geospatial data.

### Geometry

The spatial component of a feature describing its shape and location. Types include Point, LineString, Polygon, and their Multi* variants.

### GeoParquet

A specification for storing geospatial data in Apache Parquet format, combining Parquet's columnar efficiency with geospatial capabilities.

### GDAL

Geospatial Data Abstraction Library. A comprehensive translator library for geospatial data formats. GeoETL is inspired by GDAL's ogr2ogr tool.

## I

### Info

A GeoETL command that displays information about a dataset:
```bash
geoetl-cli info data.geojson --driver GeoJSON
```

## J

### JSON

JavaScript Object Notation. A lightweight data-interchange format that is easy for humans to read and write. GeoJSON is based on JSON.

## L

### LineString

A geometry type representing a series of connected line segments. Example:
```json
{
  "type": "LineString",
  "coordinates": [[-122.4, 37.7], [-74.0, 40.7]]
}
```

## M

### MultiPoint, MultiLineString, MultiPolygon

Geometry types representing collections of Points, LineStrings, or Polygons respectively.

## O

### ogr2ogr

A GDAL command-line utility for converting between different vector data formats. GeoETL provides a modern alternative for common use cases.

## P

### Parquet

See **Apache Parquet**.

### Point

A geometry type representing a single location in coordinate space. Example:
```json
{
  "type": "Point",
  "coordinates": [-122.4194, 37.7749]
}
```

### Polygon

A geometry type representing a closed area. Example:
```json
{
  "type": "Polygon",
  "coordinates": [[
    [-122.4, 37.8], [-122.4, 37.7],
    [-122.5, 37.7], [-122.5, 37.8],
    [-122.4, 37.8]
  ]]
}
```

### Predicate Pushdown

A query optimization technique where filters are applied as early as possible, ideally before reading data from disk. GeoParquet supports this through columnar storage.

## R

### RFC 7946

The Internet Engineering Task Force (IETF) standard that defines the GeoJSON format. Published in August 2016.

### Row-Oriented Format

A file format that stores data by row (record). Examples include CSV and traditional JSON. Contrast with columnar formats.

### Rust

A systems programming language focused on safety, speed, and concurrency. GeoETL is written in Rust.

## S

### Shapefile

A popular geospatial vector data format developed by Esri. Support planned for GeoETL in Q1 2026.

### Spatial Reference System (SRS)

See **Coordinate Reference System (CRS)**.

### Streaming

A processing technique where data is read and written in chunks rather than loading everything into memory. GeoETL uses streaming for constant memory usage.

## V

### Vector Data

Geospatial data represented using geometric primitives such as points, lines, and polygons.

### Vectorized Execution

A processing technique where operations are performed on batches of data rather than individual records, leveraging modern CPU SIMD instructions for performance.

## W

### WGS84

World Geodetic System 1984. The standard coordinate system used by GPS and the default for GeoJSON (EPSG:4326).

### WKB

Well-Known Binary. A binary representation of geometries as defined by the OpenGIS Simple Features specification. Used in GeoParquet for geometry storage.

### WKT

Well-Known Text. A text representation of geometries as defined by the OpenGIS Simple Features specification. Widely used across geospatial data formats for human-readable geometry storage.

**Examples**:
```
POINT(-122.4194 37.7749)
LINESTRING(-122 37, -121 38)
POLYGON((-122 37, -121 37, -121 38, -122 38, -122 37))
```

## Format Abbreviations

| Abbreviation | Full Name | Description |
|--------------|-----------|-------------|
| **CSV** | Comma Separated Value | Text-based tabular format |
| **JSON** | JavaScript Object Notation | Text-based data interchange format |
| **WKB** | Well-Known Binary | Binary geometry encoding |
| **WKT** | Well-Known Text | Text geometry encoding |
| **CRS** | Coordinate Reference System | Coordinate system definition |
| **SRS** | Spatial Reference System | Same as CRS |
| **EPSG** | European Petroleum Survey Group | Coordinate system authority |
| **RFC** | Request for Comments | Internet standards documents |

## GeoETL-Specific Terms

### Driver

In GeoETL, a driver is a module that handles reading from and writing to a specific geospatial format. See [Supported Drivers](./drivers/supported-drivers) for the complete list.

### Capability

What a driver can do: **Info** (read metadata), **Read** (load data), **Write** (save data).

### Geometry Column

For CSV files, the column name containing WKT geometries. Must be specified with `--geometry-column`:
```bash
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry
```

## See Also

- [FAQ](./faq.md) - Frequently asked questions
- [Supported Drivers](./drivers/supported-drivers.md) - Complete driver list
- [GeoJSON Driver](./drivers/vector/geojson.md) - GeoJSON reference
- [CSV Driver](./drivers/vector/csv.md) - CSV reference
- [GeoParquet Driver](./drivers/vector/geoparquet.md) - GeoParquet reference

---

**Missing a term?** [Suggest an addition on GitHub](https://github.com/geoyogesh/geoetl/discussions) →
