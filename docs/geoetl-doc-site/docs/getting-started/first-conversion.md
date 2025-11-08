---
sidebar_position: 2
---

# Your First Conversion

Let's convert your first geospatial file with GeoETL! This hands-on tutorial will walk you through converting a GeoJSON file to CSV format.

## What You'll Learn

By the end of this tutorial, you'll know how to:

- Download sample geospatial data
- Use the `convert` command
- Verify the conversion succeeded
- Understand command options

**Time needed**: 5 minutes

## Prerequisites

- GeoETL installed (see [Installation Guide](./installation))
- Command-line terminal open

## Step 1: Download Sample Data

Download the Natural Earth cities dataset:

```bash
curl -O https://pub-f49e62c2a4114dc1bbbb62a1167ab950.r2.dev/readers/geojson/natural-earth_cities.geojson
```

This GeoJSON file contains 243 major world cities with population and geographic data.

## Step 2: Convert GeoJSON to CSV

Now let's convert this GeoJSON file to CSV format.

### Basic Conversion Command

```bash
geoetl-cli convert \
  --input natural-earth_cities.geojson \
  --output cities.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

### Command Breakdown

Let's understand each part:

- `geoetl-cli` - The GeoETL command-line tool
- `convert` - The conversion subcommand
- `--input natural-earth_cities.geojson` or `-i natural-earth_cities.geojson` - Input file
- `--output cities.csv` or `-o cities.csv` - Output file
- `--input-driver GeoJSON` - Tell GeoETL the input format
- `--output-driver CSV` - Tell GeoETL the output format

### Short Form (Equivalent)

You can use short flags to save typing:

```bash
geoetl-cli convert -i natural-earth_cities.geojson -o cities.csv \
  --input-driver GeoJSON --output-driver CSV
```

### Run the Conversion

Execute the command. You should see output like:

```
Conversion complete.
```

If successful, you'll have a new file: `cities.csv`

## Step 3: Verify the Result

Let's check the converted CSV file:

```bash
cat cities.csv
# or on Windows: type cities.csv
```

Expected output:
```csv
name,population,state,geometry
San Francisco,873965,California,"POINT(-122.4194 37.7749)"
New York,8336817,New York,"POINT(-74.006 40.7128)"
Chicago,2693976,Illinois,"POINT(-87.6298 41.8781)"
```

Notice that:
- ✅ All property fields became CSV columns
- ✅ Geometries were converted to WKT (Well-Known Text) format
- ✅ Data order is preserved

## Step 4: Understanding Verbose Output

Want to see more details during conversion?

```bash
geoetl-cli -v convert \
  -i natural-earth_cities.geojson \
  -o cities_verbose.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

The `-v` or `--verbose` flag shows:
- Input and output file paths
- Drivers being used
- Number of records processed
- Timing information

Example verbose output:
```
INFO geoetl_cli: Converting natural-earth_cities.geojson to cities_verbose.csv
INFO geoetl_core::operations: Starting conversion:
INFO geoetl_core::operations: Input: natural-earth_cities.geojson (Driver: GeoJSON)
INFO geoetl_core::operations: Output: cities_verbose.csv (Driver: CSV)
INFO geoetl_core::operations: Read 1 record batch(es)
INFO geoetl_core::operations: Total rows: 3
INFO geoetl_core::operations: Conversion completed successfully
```

## Common Issues and Solutions

### Issue: "Driver not found"

**Error**: `Input driver 'geojson' not found`

**Solution**: Driver names are case-sensitive. Use `GeoJSON` not `geojson`

```bash
# ❌ Wrong
--input-driver geojson

# ✅ Correct
--input-driver GeoJSON
```

### Issue: "File not found"

**Error**: `No such file or directory`

**Solution**: Check your file path and current directory

```bash
# Check current directory
pwd

# List files
ls -la

# Use absolute path if needed
geoetl-cli convert -i /full/path/to/natural-earth_cities.geojson -o cities.csv \
  --input-driver GeoJSON --output-driver CSV
```

### Issue: "Permission denied"

**Error**: `Permission denied`

**Solution**: Check file permissions

```bash
# Make file readable
chmod +r natural-earth_cities.geojson

# Or ensure output directory is writable
chmod +w .
```

## Practice Exercises

Try these to reinforce your learning:

### Exercise 1: Create Your Own Data

Create a GeoJSON file with your favorite locations:

```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {
        "name": "Your Favorite Place",
        "notes": "Why you like it"
      },
      "geometry": {
        "type": "Point",
        "coordinates": [longitude, latitude]
      }
    }
  ]
}
```

Convert it to CSV!

### Exercise 2: Try Different Formats

Convert between different format pairs:

```bash
# GeoJSON to CSV
geoetl-cli convert -i data.geojson -o data.csv \
  --input-driver GeoJSON --output-driver CSV

# CSV to GeoJSON
geoetl-cli convert -i data.csv -o data.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column geometry
```

### Exercise 3: Use Verbose Mode

Run all your conversions with `-v` flag and observe the output

## Key Takeaways

🎯 **What you learned**:

- The basic structure of the `convert` command
- How to specify input and output files
- How to specify format drivers
- How to verify conversions succeeded
- How to use verbose mode for debugging

🚀 **Skills unlocked**:

- Converting GeoJSON ↔ CSV
- Understanding WKT geometry format
- Troubleshooting common errors
- Reading command-line output

## Next Steps

Great work! You've completed your first conversion. 🎉

Continue learning:

👉 **Next: [Understanding Drivers](../tutorial-basics/understanding-drivers)** - Learn about supported formats

Or explore:
- [Working with GeoJSON](../tutorial-basics/working-with-geojson) - Web-standard format
- [Working with CSV](../tutorial-basics/working-with-csv) - Advanced CSV operations
- [Working with GeoParquet](../tutorial-basics/working-with-geoparquet) - Modern columnar format

## Quick Reference

```bash
# Basic conversion
geoetl-cli convert -i input.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV

# Verbose output
geoetl-cli -v convert -i input.geojson -o output.csv \
  --input-driver GeoJSON --output-driver CSV

# CSV to GeoJSON with geometry column
geoetl-cli convert -i input.csv -o output.geojson \
  --input-driver CSV --output-driver GeoJSON \
  --geometry-column wkt

# Get help
geoetl-cli convert --help
```

## Need Help?

- **Command help**: `geoetl-cli convert --help`
- **GitHub Issues**: [Report problems](https://github.com/geoyogesh/geoetl/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/geoyogesh/geoetl/discussions)
