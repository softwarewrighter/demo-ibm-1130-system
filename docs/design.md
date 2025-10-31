# Technical Design

This document describes key technical design decisions and their rationale.

## Disk Addressing

### Sector Numbering Convention

Sectors on the IBM 2315 are numbered 0-7 across both heads:
- Sectors 0-3: Head 0 (top surface)
- Sectors 4-7: Head 1 (bottom surface)

**Rationale**: Matches IBM 1130 hardware convention and simplifies linear sector indexing.

### Linear Sector Index Formula

```rust
fn sector_to_index(cyl: u16, head: u8, sector: u8) -> usize {
    (cyl as usize) * 8 + (head as usize) * 4 + (sector as usize)
}
```

**Example**: Cylinder 50, Head 1, Sector 2 -> Index 403
- 50 * 8 = 400
- 1 * 4 = 4
- sector 2 = 2
- Total: 406 (sectors 4-7 map to head 1, so sector 2 on head 1 is actually physical sector 6)

### Block Addressing

Each sector contains 16 logical blocks of 20 words each:

```rust
pub struct BlockAddr {
    pub cyl: u16,      // 0..199
    pub head: u8,      // 0..1
    pub sector: u8,    // 0..7 (0-3 head 0, 4-7 head 1)
    pub block: u8,     // 0..15
}

fn block_word_offset(block: u8) -> usize {
    1 + (block as usize * 20)  // Skip sector address word at offset 0
}
```

**Rationale**: Disk Monitor System (DMS) uses block addressing for file allocation.

## Seek Timing

### Two-Cylinder Quantization

The IBM 2315 actuator moves in increments of 2 cylinders:

```rust
fn quantize_cylinder(target: u16) -> u16 {
    (target / 2) * 2  // Round down to nearest even cylinder
}
```

### Seek Time Formula

```rust
fn calculate_seek_time_us(from: u16, to: u16) -> u64 {
    let quantized_from = quantize_cylinder(from);
    let quantized_to = quantize_cylinder(to);
    let delta_even = ((quantized_to as i32 - quantized_from as i32).abs() / 2) as f64;
    let seek_ms = delta_even * 7.5 + 22.5;  // Settle time always added
    (seek_ms * 1000.0) as u64
}
```

**Examples**:
- Cyl 0 -> Cyl 0: 22.5ms (settle only)
- Cyl 0 -> Cyl 2: 30.0ms (7.5ms + 22.5ms)
- Cyl 0 -> Cyl 100: 397.5ms (50 * 7.5ms + 22.5ms)

**Rationale**: Matches IBM 1130 Reference Manual specifications.

## Rotational Latency

### Timing Constants

```rust
const RPM: f64 = 1500.0;
const REVOLUTION_TIME_MS: f64 = 40.0;  // 60000ms / 1500 RPM
const AVG_ROTATIONAL_LATENCY_MS: f64 = 20.0;  // Half revolution
```

### Sector Access Time

```rust
const WORDS_PER_SECTOR: usize = 321;
const WORD_TRANSFER_TIME_US: f64 = 27.8;
const SECTOR_TRANSFER_TIME_MS: f64 = 8.9238;  // 321 * 27.8us
```

**Rationale**: 1500 RPM with 8 sectors per track yields these timing values.

## File Format Design

### .dsk Format

```
Offset  Size   Description
------  -----  -----------
0       8      Magic: "I1130DSK" (ASCII)
8       2      Version (u16 little-endian)
10      2      Cylinders (u16)
12      2      Heads (u8) + Sectors per track (u8)
14      2      Words per sector (u16)
16      32     Reserved (zeros)
48      2      First sector address word
50      640    First sector data (320 words * 2 bytes)
...     ...    Remaining sectors
```

**Design Decisions**:
- **Magic Number**: Identifies file type and prevents accidental loading of non-disk files
- **Little-Endian**: Matches x86/WASM target architecture
- **Reserved Space**: Allows future extensions without breaking format
- **Sector Address Word**: Preserved for historical accuracy

### .deck Format

```
Offset  Size   Description
------  -----  -----------
0       4      Magic: "DECK" (ASCII)
4       1      Version
5       1      Encoding: 0=EBCDIC, 1=ASCII
6       1      Binary mode flag
7       1      Reserved
8       80     Card 1 (80 columns)
88      80     Card 2
...     ...    Remaining cards
```

**Design Decisions**:
- **80-Column Format**: Standard IBM card size
- **Encoding Flag**: Support both EBCDIC (historical) and ASCII (modern)
- **Binary Mode**: Allow arbitrary byte patterns, not just printable characters

## Timing Mode Implementation

### Enum Design

```rust
pub enum TimingModel {
    None,              // Zero delays, deterministic
    Realistic,         // 1x historical timing
    Fast(f64),         // Multiplier (2.0 = 2x faster)
}

impl TimingModel {
    pub fn delay_us(&self, nominal_us: u64) -> u64 {
        match self {
            TimingModel::None => 0,
            TimingModel::Realistic => nominal_us,
            TimingModel::Fast(mult) => ((nominal_us as f64) / mult) as u64,
        }
    }

    pub fn none() -> Self { TimingModel::None }
    pub fn realistic() -> Self { TimingModel::Realistic }
    pub fn fast(multiplier: f64) -> Self { TimingModel::Fast(multiplier) }
}
```

**Rationale**:
- **None Mode**: Enables fast, deterministic testing without time-dependent behavior
- **Realistic Mode**: Educational value, shows actual hardware performance
- **Fast Mode**: Practical for demos and development (10x-50x speedup)

## Device Status Word (DSW)

### Bit Layout

```rust
pub struct DeviceStatusWord {
    pub busy: bool,       // Device performing operation
    pub error: bool,      // Error occurred
    pub attention: bool,  // Device needs service
    pub ready: bool,      // Device ready for command
}

impl DeviceStatusWord {
    pub fn to_u16(&self) -> u16 {
        let mut word = 0u16;
        if self.busy      { word |= 0x8000; }  // Bit 15
        if self.error     { word |= 0x4000; }  // Bit 14
        if self.attention { word |= 0x2000; }  // Bit 13
        if self.ready     { word |= 0x1000; }  // Bit 12
        word
    }
}
```

**Rationale**: Matches IBM 1130 I/O channel status conventions.

## Error Handling Strategy

### Result Types

```rust
pub type Result<T> = std::result::Result<T, SimError>;

pub enum SimError {
    InvalidAddress { cyl: u16, head: u8, sector: u8 },
    DeviceBusy,
    DeviceNotReady,
    ParityError { sector_index: usize },
    FileFormatError { reason: String },
    IoError(std::io::Error),
}
```

**Design Decisions**:
- **Typed Errors**: Each error carries relevant context
- **No Panics**: All errors are recoverable Result types
- **Display Impl**: User-friendly error messages for UI

## Audio Synthesis

### Seek Sound Model

```rust
pub struct SeekProfile {
    pub distance_cyl: u16,
    pub duration_ms: f64,
}

impl SeekProfile {
    pub fn to_audio_params(&self) -> AudioParams {
        // Pitch proportional to seek distance
        let base_freq = 220.0;  // A3
        let freq = base_freq * (1.0 + (self.distance_cyl as f64 / 200.0));

        AudioParams {
            frequency: freq,
            duration_ms: self.duration_ms,
            envelope: Envelope::ADSR {
                attack_ms: self.duration_ms * 0.1,
                decay_ms: self.duration_ms * 0.2,
                sustain_level: 0.6,
                release_ms: self.duration_ms * 0.3,
            },
        }
    }
}
```

**Rationale**: Provides audible feedback that correlates with operation characteristics.

## Testing Strategy

### Test Organization

```
tests/
  unit/           # Test individual functions
    addressing_test.rs
    timing_test.rs
  integration/    # Test device interactions
    disk_ops_test.rs
  property/       # Test invariants
    file_format_roundtrip_test.rs
```

### Test Timing Models

All device tests use `TimingModel::None` by default for deterministic, fast execution:

```rust
#[test]
fn test_seek_addressing() {
    let mut disk = Ibm2310::new(TimingModel::none());
    // Test completes instantly, no waiting for seek
}
```

Timing accuracy tests explicitly use `TimingModel::Realistic` and verify delays.

## WASM Considerations

### no_std Compatibility

Core simulation crate uses `#![no_std]` with:
```rust
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
```

**Rationale**: Smaller WASM binary size, explicit about allocations.

### Memory Management

- No global allocator required for core types
- UI layer handles all dynamic allocation
- Disk images use fixed-size buffers when possible

## Related Documents

- [architecture.md](architecture.md) - Overall system architecture
- [PRD.md](PRD.md) - Product requirements
- [research.md](research.md) - Historical IBM 1130 facts
- [ibm_1130_disk_i_o_simulator_starter_docs.md](ibm_1130_disk_i_o_simulator_starter_docs.md) - Detailed specifications
