# GeoETL Documentation Structure and Guidelines

**Version:** 1.0
**Date:** 2025-01-08
**Status:** Proposed
**Inspired by:** [GDAL Documentation](https://gdal.org/documentation.html)

---

## Table of Contents

1. [Overview](#overview)
2. [Documentation Philosophy](#documentation-philosophy)
3. [Proposed Structure](#proposed-structure)
4. [Directory Organization](#directory-organization)
5. [Content Guidelines](#content-guidelines)
6. [Driver Documentation Template](#driver-documentation-template)
7. [Command Documentation Template](#command-documentation-template)
8. [Migration Plan](#migration-plan)
9. [Writing Style Guide](#writing-style-guide)

---

## Overview

This document defines the structure and guidelines for GeoETL's documentation website. The structure is inspired by GDAL's mature documentation organization, adapted for GeoETL's specific needs as a modern CLI tool.

### Goals

- **Discoverability**: Users can find information quickly
- **Consistency**: Uniform structure across all pages
- **Scalability**: Easy to add new drivers, commands, and guides
- **Multi-level**: Serve both beginners and advanced users
- **Maintainability**: Clear ownership and update processes

---

## Documentation Philosophy

### Core Principles

1. **Separation of Concerns**: Keep tutorials, reference, and conceptual content separate
2. **Progressive Disclosure**: Start simple, allow drilling down
3. **Task-Oriented**: Organize by what users want to accomplish
4. **Examples First**: Show working code/commands before explaining
5. **Cross-Linking**: Connect related content extensively

### Target Audiences

1. **New Users** → Quick Start, Tutorials
2. **Regular Users** → User Guide, How-To Guides
3. **Advanced Users** → Reference, Performance Tuning
4. **Contributors** → Development, Contributing
5. **Migrators** → Migration Guides (from ogr2ogr, GDAL, etc.)

---

## Proposed Structure

```
docs/geoetl-doc-site/docs/
├── intro.md                                # Welcome & Quick Start
├── download.md                             # Downloads & Installation
├── faq.md                                  # Frequently Asked Questions
├── glossary.md                             # Terms and Definitions
│
├── getting-started/                        # Quick Start Guides
│   ├── _category_.json
│   ├── index.md                           # Getting Started Overview
│   ├── installation.md                    # Detailed installation
│   ├── first-conversion.md                # Your first command
│   └── common-workflows.md                # Common use cases
│
├── programs/                               # CLI Commands Reference
│   ├── _category_.json
│   ├── index.md                           # Programs Overview
│   ├── geoetl-cli.md                      # Main CLI entry point
│   ├── convert.md                         # convert command
│   ├── info.md                            # info command
│   ├── drivers.md                         # drivers command
│   ├── completions.md                     # completions command
│   └── common-options.md                  # Shared options reference
│
├── drivers/                                # Format Driver Documentation
│   ├── _category_.json
│   ├── index.md                           # Drivers Overview
│   ├── driver-comparison.md               # Format comparison table
│   ├── vector/                            # Vector format drivers
│   │   ├── _category_.json
│   │   ├── index.md                       # Vector drivers overview
│   │   ├── geojson.md                     # GeoJSON driver
│   │   ├── csv.md                         # CSV driver
│   │   ├── geoparquet.md                  # GeoParquet driver
│   │   └── [future-drivers].md
│   └── raster/                            # (Future) Raster drivers
│       └── index.md                       # Placeholder
│
├── user-guide/                             # Conceptual Documentation
│   ├── _category_.json
│   ├── index.md                           # User Guide Overview
│   ├── data-models.md                     # Understanding data models
│   ├── geometry-formats.md                # WKT, WKB, GeoJSON geometries
│   ├── coordinate-systems.md              # CRS and projections
│   ├── performance.md                     # Performance considerations
│   ├── error-handling.md                  # Understanding errors
│   ├── configuration.md                   # Config files and env vars
│   └── security.md                        # Security best practices
│
├── tutorials/                              # Step-by-Step Tutorials
│   ├── _category_.json
│   ├── index.md                           # Tutorials Overview
│   ├── basic/                             # Beginner tutorials
│   │   ├── _category_.json
│   │   ├── converting-geojson-csv.md
│   │   ├── working-with-large-files.md
│   │   └── batch-processing.md
│   ├── intermediate/                      # Intermediate tutorials
│   │   ├── _category_.json
│   │   ├── geoparquet-analytics.md
│   │   ├── csv-coordinate-conversion.md
│   │   └── format-migration.md
│   └── advanced/                          # Advanced tutorials
│       ├── _category_.json
│       ├── performance-tuning.md
│       ├── custom-workflows.md
│       └── integration-pipelines.md
│
├── how-to/                                 # Task-Oriented Guides
│   ├── _category_.json
│   ├── index.md                           # How-To Overview
│   ├── convert-formats.md                 # Format conversion recipes
│   ├── validate-data.md                   # Data validation
│   ├── optimize-storage.md                # Storage optimization
│   ├── handle-large-datasets.md           # Large dataset strategies
│   ├── integrate-with-tools.md            # Tool integration
│   ├── troubleshooting.md                 # Common problems & solutions
│   └── best-practices.md                  # Best practices
│
├── reference/                              # Quick Reference Materials
│   ├── _category_.json
│   ├── index.md                           # Reference Overview
│   ├── command-cheatsheet.md              # Quick command reference
│   ├── driver-matrix.md                   # Driver capability matrix
│   ├── geometry-types.md                  # Geometry type reference
│   ├── error-codes.md                     # Error code reference
│   └── benchmarks.md                      # Performance benchmarks
│
├── migration/                              # Migration Guides
│   ├── _category_.json
│   ├── index.md                           # Migration Overview
│   ├── from-ogr2ogr.md                    # Migrating from ogr2ogr
│   ├── from-gdal.md                       # Migrating from GDAL
│   └── from-other-tools.md                # Other tools
│
├── community/                              # Community & Support
│   ├── _category_.json
│   ├── index.md                           # Community Overview
│   ├── getting-help.md                    # Where to get help
│   ├── contributing.md                    # Contributing guide
│   ├── roadmap.md                         # Project roadmap
│   └── changelog.md                       # Version changelog
│
└── development/                            # Developer Documentation
    ├── _category_.json
    ├── index.md                           # Development Overview
    ├── architecture.md                    # System architecture
    ├── building.md                        # Building from source
    ├── testing.md                         # Testing guide
    ├── driver-development.md              # Creating new drivers
    └── contributing-code.md               # Code contribution guide
```

---

## Directory Organization

### Category Configuration Files

Each directory should have a `_category_.json` file:

```json
{
  "label": "Category Display Name",
  "position": 1,
  "link": {
    "type": "generated-index",
    "description": "Brief description of this section"
  },
  "collapsed": false
}
```

### Index Files

Each directory should have an `index.md` that:
- Provides an overview of the section
- Lists key pages with descriptions
- Guides users to appropriate content
- Shows navigation paths

---

## Content Guidelines

### Page Structure

Every documentation page should follow this structure:

```markdown
---
sidebar_position: N
title: Page Title
description: Brief description for SEO
---

# Page Title

Brief introduction paragraph (1-2 sentences).

## Overview / What You'll Learn

Bullet points of key takeaways.

## Prerequisites (if applicable)

What users need before starting.

## Main Content

Organized with clear H2 and H3 headings.

### Examples

Always include working examples.

## Common Issues / Troubleshooting (if applicable)

Known problems and solutions.

## See Also

Links to related documentation.

## References

External links and resources.
```

### Required Elements

1. **Frontmatter**: `sidebar_position`, `title`, optionally `description`
2. **Introduction**: Clear opening that explains the page's purpose
3. **Examples**: Real, working code/commands
4. **Cross-links**: Link to related content
5. **Metadata**: Version introduced, status (stable/experimental)

### Optional Elements

- Tips/Warnings (Docusaurus admonitions)
- Performance notes
- Version compatibility
- Migration notes

---

## Driver Documentation Template

All driver documentation pages should follow this template:

```markdown
---
sidebar_position: N
title: DriverName
description: Brief description of the format
---

# DriverName

Brief description of the format and its purpose.

## Driver Metadata

| Property | Value |
|----------|-------|
| **Short Name** | DRIVERNAME |
| **Long Name** | Full Format Name |
| **Supported Since** | v0.X.0 |
| **Status** | Stable / Experimental / Planned |
| **Capabilities** | Read / Write / Info |

## Driver Capabilities

- ✅ **Read Support**: Full / Partial / Planned
- ✅ **Write Support**: Full / Partial / Planned
- ✅ **Info Support**: Yes / No
- ✅ **Geometry Types**: Point, LineString, Polygon, etc.
- ✅ **Coordinate Systems**: Supported / Planned

## Format Description

Detailed description of the format:
- What it is
- Common use cases
- Strengths and weaknesses
- Industry adoption

## File Structure

Explain the file format structure (if relevant).

## Reading Data

### Basic Read

```bash
geoetl-cli info input.ext -f DRIVERNAME
```

### Read with Options

```bash
geoetl-cli convert -i input.ext -o output.ext \
  --input-driver DRIVERNAME \
  --option1 value1
```

### Supported Read Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `option1` | string | default | What it does |

## Writing Data

### Basic Write

```bash
geoetl-cli convert -i input.ext -o output.ext \
  --output-driver DRIVERNAME
```

### Write with Options

```bash
geoetl-cli convert -i input.ext -o output.ext \
  --output-driver DRIVERNAME \
  --option1 value1
```

### Supported Write Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `option1` | string | default | What it does |

## Examples

### Example 1: [Use Case]

```bash
# Description
geoetl-cli convert -i input.geojson -o output.ext \
  --input-driver GeoJSON \
  --output-driver DRIVERNAME
```

### Example 2: [Another Use Case]

```bash
# Description
geoetl-cli convert -i input.ext -o output.geojson \
  --input-driver DRIVERNAME \
  --output-driver GeoJSON
```

## Performance Characteristics

### File Size

Comparison with other formats (if benchmarked).

### Processing Speed

Throughput data (if benchmarked).

### Memory Usage

Memory characteristics (if benchmarked).

## Limitations

- Known limitation 1
- Known limitation 2

## Working with Other Tools

### QGIS

How to use this format in QGIS.

### DuckDB / Database Tools

Integration examples.

### Programming Languages

Python, JavaScript, etc. examples.

## Troubleshooting

### Common Error 1

**Problem**: Description
**Solution**: Fix

### Common Error 2

**Problem**: Description
**Solution**: Fix

## Format Specification

Link to official format specification (if available).

## See Also

- Related drivers
- Related tutorials
- Related how-to guides

## References

- Official specification
- Related RFCs
- External documentation
```

---

## Command Documentation Template

All command documentation pages should follow this template:

```markdown
---
sidebar_position: N
title: geoetl-cli commandname
description: Brief description of what the command does
---

# geoetl-cli commandname

Brief description of the command's purpose.

## Synopsis

```bash
geoetl-cli commandname [OPTIONS] ARGUMENTS
```

## Description

Detailed explanation of what the command does and when to use it.

## Options

### Required Options

| Option | Short | Type | Description |
|--------|-------|------|-------------|
| `--input` | `-i` | path | Input file path |
| `--output` | `-o` | path | Output file path |

### Optional Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--verbose` | `-v` | flag | false | Enable verbose output |

### Global Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--help` | `-h` | flag | - | Show help message |
| `--version` | `-V` | flag | - | Show version |

## Examples

### Example 1: Basic Usage

```bash
geoetl-cli commandname -i input.geojson -o output.csv
```

**What this does:**
- Explains the command

### Example 2: With Options

```bash
geoetl-cli commandname -i input.geojson -o output.csv \
  --option1 value1 \
  --option2 value2
```

**What this does:**
- Explains with options

### Example 3: Advanced Usage

```bash
geoetl-cli -v commandname \
  -i large_dataset.geojson \
  -o optimized.parquet \
  --input-driver GeoJSON \
  --output-driver GeoParquet
```

**What this does:**
- Explains advanced usage

## Common Workflows

### Workflow 1: [Common Task]

```bash
# Step 1: Prepare data
geoetl-cli info input.geojson -f GeoJSON

# Step 2: Convert
geoetl-cli commandname -i input.geojson -o output.csv
```

### Workflow 2: [Another Task]

Description and commands.

## Error Messages

### Error: "Message text"

**Cause**: Why this happens
**Solution**: How to fix it

## Performance Considerations

Tips for optimal performance with this command.

## See Also

- Related commands
- Related tutorials
- Related how-to guides

## Version History

- **v0.3.0**: Added feature X
- **v0.1.0**: Initial implementation
```

---

## Migration Plan

### Phase 1: Foundation (Week 1)
- [ ] Create new directory structure
- [ ] Write `faq.md`
- [ ] Write `glossary.md`
- [ ] Update `intro.md` with new navigation

### Phase 2: Core Documentation (Week 2)
- [ ] Migrate and enhance `getting-started/` section
- [ ] Create `programs/` section with command references
- [ ] Reorganize driver documentation into `drivers/vector/`

### Phase 3: Enhanced Content (Week 3)
- [ ] Create `user-guide/` with conceptual content
- [ ] Reorganize tutorials into difficulty levels
- [ ] Create `how-to/` guides for common tasks

### Phase 4: Reference & Support (Week 4)
- [ ] Create `reference/` section with cheatsheets
- [ ] Create `migration/` guides from other tools
- [ ] Enhance `community/` section

### Phase 5: Polish & Launch
- [ ] Review all cross-links
- [ ] Add search keywords
- [ ] Test all examples
- [ ] Update navigation
- [ ] Announce new documentation

---

## Writing Style Guide

### Voice and Tone

- **Active voice**: "Convert the file" not "The file is converted"
- **Direct address**: Use "you" to address the reader
- **Clear and concise**: Short sentences and paragraphs
- **Professional but friendly**: Approachable technical writing

### Formatting Conventions

#### Code Blocks

```bash
# Always include comments explaining complex commands
geoetl-cli convert -i input.geojson -o output.csv \
  --input-driver GeoJSON \
  --output-driver CSV
```

#### File Paths

- Use backticks: `path/to/file.geojson`
- Use forward slashes (Unix-style) by default
- Mention Windows alternatives when relevant

#### Command Options

- Long form: `--input-driver`
- Short form: `-i`
- Format: `--option value` or `--option=value`

#### Admonitions

```markdown
:::tip
Use GeoParquet for files larger than 1M features.
:::

:::warning
The `--geometry-column` option is required for CSV input.
:::

:::danger
This operation cannot be undone.
:::

:::note
This feature was added in v0.3.0.
:::

:::info
See [Working with CSV](./csv.md) for more details.
:::
```

### Terminology

Use consistent terminology throughout:

- **Driver** (not format handler, converter)
- **Command** (not utility, tool when referring to CLI)
- **Dataset** (not file, when referring to data)
- **Feature** (not record, when referring to vector data)
- **Geometry** (not shape, spatial object)

### Version References

- Current features: No version mentioned
- New features: "Added in v0.3.0"
- Deprecated features: "Deprecated since v0.3.0, removed in v0.4.0"
- Changed features: "Changed in v0.3.0: [description of change]"

### Cross-Referencing

```markdown
See [Working with CSV](../tutorials/basic/working-with-csv.md) for details.

For more information, consult the [CSV Driver Reference](../drivers/vector/csv.md).

Related commands:
- [geoetl-cli convert](../programs/convert.md)
- [geoetl-cli info](../programs/info.md)
```

---

## Glossary of Common Terms

Maintain consistency by using these defined terms:

- **CRS**: Coordinate Reference System
- **WKT**: Well-Known Text
- **WKB**: Well-Known Binary
- **Feature**: A geographic entity with properties and geometry
- **Dataset**: A collection of features or rasters
- **Driver**: Software component that reads/writes a format
- **CLI**: Command-Line Interface

---

## Review Process

### Before Merging

All documentation changes should:

1. ✅ Follow the structure guidelines
2. ✅ Include working examples
3. ✅ Have proper cross-links
4. ✅ Use consistent terminology
5. ✅ Pass spell check
6. ✅ Render correctly in Docusaurus
7. ✅ Include version information if relevant

### Documentation Checklist

```markdown
- [ ] Follows template structure
- [ ] Examples are tested and working
- [ ] Cross-links are valid
- [ ] Terminology is consistent
- [ ] Grammar and spelling checked
- [ ] Renders correctly locally
- [ ] Screenshots current (if any)
- [ ] Version info included (if new feature)
```

---

## Maintenance

### Regular Updates

- **Weekly**: Review new issues for documentation gaps
- **Monthly**: Update benchmarks if performance changes
- **Per Release**: Update version references, changelog, roadmap

### Ownership

| Section | Owner | Review Frequency |
|---------|-------|------------------|
| Getting Started | Core Team | Every release |
| Programs | Core Team | Every release |
| Drivers | Driver maintainers | Per driver update |
| Tutorials | Community + Core | Quarterly |
| User Guide | Core Team | Semi-annually |

---

## Resources

### Inspiration Sources

- [GDAL Documentation](https://gdal.org/)
- [Divio Documentation System](https://documentation.divio.com/)
- [Write the Docs](https://www.writethedocs.org/)
- [Google Developer Documentation Style Guide](https://developers.google.com/style)

### Tools

- **Docusaurus**: Static site generator
- **Markdown**: Documentation format
- **Vale**: Prose linting
- **markdownlint**: Markdown linting

---

## Appendix: Quick Reference

### Document Type Decision Tree

```
Need to...
├─ Learn how to do something specific?
│  └─ How-To Guide
├─ Understand a concept?
│  └─ User Guide
├─ Follow step-by-step learning?
│  └─ Tutorial
├─ Look up command/driver details?
│  └─ Reference (Programs/Drivers)
└─ Get started quickly?
   └─ Getting Started
```

### Metadata Template

```markdown
---
sidebar_position: N
title: Page Title
description: Brief description
tags:
  - category1
  - category2
---
```

---

**Last Updated**: 2025-01-08
**Next Review**: After v0.4.0 release
