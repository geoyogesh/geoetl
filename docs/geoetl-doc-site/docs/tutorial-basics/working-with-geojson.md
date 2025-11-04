---
sidebar_position: 4
---

# Working with GeoJSON

Learn how to use GeoJSON, the web-standard format for geospatial data.

## What is GeoJSON?

**GeoJSON** is a JSON-based format for encoding geographic data structures. It's the most widely used format for web mapping and JavaScript applications.

**Key features**:
- 📄 **Human-readable**: Plain JSON text format
- 🌐 **Web-standard**: Native support in browsers and mapping libraries
- 🔧 **Simple**: Easy to create, edit, and debug
- 📦 **Self-contained**: Geometries and properties in one file

**Why use GeoJSON?**
- Web mapping applications (Leaflet, Mapbox, OpenLayers)
- JavaScript/TypeScript projects
- API responses and data exchange
- Version control friendly (readable diffs)
- Quick prototyping and testing

## Quick Start

### Reading GeoJSON

**Inspect a GeoJSON file**:
```bash
geoetl-cli info data.geojson --driver GeoJSON
```

**Example output**:
```
Dataset: /path/to/data.geojson
Driver: GeoJSON

=== Geometry Columns ===
+----------+-------------------+-----+
| Column   | Extension         | CRS |
+----------+-------------------+-----+
| geometry | geoarrow.geometry | N/A |
+----------+-------------------+-----+

=== Fields ===
+------------+--------+----------+
| Field      | Type   | Nullable |
+------------+--------+----------+
| name       | String | Yes      |
| population | Int64  | Yes      |
+------------+--------+----------+
```

### Converting from GeoJSON

**To CSV** (for Excel):
```bash
geoetl-cli convert \
  --input data.geojson \
  --output data.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

**To GeoParquet** (for storage/performance):
```bash
geoetl-cli convert \
  --input data.geojson \
  --output data.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

### Converting to GeoJSON

**From CSV**:
```bash
geoetl-cli convert \
  --input data.csv \
  --output data.geojson \
  --input-driver CSV \
  --output-driver GeoJSON \
  --geometry-column wkt
```

**From GeoParquet**:
```bash
geoetl-cli convert \
  --input data.parquet \
  --output data.geojson \
  --input-driver GeoParquet \
  --output-driver GeoJSON
```

## GeoJSON Structure

### Feature Collection

The most common GeoJSON structure:

```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {
        "name": "San Francisco",
        "population": 873965,
        "state": "California"
      },
      "geometry": {
        "type": "Point",
        "coordinates": [-122.4194, 37.7749]
      }
    },
    {
      "type": "Feature",
      "properties": {
        "name": "New York",
        "population": 8336817,
        "state": "New York"
      },
      "geometry": {
        "type": "Point",
        "coordinates": [-74.006, 40.7128]
      }
    }
  ]
}
```

### Geometry Types

GeoJSON supports these geometry types:

**Point**:
```json
{
  "type": "Point",
  "coordinates": [-122.4194, 37.7749]
}
```

**LineString**:
```json
{
  "type": "LineString",
  "coordinates": [
    [-122.4194, 37.7749],
    [-74.006, 40.7128]
  ]
}
```

**Polygon**:
```json
{
  "type": "Polygon",
  "coordinates": [
    [
      [-122.4, 37.8],
      [-122.4, 37.7],
      [-122.5, 37.7],
      [-122.5, 37.8],
      [-122.4, 37.8]
    ]
  ]
}
```

**MultiPoint**:
```json
{
  "type": "MultiPoint",
  "coordinates": [
    [-122.4194, 37.7749],
    [-74.006, 40.7128]
  ]
}
```

**MultiLineString**, **MultiPolygon**, **GeometryCollection** are also supported.

## Use Cases

### 1. Web Mapping

**Problem**: Display data on a web map

**Solution**: Use GeoJSON with Leaflet, Mapbox, or OpenLayers

```bash
# Convert your data to GeoJSON for web use
geoetl-cli convert -i cities.csv -o cities.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt
```

Then use in JavaScript:
```javascript
// Leaflet example
fetch('cities.geojson')
  .then(response => response.json())
  .then(data => {
    L.geoJSON(data).addTo(map);
  });
```

### 2. API Responses

**Problem**: Serve geospatial data via REST API

**Solution**: Convert to GeoJSON for standard API responses

```bash
# Convert data to GeoJSON format
geoetl-cli convert -i database.parquet -o api_response.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

### 3. Data Sharing

**Problem**: Share geospatial data with collaborators

**Solution**: GeoJSON is human-readable and works everywhere

```bash
# Create shareable GeoJSON
geoetl-cli convert -i proprietary_format.csv -o shareable.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry
```

### 4. Quick Prototyping

**Problem**: Test spatial queries and operations

**Solution**: GeoJSON is easy to create and modify manually

**Benefits**:
- Edit in any text editor
- View on GitHub (renders maps automatically)
- Easy to debug (readable JSON)
- Copy-paste into online tools

## Performance Characteristics

### File Size Comparison (1M features)

| Format | Size | vs GeoJSON |
|--------|------|-----------|
| **GeoJSON** | **114.13 MB** | **baseline** |
| CSV | 32.11 MB | 3.5x smaller |
| GeoParquet | 16.86 MB | 6.8x smaller |

**Observation**: GeoJSON files are larger due to verbose JSON text format.

### Processing Speed (1M features)

| Conversion | Throughput | Duration |
|------------|-----------|----------|
| **GeoJSON → GeoJSON** | **300 MB/min** | **23s** |
| GeoJSON → CSV | 300 MB/min | 23s |
| GeoJSON → GeoParquet | 3,804 MB/min | 2s |

**Observation**: Converting GeoJSON to GeoParquet is much faster than GeoJSON to GeoJSON.

### When to Use GeoJSON

✅ **Use GeoJSON when**:
- Building web applications
- Small to medium datasets (&lt;100k features)
- Need human-readable format
- Sharing data with others
- Version control is important
- Working with JavaScript/TypeScript

❌ **Avoid GeoJSON when**:
- Large datasets (&gt;1M features) - use GeoParquet instead
- Storage efficiency is critical - use GeoParquet (6.8x smaller)
- Processing performance matters - use GeoParquet (11x faster)
- Working with analytics tools - use GeoParquet (columnar format)

## Common Patterns

### Web + Storage Workflow

**Pattern**: Store efficiently, serve as GeoJSON

```bash
# 1. Store in efficient format (GeoParquet)
geoetl-cli convert -i source.geojson -o storage.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# 2. Generate GeoJSON for web on-demand
geoetl-cli convert -i storage.parquet -o web_api.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

**Benefits**:
- Save 6.8x storage space with GeoParquet
- Fast generation when needed
- Best of both worlds

### Data Collection to Production

**Pattern**: Collect as GeoJSON, process as GeoParquet

```bash
# 1. Collect data as GeoJSON (easy to create)
# ... create data.geojson manually or via API

# 2. Convert to GeoParquet for processing
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# 3. Process with efficient format
# ... run analytics on data.parquet

# 4. Export results as GeoJSON for sharing
geoetl-cli convert -i results.parquet -o results.geojson \
  --input-driver GeoParquet --output-driver GeoJSON
```

### Validation Workflow

**Pattern**: Validate GeoJSON before using

```bash
# 1. Check GeoJSON structure
geoetl-cli info data.geojson -f GeoJSON

# 2. Test conversion (validates format)
geoetl-cli convert -i data.geojson -o test.geojson \
  --input-driver GeoJSON --output-driver GeoJSON

# 3. If successful, use the file
```

## Working with Tools

### GitHub

GitHub automatically renders GeoJSON files as maps:

1. Push `.geojson` file to repository
2. View file on GitHub
3. See automatic map visualization

### QGIS

Open GeoJSON directly:
1. **Layer → Add Layer → Add Vector Layer**
2. Select your `.geojson` file
3. QGIS loads geometry and properties

### Online Viewers

**geojson.io**:
1. Visit http://geojson.io
2. Drag-drop your `.geojson` file
3. View, edit, and validate

**mapshaper.org**:
1. Visit https://mapshaper.org
2. Import `.geojson` file
3. Simplify, convert, or analyze

### JavaScript Libraries

**Leaflet**:
```javascript
L.geoJSON(geojsonData).addTo(map);
```

**Mapbox GL JS**:
```javascript
map.addSource('data', {
  type: 'geojson',
  data: 'data.geojson'
});
```

## Best Practices

### 1. Keep Files Reasonably Sized

**Rule of thumb**:
- &lt; 1 MB: Excellent for web
- 1-10 MB: Good for web (consider tiling)
- 10-100 MB: Slow for web, convert to tiles
- &gt; 100 MB: Use GeoParquet instead

### 2. Validate Before Sharing

```bash
# Validate by attempting conversion
geoetl-cli convert -i data.geojson -o validated.geojson \
  --input-driver GeoJSON --output-driver GeoJSON

# Check schema
geoetl-cli info data.geojson -f GeoJSON
```

### 3. Use Proper Coordinate Order

**Correct**: `[longitude, latitude]`
```json
"coordinates": [-122.4194, 37.7749]
```

**Wrong**: `[latitude, longitude]` ❌
```json
"coordinates": [37.7749, -122.4194]
```

### 4. Pretty Print for Version Control

GeoJSON works well with Git when formatted:

```bash
# Pretty-print JSON for better diffs
cat data.geojson | python3 -m json.tool > formatted.geojson
```

### 5. Include Properties

Always include meaningful properties:

```json
{
  "type": "Feature",
  "properties": {
    "id": "SF001",
    "name": "San Francisco",
    "population": 873965,
    "updated": "2024-01-15"
  },
  "geometry": { "type": "Point", "coordinates": [-122.4194, 37.7749] }
}
```

## Limitations & Solutions

### 1. Large File Size

**Limitation**: GeoJSON files are verbose

**Solution**: Convert to GeoParquet for storage
```bash
geoetl-cli convert -i large.geojson -o compressed.parquet \
  --input-driver GeoJSON --output-driver GeoParquet
```

### 2. Slow Processing

**Limitation**: Text parsing is slower than binary formats

**Solution**: Use GeoParquet for processing pipelines
```bash
# Convert once
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# Process with GeoParquet (11x faster)
```

### 3. No Spatial Index

**Limitation**: GeoJSON has no built-in spatial index

**Solution**: Use FlatGeobuf or GeoPackage for indexed access (coming in v0.4.0+)

## Troubleshooting

### Invalid JSON

**Error**: `Parse error in GeoJSON at line 15`

**Fix**: Validate JSON syntax
```bash
# Use jq to validate
cat data.geojson | jq . > /dev/null

# Or Python
python3 -m json.tool data.geojson > /dev/null
```

### Wrong Coordinate Order

**Error**: Points appear in wrong location

**Fix**: Ensure `[longitude, latitude]` order
```json
// Correct: longitude first
"coordinates": [-122.4194, 37.7749]
```

### Missing Properties

**Error**: Empty properties object

**Fix**: Ensure each feature has properties
```json
{
  "type": "Feature",
  "properties": {},  // ❌ Empty
  "geometry": { ... }
}

{
  "type": "Feature",
  "properties": { "name": "value" },  // ✅ Good
  "geometry": { ... }
}
```

## Quick Reference

### Essential Commands

```bash
# Inspect GeoJSON
geoetl-cli info data.geojson -f GeoJSON

# GeoJSON to CSV
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV

# GeoJSON to GeoParquet
geoetl-cli convert -i data.geojson -o data.parquet \
  --input-driver GeoJSON --output-driver GeoParquet

# CSV to GeoJSON
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt

# Validate GeoJSON
geoetl-cli convert -i data.geojson -o validated.geojson \
  --input-driver GeoJSON --output-driver GeoJSON
```

## Key Takeaways

🎯 **What you learned**:
- GeoJSON is the web-standard format for geospatial data
- Best for web apps, small datasets, and human readability
- Works seamlessly with JavaScript and mapping libraries
- GitHub and many tools render GeoJSON automatically

🚀 **Skills unlocked**:
- Converting to/from GeoJSON
- Understanding GeoJSON structure and geometry types
- Choosing between GeoJSON and GeoParquet
- Using GeoJSON with web mapping tools

## Next Steps

Continue learning:

👉 **Next: [Working with GeoParquet](./working-with-geoparquet)** - High-performance columnar format

Or explore:
- [Understanding Drivers](./understanding-drivers) - Driver capabilities
- [Error Handling & Troubleshooting](./troubleshooting) - Debug issues

## References

- [GeoJSON Specification (RFC 7946)](https://datatracker.ietf.org/doc/html/rfc7946)
- [geojson.io](http://geojson.io) - Online editor and validator
- [More than you ever wanted to know about GeoJSON](https://macwright.com/2015/03/23/geojson-second-bite.html)

## Need Help?

- **Command help**: `geoetl-cli convert --help`
- **GitHub Issues**: [Report problems](https://github.com/geoyogesh/geoetl/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/geoyogesh/geoetl/discussions)
