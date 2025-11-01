# IBM 1130 Software Research

## Executive Summary

This document catalogs available IBM 1130 software, programming languages, card decks, and documentation resources for use in the demo-ibm-1130-system simulator. The research prioritizes APL, Forth, FORTRAN, COBOL, Algol, and Lisp implementations, along with utilities, applications, and demos that can be downloaded and emulated.

## Primary Software Repositories

### IBM1130.org
**URL:** http://ibm1130.org/

The premier resource for IBM 1130 preservation and emulation:
- **Software Archive:** http://ibm1130.org/sw/
- **Downloads:** http://ibm1130.org/sim/downloads/
- **Manuals Library:** http://ibm1130.org/lib/manuals/
- **LISP Page:** http://ibm1130.org/sw/lisp/

**Available Software Packages:**
- Disk Monitor System (DMS) R2V12 with FORTRAN compiler and assembler
- APL\1130 source code (apl_source.zip)
- LISP 1.6 disk image
- Windows executables: ibm1130software.zip

**Note:** Site uses self-signed certificate; some browsers may show security warnings.

### SIMH (Simulator for Historical Computers)
**URL:** https://github.com/simh/simh

The SIMH IBM 1130 simulator includes:
- **Disk Images:**
  - `dms.dsk` - Disk Monitor System
- **Source Code:**
  - `dmsboot.asm` - DMS cold start loader
  - `zdcip.asm` - Disk cartridge initialization program
  - `zcrdumpc.asm` - Cold-start memory dump
- **Sample Programs:**
  - `roots.job` - FORTRAN program
  - `csort.job` - FORTRAN sorting program
  - `list.job` - File listing utility
  - One-card programs by Oscar Wyss
- **Cross-Development Tools:**
  - `asm1130` - Cross assembler
  - `bindump` - Object deck dumper
  - `checkdisk` - DMS disk format validator
  - `diskview` - DMS directory viewer
  - `mkboot` - IPL and Core Image Format creator
  - `viewdeck` - Hollerith binary deck viewer

**Documentation:** https://opensimh.org/simdocs/ibm1130_doc.html

### Bitsavers.org
**URL:** http://bitsavers.org/pdf/ibm/1130/

Comprehensive archive of scanned IBM 1130 documentation and software:
- Functional characteristics manuals
- Language reference guides
- Program catalogs
- Subroutine libraries
- Operating procedures

**Mirror Sites:**
- https://bitsavers.trailing-edge.com/
- http://bitsavers.informatik.uni-stuttgart.de/
- https://bitsavers.computerhistory.org/

**Archive.org Integration:** Many bitsavers documents are mirrored at https://archive.org/ for easier access.

## Programming Languages

### APL\1130 + HIGH PRIORITY

**Status:** + Available - Open Source

**Historical Significance:**
- **First publicly available APL system** (Spring 1968)
- Most popular IBM Type-III Library ever released
- Designed by Larry Breed and Phil Abrams
- Version 2 (1969) by Eric Iverson and David Oldacre

**Technical Details:**
- Written in IBM 1130 assembler
- Flat array model (no boxes)
- Configurable index origin (0 or 1 via `)ORIGIN`)
- 32-bit floating-point numeric support
- Defined functions
- Single-character identifiers (V1) or 6-character max (V2)

**Downloads:**
- **Source Code:** http://ibm1130.org/ (apl_source.zip)
- **User Manual:** http://bitsavers.org/pdf/ibm/1130/lang/1130-03.3.001_APL_1130_May69.pdf (May 1969)
- **Primer:** http://bitsavers.informatik.uni-stuttgart.de/pdf/ibm/apl/C20-1697-0_APL_1130_Primer_1968.pdf (1968)

**License:** IBM Type-III Library (open source)

**Implementation Path:**
1. Download APL\1130 source from IBM1130.org
2. Assemble using `asm1130` cross-assembler
3. Load onto DMS disk image
4. Boot and run interactively

**Demo Opportunities:**
- Interactive matrix operations
- Array programming examples
- Mathematical notation demonstrations
- Comparison with modern APL implementations

### Forth + HIGH PRIORITY

**Status:** + Available - Public Domain

**Historical Significance:**
- **Original Forth implementation** - Created by Charles Moore in 1968
- Developed at Mohasco Industries, Amsterdam, NY
- Named "FORTH" (not "FOURTH") due to IBM 1130's 5-character identifier limit
- Used for graphics programming with IBM 2250 display

**Technical Details:**
- Written in IBM 1130 assembly (645 lines)
- Targets 16-bit load-store architecture
- Single accumulator + 3 index registers
- 28 primitive functions
- Packed character strings (2 chars per 16-bit word)
- Dictionary-based extensibility
- Dot-comma syntax (precursor to colon-semicolon)

**Downloads:**
- **Source Code:** https://github.com/monsonite/1968-FORTH
  - `FORTH68asm.txt` - Assembly implementation
  - `FORTH68lst.txt` - Card deck listing
  - `FORTH-68_notes.txt` - Technical documentation
  - `notes on FORTX assem code.pdf` - Carl Claunch's analysis

**Restoration Status:**
- Source recovered circa 2011
- Being updated for restored/emulated 1130 systems (as of 2018)
- Documented by Carl Claunch (IBM 1130 restoration enthusiast)

**License:** No explicit license on GitHub repo (assume public domain/historical preservation)

**Implementation Path:**
1. Clone GitHub repository: `git clone https://github.com/monsonite/1968-FORTH.git`
2. Convert FORTH68asm.txt to assembler format
3. Assemble and load onto simulator
4. Create interactive demos

**Demo Opportunities:**
- Stack-based programming introduction
- Word definition and compilation
- Graphics programming with 2250 display
- Historical comparison with modern Forth systems

### FORTRAN

**Status:** + Available - Multiple Versions

**Versions:**
- **IBM FORTRAN IV** - Included in DMS R2V12
- **FORTRAN-EMU** - Enhanced version by Eastern Michigan University
  - Adds LOGICAL data type
  - Supports 6-letter variable names
  - Additional features over IBM subset compiler

**Technical Details:**
- Runs on systems with as little as 4,096 words of core
- Full compiler, assembler, and system library in DMS R2V12
- Written entirely in assembly language
- Self-modifying code techniques

**Downloads:**
- **DMS R2V12:** http://ibm1130.org/ (includes FORTRAN compiler)
- **Language Manual:** http://bitsavers.org/pdf/ibm/1130/lang/GC26-3715-8_1130_1800_Basic_FORTRAN_IV_Language_Jan73.pdf
- **1965 Manual:** http://bitsavers.org/pdf/ibm/1130/lang/ (C26-5933-2)
- **Core Requirements:** http://bitsavers.org/pdf/ibm/1130/lang/ (C20-1641-1)

**Sample Programs in SIMH:**
- `roots.job` - Root finding
- `csort.job` - Sorting algorithm
- `list.job` - File listing

**Demo Opportunities:**
- Scientific computation examples
- Comparison with modern FORTRAN
- Batch job processing workflow
- Subroutine libraries (SSP)

### COBOL

**Status:** ++ Limited Availability

**Historical Context:**
- Available as add-on product for IBM 1130
- Less common than FORTRAN (1130 targeted scientific market)
- Currently sought by SIMH project and preservation community

**Documentation:**
- No specific COBOL manual found on bitsavers for 1130
- May exist in private collections or IBM archives

**Future Work:**
- Contact IBM1130.org community for leads
- Check with Computer History Museum archives
- May need to use generic IBM COBOL examples

### Algol

**Status:** + Available

**Versions:**
- **Algol 60** - Developed 1967-68 in Grenoble, France
  - University Computer Science department
  - Widely distributed and used
- **Algol 68** - Compiler by Oklahoma State University
  - Written in ANSI FORTRAN 1966

**Documentation:**
- References found in comp.compilers archives
- French language Algol compiler mentioned

**Download Status:**
- Source code not immediately located
- May require contacting original institutions
- Check French computing history archives

### LISP

**Status:** + Available

**Version:** LISP 1.6

**Downloads:**
- **Disk Image:** http://ibm1130.org/sw/lisp/
- Documented on IBM1130.org preservation site

**Implementation Details:**
- LISP 1.6 was the stable version from early 1960s
- Adapted for IBM 1130's 16-bit architecture
- Likely uses disk for list storage (limited RAM)

**Demo Opportunities:**
- Classic LISP examples (car, cdr, cons)
- Recursive function demonstrations
- S-expression evaluation
- Early AI programming techniques

### BASIC

**Status:** ++ Mentioned but Not Located

**Historical Context:**
- Listed as available language for IBM 1130
- BASIC was becoming popular in mid-1960s
- Likely developed by third party or university

**Future Work:**
- Search university archives (Eastern Michigan, Oklahoma State)
- Check Dartmouth BASIC archives for ports
- May need to implement subset interpreter

### RPG (Report Program Generator)

**Status:** + Available in DMS

**Downloads:**
- Included in DMS distributions
- Language manual likely on bitsavers

**Use Case:**
- Business report generation
- Card-to-printer data processing
- Good demo for commercial applications

### PL/I

**Status:** ++ Mentioned but Not Located

**Historical Context:**
- IBM's ambitious "universal" language
- More common on larger systems (360, 370)
- Unclear if full compiler existed for 1130

## System Software

### Disk Monitor System (DMS)

**Versions Available:**
- **DMS V10** - Early version
- **DMS V11** - Mid version
- **DMS R2V12** - Recommended production version

**DMS R2V12 Contents:**
- Macro Assembler
- FORTRAN IV Compiler
- System Library (SSP)
- Loader/Linker
- Disk utilities

**Downloads:**
- **Disk Image:** Available in SIMH distribution (`dms.dsk`)
- **Documentation:** http://bitsavers.org/pdf/ibm/1130/monitor/
  - Programming and Operator's Guide (C26-3717-9, May 1972)
  - Reference Manual (C26-3750-0, 1966)

**Features:**
- File system with directories
- Batch job processing
- Library management
- Source/object deck handling

### Macro Assembler

**Status:** + Available

**Features:**
- Full IBM 1130 instruction set
- Macro definition and expansion
- Symbol table management
- Relocatable output

**Documentation:**
- Assembler Language Manual (GC26-5927)
- Available in DMS R2V12

### Utilities

**Available Programs:**
- **Disk Copy/Backup** - Cartridge duplication
- **Disk Initialization** - Format new cartridges (zdcip.asm)
- **Memory Dump** - Core dump to cards (zcrdumpc.asm)
- **File List** - Directory listing
- **Source List** - Print source decks
- **Card Deck Utilities** - Merge, split, duplicate decks

## Application Software

### Scientific Subroutine Package (SSP)

**Status:** + Available

**Documentation:**
- http://bitsavers.org/pdf/ibm/1130/subroutines/
- H20-0225-0: Application Description (1966)

**Contents:**
- Matrix operations
- Differential equation solvers
- Statistical functions
- Fourier transforms
- Root finding
- Integration/differentiation

### Commercial Subroutine Package

**Status:** + Available

**Documentation:**
- http://bitsavers.org/pdf/ibm/1130/subroutines/
- H20-0221-2: Version 2 Application Description (1967)

**Contents:**
- Business calculations
- Date/time functions
- Report formatting
- Financial functions

### Magnetic Tape Subroutines

**Status:** + Available

**Documentation:**
- Available on archive.org via bitsavers
- For assembler and FORTRAN programs

**Use Case:**
- Data backup and archival
- Large dataset processing
- Multi-reel applications

### Graphics Programs

**Status:** ++ Partial Availability

**Known Applications:**
- **2250 Display** - Vector graphics demos
  - Charles Moore's Forth graphics (1968)
  - Interactive drawing programs
- **1627 Plotter** - Scientific plotting
  - Graph generation from FORTRAN
  - CAD applications

**Future Work:**
- Search for 2250 display demos
- Locate plotter library subroutines
- Find example graphics programs

## Demo Card Decks Available

### SIMH Distribution

**Included Examples:**
1. **roots.job** - FORTRAN root-finding program
   - Demonstrates scientific computation
   - Uses mathematical subroutines
   - Shows formatted output

2. **csort.job** - FORTRAN sorting program
   - Array manipulation
   - Algorithm demonstration
   - Performance characteristics

3. **list.job** - File listing utility
   - Disk I/O operations
   - Formatted printing
   - DMS file system interaction

4. **One-card Programs** - Oscar Wyss contributions
   - Minimal demonstrations
   - Single-card bootstrap concepts
   - Educational examples

### IBM Program Catalog (1966)

**Reference:** C20-1630-1 (December 1966)

**Location:** http://archive.org/details/bitsavers_ibm1130pro0ProgramCatalogDec66_1891857

**Contents:**
- Type-III Library programs (user-contributed)
- Application programs by category:
  - Business
  - Engineering
  - Scientific
  - Educational
  - System utilities

**Note:** Actual card deck downloads may require further investigation. Catalog provides program descriptions and ordering information.

## Documentation for Reference Tab

### System Documentation

**IBM 1130 Functional Characteristics**
- **Manual ID:** A26-5881-2 (1966), GA26-5881-6 (April 1972)
- **PDF:** http://bitsavers.org/pdf/ibm/1130/functional_characteristics/
- **HTML:** https://ibm1130.net/functional/
- **Contents:**
  - CPU architecture and instructions
  - I/O programming
  - Timing and performance data
  - Device specifications

**IBM 1130 System Summary**
- **Manual ID:** GA26-5917-9 (December 1971)
- **PDF:** http://bitsavers.org/pdf/ibm/1130/GA26-5917-9_1130_System_Summary_Dec71.pdf
- **Contents:**
  - Complete system overview
  - Configuration options
  - Capacity and speed specifications

**IBM 1130 Operating Procedures**
- **Manual ID:** GA26-5717-1 (August 1971)
- **PDF:** http://bitsavers.org/pdf/ibm/1130/GA26-5717-1_1130_Operating_Procedures_Aug71.pdf
- **Contents:**
  - Operator console procedures
  - Device operation
  - Error recovery
  - Preventive maintenance

**IBM 1130 Programmer's Reference Card**
- **Manual ID:** X26-3566-4 (1968)
- **PDF:** http://archive.org/details/bitsavers_ibm1130X26ard1968_1436202
- **Contents:**
  - Quick reference for assembly programming
  - Instruction formats and opcodes
  - I/O commands
  - Register usage conventions

### Language Documentation

**FORTRAN**
- **Basic FORTRAN IV Language Manual**
  - Manual ID: GC26-3715-8 (January 1973)
  - URL: http://bitsavers.org/pdf/ibm/1130/lang/GC26-3715-8_1130_1800_Basic_FORTRAN_IV_Language_Jan73.pdf
- **1965 FORTRAN Language**
  - Manual ID: C26-5933-2
  - URL: http://bitsavers.org/pdf/ibm/1130/lang/
- **Core Requirements**
  - Manual ID: C20-1641-1
  - URL: http://bitsavers.org/pdf/ibm/1130/lang/

**APL\1130**
- **APL\1130 Manual**
  - Manual ID: 1130-03.3.001 (May 1969)
  - URL: http://bitsavers.org/pdf/ibm/1130/lang/1130-03.3.001_APL_1130_May69.pdf
- **APL\1130 Primer**
  - Manual ID: C20-1697-0 (1968)
  - URL: http://bitsavers.informatik.uni-stuttgart.de/pdf/ibm/apl/C20-1697-0_APL_1130_Primer_1968.pdf

**Assembler**
- **Assembler Language Manual**
  - Manual ID: GC26-5927
  - URL: Search http://ibm1130.org/lib/manuals/ or bitsavers

### DMS Documentation

**Disk Monitor System Programming and Operator's Guide**
- **Manual ID:** C26-3717-9 (May 1972)
- **URL:** http://bitsavers.org/pdf/ibm/1130/monitor/C26-3717-9_1130_1130_Disk_Monitor_System_Version_2_Programming_and_Operators_Guide_May72.pdf
- **Contents:**
  - DMS commands and job control
  - File system operations
  - Batch processing
  - System generation

**Disk Monitor System Reference Manual**
- **Manual ID:** C26-3750-0 (1966)
- **URL:** http://bitsavers.org/pdf/ibm/1130/monitor/C26-3750-0_1130_Disk_Monitor_System_Reference_Manual_1966.pdf

### Device Documentation

Search http://bitsavers.org/pdf/ibm/ for specific device manuals:
- 1442 Card Reader/Punch
- 1403/1132 Line Printer
- 2250 Vector Graphics Display
- 1627 Plotter
- 2310/2311 Disk Drives
- 1133 Multiplexor

## Recommended Implementation Priority

### Phase 1: Core System (Immediate)

1. **DMS R2V12** - Essential operating system
   - Download and integrate disk image
   - Test boot process in simulator
   - Verify file system access

2. **FORTRAN Compiler** - Most common language
   - Already in DMS R2V12
   - Create demo programs (roots, csort, custom examples)
   - Show batch job workflow

3. **Assembler** - System programming
   - Demonstrate low-level I/O
   - Show interrupt handling
   - Multi-device coordination examples

### Phase 2: High-Level Languages (High Priority)

4. **APL\1130** + - First public APL system
   - Download source from IBM1130.org
   - Assemble and integrate
   - Create matrix/array demos
   - Interactive programming examples

5. **Forth** + - Original implementation
   - Clone GitHub repository
   - Convert and assemble source
   - Graphics demos with 2250 display
   - Stack programming tutorial

6. **LISP 1.6** - Early AI language
   - Download disk image from IBM1130.org
   - Create classic LISP demos
   - Recursive function examples

### Phase 3: Additional Languages (Medium Priority)

7. **RPG** - Business applications
   - Already in some DMS distributions
   - Report generation demos
   - Commercial workflow examples

8. **Algol 60** - If source located
   - Contact Grenoble archives
   - Academic programming examples

### Phase 4: Utilities and Applications (Lower Priority)

9. **Scientific Subroutine Package**
   - Mathematical computation demos
   - Engineering applications

10. **Graphics Programs**
    - 2250 display demos
    - 1627 plotter examples

## Download Action Items

### Immediate Downloads

- [ ] **SIMH Repository:** Clone full repository for simulator and sample programs
  ```bash
  git clone https://github.com/simh/simh.git
  ```

- [ ] **APL\1130 Source:** Download apl_source.zip from IBM1130.org
  - Requires navigating site with self-signed cert
  - Alternative: Contact site maintainers

- [ ] **Forth Source:** Clone 1968-FORTH repository
  ```bash
  git clone https://github.com/monsonite/1968-FORTH.git
  ```

- [ ] **DMS R2V12:** Obtain from IBM1130.org or SIMH distribution
  - May already have from previous work
  - Verify version and completeness

### Documentation Downloads

- [ ] **Functional Characteristics:** Download PDF from bitsavers
- [ ] **FORTRAN Manual:** Download GC26-3715-8
- [ ] **APL Manual:** Download 1130-03.3.001 and Primer
- [ ] **DMS Guide:** Download C26-3717-9
- [ ] **Programmer's Reference Card:** Download X26-3566-4

### Future Investigations

- [ ] **COBOL:** Contact IBM1130.org community for leads
- [ ] **BASIC:** Search university archives
- [ ] **Algol:** Contact Grenoble university or French archives
- [ ] **2250 Graphics:** Search for demo programs
- [ ] **Program Catalog:** Review for interesting applications

## License and Usage Considerations

### Public Domain / Open Source

- **APL\1130:** IBM Type-III Library (open source)
- **Forth (1968):** No explicit license, historical preservation
- **DMS:** Distributed freely by IBM1130.org community
- **SIMH Software:** Preservation use

### Educational Use

- All software being 50+ years old, preservation and educational use widely accepted
- IBM has not enforced copyright on vintage software
- Community preservation efforts sanctioned by historical societies

### Attribution

Always include:
- Original author/institution credits
- Source URL for downloads
- Version and date information
- IBM copyright notices where present

## Community Resources

### Active Communities

- **IBM1130.org** - Primary preservation site and forum
- **SIMH Project** - Simulator development and support
- **Bitsavers.org** - Document preservation (no forum)
- **Computer History Museum** - Archives and preservation
- **VintageTech Forums** - Enthusiast discussions

### Contacts

- **Carl Claunch** - IBM 1130 restoration, Forth documentation
- **IBM1130.org Maintainers** - Software and manual access
- **SIMH Developers** - Simulator enhancements

### Mailing Lists / Forums

- SIMH mailing list for simulator questions
- VCF (Vintage Computer Federation) forums
- comp.sys.ibm newsgroup (historical)

## Conclusion

The IBM 1130 has excellent software preservation thanks to IBM1130.org, SIMH, and bitsavers.org. The highest-priority languages for our simulator are:

1. **APL\1130** - Complete source available, historically significant
2. **Forth** - Original 1968 implementation available on GitHub
3. **FORTRAN** - Already in DMS, well-documented
4. **LISP 1.6** - Disk image available
5. **Assembler** - Full system programming capability

Documentation is comprehensive, with functional characteristics, language manuals, and programmer references all freely available online.

The next steps are to download the identified software, integrate it into our simulator, and create compelling demos showcasing each language and the unique characteristics of the IBM 1130 system.
