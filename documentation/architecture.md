# System Architecture

## Overview

The IBM 1130 System Simulator is built as a multi-layered architecture separating device simulation logic from visualization and user interaction. The design prioritizes historical accuracy, test-driven development, and modern web delivery.

## Architecture Principles

1. **Separation of Concerns**: Core simulation logic is independent of UI rendering
2. **Platform Agnostic Core**: Device simulation runs on native Rust or WASM
3. **Trait-Based Design**: All devices implement common traits for uniform interaction
4. **Parameterized Timing**: Realistic/fast/none timing modes for different use cases
5. **Test-First Development**: All features driven by tests before implementation

## System Layers

```
+------------------------------------------------------------------+
|                         Web Browser                               |
|  +------------------------------------------------------------+  |
|  |                      Yew UI (WASM)                         |  |
|  |  +------------------------------------------------------+  |  |
|  |  | Views: DiskMap | CardReader | Printer | Console     |  |  |
|  |  +------------------------------------------------------+  |  |
|  |  | Services: Audio | Storage (IndexedDB) | Bridge      |  |  |
|  |  +------------------------------------------------------+  |  |
|  +------------------------------------------------------------+  |
+------------------------------------------------------------------+
                              |
                              | WASM Bindings
                              v
+------------------------------------------------------------------+
|                    Core Simulation (no_std)                       |
|  +------------------------------------------------------------+  |
|  | Device Traits: Device | DiskDevice | CardDevice |         |  |
|  |                LinePrinter | Multiplexor                   |  |
|  +------------------------------------------------------------+  |
|  | Implementations: Ibm2310 | Ibm2311 | Ibm1442 |             |  |
|  |                  Ibm1403 | Ibm1133                         |  |
|  +------------------------------------------------------------+  |
|  | Support: TimingModel | AudioModel | Geometry | Addressing  |  |
|  +------------------------------------------------------------+  |
|  | File Formats: .dsk | .deck | Validation                    |  |
|  +------------------------------------------------------------+  |
+------------------------------------------------------------------+
                              |
                              | File I/O
                              v
+------------------------------------------------------------------+
|              Fixtures & Test Data                                 |
|  Disk Images (.dsk) | Card Decks (.deck) | Metadata              |
+------------------------------------------------------------------+
```

## Core Simulation Layer (crates/core-sim)

### Purpose
Provides accurate simulation of IBM 1130 peripheral devices with historically correct timing, geometry, and behavior. No dependencies on UI or web APIs.

### Key Components

#### Device Traits
```rust
pub trait Device {
    fn reset(&mut self);
    fn poll(&mut self, now_us: u64);
    fn dsw(&self) -> DeviceStatusWord;
}

pub trait DiskDevice: Device {
    fn geometry(&self) -> &DiskGeometry;
    fn seek(&mut self, cyl: u16) -> SeekOutcome;
    fn read_sector(&mut self, addr: SectorAddr) -> Result<Vec<u16>>;
    fn write_sector(&mut self, addr: SectorAddr, data: &[u16]) -> Result<()>;
    fn read_block(&mut self, addr: BlockAddr) -> Result<Vec<u16>>;
    fn write_block(&mut self, addr: BlockAddr, data: &[u16]) -> Result<()>;
}

pub trait CardDevice: Device {
    fn load_deck(&mut self, deck: CardDeck);
    fn read_card(&mut self) -> Result<Card>;
    fn punch_card(&mut self, card: &Card) -> Result<()>;
}

pub trait LinePrinter: Device {
    fn print_line(&mut self, line: &str) -> Result<()>;
    fn advance_forms(&mut self, lines: u8) -> Result<()>;
}

pub trait Multiplexor: Device {
    fn attach_device(&mut self, addr: u8, device: Box<dyn Device>);
    fn route_command(&mut self, addr: u8, cmd: IoCommand) -> Result<()>;
}
```

#### Timing Model
Provides three modes for different use cases:
- **None**: Zero delays for deterministic testing
- **Realistic**: 1x historical timing (1500 RPM, 27.8us/word, etc.)
- **Fast(n)**: n-times faster than realistic for demos

```rust
pub enum TimingModel {
    None,
    Realistic,
    Fast(f64),
}

impl TimingModel {
    pub fn delay_us(&self, us: u64) -> u64 {
        match self {
            TimingModel::None => 0,
            TimingModel::Realistic => us,
            TimingModel::Fast(multiplier) => (us as f64 / multiplier) as u64,
        }
    }
}
```

#### IBM 2310/2315 Disk (Removable Cartridge)
- **Geometry**: 200 logical cylinders, 2 heads, 4 sectors/track, 321 words/sector
- **Capacity**: 512,000 16-bit words (~1 MB)
- **Timing**: 1500 RPM (40ms/revolution), 27.8us/word transfer rate
- **Seek**: 2-cylinder increments, 7.5ms per increment + 22.5ms settle
- **Addressing**: Cylinder/Head/Sector (CHS) and logical block addressing

#### IBM 2311 Disk (Fixed Multi-Platter)
- **Models**: Model 11 (2.56M words) or Model 12 (1.536M words)
- **Geometry**: Multiple platters, shared actuator
- **Compatibility**: Cannot coexist with 2310 on same system

#### IBM 1442 Card Reader/Punch
- **Read Speed**: Up to 400 cards per minute
- **Punch Speed**: Up to 360 cards per minute (model dependent)
- **Format**: 80-column cards, EBCDIC or ASCII encoding, binary mode supported
- **Stackers**: Two output stackers for punch

#### IBM 1403 Line Printer
- **Models**: Model 6 (340 lpm) or Model 7 (600 lpm)
- **Technology**: Chain/train printing
- **Attachment**: Via IBM 1133 multiplexor

#### IBM 1133 Multiplexor
- **Purpose**: Device attachment and I/O command routing
- **Function**: Manages communication between CPU and peripherals

### File Formats

#### .dsk (Disk Image)
```
+------------------+
| Magic: "I1130DSK" | 8 bytes
+------------------+
| Geometry         | 16 bytes (cylinders, heads, sectors, words)
+------------------+
| Reserved         | 32 bytes
+------------------+
| Sector Data      | Raw 16-bit words (little-endian)
| (all C/H/S)      |
+------------------+
```

#### .deck (Card Deck)
```
+------------------+
| Header           | Encoding (EBCDIC/ASCII), binary mode flag
+------------------+
| Card 1 (80 bytes) |
+------------------+
| Card 2 (80 bytes) |
+------------------+
| ...              |
+------------------+
```

## Web UI Layer (crates/yew-ui)

### Purpose
Browser-based visualization and interaction with the simulated devices. Built with Yew framework targeting WASM.

### Key Components

#### Views
- **DiskMap**: Visual representation of disk cylinders/heads/sectors
  - Color-coded by allocation status (free/used/system)
  - Hover shows C/H/S coordinates, block number, file owner
  - Click to inspect sector/block contents

- **CardReader**: Animated hopper, transport path, and stackers
  - Shows card movement and current operation
  - Displays card content (text or binary)

- **Printer**: Page buffer and output display
  - Shows current print position
  - Form feed and line advance visualization

- **Console**: Device status word (DSW) display
  - Busy, error, and attention flags
  - Current operation and progress

- **StatusBar**: System-wide status
  - Active devices
  - Operation timeline

#### Services

- **Audio Service**: WebAudio integration for seek sounds
  - Pitch proportional to seek distance
  - Duration based on seek time
  - Acceleration/deceleration profile

- **Storage Service**: IndexedDB for persistent disk images
  - Save/load disk images
  - Multiple disk management

- **Bridge Service**: WASM bindings to core-sim
  - Command dispatch to devices
  - Event streaming from simulation to UI

## Fixtures Layer (crates/fixtures)

### Purpose
Test data and sample files for development and testing.

### Contents
- **Disk Images**: Pre-created .dsk files with various content
- **Card Decks**: Sample programs and data in .deck format
- **Metadata**: catalog.json describing available fixtures
- **Test Infrastructure**: Automated tests for file formats and project standards

## Data Flow

### Device Operation Flow
```
1. User Action (UI)
   |
   v
2. Command Dispatch (Bridge Service)
   |
   v
3. Device Method Call (Core Sim)
   |
   v
4. Operation Execution with Timing
   |
   v
5. Status Update (DSW)
   |
   v
6. Event Notification (Bridge)
   |
   v
7. UI Update (View)
```

### File I/O Flow
```
1. User Loads .dsk File (UI)
   |
   v
2. File Read (IndexedDB or FileAPI)
   |
   v
3. Parse & Validate (Core Sim)
   |
   v
4. Mount to Device (Ibm2310/2311)
   |
   v
5. Ready for Operations
```

## Technology Stack

- **Language**: Rust (edition 2024)
- **UI Framework**: Yew (WASM-based React-like framework)
- **Build Tool**: Trunk (WASM bundler and dev server)
- **Testing**: cargo test, wasm-bindgen-test
- **Storage**: IndexedDB via gloo
- **Audio**: WebAudio API

## Design Patterns

### Trait-Based Polymorphism
All devices implement common traits, allowing uniform handling and testing.

### State Machine Pattern
Devices maintain internal state machines for operations (idle -> seeking -> reading -> idle).

### Observer Pattern
UI components subscribe to device events for real-time updates.

### Strategy Pattern
TimingModel allows swapping timing strategies without changing device logic.

## Performance Considerations

- **WASM Optimization**: Core simulation compiled with optimizations for WASM target
- **Incremental Rendering**: UI updates only changed components
- **Web Workers**: Heavy simulation work can move to workers (future)
- **Lazy Loading**: Disk images loaded on-demand

## Security Considerations

- **Sandboxed Execution**: WASM provides memory safety and sandboxing
- **Input Validation**: All file formats validated before processing
- **No Network Access**: Simulator operates entirely locally in browser
- **Safe Rust**: No unsafe code in public API surface

## Testing Strategy

- **Unit Tests**: Each module and function tested in isolation
- **Integration Tests**: Device interactions and data flow
- **Property Tests**: File format round-trip validation
- **UI Tests**: WASM-bindgen-test for browser components
- **Performance Tests**: Timing accuracy verification

## Future Architecture Considerations

### CPU Integration (Phase 2)
When CPU simulation is added:
- Extend ChannelBus trait for full I/O channel protocol
- Implement interrupt handling
- Add DMA simulation
- Memory-mapped I/O

### Multi-User Support (Future)
- WebSocket server for shared simulation state
- Collaborative debugging and education

### Historical Software (Phase 3)
- Load and run actual DMS disk images
- Execute IBM 1130 programs
- Emulate full system including CPU

## Related Documents

- [design.md](design.md) - Detailed technical design decisions
- [PRD.md](PRD.md) - Product requirements and use cases
- [research.md](research.md) - Historical IBM 1130 facts
- [ibm_1130_disk_i_o_simulator_starter_docs.md](ibm_1130_disk_i_o_simulator_starter_docs.md) - Complete specifications
