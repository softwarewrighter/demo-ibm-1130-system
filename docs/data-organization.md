# Data Organization Plan

## Overview

This document defines the directory structure and naming conventions for organizing downloaded IBM 1130 software assets including source code, binary decks, and normalized formats ready for emulator loading.

## Directory Structure

```
data/
├── decks/
│   ├── algol/
│   │   ├── source/
│   │   │   ├── algol60-grenoble.txt
│   │   │   ├── algol68-osu.txt
│   │   │   └── README.md
│   │   ├── binary/
│   │   │   ├── algol60-grenoble.bin
│   │   │   └── README.md
│   │   └── normalized/
│   │       ├── algol60-grenoble.deck
│   │       └── metadata.toml
│   ├── apl/
│   │   ├── source/
│   │   │   ├── apl1130-v1.txt
│   │   │   ├── apl1130-v2.txt
│   │   │   ├── apl1130-examples.txt
│   │   │   └── README.md
│   │   ├── binary/
│   │   │   ├── apl1130-v1.bin
│   │   │   ├── apl1130-v2.bin
│   │   │   └── README.md
│   │   └── normalized/
│   │       ├── apl1130-v2.deck
│   │       ├── examples/
│   │       │   ├── matrix-ops.deck
│   │       │   ├── statistics.deck
│   │       │   └── factorial.deck
│   │       └── metadata.toml
│   ├── cobol/
│   │   ├── source/
│   │   │   ├── README.md (status: not yet located)
│   │   ├── binary/
│   │   │   └── README.md
│   │   └── normalized/
│   │       └── metadata.toml
│   ├── forth/
│   │   ├── source/
│   │   │   ├── forth68-charles-moore.asm
│   │   │   ├── forth68-listing.txt
│   │   │   ├── forth68-notes.txt
│   │   │   └── README.md
│   │   ├── binary/
│   │   │   ├── forth68.bin
│   │   │   └── README.md
│   │   └── normalized/
│   │       ├── forth68.deck
│   │       ├── examples/
│   │       │   ├── stack-demo.deck
│   │       │   ├── factorial.deck
│   │       │   └── graphics-2250.deck
│   │       └── metadata.toml
│   ├── fortran/
│   │   ├── source/
│   │   │   ├── roots.f
│   │   │   ├── csort.f
│   │   │   ├── list.f
│   │   │   ├── ssp-examples/
│   │   │   │   ├── matrix-mult.f
│   │   │   │   ├── diff-eq.f
│   │   │   │   └── fourier.f
│   │   │   └── README.md
│   │   ├── binary/
│   │   │   ├── roots.bin
│   │   │   ├── csort.bin
│   │   │   ├── list.bin
│   │   │   └── README.md
│   │   └── normalized/
│   │       ├── roots.deck
│   │       ├── csort.deck
│   │       ├── list.deck
│   │       ├── ssp/
│   │       │   └── (Scientific Subroutine Package decks)
│   │       └── metadata.toml
│   ├── lisp/
│   │   ├── source/
│   │   │   ├── lisp16-examples.txt
│   │   │   └── README.md
│   │   ├── binary/
│   │   │   ├── lisp16.bin
│   │   │   └── README.md
│   │   └── normalized/
│   │       ├── lisp16.deck
│   │       ├── examples/
│   │       │   ├── factorial.deck
│   │       │   ├── list-processing.deck
│   │       │   └── symbolic-diff.deck
│   │       └── metadata.toml
│   ├── rpg/
│   │   ├── source/
│   │   │   ├── report-gen-examples.txt
│   │   │   └── README.md
│   │   ├── binary/
│   │   │   └── README.md
│   │   └── normalized/
│   │       ├── examples/
│   │       │   ├── sales-report.deck
│   │       │   └── inventory-list.deck
│   │       └── metadata.toml
│   ├── assembly/
│   │   ├── source/
│   │   │   ├── dmsboot.asm
│   │   │   ├── zdcip.asm
│   │   │   ├── zcrdumpc.asm
│   │   │   ├── io-examples/
│   │   │   │   ├── card-reader.asm
│   │   │   │   ├── printer.asm
│   │   │   │   ├── disk-io.asm
│   │   │   │   └── plotter.asm
│   │   │   └── README.md
│   │   ├── binary/
│   │   │   ├── dmsboot.bin
│   │   │   └── README.md
│   │   └── normalized/
│   │       ├── dmsboot.deck
│   │       ├── io-examples/
│   │       │   └── (I/O demo decks)
│   │       └── metadata.toml
│   └── utilities/
│       ├── source/
│       │   ├── disk-copy.asm
│       │   ├── disk-init.asm
│       │   ├── memory-dump.asm
│       │   └── README.md
│       ├── binary/
│       │   └── README.md
│       └── normalized/
│           ├── disk-copy.deck
│           └── metadata.toml
├── disks/
│   ├── dms/
│   │   ├── dms-r2v12.dsk
│   │   ├── dms-r2v12-fortran.dsk
│   │   ├── dms-r2v12-apl.dsk
│   │   ├── dms-r2v12-rpg.dsk
│   │   └── README.md
│   ├── lisp/
│   │   ├── lisp16.dsk
│   │   └── README.md
│   ├── examples/
│   │   ├── blank-2315.dsk
│   │   ├── sample-data.dsk
│   │   └── README.md
│   └── metadata.toml
├── documentation/
│   ├── manuals/
│   │   ├── functional-characteristics-1966.pdf (symlink to downloaded)
│   │   ├── fortran-iv-language-1973.pdf
│   │   ├── apl1130-manual-1969.pdf
│   │   ├── dms-programming-guide-1972.pdf
│   │   └── README.md
│   ├── quick-reference/
│   │   ├── programmer-card-1968.pdf
│   │   ├── instruction-set.md
│   │   └── io-commands.md
│   └── metadata.toml
└── metadata/
    ├── catalog.toml          # Master catalog of all assets
    ├── downloads.toml        # Download sources and checksums
    └── licenses.toml         # License information for all files
```

## File Naming Conventions

### Source Files (`.txt`, `.asm`, `.f`, etc.)

**Pattern:** `{program-name}-{version}.{ext}`

**Examples:**
- `apl1130-v2.txt` - APL\1130 version 2 source code
- `forth68-charles-moore.asm` - Original 1968 Forth assembly
- `roots.f` - FORTRAN root-finding program
- `algol60-grenoble.txt` - Algol 60 from Grenoble

**Rules:**
- Lowercase with hyphens (kebab-case)
- Include version if multiple versions exist
- Include notable author/institution if relevant
- Use original file extension where possible
- For assembly: `.asm`
- For FORTRAN: `.f` or `.for`
- For generic text: `.txt`

### Binary Files (`.bin`)

**Pattern:** `{program-name}-{version}.bin`

**Examples:**
- `apl1130-v2.bin` - Binary object deck
- `forth68.bin` - Assembled Forth binary
- `roots.bin` - Compiled FORTRAN object code

**Rules:**
- Match corresponding source file name
- Always use `.bin` extension
- Raw binary format (Hollerith punched card encoding)

### Normalized Files (`.deck`)

**Pattern:** `{program-name}.deck`

**Examples:**
- `apl1130-v2.deck` - Normalized APL interpreter
- `forth68.deck` - Normalized Forth
- `matrix-ops.deck` - Matrix operations demo
- `sales-report.deck` - RPG sales report generator

**Rules:**
- Simulator-ready format
- Includes header with encoding metadata
- Standardized 80-column card format
- ASCII or EBCDIC encoding specified in header
- Can be directly loaded by emulator

## Metadata Format (TOML)

Each language/category directory contains a `metadata.toml` file:

```toml
[package]
language = "APL"
variant = "APL\\1130"
version = "2.0"
release_date = "1969"
authors = ["Larry Breed", "Phil Abrams", "Eric Iverson", "David Oldacre"]

[source]
origin = "IBM Type-III Library"
download_url = "http://ibm1130.org/sw/"
download_file = "apl_source.zip"
download_date = "2025-01-15"
checksum_sha256 = "..."

[license]
type = "IBM Type-III Library"
commercial_use = true
attribution_required = true
license_file = "../../licenses/IBM-Type-III.txt"

[[files]]
name = "apl1130-v2"
source = "source/apl1130-v2.txt"
binary = "binary/apl1130-v2.bin"
normalized = "normalized/apl1130-v2.deck"
description = "APL\\1130 Version 2 interpreter with extended identifiers"
entry_point = "APLSTART"
memory_required = 8192  # words

[[files]]
name = "matrix-ops"
normalized = "normalized/examples/matrix-ops.deck"
description = "Matrix multiplication and inversion examples"
requires = ["apl1130-v2"]
demo_category = "beginner"

[build]
assembler = "asm1130"
assembler_flags = ["-l", "-s"]
target_system = "DMS R2V12"

[notes]
historical_significance = "First publicly available APL system (1968)"
demo_priority = "high"
integration_status = "ready"
```

## File Type Specifications

### Source Files (`.txt`, `.asm`, `.f`)

**Format:** Plain text, UTF-8 encoding

**Contents:**
- Original source code as downloaded
- Comments preserved
- Original formatting maintained
- May include headers/documentation from original distribution

**Provenance:**
- Include URL in README.md
- Record download date
- Preserve original filename in comments

### Binary Files (`.bin`)

**Format:** Raw binary, Hollerith card encoding

**Layout:**
- 80 bytes per card
- Binary punched card format
- May include control cards
- Loader format (absolute or relocatable)

**Metadata Header (first card):**
```
Columns 1-8:   Magic number (0x49313133 = "I113")
Columns 9-16:  Entry point address (hex)
Columns 17-24: Load address (hex)
Columns 25-32: Length in words (hex)
Columns 33-80: Reserved
```

### Normalized Files (`.deck`)

**Format:** Structured card deck format for emulator

**Header Card (columns 1-80):**
```
@DECK {name} {encoding} {binary-mode} {entry-point}
```

**Example:**
```
@DECK APL1130V2 EBCDIC BINARY APLSTART
... card data follows ...
```

**Encoding Values:**
- `ASCII` - 7-bit ASCII encoding
- `EBCDIC` - IBM EBCDIC encoding
- `HOLLERITH` - Raw 12-punch encoding

**Binary Mode:**
- `TEXT` - Character data only
- `BINARY` - Binary object code
- `MIXED` - Both text and binary cards

**Card Data:**
- Lines 2-N: 80-character card images
- Trailing spaces preserved
- CRLF or LF line endings accepted
- UTF-8 file encoding (EBCDIC/ASCII in card data)

## Metadata Catalog (`data/metadata/catalog.toml`)

Master index of all available software:

```toml
[catalog]
version = "1.0"
last_updated = "2025-01-15"

[[languages]]
name = "APL\\1130"
category = "high-level"
status = "ready"
priority = "high"
file_count = 12
demo_count = 5

[[languages]]
name = "Forth"
category = "high-level"
status = "ready"
priority = "high"
file_count = 8
demo_count = 3

[[languages]]
name = "FORTRAN IV"
category = "high-level"
status = "ready"
priority = "medium"
file_count = 24
demo_count = 8

[[languages]]
name = "LISP 1.6"
category = "high-level"
status = "ready"
priority = "medium"
file_count = 6
demo_count = 4

[[languages]]
name = "Algol 60"
category = "high-level"
status = "pending-download"
priority = "low"
file_count = 0
demo_count = 0

[[languages]]
name = "COBOL"
category = "high-level"
status = "not-located"
priority = "low"
file_count = 0
demo_count = 0

[[demo_categories]]
name = "single-device"
description = "Demos using one I/O device"
count = 15

[[demo_categories]]
name = "multi-device"
description = "Demos coordinating multiple devices"
count = 8

[[demo_categories]]
name = "language-tutorial"
description = "Language learning examples"
count = 12
```

## Download Tracking (`data/metadata/downloads.toml`)

Track where files came from and verify integrity:

```toml
[[download]]
source_url = "http://ibm1130.org/sw/apl_source.zip"
local_file = "data/decks/apl/source/apl1130-v2.txt"
download_date = "2025-01-15"
checksum_sha256 = "abc123..."
file_size_bytes = 45678
verified = true

[[download]]
source_url = "https://github.com/monsonite/1968-FORTH/blob/master/FORTH68asm.txt"
local_file = "data/decks/forth/source/forth68-charles-moore.asm"
download_date = "2025-01-15"
checksum_sha256 = "def456..."
file_size_bytes = 28934
verified = true

[[download]]
source_url = "https://github.com/simh/simh/blob/master/Ibm1130/tests/roots.job"
local_file = "data/decks/fortran/source/roots.f"
download_date = "2025-01-15"
checksum_sha256 = "789abc..."
file_size_bytes = 1234
verified = true
```

## README.md Templates

Each language directory should have a README.md:

```markdown
# APL\1130

## Description

APL\1130 was the first publicly available APL system, released in Spring 1968
for the IBM 1130 computer. It became the most popular IBM Type-III Library
program ever released.

## Version Information

- **Version 1**: Single-character identifiers (1968)
- **Version 2**: Extended 6-character identifiers (1969)

## Files

### Source Code

- `apl1130-v1.txt` - Original version 1 source (645 lines)
- `apl1130-v2.txt` - Version 2 with extended identifiers
- `apl1130-examples.txt` - Example programs and tutorials

### Binary Decks

- `apl1130-v2.bin` - Assembled binary ready for loading

### Normalized Decks

- `normalized/apl1130-v2.deck` - Simulator-ready interpreter
- `normalized/examples/` - Demo programs

## Origin

Downloaded from IBM1130.org (http://ibm1130.org/sw/)

**Source File:** apl_source.zip
**Download Date:** 2025-01-15
**License:** IBM Type-III Library (open source)

## Authors

- Larry Breed (primary implementation)
- Phil Abrams (primary implementation)
- Eric Iverson (version 2 extensions)
- David Oldacre (version 2 extensions)

## Documentation

See `data/documentation/manuals/apl1130-manual-1969.pdf` for the complete
user manual (1130-03.3.001, May 1969).

## Integration Status

✅ **Ready for integration**

- Source code available and verified
- Assembler-compatible format
- Metadata complete
- Demos prepared
```

## Directory Creation Script

```bash
#!/bin/bash
# create-data-structure.sh

BASE="data"

# Create language directories
for lang in algol apl cobol forth fortran lisp rpg assembly utilities; do
    mkdir -p "$BASE/decks/$lang/{source,binary,normalized/examples}"
    touch "$BASE/decks/$lang/source/README.md"
    touch "$BASE/decks/$lang/binary/README.md"
    touch "$BASE/decks/$lang/normalized/metadata.toml"
done

# Create disk directories
mkdir -p "$BASE/disks/{dms,lisp,examples}"
touch "$BASE/disks/README.md"
touch "$BASE/disks/metadata.toml"

# Create documentation directories
mkdir -p "$BASE/documentation/{manuals,quick-reference}"
touch "$BASE/documentation/README.md"
touch "$BASE/documentation/metadata.toml"

# Create metadata directory
mkdir -p "$BASE/metadata"
touch "$BASE/metadata/catalog.toml"
touch "$BASE/metadata/downloads.toml"
touch "$BASE/metadata/licenses.toml"

echo "Data directory structure created successfully!"
```

## Integration Workflow

### 1. Download Phase

```bash
# Clone Forth repository
git clone https://github.com/monsonite/1968-FORTH.git temp/forth

# Extract source files
cp temp/forth/FORTH68asm.txt data/decks/forth/source/forth68-charles-moore.asm
cp temp/forth/FORTH68lst.txt data/decks/forth/source/forth68-listing.txt
cp temp/forth/FORTH-68_notes.txt data/decks/forth/source/forth68-notes.txt

# Update metadata
cd data/decks/forth
cat > source/README.md <<EOF
# Forth (1968) - Original Implementation

## Description
... (see template above) ...
EOF
```

### 2. Normalization Phase

```bash
# Convert to normalized format
cd data/decks/forth
python3 ../../../scripts/normalize-deck.py \
    --input source/forth68-charles-moore.asm \
    --output normalized/forth68.deck \
    --encoding EBCDIC \
    --mode BINARY \
    --entry-point FORTH

# Verify format
python3 ../../../scripts/verify-deck.py normalized/forth68.deck
```

### 3. Metadata Update Phase

```bash
# Update catalog
cd data/metadata
python3 ../../scripts/update-catalog.py \
    --language forth \
    --status ready \
    --priority high

# Update downloads tracker
python3 ../../scripts/track-download.py \
    --url "https://github.com/monsonite/1968-FORTH/blob/master/FORTH68asm.txt" \
    --file "data/decks/forth/source/forth68-charles-moore.asm" \
    --checksum "$(sha256sum ../decks/forth/source/forth68-charles-moore.asm | cut -d' ' -f1)"
```

### 4. Demo Creation Phase

```bash
# Create demo deck
cd data/decks/forth/normalized/examples
cat > factorial.deck <<EOF
@DECK FACTORIAL EBCDIC TEXT FORTH
: FACTORIAL DUP 1 > IF DUP 1- FACTORIAL * THEN ;
5 FACTORIAL .
EOF

# Update metadata
python3 ../../../../scripts/add-demo.py \
    --language forth \
    --name factorial \
    --category beginner \
    --description "Recursive factorial calculation"
```

## Future Enhancements

### Planned Features

1. **Automated Downloads**
   - Script to download all available software
   - Checksum verification
   - Update tracking

2. **Format Converters**
   - Binary → Normalized converter
   - ASCII ↔ EBCDIC transcoder
   - Card deck merger/splitter

3. **Validation Tools**
   - Deck format validator
   - Metadata completeness checker
   - Link verification

4. **Integration Scripts**
   - Auto-import into emulator
   - Demo generation wizard
   - Documentation extractor

5. **Web Interface**
   - Browse available software
   - Download individual decks
   - View metadata and documentation

## References

- Software Research: `docs/software-research.md`
- Extended Demos Plan: `docs/extended-demos.md`
- Emulator Integration: `docs/emulator-integration.md` (TODO)
