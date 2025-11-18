# File Formats

Binary file format specifications for disk images and card decks.

## Overview

The simulator uses two primary file formats:

| Format | Extension | Purpose | Encoding |
|--------|-----------|---------|----------|
| Disk Image | `.dsk` | 2310/2311 disk contents | Binary (little-endian u16) |
| Card Deck | `.deck` | 80-column card stack | Text (EBCDIC/ASCII) or binary |

## .dsk File Format (Disk Image)

### Purpose

Store complete disk cartridge/pack contents with geometry metadata for loading into simulated disk drives.

### File Structure

```
+------------------+
| Header (56 bytes)|
+------------------+
| Sector Data      |
| (variable size)  |
+------------------+
```

### Header Layout

```rust
struct DskHeader {
    magic: [u8; 8],        // Offset 0: "I1130DSK" (ASCII)
    geometry: Geometry,    // Offset 8: 16 bytes
    reserved: [u8; 32],    // Offset 24: Reserved for future use
}
// Total: 56 bytes

struct Geometry {
    cylinders: u16,        // Number of cylinders (e.g., 200)
    heads: u8,             // Number of heads (e.g., 2)
    sectors_per_track: u8, // Sectors per track (e.g., 4)
    words_per_sector: u16, // Words per sector (e.g., 321)
    reserved: [u8; 10],    // Padding to 16 bytes
}
```

**Header fields:**
- **magic (8 bytes):** ASCII string "I1130DSK" for format identification
- **cylinders (2 bytes):** Number of logical cylinders (200 for 2315)
- **heads (1 byte):** Number of read/write heads (2 for 2315)
- **sectors_per_track (1 byte):** Sectors per track (4 for 2315)
- **words_per_sector (2 bytes):** Words per sector including address word (321)
- **reserved (32 bytes):** Reserved for future extensions (bad block tables, etc.)

### Sector Data Layout

Following the 56-byte header, sector data is stored sequentially:

```
For each cylinder (0..cylinders-1):
    For each head (0..heads-1):
        For each sector (0..sectors_per_track-1):
            Store 321 words (642 bytes) as little-endian u16
```

**Total data size calculation:**
```
data_size = cylinders * heads * sectors_per_track * words_per_sector * 2
```

**For IBM 2315:**
```
data_size = 200 * 2 * 4 * 321 * 2 = 1,027,200 bytes (~1 MB)
Total file size = 56 + 1,027,200 = 1,027,256 bytes
```

### Word Format

Each word is stored as 16-bit unsigned integer, little-endian:

```
Word value: 0x1234
Byte 0: 0x34 (low byte)
Byte 1: 0x12 (high byte)
```

### Sector Word 0 (Address Word)

First word of each sector typically contains sector address:

```
Bits 15-8: Cylinder number
Bits 7-4:  Head number
Bits 3-0:  Sector number
```

Remaining 320 words are data payload.

### Example: Reading .dsk File

```rust
use std::fs::File;
use std::io::Read;

fn load_disk(path: &str) -> Result<DiskImage> {
    let mut file = File::open(path)?;
    let mut header = [0u8; 56];
    file.read_exact(&mut header)?;

    // Verify magic
    if &header[0..8] != b"I1130DSK" {
        return Err("Invalid magic");
    }

    // Parse geometry
    let cylinders = u16::from_le_bytes([header[8], header[9]]);
    let heads = header[10];
    let sectors = header[11];
    let words = u16::from_le_bytes([header[12], header[13]]);

    // Read sector data
    let data_size = cylinders as usize * heads as usize
                  * sectors as usize * words as usize * 2;
    let mut data = vec![0u8; data_size];
    file.read_exact(&mut data)?;

    // Convert to u16 words
    let words: Vec<u16> = data.chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();

    Ok(DiskImage { cylinders, heads, sectors, words, data: words })
}
```

### Example: Creating .dsk File

```rust
use std::fs::File;
use std::io::Write;

fn save_disk(disk: &DiskImage, path: &str) -> Result<()> {
    let mut file = File::create(path)?;

    // Write header
    file.write_all(b"I1130DSK")?;
    file.write_all(&disk.cylinders.to_le_bytes())?;
    file.write_all(&[disk.heads])?;
    file.write_all(&[disk.sectors_per_track])?;
    file.write_all(&disk.words_per_sector.to_le_bytes())?;
    file.write_all(&[0u8; 10])?;  // Geometry padding
    file.write_all(&[0u8; 32])?;  // Reserved area

    // Write sector data
    for word in &disk.data {
        file.write_all(&word.to_le_bytes())?;
    }

    Ok(())
}
```

### Reserved Area Usage (Future)

The 32-byte reserved area may be used for:
- **Bad block tables:** List of defective cylinders/sectors
- **Cartridge ID:** Serial number or label
- **Bootstrap sectors:** Shadow of cylinder 0 for emergency boot
- **Metadata:** Creation date, simulator version, etc.

## .deck File Format (Card Deck)

### Purpose

Store sequences of 80-column punched cards in text or binary format.

### File Structure

```
+------------------+
| Header (JSON)    |
+------------------+
| Card 1 (80 bytes)|
+------------------+
| Card 2 (80 bytes)|
+------------------+
| ...              |
+------------------+
| Card N (80 bytes)|
+------------------+
```

### Header Format

JSON header on first line:

```json
{"encoding": "ascii", "binary": false, "cards": 42}
```

**Fields:**
- **encoding:** "ascii" or "ebcdic"
- **binary:** true (12-bit binary) or false (text)
- **cards:** Number of cards in deck

### Text Mode Cards

Each card is 80 bytes representing columns 1-80:

```
Column:  1         10        20        30        40        50        60        70        80
Data:    // JOB HELLO
```

**Encoding:**
- **ASCII:** Standard ASCII characters (0x20-0x7E)
- **EBCDIC:** IBM EBCDIC character set

**Padding:** Unused columns filled with spaces (0x20 in ASCII)

### Binary Mode Cards

Each card is 80 bytes, but interpreted as 12-bit values per column:

```
Bit layout per column (Hollerith 12-row encoding):
Bit 11: 12-punch (top)
Bit 10: 11-punch
Bit 9:  0-punch
Bits 8-0: Rows 1-9
```

**Use cases:**
- Object code (assembled programs)
- Binary data files
- Compressed data

### Example: Reading .deck File

```rust
use std::fs::File;
use std::io::{BufReader, BufRead};

fn load_deck(path: &str) -> Result<CardDeck> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Read header
    let mut header_line = String::new();
    reader.read_line(&mut header_line)?;
    let header: DeckHeader = serde_json::from_str(&header_line)?;

    // Read cards
    let mut cards = Vec::new();
    for _ in 0..header.cards {
        let mut card = [0u8; 80];
        reader.read_exact(&mut card)?;
        cards.push(card);
    }

    Ok(CardDeck { encoding: header.encoding, binary: header.binary, cards })
}
```

### Example: Creating .deck File

```rust
use std::fs::File;
use std::io::Write;

fn save_deck(deck: &CardDeck, path: &str) -> Result<()> {
    let mut file = File::create(path)?;

    // Write header
    let header = format!(
        "{{\"encoding\": \"{}\", \"binary\": {}, \"cards\": {}}}\n",
        deck.encoding, deck.binary, deck.cards.len()
    );
    file.write_all(header.as_bytes())?;

    // Write cards
    for card in &deck.cards {
        file.write_all(card)?;
    }

    Ok(())
}
```

### IBM Job Control Card Format

Card decks often contain job control cards for the Disk Monitor System:

```
// JOB jobname
// ASM
   <assembler source cards>
// DUP
*STORE programname
// XEQ programname
```

**Control cards:**
- `// JOB name` - Start job
- `// ASM` - Assemble source
- `// FOR` - Compile Fortran
- `// DUP` - Disk utility
- `// XEQ name` - Execute program

## Catalog Format (metadata/catalog.json)

### Purpose

Enumerate available disk images and card decks for UI display.

### Structure

```json
{
  "disks": [
    {
      "id": "demo2315",
      "name": "Demo 2315 Cartridge",
      "path": "disks/demo2315.dsk",
      "type": "2315",
      "description": "Sample disk with system tracks",
      "size_kb": 1024
    }
  ],
  "decks": [
    {
      "id": "hello",
      "name": "Hello World",
      "path": "cards/HELLO.deck",
      "description": "Simple hello world program",
      "cards": 12
    }
  ]
}
```

**Disk entry fields:**
- **id:** Unique identifier
- **name:** Display name
- **path:** Relative path to .dsk file
- **type:** "2315" or "2311"
- **description:** Human-readable description
- **size_kb:** Size in kilobytes

**Deck entry fields:**
- **id:** Unique identifier
- **name:** Display name
- **path:** Relative path to .deck file
- **description:** Human-readable description
- **cards:** Number of cards in deck

## File Format Validation

### Disk Image Validation

```rust
fn validate_disk(data: &[u8]) -> Result<()> {
    // Check minimum size (header)
    if data.len() < 56 {
        return Err("File too small");
    }

    // Check magic
    if &data[0..8] != b"I1130DSK" {
        return Err("Invalid magic");
    }

    // Parse geometry
    let cylinders = u16::from_le_bytes([data[8], data[9]]);
    let heads = data[10];
    let sectors = data[11];
    let words = u16::from_le_bytes([data[12], data[13]]);

    // Validate ranges
    if cylinders == 0 || cylinders > 300 {
        return Err("Invalid cylinder count");
    }
    if heads == 0 || heads > 10 {
        return Err("Invalid head count");
    }
    if sectors == 0 || sectors > 32 {
        return Err("Invalid sector count");
    }
    if words < 100 || words > 500 {
        return Err("Invalid words per sector");
    }

    // Check data size
    let expected = 56 + cylinders as usize * heads as usize
                      * sectors as usize * words as usize * 2;
    if data.len() != expected {
        return Err("Data size mismatch");
    }

    Ok(())
}
```

### Card Deck Validation

```rust
fn validate_deck(data: &[u8]) -> Result<()> {
    // Find header line
    let header_end = data.iter().position(|&b| b == b'\n')
        .ok_or("No header found")?;

    // Parse header
    let header_str = std::str::from_utf8(&data[0..header_end])?;
    let header: DeckHeader = serde_json::from_str(header_str)?;

    // Validate encoding
    if header.encoding != "ascii" && header.encoding != "ebcdic" {
        return Err("Invalid encoding");
    }

    // Check data size
    let expected = header_end + 1 + header.cards * 80;
    if data.len() != expected {
        return Err("Data size mismatch");
    }

    Ok(())
}
```

## Compatibility

### SIMH IBM 1130 Disk Format

The simulator's .dsk format is **similar but not identical** to SIMH format:

**Differences:**
- SIMH uses different magic string
- SIMH may use different byte ordering
- SIMH includes additional metadata

**Conversion:** Import/export adapter planned for future (see documentation/status.md)

### Real IBM 1130 Disk Images

Historical disk images from real systems may be imported with conversion:

- Parse original format (varies by preservation method)
- Convert to .dsk format
- Validate geometry and contents
- Preserve sector addresses

## File Format Extensions

### Proposed Extensions

**Compressed .dsk.gz:**
- gzip-compressed disk images
- Reduces file size (~50% typical)
- Transparent decompression on load

**Incremental .dsk.diff:**
- Store only changed sectors
- Base image + delta
- For version control and backups

**Encrypted .dsk.enc:**
- Encrypted disk images
- Password-protected
- For sensitive educational content

## Related Pages

- [[Core-Simulation]] - File I/O implementation
- [[Devices]] - Device specifications
- [[Development]] - Testing file format code

## Related Documentation

- [Complete Specifications](../documentation/ibm_1130_disk_i_o_simulator_starter_docs.md)
- [Design Decisions](../documentation/design.md)
