# IBM 1130 Disk and I/O Simulator

Welcome to the IBM 1130 System Simulator wiki! This educational simulator recreates vintage IBM 1130 peripheral hardware with historically accurate timing, geometry, and behavior in a modern browser-based interface.

## Quick Links

- [[Architecture]] - System architecture and design overview
- [[Core-Simulation]] - Core simulation layer (crates/core-sim)
- [[Web-UI]] - Browser UI layer (crates/yew-ui)
- [[Devices]] - Device specifications and operations
- [[File-Formats]] - .dsk and .deck file format details
- [[Development]] - Development guide and workflow

## What is the IBM 1130?

The IBM 1130 was a 16-bit computing system introduced in 1965 for scientific and educational purposes. This simulator focuses on the peripheral I/O subsystem including:

- **IBM 2310/2315** - Disk drive with removable cartridge (512K words)
- **IBM 2311** - Multi-platter disk pack drive (1.5-2.5M words)
- **IBM 1442** - Card reader/punch (80-column cards)
- **IBM 1403** - Chain/train line printer (340-600 lpm)
- **IBM 1133** - Multiplexor for device attachment

## Project Goals

1. **Educational Accuracy** - Model geometry, timing, and operations faithfully
2. **Modern Visualization** - Browser-based UI with disk maps, animations, and audio feedback
3. **Test-Driven** - Comprehensive test coverage with deterministic timing modes
4. **Extensible** - Trait-based architecture for future CPU integration

## Key Features

- **Realistic Timing Models** - 1x historical speed or fast-forward modes
- **Visual Disk Maps** - See cylinder/head/sector allocation and activity
- **Audio Feedback** - WebAudio seek sounds based on actual mechanics
- **File Format Support** - Load/save .dsk disk images and .deck card decks
- **Pure Rust** - Core simulation in platform-agnostic Rust, UI compiled to WASM

## Architecture Overview

The system is organized in three crates:

```
+----------------------------------------------------------+
|  yew-ui (WASM/Yew)                                       |
|  - Views: DiskMap, CardReader, Console, StatusBar        |
|  - Services: Audio, Storage (IndexedDB), Bridge          |
+----------------------------------------------------------+
                        |
                   WASM Bindings
                        |
+----------------------------------------------------------+
|  core-sim (no_std Rust)                                  |
|  - Device traits: DiskDevice, CardDevice, etc.           |
|  - Implementations: Ibm2310, Ibm2311, Ibm1442, etc.      |
|  - TimingModel, Geometry, File I/O                       |
+----------------------------------------------------------+
                        |
                    File I/O
                        |
+----------------------------------------------------------+
|  fixtures                                                 |
|  - Sample .dsk disk images                                |
|  - Sample .deck card decks                                |
|  - Test infrastructure                                    |
+----------------------------------------------------------+
```

See [[Architecture]] for detailed diagrams and component descriptions.

## Getting Started

### Prerequisites

```bash
# Install Rust toolchain
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM bundler)
cargo install trunk
```

### Build and Run

```bash
# Build entire workspace
cargo build

# Run development server with hot reload
cd crates/yew-ui
trunk serve --open
```

### Run Tests

```bash
# All tests
cargo test --all

# Specific crate
cargo test -p core-sim
```

See [[Development]] for complete development workflow and quality standards.

## Documentation

### In This Wiki

- **[[Architecture]]** - System design and layers
- **[[Core-Simulation]]** - Device simulation internals
- **[[Web-UI]]** - Browser UI components
- **[[Devices]]** - Device specifications and timing
- **[[File-Formats]]** - Binary file format details
- **[[Development]]** - Coding standards and TDD workflow

### In Repository

- [Process Guide](../documentation/process.md) - Development methodology (TDD, quality checks)
- [Complete Specifications](../documentation/ibm_1130_disk_i_o_simulator_starter_docs.md) - Detailed device specs
- [Research Notes](../documentation/research.md) - Historical IBM 1130 facts
- [Project Status](../documentation/status.md) - Current implementation status

## Quick Reference

### IBM 2315 Disk Geometry

- **Cylinders:** 200 logical (203 physical with 3 alternates)
- **Heads:** 2 (top/bottom)
- **Sectors per track:** 4
- **Words per sector:** 321 (word 0 = address, 320 payload)
- **Capacity:** 512,000 words (~1 MB)

### Timing Characteristics

- **RPM:** 1500 (40ms/revolution)
- **Avg rotational latency:** ~20ms
- **Word transfer rate:** 27.8us/word
- **Seek time:** 7.5ms x N_even + 22.5ms settle

See [[Devices]] for complete specifications.

## Contributing

This project follows strict Test-Driven Development (TDD):

1. **Red** - Write failing test first
2. **Green** - Implement minimum code to pass
3. **Refactor** - Clean up while tests stay green

Before every commit:

```bash
cargo fmt --all                                    # Format
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo test --all                                   # Test
cargo build --all                                  # Build
```

See [[Development]] for complete guidelines.

## License

This is an educational project. See repository LICENSE file for details.

## Resources

- [IBM 1130 Wikipedia](https://en.wikipedia.org/wiki/IBM_1130)
- [Bitsavers IBM 1130 Documentation](http://bitsavers.org/pdf/ibm/1130/)
- [Repository Documentation](../documentation/)
