# Extended Demos Plan

## Executive Summary

This document outlines a comprehensive plan to transform the IBM 1130 System Simulator from a hardware visualization tool into a complete educational platform featuring interactive demos, executable code examples, and visual representations of system behavior. The goal is to enable users to discover, understand, and experiment with vintage computing concepts through hands-on interaction.

## Vision

Create an accessible, visually compelling learning environment where users can:
- **Explore** each hardware device through focused, single-device demos
- **Understand** how multiple devices work together in real programs
- **Experiment** with high-level languages (APL, Forth) running on the simulated system
- **Visualize** control flow, data flow, and system state in real-time
- **Learn** 1960s computing concepts through step-by-step annotated examples

## Demo Categories

### 1. Single Device Demos

Each I/O device gets dedicated demos showing its unique capabilities:

**IBM 1131 CPU**
- Basic arithmetic operations (ADD, SUB, MPY, DIV)
- Logical operations (AND, OR, XOR, shifts)
- Branch and loop examples
- Subroutine calling conventions
- Interrupt handling basics

**IBM 1442 Card Reader/Punch**
- Read a card deck into memory
- Punch formatted output to cards
- Column binary vs. EBCDIC encoding
- Error handling (card jams, read errors)
- Batch processing pattern

**IBM 1403/1132 Line Printer**
- Print formatted reports
- Character set demonstration
- Forms control (carriage control tape)
- Print chain timing
- Multiple copy printing

**IBM 2310/2311 Disk Drives**
- Sequential file access
- Random access patterns
- Disk Monitor System (DMS) operations
- Seek optimization examples
- Block vs. sector addressing

**IBM 1133 Multiplexor**
- Device attachment/detachment
- I/O command routing
- Device polling strategies
- Interrupt prioritization

**IBM 1627 Plotter**
- Basic line drawing
- Character plotting
- Coordinate systems
- Pen control (up/down/change)
- Scientific graph plotting

**IBM 2250 Vector Graphics Display**
- Vector drawing primitives
- Text rendering
- Light pen interaction
- Display list management
- Interactive graphics loop

### 2. Multi-Device Demos

**Data Processing Pipeline**
- Read cards -> Process in CPU -> Print results
- Shows coordination between card reader, CPU, and printer
- Demonstrates typical business data processing workflow

**Disk-Based Report Generation**
- Read data from disk -> Format -> Print report
- Shows disk I/O combined with printer output
- Example: Student grade reports, inventory listings

**Interactive Data Entry**
- Console keyboard -> Validate -> Write to disk -> Display confirmation
- Shows real-time interaction pattern
- Example: Simple data entry application

**Plotter from Disk Data**
- Read coordinates from disk -> Plot graph
- Shows scientific data visualization workflow
- Example: Temperature/pressure charts

**Card to Disk Transfer**
- Read card deck -> Write to disk file -> Verify
- Shows data migration and validation
- Example: Loading source code or data sets

**Display with Disk Backing Store**
- Load graphics from disk -> Display -> Update with light pen -> Save back
- Shows interactive graphics editing
- Example: Simple CAD or drawing program

### 3. High-Level Language Demos

**APL Interpreter**
- Public domain APL implementation (if available)
- Matrix operations demo
- Array manipulation examples
- Shows expressive power of APL notation

**Forth System**
- Public domain Forth interpreter
- Stack manipulation examples
- Word definition and compilation
- Interactive development environment

**FORTRAN Programs**
- Scientific computation examples
- Subroutine libraries
- Shows compiled vs. interpreted languages

**Assembly Language Tutorial**
- Progressively complex examples
- From "Hello World" to system programming
- Annotated source code with explanations

### 4. System Software Demos

**Disk Monitor System (DMS)**
- File creation and deletion
- Directory listing
- Disk space management
- System calls from user programs

**Assembler Operation**
- Source code -> Object code transformation
- Symbol table construction
- Macro expansion
- Linking multiple modules

**Loader/Linker**
- Loading object decks
- Address relocation
- External reference resolution

**Utilities**
- Disk copy/backup
- Card deck utilities
- Print spooler
- File dump/list programs

## UI/UX Design

### Demos Tab

Add a new top-level tab in the header navigation:

```
[ Overview ] [ Hardware ] [ Demos ] [ Documentation ]
```

The Demos tab provides a curated, browsable catalog of executable examples.

### Demo Browser Structure

```
Demos Tab
+- By Device
|  +- 1131 CPU
|  +- 1442 Card Reader/Punch
|  +- 1403/1132 Printer
|  +- 2310/2311 Disk
|  +- 1133 Multiplexor
|  +- 1627 Plotter
|  +- 2250 Display
+- By Complexity
|  +- Beginner (single device, <50 lines)
|  +- Intermediate (2-3 devices, <200 lines)
|  +- Advanced (multi-device, system software)
+- By Language
|  +- Assembly Language
|  +- FORTRAN
|  +- APL
|  +- Forth
+- Featured Collections
   +- Data Processing Workflows
   +- Scientific Computing
   +- Interactive Graphics
   +- System Programming
```

### Demo Page Layout

Each demo page includes:

**Left Panel: Code View**
- Syntax-highlighted source code
- Line numbers
- Annotations/comments explaining key sections
- Ability to toggle annotations on/off

**Center Panel: Execution View**
- Real-time device state visualization
- Memory viewer showing key addresses
- Register contents (ACC, XR, overflow, etc.)
- Device status indicators

**Right Panel: Output View**
- Printer output (formatted text)
- Plotter graphics (rendered SVG)
- Display output (vector graphics)
- Card deck output (visual representation)

**Bottom Panel: Controls**
- [ Load ] [ Run ] [ Step ] [ Reset ]
- Speed control slider (1x -> 100x)
- Breakpoint controls
- Execution timeline

### Integration with Hardware Tab

Each hardware component page includes:

**"Try It" Section**
- 2-3 featured demos specific to that device
- Quick launch buttons with thumbnails
- Brief description of what each demo shows

**"Related Demos" Links**
- Links to demos using this hardware
- Organized by complexity level
- Visual indicator of multi-device demos

Example (from IBM 1442 Card Reader page):

```
+-----------------------------------------+
| Try These Demos:                        |
+-----------------------------------------+
| [>] Read and Echo Cards (Beginner)     |
|     Read 10 cards and print contents    |
|                                         |
| [>] Card Sort Program (Intermediate)    |
|     Sort cards by column data           |
|                                         |
| [>] Load Source Deck (Advanced)         |
|     Assembler reads source from cards   |
+-----------------------------------------+

Related Demos Using This Device:
- Data Processing Pipeline (3 devices) ->
- Card to Disk Transfer (2 devices) ->
- Batch Processing System (4 devices) ->
```

## Visual Demonstration Features

### Control Flow Visualization

**Instruction Trace View**
- Highlight current instruction in source code
- Show program counter (PC) value
- Visualize branches (taken/not taken)
- Display call stack for subroutines
- Color-coded: loops (blue), branches (yellow), calls (green)

**Flowchart Overlay**
- Auto-generate flowchart from assembly code
- Highlight active path during execution
- Show loop iterations count
- Visualize conditional branches

### Data Flow Visualization

**Memory Map View**
- Visual representation of memory layout
- Color-coded regions: code (gray), data (blue), stack (green), I/O buffers (orange)
- Highlight memory access in real-time (reads: cyan, writes: red)
- Show data movement: memory <-> registers <-> I/O devices

**Data Path Animation**
- Animated arrows showing data movement
- Example: Card reader -> Memory -> Printer
- Timing-accurate animation (scaled to demo speed)

**Device Buffer States**
- Visual representation of device buffers
- Show fill levels, ready/busy states
- Animate data transfer into/out of buffers

### Register/Memory Visualization

**CPU State Panel**
```
+---------------------------------+
| CPU Registers                   |
+---------------------------------+
| ACC:  0x1234  [########--] 52%  |
| XR:   0x0100  [###-------] 12%  |
| PC:   0x0456                    |
| IAR:  0x0450                    |
+---------------------------------+
| Condition Codes                 |
| [[X]] Carry  [[ ]] Overflow         |
| [[ ]] Zero   [[X]] Negative         |
+---------------------------------+
```

**Memory Viewer**
- Hex dump with address labels
- ASCII/EBCDIC interpretation
- Highlight modified addresses
- Bookmarks for key data structures
- Follow-pointer navigation

**Call Stack Visualizer**
- Show subroutine call hierarchy
- Display return addresses
- Local variable regions
- Stack growth/shrink animation

### Device State Visualization

**Disk Drive Animation**
- Rotating platter graphic (based on RPM)
- Seek arm movement during seeks
- Current cylinder/head/sector position
- Read/write indicator LED
- Timing-accurate animation (scaled)

**Card Reader Animation**
- Card moving through reader
- Highlight columns being read
- Card hopper levels (input/output)
- Error states (jam, misread)

**Printer Animation**
- Print chain/line buffer visualization
- Carriage position
- Forms advance
- Characters printing in real-time

**Plotter Animation**
- Pen position (X, Y coordinates)
- Pen state (up/down, color)
- Paper feed direction
- Trace path history (ghosted previous lines)

## Demo Catalog (Detailed Examples)

### Beginner Level

#### Demo: "Hello World" Printer
**Device:** IBM 1403 Line Printer
**Lines:** 25
**Concepts:** Basic I/O, print buffer, carriage control

```assembly
* Print "HELLO WORLD" on line printer
* Demonstrates basic printer I/O

START  DC   0
       BSI  PRINT, HELLO    Load address of message
       WAIT 1403            Wait for print complete
       HALT

HELLO  DC   /HELLO WORLD/   Message text
PRINT  ...printer routine...
```

**Visualization:**
- Show message being copied from memory to print buffer
- Animate print chain selecting characters
- Show paper advancing after print

#### Demo: Read One Card
**Device:** IBM 1442 Card Reader
**Lines:** 30
**Concepts:** Card reading, buffer management, error detection

```assembly
* Read a single card from the 1442
* Check for errors and display contents

START  DC   0
       XIO  READ, 1442      Initiate read
       WAIT 1442            Wait for complete
       BSI  CHECK, ERROR    Check device status
       BSI  PRINT, BUFFER   Print card contents
       HALT

BUFFER DS   80              80-character buffer
...
```

**Visualization:**
- Show card moving through reader
- Columns lighting up as read
- Data flowing into memory buffer
- Error detection logic (if error injected)

### Intermediate Level

#### Demo: Card-to-Printer Data Processing
**Devices:** 1442 Card Reader, 1131 CPU, 1403 Printer
**Lines:** 120
**Concepts:** Multi-device coordination, data validation, formatted output

**Step-by-step narrative:**
1. Initialize devices and buffers
2. Read card into input buffer
3. Validate data fields (numeric ranges, required fields)
4. Transform data (e.g., compute totals, format dates)
5. Format output line
6. Print result
7. Loop back for next card

**Visualization:**
- Split-screen showing card reader, memory, and printer simultaneously
- Data flow arrows: card -> buffer -> processing -> output buffer -> printer
- Highlight validation steps (green=pass, red=fail)
- Show accumulating totals in memory

#### Demo: Disk File Sort
**Devices:** 2310 Disk, 1131 CPU
**Lines:** 250
**Concepts:** File I/O, sorting algorithms, multi-pass processing

**Algorithm:** Two-way merge sort on disk
1. Read file into memory (as much as fits)
2. Sort in-memory chunk
3. Write sorted chunk to temp file
4. Repeat for all chunks
5. Merge sorted chunks

**Visualization:**
- Disk map showing sectors being read/written
- Memory viewer showing sort in progress (bubble sort or quicksort)
- Merge visualization showing two input streams -> one output
- Seek arm moving between cylinders

### Advanced Level

#### Demo: Multi-Device Batch System
**Devices:** 1442 Card Reader, 2310 Disk, 1403 Printer, 1131 CPU
**Lines:** 500+
**Concepts:** Job control, spooling, resource management, interrupts

**System behavior:**
1. Read job control cards
2. Load program from disk
3. Read data cards into memory/disk
4. Execute program
5. Spool output to disk
6. Print spooled output
7. Repeat for next job

**Visualization:**
- Job queue visualization
- Device utilization timeline
- Interrupt handling (show interrupt occurring, ISR executing, return)
- Gantt chart showing overlapped I/O and computation

#### Demo: Interactive Graphics Editor
**Devices:** 2250 Display, Light Pen, 2310 Disk, 1131 CPU
**Lines:** 800+
**Concepts:** Event-driven programming, display lists, coordinate transforms

**Features:**
- Draw lines and shapes with light pen
- Select and move objects
- Save/load drawings from disk
- Zoom and pan

**Visualization:**
- Display list structure in memory
- Light pen detection logic
- Coordinate transformation matrices
- Display refresh cycle

### High-Level Language Demos

#### APL\1130 Matrix Operations [AVAILABLE]
**Language:** APL\1130 Version 2 (1969)
**Source:** http://ibm1130.org/sw/ (apl_source.zip)
**Status:** [OK] Ready for integration
**Concepts:** Array programming, mathematical notation

**Historical Note:** APL\1130 was the first publicly available APL system (Spring 1968) and the most popular IBM Type-III Library program ever released.

```apl
; Solve system of linear equations
; Using matrix operations

A  3 3  2 1 1  1 3 2  1 1 2   ; Coefficient matrix
B  5 8 7                        ; Constants
X  B  A                        ; Solve A??X=B

'SOLUTION: ' , X
```

**Available Examples:**
- Matrix multiplication and inversion
- Statistical functions (mean, variance, correlation)
- Array manipulation demonstrations
- Factorial and recursive operations

**Implementation Details:**
- V1: Single-character identifiers
- V2: Extended 6-character identifiers (recommended)
- 32-bit floating-point support
- Configurable index origin (0 or 1)

**Documentation:**
- User Manual: 1130-03.3.001_APL_1130_May69.pdf (bitsavers)
- Primer: C20-1697-0_APL_1130_Primer_1968.pdf (Paul Berry)

**Visualization:**
- Matrix contents displayed graphically
- Operation steps shown algebraically
- Memory allocation for arrays
- APL workspace viewer

#### Forth Stack Operations [AVAILABLE]
**Language:** Forth (1968 Original Implementation)
**Source:** https://github.com/monsonite/1968-FORTH
**Status:** [OK] Ready for integration
**Concepts:** Stack-based programming, interactive development

**Historical Note:** Original Forth created by Charles Moore in 1968 on IBM 1130 at Mohasco Industries. Named "FORTH" (not "FOURTH") due to 5-character identifier limit.

```forth
: FACTORIAL  ( n -- n! )
  DUP 1 > IF
    DUP 1- FACTORIAL *
  THEN ;

5 FACTORIAL .
```

**Available Source:**
- FORTH68asm.txt (645 lines of assembly)
- FORTH68lst.txt (235 lines, original card deck)
- FORTH-68_notes.txt (technical documentation)
- Detailed analysis by Carl Claunch

**Implementation Details:**
- 28 primitive functions
- Packed character strings (2 chars per 16-bit word)
- Dictionary structure for extensibility
- Dot-comma syntax (precursor to colon-semicolon)
- Designed for IBM 2250 graphics programming

**Visualization:**
- Stack contents shown graphically (X1, X2, X3 registers)
- Word definitions and dictionary structure
- Compilation vs. interpretation phases
- 2250 graphics output (if available)

#### FORTRAN Scientific Computing [AVAILABLE]
**Language:** FORTRAN IV (DMS R2V12)
**Source:** SIMH distribution + IBM1130.org
**Status:** [OK] Ready for integration

**Available Examples:**
1. **roots.job** - Root-finding algorithm
2. **csort.job** - Sorting demonstration
3. **list.job** - File listing utility

**Scientific Subroutine Package (SSP) Demos:**
- Matrix operations (multiplication, inversion)
- Differential equation solvers
- Statistical functions
- Fourier transforms
- Numerical integration

**Implementation Details:**
- Runs on 4,096-word systems
- Compiler included in DMS R2V12
- FORTRAN-EMU variant available (Eastern Michigan University)
- Adds LOGICAL type and 6-letter variables

**Visualization:**
- Variable watch window
- Array/matrix display
- Call stack for subroutines
- Batch job progress

#### LISP 1.6 Symbolic Processing [AVAILABLE]
**Language:** LISP 1.6
**Source:** http://ibm1130.org/sw/lisp/
**Status:** [OK] Ready for integration

**Example Demonstrations:**
```lisp
(DEFUN FACTORIAL (N)
  (COND
    ((EQUAL N 0) 1)
    (T (TIMES N (FACTORIAL (SUB1 N))))))

(FACTORIAL 5)
```

**Available Features:**
- Classic LISP functions (CAR, CDR, CONS)
- Recursive function definitions
- S-expression evaluation
- List processing primitives

**Visualization:**
- List structure diagrams
- Cons cell allocation
- Garbage collection activity
- Environment frames

## Implementation Plan

### Phase 1: Infrastructure (Milestone 1)

**Tasks:**
- [ ] Create Demos tab UI component
- [ ] Implement demo browser (category tree)
- [ ] Design demo metadata format (JSON or TOML)
- [ ] Create demo loader service
- [ ] Build code syntax highlighter for assembly
- [ ] Implement basic execution controls (Load, Run, Reset)

**Deliverable:** Empty Demos tab with category structure and navigation

### Phase 2: Single Device Demos (Milestones 2-4)

**Tasks:**
- [ ] Implement CPU state visualization component
- [ ] Create memory viewer component
- [ ] Build register display component
- [ ] Write 3-5 demos per device (CPU, Card, Printer, Disk)
- [ ] Add device-specific visualizations
- [ ] Create demo annotations system

**Deliverable:** 20+ single-device demos with basic visualization

### Phase 3: Multi-Device Demos (Milestone 5)

**Tasks:**
- [ ] Implement multi-panel layout for concurrent device views
- [ ] Create data flow animation system
- [ ] Build device coordination timeline
- [ ] Write 5-10 multi-device demos
- [ ] Add control flow visualization

**Deliverable:** 10+ multi-device demos with data flow animation

### Phase 4: High-Level Languages (Milestone 6)

**Tasks:**
- [ ] Research public domain APL/Forth implementations
- [ ] Port or implement interpreter for WASM
- [ ] Create language-specific syntax highlighters
- [ ] Write language tutorial demos
- [ ] Integrate with existing hardware visualization

**Deliverable:** Working APL or Forth interpreter with 5+ demos

### Phase 5: Visual Enhancements (Milestone 7)

**Tasks:**
- [ ] Implement instruction trace view
- [ ] Create flowchart auto-generation
- [ ] Build animated data path visualizations
- [ ] Add step-through debugging controls
- [ ] Create execution timeline/history
- [ ] Polish all visualizations

**Deliverable:** Full visual debugging environment

## Technical Considerations

### Demo Format

Demos stored as structured metadata:

```toml
[demo]
id = "hello-world-printer"
title = "Hello World - Line Printer"
device = ["1403"]
difficulty = "beginner"
language = "assembly"
lines = 25

[demo.description]
short = "Print 'HELLO WORLD' to the line printer"
long = """
This demo demonstrates the basics of printer I/O on the IBM 1130.
It shows how to load a message into the print buffer and initiate
a print operation.
"""

[demo.learning_objectives]
objectives = [
  "Understand print buffer structure",
  "Learn XIO instruction for I/O",
  "See WAIT synchronization",
]

[demo.code]
source_file = "demos/printer/hello-world.asm"
entry_point = "START"

[demo.visualization]
highlight_regions = [
  { label = "Message", address = "HELLO", color = "blue" },
  { label = "Print Buffer", address = "PBUF", color = "orange" },
]

[[demo.steps]]
breakpoint = "0x0010"
description = "Load message address into index register"
highlight = ["XR"]

[[demo.steps]]
breakpoint = "0x0012"
description = "Initiate print operation"
devices = ["1403"]
```

### Performance Optimization

- **Lazy loading:** Load demo code only when selected
- **Web Workers:** Run simulation in background thread
- **Canvas rendering:** Use GPU-accelerated graphics for visualizations
- **Throttled updates:** Limit UI refresh rate (60 FPS max)

### Accessibility

- **Keyboard navigation:** All demos controllable without mouse
- **Screen reader support:** ARIA labels for all controls and states
- **Color blindness:** Use patterns in addition to colors
- **Text alternatives:** Describe animations in text for screen readers

## Future Enhancements

### Interactive Challenges

- **Coding challenges:** "Write a program to sort cards by column 5"
- **Debugging exercises:** "This program has a bug, find and fix it"
- **Optimization tasks:** "Reduce seek time by reordering disk accesses"

### User-Contributed Demos

- **Demo submission form:** Users can submit their own demos
- **Review process:** Maintainer approval before publishing
- **Rating system:** Users vote on helpful/interesting demos

### Live Editing

- **In-browser assembler:** Edit and assemble code in the UI
- **Instant feedback:** See results immediately
- **Share URL:** Permalink to modified demo

### Historical Context

- **Timeline view:** Show when each device was introduced/discontinued
- **Cost information:** "The IBM 1403 cost $40,000 in 1965"
- **Marketing materials:** Scanned brochures and manuals
- **Oral histories:** Interviews with 1130 users

## Software Availability and Integration

See **`docs/software-research.md`** for complete details on available IBM 1130 software.

### Confirmed Available Software

**High Priority (Ready for Integration):**
1. **APL\1130** - Source code at http://ibm1130.org/sw/ (apl_source.zip)
2. **Forth (1968)** - GitHub: https://github.com/monsonite/1968-FORTH
3. **FORTRAN IV** - Included in DMS R2V12 (SIMH distribution)
4. **LISP 1.6** - Disk image at http://ibm1130.org/sw/lisp/

**Medium Priority:**
5. **RPG** - Included in DMS distributions
6. **Algol 60** - Grenoble implementation (needs sourcing)

**Lower Priority/Not Yet Located:**
7. **COBOL** - Mentioned but limited availability
8. **BASIC** - Needs university archive search

### Download Repositories

**Primary Sources:**
- **IBM1130.org** - http://ibm1130.org/
  - DMS R2V12 with FORTRAN
  - APL\1130 source (apl_source.zip)
  - LISP 1.6 disk image
  - Extensive manual library

- **SIMH GitHub** - https://github.com/simh/simh
  - IBM 1130 simulator
  - Sample FORTRAN programs (roots.job, csort.job, list.job)
  - Cross-development tools (asm1130, diskview, etc.)

- **Bitsavers.org** - http://bitsavers.org/pdf/ibm/1130/
  - Complete documentation archive
  - Language manuals
  - System references

- **1968 Forth** - https://github.com/monsonite/1968-FORTH
  - Original source code
  - Technical documentation
  - Historical analysis

### Integration Roadmap

**Phase 1: Foundation (Weeks 1-2)**
- Download and organize software (see `docs/data-organization.md`)
- Set up `data/decks/` directory structure
- Create normalization tools for card deck formats

**Phase 2: APL Integration (Weeks 3-4)**
- Assemble APL\1130 source
- Create APL demo decks
- Build APL workspace viewer
- Test matrix operation examples

**Phase 3: Forth Integration (Weeks 5-6)**
- Assemble 1968 Forth source
- Create stack visualization
- Build dictionary browser
- Test factorial and graphics examples

**Phase 4: FORTRAN Demos (Weeks 7-8)**
- Import SIMH sample programs
- Create SSP library demos
- Build batch job visualizer
- Test scientific computation examples

**Phase 5: Polish and Documentation (Weeks 9-10)**
- Complete all visualizations
- Write user tutorials
- Create video demonstrations
- Deploy to production

## Success Metrics

- **Engagement:** Average time spent on Demos tab
- **Completion:** Percentage of users who run at least 3 demos
- **Diversity:** Usage across different demo categories
- **Education:** User feedback on learning value
- **Historical Value:** Downloads of source code and documentation

## Related Documentation

- **Software Research:** `docs/software-research.md` - Complete catalog of available software
- **Data Organization:** `docs/data-organization.md` - Directory structure and file formats
- **Process Guidelines:** `docs/process.md` - Development methodology
- **TODO List:** `docs/todo.md` - Current tasks and technical debt

## Conclusion

The Extended Demos initiative transforms the IBM 1130 simulator from a technical visualization tool into a comprehensive educational platform. By providing curated, interactive examples with rich visual feedback, we enable users to understand vintage computing concepts through hands-on experimentation.

**Key Advantages:**
- **Real Software:** Using actual IBM 1130 programs from the 1960s
- **Historical Accuracy:** APL\1130 was the first public APL; Forth originated on the 1130
- **Open Source:** All software available under IBM Type-III or public domain
- **Educational Value:** Learn both vintage and modern programming concepts
- **Preservation:** Keep computing history accessible for future generations

The phased implementation plan, combined with confirmed software availability, ensures steady progress toward a feature-rich, accessible learning environment that honors the legacy of the IBM 1130 computing system.
