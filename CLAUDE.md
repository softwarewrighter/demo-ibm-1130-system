# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an IBM 1130 disk and I/O simulator written in Rust. The project simulates vintage IBM hardware including:
- **Disk drives**: IBM 2310 (2315 cartridge) and IBM 2311 (1316 disk pack)
- **Card reader/punch**: IBM 1442
- **Line printer**: IBM 1403
- **Multiplexor**: IBM 1133

The simulator is designed for educational purposes, providing accurate timing models, geometry, and visual feedback through a browser-based WASM/Yew UI.

## Repository Structure

The project uses a Cargo workspace with three crates:

- **`crates/core-sim/`**: Pure Rust simulation core (no_std on WASM)
  - Device implementations: `disk/`, `card/`, `printer/`, `mux/`
  - Core modules: `timing.rs`, `audio.rs`, `cpu_bus.rs`, `util.rs`
  - Defines device traits: `DiskDevice`, `CardDevice`, `LinePrinter`, `Multiplexor`

- **`crates/yew-ui/`**: WASM/Yew-based web UI
  - Main entry: `src/main.rs`, `src/app.rs`
  - Views: `disk_map.rs`, `card_reader.rs`, `console.rs`, `status_bar.rs`
  - Services: `audio.rs` (WebAudio), `storage.rs` (IndexedDB), `bridge.rs` (WASM bindings)

- **`crates/fixtures/`**: Sample data and test fixtures
  - `data/disks/`: Disk image files (`.dsk` format)
  - `data/cards/`: Card deck files (`.deck` format)
  - `data/metadata/catalog.json`: Catalog of available fixtures

## Build and Development Commands

### Prerequisites
```bash
# Install Rust toolchain
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM bundler for Yew UI)
cargo install trunk
```

### Build Commands
```bash
# Build the entire workspace
cargo build

# Build release version
cargo build --release

# Build WASM UI only
cd crates/yew-ui
trunk build --release
```

### Development Commands
```bash
# Run development server with hot reload (from yew-ui directory)
cd crates/yew-ui
trunk serve --open

# Format code
cargo fmt --all

# Run clippy linter
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all

# Run tests for specific crate
cargo test -p core-sim
```

## Key Architecture Details

### Device Trait System
All devices implement the base `Device` trait with common lifecycle methods:
```rust
pub trait Device {
    fn reset(&mut self);
    fn poll(&mut self, now_us: u64);  // Advance timers, complete operations
    fn dsw(&self) -> DeviceStatusWord; // Get status: busy, error, attention
}
```

Specialized traits extend `Device`:
- `DiskDevice`: Disk operations with geometry, seek, read/write sectors and blocks
- `CardDevice`: Card reading/punching operations
- `LinePrinter`: Line printing operations
- `Multiplexor`: Device attachment and I/O command routing

### IBM 2315 Disk Geometry
The simulator models the physical characteristics of IBM 1130 disks:
- **Cylinders**: 200 logical (203 physical with 3 alternates)
- **Heads**: 2 (top/bottom surfaces)
- **Sectors per track**: 4
- **Words per sector**: 321 (word 0 = sector address, 320 words payload)
- **Logical blocks**: 16 blocks x 20 words per sector

### Timing Model
The `TimingModel` struct enables realistic hardware timing simulation:
- `TimingModel::none()`: No delays (for tests)
- `TimingModel::realistic()`: 1x  historical timing
- `TimingModel::fast(multiplier)`: Accelerated timing

**2315 Timing characteristics**:
- RPM: 1500 -> 40ms/revolution
- Avg rotational latency: ~20ms
- Word rate: 27.8us/word
- Seek time: `7.5ms x N_even + 22.5ms settle` (seeks in 2-cylinder increments)

### Block Addressing
`BlockAddr` represents Disk Monitor System logical blocks:
```rust
pub struct BlockAddr {
    pub cyl: u16,    // Cylinder number
    pub head: u8,    // 0 or 1 for 2315
    pub sector: u8,  // 0..=7 (0..=3 top, 4..=7 bottom)
    pub block: u8,   // 0..=15 within sector (20 words each)
}
```

Conversion to linear sector index:
```
idx = cyl * 8 + (head * 4 + sector)
```

### File Formats

**`.dsk` (Disk Image)**:
- Header: 8-byte magic "I1130DSK", geometry struct, 32-byte reserved area
- Data: Raw u16 words (little-endian) for all cylinders/heads/sectors

**`.deck` (Card Deck)**:
- Header with encoding (EBCDIC/ASCII) and binary mode flag
- 80-byte card frames

## Important Implementation Notes

1. **Seek quantization**: The 2315 seeks in increments of 2 cylinders (see `ibm2310.rs:quantize_cylinder`)

2. **Sector numbering**: Sectors 0-3 are on head 0, sectors 4-7 are on head 1

3. **Word offsets**: When accessing block data, skip the sector address word (offset starts at 1)

4. **Timing delays**: All timing delays can be disabled via `TimingModel::none()` for deterministic testing

5. **Workspace edition**: Uses Rust edition 2024 (see root `Cargo.toml`)

## Development Process (CRITICAL - READ FIRST)

This project follows strict development standards documented in `documentation/process.md`. Key principles:

### Test-Driven Development (TDD)
**All features and fixes MUST follow the Red/Green/Refactor cycle**:
1. **Red**: Write a failing test first
2. **Green**: Implement minimum code to pass the test
3. **Refactor**: Clean up while keeping tests green

Never write implementation code without tests. Never commit failing tests.

### Modular Design
- **Separate crates** for major components (core-sim, yew-ui, fixtures)
- **Separate modules** for orthogonal concerns (disk/, card/, printer/, timing, etc.)
- **Short functions** (<=30 lines when possible)
- **Test each function** with unit tests in `#[cfg(test)]` modules
- **Document public APIs** with doc comments (///)

### Pre-Commit Quality Checks (MANDATORY)
Before **every commit**, run these checks in order:

```bash
# 1. Format code (auto-fixes)
cargo fmt --all

# 2. Fix all clippy warnings (do NOT disable clippy checks)
cargo clippy --all-targets --all-features -- -D warnings

# 3. Verify all tests pass (do NOT disable tests)
cargo test --all

# 4. Verify build succeeds
cargo build --all
```

**NEVER** (Absolute Prohibitions):
- Skip formatting or clippy checks
- Use `#[allow(clippy::...)]` to disable ANY clippy warnings
- Use `#[allow(dead_code)]` to suppress unused code warnings
- Use `#[allow(unused_imports)]`, `#[allow(unused_variables)]`, or similar suppressions
- Use `#[ignore]` to disable tests without explicit documentation and issue tracking
- Commit failing tests or code that doesn't compile
- Suppress warnings instead of fixing the root cause

**Proper Responses to Warnings:**
- **Dead code** -> Delete it completely OR actually use it in the codebase
- **Unused imports** -> Remove them immediately
- **Clippy suggestions** -> Apply the suggested fix
- **Cannot fix immediately** -> Create entry in `docs/todo.md` with tracking issue, do NOT suppress
- **Disagree with clippy** -> Discuss with maintainer, provide justification, do NOT suppress

### Documentation Standards (CRITICAL - ASCII ONLY)

**NEVER use non-ASCII characters in ANY markdown (.md) files**

This is ABSOLUTELY FORBIDDEN and will cause test failures:
- NO Unicode arrows (right-arrow left-arrow) - use -> <- <->
- NO Unicode bullets (bullet) - use - or *
- NO Unicode boxes (box chars) - use + | and -
- NO Unicode checkmarks/crosses - use [X] [ ] [OK]
- NO Unicode stars - use [*] or *
- NO Unicode emojis or symbols of any kind
- NO APL characters - use ASCII equivalents or comments
- NO special punctuation (em-dash en-dash curly quotes) - use - -- " " '
- NO accented characters - use plain ASCII equivalents
- NO degree symbols - use 'deg' or write out 'degrees'
- NO multiplication/division symbols - use * /
- NO mathematical symbols (less-than-or-equal etc) - use <= >= != ~=
- NO Greek letters (mu pi alpha beta) - write out 'mu' 'pi' 'alpha' 'beta'

**Why ASCII-only?**
1. Maximum compatibility across all platforms
2. No encoding issues or mojibake
3. Works in all text editors without special fonts
4. Copy-paste safe across different systems
5. Project has automated tests that WILL FAIL with non-ASCII

**How to Check:**
```bash
# Test will fail if non-ASCII found
cargo test --all
```

**How to Fix:**
If you accidentally use non-ASCII, replace immediately:
- Delete the character and type ASCII equivalent
- Use plain ASCII arrows: ->  <-  <->
- Use plain ASCII bullets: -  *
- Use plain ASCII boxes: +--+ | |
- Use plain brackets: [X] [ ] [OK] [!]

**NO EXCEPTIONS** - This is enforced by automated tests

**Consequences of Violations:**
- Creates technical debt that must be tracked and fixed later
- Violates core project quality standards (see `docs/process.md`)
- Requires additional cleanup commits
- Sets bad precedent for future development

**EXCEPTION POLICY:**
The ONLY acceptable use of warning suppression is:
1. Documented in an open GitHub issue explaining why
2. Approved by project maintainer in issue comments
3. Includes TODO comment with issue number: `#[allow(...)] // TODO(#123): Remove when fixed`
4. Tracked in `docs/todo.md` with remediation plan

If you are tempted to use `#[allow(...)]`, STOP and ask yourself:
- Can I delete this code instead?
- Can I fix the underlying issue properly?
- Why am I avoiding the proper fix?

The answer is almost always: fix it properly now, not later.

### Commit Standards
Write detailed commit messages:
```
<type>: <short summary (50 chars)>

<detailed explanation of what changed and why>

- Bullet points for multiple changes
- Reference issue numbers (#123)
- Explain design decisions
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`

**Always push commits** after quality checks pass (for backup and CI/CD).

### Code Organization Example
```rust
/// Calculate seek time based on cylinder distance.
///
/// The 2315 seeks in 2-cyl increments: t = 7.5ms x N + 22.5ms
fn calculate_seek_time(&self, from: u16, to: u16) -> u64 {
    let delta = ((to as i32 - from as i32).abs() / 2) as f64;
    let time_ms = delta * 7.5 + 22.5;
    self.timing.delay_us((time_ms * 1000.0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seek_same_cylinder() {
        let disk = Ibm2310::new(TimingModel::realistic());
        assert_eq!(disk.calculate_seek_time(0, 0), 22_500);
    }
}
```

## Testing Strategy

- **TDD approach**: Write tests first, then implement
- **Unit tests**: Embedded in source files using `#[cfg(test)]`
- **Native tests**: For core-sim logic (no browser required)
- **WASM tests**: Use `wasm-bindgen-test` for browser-specific UI components
- **Deterministic timing**: Use `TimingModel::none()` for test reproducibility
- **Test names**: Descriptive (e.g., `test_read_sector_with_invalid_address_returns_error`)

## Rust Edition

**All crates use Rust edition 2024** (specified in workspace and crate `Cargo.toml` files).

## Markdown File Requirements

**All `.md` files must use ASCII-only encoding** (a subset of UTF-8). Only printable ASCII characters (0x20-0x7E) plus newlines and tabs are allowed.

This requirement is automatically enforced by `test_markdown_files_are_ascii_only()` in the test suite. Any markdown file containing:
- Unicode characters (bytes > 127)
- Control characters (0x00-0x1F except newline/tab)
- Invisible Unicode characters (zero-width spaces, non-breaking spaces, etc.)

will cause the build to fail.

**ASCII equivalents to use**:
- Microseconds: `us` not `U+00B5 (mu) + s`
- Less/equal: `<=` not `U+2264`
- Multiply: `x` not `U+00D7`
- Arrows: `->` not `U+2192`
- Quotes: `"` and `'` not smart quotes
- Dashes: `-` and `--` not en/em dashes

See `documentation/process.md` section "Markdown File Encoding" for complete details.

## Documentation References

Comprehensive design documentation is in `documentation/`:
- **`process.md`**: Development methodology and quality standards (**READ THIS FIRST**)
- `ibm_1130_disk_i_o_simulator_starter_docs.md`: Complete system specification
- `architecture.md`, `design.md`, `PRD.md`: Architecture and requirements (currently TODO)
- `research.md`: Historical IBM 1130 facts and fidelity policy
- `plan.md`: Development milestones
- `status.md`: Current project status
- `ui-testing.md`: UI testing guide with Playwright/MCP
- `ui-test-results.md`: Latest UI test results

The starter docs contain detailed device specifications, timing formulas, file format definitions, and sample card deck layouts.
