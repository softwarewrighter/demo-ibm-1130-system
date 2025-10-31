# Repository seed

```
demo-ibm-1130-system/
|-- crates/
|  |-- core-sim/                 # pure Rust: devices, timing, geometry, file formats
|  |  |-- src/
|  |  |  |-- lib.rs
|  |  |  |-- disk/
|  |  |  |  |-- mod.rs
|  |  |  |  |-- ibm2310.rs
|  |  |  |  |-- ibm2311.rs
|  |  |  |-- card/
|  |  |  |  |-- ibm1442.rs
|  |  |  |-- printer/
|  |  |  |  |-- ibm1403.rs
|  |  |  |-- mux/
|  |  |  |  |-- ibm1133.rs
|  |  |  |-- cpu_bus.rs          # minimal device-channel handshake (pre-CPU integration)
|  |  |  |-- timing.rs
|  |  |  |-- audio.rs            # seek sound synthesis model
|  |  |  `-- util.rs
|  |  `-- Cargo.toml
|  |-- yew-ui/                   # WASM/Yew visualization & WebAudio
|  |  |-- src/
|  |  |  |-- main.rs
|  |  |  |-- app.rs
|  |  |  |-- views/
|  |  |  |  |-- disk_map.rs
|  |  |  |  |-- card_reader.rs
|  |  |  |  |-- console.rs
|  |  |  |  `-- status_bar.rs
|  |  |  |-- services/
|  |  |  |  |-- audio.rs
|  |  |  |  |-- storage.rs       # IndexedDB via gloo
|  |  |  |  `-- bridge.rs        # web worker / wasm_bindgen channels to core-sim
|  |  `-- Cargo.toml
|  `-- fixtures/
|     |-- data/
|     |  |-- disks/
|     |  |  |-- demo2315.dsk     # 2315 image (zeroed + system tracks stub)
|     |  |  `-- demo2311.dsk
|     |  |-- cards/
|     |  |  |-- HELLO.deck       # 80-col text
|     |  |  |-- DISKWRITE.deck   # demo A source
|     |  |  `-- PUNCHOUT.deck    # demo B source
|     |  `-- metadata/
|     |     `-- catalog.json
|     `-- Cargo.toml
|-- docs/
|  |-- research.md
|  |-- architecture.md
|  |-- PRD.md
|  |-- design.md
|  |-- plan.md
|  `-- status.md
`-- Cargo.toml
```

---

## docs/research.md

### Goal

Collect the minimum authoritative facts needed to build a faithful-but-pragmatic IBM 1130 disk and card I/O simulation with modern visualizations.

### Key facts (condensed)

- **Disk types**: Internal *single disk storage drive* (2315 cartridge, 512 K words ~= 1,024 KB) and external **IBM 2310** (one or two single drives), or **IBM 2311** (uses 1316 multi-platter pack; model 11 \~2.56 M words, model 12 \~1.536 M words). A given 1130 attaches **either** 2310-family **or** 2311, not both.
- **Geometry (2315/2310 logical)**: 200 cylinders (plus 3 alternates), 2 surfaces (heads), **4 sectors/track**, **321 words/sector** (first word is sector address; user payload often 320 words). 16 logical blocks x 20 words per sector (Disk Monitor view). 1500 rpm; average rotational delay \~20 ms; read/write \~27.8 us/word. Seek increments of **2 cylinders**; access time \~`7.5ms*N + 22.5ms` with N even. A full cylinder of 8 sectors can be read/written in \~100 ms once aligned.
- **Data checking**: modulo-4 parity over 16 data bits yields 4 check/space bits; read-check recommended after write.
- **Device addressing**: device codes + modifier bits; 2311 models expose multiple logical disks via modifiers; 1133 multiplexor required for some attachments (e.g., 1403 line printer).
- **Cards**: **IBM 1442** read/punch (reader up to 400 cpm; punch model-dependent up to \~360 cpm). 80-column format; binary mode allowed; two output stackers.
- **Printers**: **IBM 1403** Model 6 (340 lpm) or Model 7 (600 lpm) are attachable to the 1130 (via 1133). Chain/train technology.
- **SW stack**: **IBM 1130 DMS (Disk Monitor System) V2**: DUP (disk utility), DCIP (cartridge init), assembler, Fortran, monitor control records (`// ASM`, `// DUP`, `// XEQ`, ...). DISKZ/DISK0/DISK1 subroutines and I/O macros for device access.

### Fidelity policy

- Simulate **on-disk layout and timing** closely enough to be educational and to run real job/control flows from DMS artifacts when possible.
- Abstract **electro-mechanics** (seek quantization, rotational latency) with parameterized timing; provide **fast-mode** to disable delays for CI and demo.
- Provide **visual layers** that never existed (free/used maps, heatmaps, per-cylinder wear, defect table viz) while preserving bit-accurate sector/block addressing.

---

## docs/architecture.md

### System overview

1. **core-sim (Rust, no std on wasm)**

   - Device traits: `DiskDevice`, `CardDevice`, `LinePrinter`, `Multiplexor`, `ChannelBus`.
   - Implementations: `Ibm2310`, `Ibm2311`, `Ibm1442`, `Ibm1403`, `Ibm1133` (routing/attachment), `TimingModel`, `AudioModel` (param only).
   - File formats: `.dsk` (raw words + header), `.deck` (80-col text/binary), `.fnt` (printer chain/train glyph map, optional), `.sym` (labels for sector/block names).
   - Validation: round-trip read/write, parity insert/check, read-check after write.

2. **yew-ui (Rust/Yew to WASM)**

   - **Views**: Disk Map (cylinderxsurfacexsector grid, allocation color by FLET/LET; hover shows C/H/S, block, file owner); Head/Timing oscilloscope; 1442 Hopper/Path/Stackers animation; 1403 page buffer preview; Console & DSW status; Activity timeline.
   - **Audio**: WebAudio seek/pitch sweeps driven by `SeekProfile { delta_cyl, accel, settle }`.
   - **Persistence**: IndexedDB (via `gloo`) for disk images and decks; import/export as files.
   - **Bridging**: `wasm_bindgen` channel between UI and `core-sim`; optional Web Worker for device loops.

3. **Integration adapter (future)**

   - Bus facade for CPU simulator: memory-mapped or channel-oriented IOCC enqueue; IRQ lines; DMA word-strobe.
   - Transport: `postMessage`/`BroadcastChannel` in-browser; WebSocket for remote CPU.

### Data model highlights

```rust
#[derive(Clone, Copy)]
pub struct Geometry {
    pub cylinders: u16,      // 200 logical, 203 physical with 3 alternates
    pub heads: u8,           // 2 (top/bottom)
    pub sectors_per_track: u8, // 4
    pub words_per_sector: u16, // 321 (word 0 = addr)
}

pub struct BlockAddr { // monitor logical block
    pub cyl: u16,
    pub head: u8,  // 0 or 1
    pub sector: u8, // 0..=7 (0..=3 top, 4..=7 bottom)
    pub block: u8,  // 0..=15 within sector (20 words each)
}
```

---

## docs/PRD.md

### Vision

An educational, browser-based simulator of IBM 1130 disk & card I/O that is **accurate enough** to teach geometry, addressing, timing, and job control, while adding modern visual affordances and audio feedback.

### Users

- Retro-computing hobbyists, museum docents, students in OS/IO courses, and engineers integrating an 1130 CPU simulator.

### Core scenarios

1. **Demo A -- "Deck-to-Disk"**

   - Load an 80-col card deck into the 1442.
   - Run a sample job (`// ASM`, `// DUP *STOREDATA` style) that **writes records to a file on the 2315**.
   - UI shows card transport, head seeks/rotations, sector writes, and expanding free-map.
   - Option: fast-mode vs timed mode; optional seek audio.

2. **Demo B -- "Disk-to-Punch"**

   - Execute a disk-resident program that **reads a file and punches** result cards (CSV or binary) to the 1442 stacker B.
   - UI animates 1442 punches per column with throughput caps.

3. **Stretch**: Attach **1403** via **1133** and print a file; visualize line buffer and chain slots.

### Non-functional requirements

- **Deterministic** core for tests (timing off); **reproducible** seeds.
- **Performance**: 60 fps UI with <=5 ms per frame processing on mid-range laptop; can run 1x (timed) or 50x (fast) speed.
- **Portability**: Chrome/Firefox/Safari latest; offline-capable (PWA optional).
- **Extensibility**: trait-based devices; feature-flags `device-2311`, `device-1403`.

---

## docs/design.md

### Device traits

```rust
pub trait Device {
    fn reset(&mut self);
    fn poll(&mut self, now_us: u64); // advance timers, complete ops
    fn dsw(&self) -> DeviceStatusWord; // busy, error, attention
}

pub trait DiskDevice: Device {
    fn geometry(&self) -> Geometry;
    fn seek(&mut self, cyl: u16) -> SeekOutcome; // quantized to even cylinders for 2315
    fn select_head(&mut self, head: u8);
    fn read_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &mut [u16; 321]) -> IoResult<()>;
    fn write_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &[u16; 321]) -> IoResult<()>;
    fn read_block20(&mut self, a: BlockAddr, buf: &mut [u16; 20]) -> IoResult<()>;
    fn write_block20(&mut self, a: BlockAddr, buf: &[u16; 20]) -> IoResult<()>;
}

pub trait CardDevice: Device {
    fn hopper_load(&mut self, deck: Vec<Card80>);
    fn read_card(&mut self) -> IoResult<Card80>;  // 12-row punches; binary mode supported
    fn punch_card(&mut self, card: &Card80, to_stacker_b: bool) -> IoResult<()>;
    fn status(&self) -> CardStatus; // hopper counts, stacker selection
}

pub trait LinePrinter: Device {
    fn print_line(&mut self, line: &[u8]); // 120/132 col
}

pub trait Multiplexor: Device {
    fn attach(&mut self, dev: Box<dyn Device>, dev_code: u8);
    fn issue_iocc(&mut self, dev_code: u8, cmd: IoCommand) -> IoResult<()>; // future CPU bridge
}
```

### Timing model (2315)

- **RPM**: 1500 -> 40 ms/rev -> avg **20 ms** rotational latency.
- **Word rate**: 27.8 us/word -> **\~8.9 ms** to stream 321 words.
- **Seek**: `t = 7.5ms * N_even + 22.5ms settle` (N rounded up to even).
- **Cylinder read**: first sector latency + 8x stream windows ~= 100 ms. All delays can be disabled (`TimingModel::none()`) for tests.

### Parity/check bits

Provide `pack_word(data16: u16) -> u20` and `unpack_word(u20) -> Result<u16, ParityErr>` that emulate modulo-4 check bits (write-time) and check on read.

### File formats

- **.dsk** (little-endian for host, explicit header):

```
struct DskHeader {
  magic: [u8; 8] = *b"I1130DSK";
  geo: Geometry;
  reserved: [u8; 32];
  // optional: bad cyl table, cartridge id, bootstrap sectors shadow
}
// followed by 200 cyl x 2 heads x 4 sectors x 321 u16 words
```

- **.deck**:

```
header: { encoding: "ebcdic"|"ascii", binary: bool }
records: [80-byte frames]
```

- **Catalog** (`catalog.json`) enumerates available disks/decks for the UI.

### Yew UI primitives

- `DiskMap`: zoomable cylinder/sector heatmap (free/used/system). Click -> inspect sector payload as words/bytes, logical blocks, owner.
- `HeadViz`: seek/settle timeline, RPM tick, sector window overlay.
- `CardReader`: hopper/transport/stackers, live column positions during punch.
- `StatusBar`: device DSW bits, active IOCC, throughput.
- `AudioService`: WebAudio node graph; seek pitch proportional to `|Deltacyl|` and accel; soft "clunk" on settle.

### Minimal public API for integration

```rust
// entry points compiled both for native (tests) and wasm
pub mod api {
    use super::*;
    pub fn make_2315() -> Box<dyn DiskDevice> { /* ... */ }
    pub fn make_2311(model12: bool) -> Box<dyn DiskDevice> { /* ... */ }
    pub fn make_1442() -> Box<dyn CardDevice> { /* ... */ }
    pub fn make_1403() -> Box<dyn LinePrinter> { /* ... */ }
    pub fn make_1133() -> Box<dyn Multiplexor> { /* ... */ }
}
```

---

## docs/plan.md

### Milestone 0 -- Bootstrap (this week)

-

### Milestone 1 -- 2315 core

-

### Milestone 2 -- 1442 read path

-

### Milestone 3 -- 1442 punch path

-

### Milestone 4 -- Timing realism & 2311

-

### Milestone 5 -- 1133 + 1403 (stretch)

-

### Tooling & QA

- CI wasm build; `wasm-bindgen-test` for web; native unit tests; snapshot tests for maps; demo scripts.

---

## docs/status.md

- Repo layout + docs drafted.
- Next actionable: code Milestone 0 (geometry + Yew shell) and produce first zeroed `.dsk` image visible in DiskMap.

---

## docs/design.md (appendix) -- Mapping formulas

- **Sector numbering**: sectors `0..=3` = head 0, `4..=7` = head 1.
- **Linear sector index** for cylinder `c`, head `h  in  {0,1}`, sector `s  in  {0..3}`:

```
idx = c * 8 + (h * 4 + s)
```

- **Word offset** (u16) into payload area (skipping address word if monitor-compatible):

```
word_offset = idx * 321 + 1
```

- **Logical block** (`0..=15`) within a sector maps to words `[1 + b*20 .. 1 + b*20 + 19]`.

---

## Sample fixtures

### fixtures/data/cards/HELLO.deck (ASCII for UI demo)

```
// JOB HELLO
// ASM
         START 0
BEGIN    DC    'HELLO, 1130!'
         END   BEGIN
// DUP
*STOREDATA HELLO.TXT,RECSIZE=20,NRECS=1
// XEQ  HELLO
```

### fixtures/data/cards/DISKWRITE.deck

Pseudo-job that assembles a program which writes a file to disk using simplified DISK subroutines (for demo harness):

```
// JOB DISKWRITE
// ASM
WRITE    START 0
         USING *
         EXT   DISKZ
         CALL  DISKZ,WRITE,FILE1,BUF
         END   WRITE
// DUP
*STORE   WRITE
*STOREDATA FILE1,RECSIZE=20,NRECS=64
// XEQ WRITE
```

### fixtures/data/cards/PUNCHOUT.deck

```
// JOB PUNCHOUT
// XEQ  PUNCHCSV
```

### fixtures/data/disks/demo2315.dsk

Zero-initialized 2315 with header; cylinder 0 reserved; synthetic LET/FLET entries for UI coloring.

---

## crates/core-sim/src/lib.rs (sketch)

```rust
pub mod timing; pub mod util; pub mod cpu_bus;
pub mod disk; pub mod card; pub mod printer; pub mod mux;

pub use disk::{Geometry, BlockAddr, DiskDevice};
```

## crates/core-sim/src/disk/mod.rs (sketch)

```rust
#[derive(Clone, Copy, Debug)]
pub struct Geometry { pub cylinders: u16, pub heads: u8, pub sectors_per_track: u8, pub words_per_sector: u16 }
#[derive(Clone, Copy, Debug)]
pub struct BlockAddr { pub cyl: u16, pub head: u8, pub sector: u8, pub block: u8 }

pub trait DiskDevice: super::Device {
    fn geometry(&self) -> Geometry;
    fn seek(&mut self, cyl: u16) -> SeekOutcome;
    fn select_head(&mut self, head: u8);
    fn read_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &mut [u16; 321]) -> IoResult<()>;
    fn write_sector(&mut self, cyl: u16, head: u8, sector: u8, buf: &[u16; 321]) -> IoResult<()>;
    fn read_block20(&mut self, a: BlockAddr, buf: &mut [u16; 20]) -> IoResult<()>;
    fn write_block20(&mut self, a: BlockAddr, buf: &[u16; 20]) -> IoResult<()>;
}
```

## crates/yew-ui/src/app.rs (sketch)

```rust
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="p-4 space-y-3">
          <h1>{"IBM 1130 Disk & I/O Simulator"}</h1>
          <div class="grid grid-cols-3 gap-3">
            <div class="col-span-2"><DiskMap/></div>
            <div class="col-span-1 space-y-2">
              <CardReader/>
              <StatusBar/>
            </div>
          </div>
        </div>
    }
}
```

## UI behaviors

- Drag-drop `.dsk`/`.deck` files -> persisted in IndexedDB -> selectable in UI.
- Click sector -> hex/word viewer; toggle "show address word".
- Play/pause timing; slider for speed multiplier.

---

### Open questions (tracked in status.md)

- Import/export of SIMH 1130 disk images; mapping adapter.
- Authentic DISKZ/DISK0 call sequences for true DMS compatibility vs demo harness.
- EBCDIC table variants for 1442 binary mode.
- 2311 shared-actuator timing visualization.

