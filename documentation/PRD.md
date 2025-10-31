# Product Requirements Document (PRD)

## Project Overview

**Product Name**: IBM 1130 System Simulator
**Version**: 0.1.0 (Pre-Alpha)
**Target Release**: TBD
**Product Owner**: Michael A. Wright

### Vision Statement

Create an educational, browser-based simulator of IBM 1130 peripheral devices that is accurate enough to teach disk geometry, addressing, timing, and I/O operations, while adding modern visual affordances and audio feedback that enhance understanding of 1960s computing technology.

### Product Goals

1. **Educational**: Teach computer architecture, I/O systems, and historical computing
2. **Accurate**: Faithfully model IBM 1130 hardware timing and behavior
3. **Accessible**: Run in any modern web browser without installation
4. **Engaging**: Provide visual and audio feedback that makes learning intuitive
5. **Extensible**: Support future addition of CPU simulation and historical software

## Target Users

### Primary Audience
- **Computer Science Students**: Learning about I/O systems, disk geometry, and historical architecture
- **Retro-Computing Enthusiasts**: Interested in IBM 1130 and 1960s computing history
- **Museum Docents/Educators**: Teaching computing history with interactive demonstrations
- **Systems Programmers**: Understanding low-level disk operations and timing

### User Personas

#### Persona 1: CS Student (Sarah)
- **Background**: Third-year computer science student taking Operating Systems course
- **Goals**: Understand how disk I/O actually works at the hardware level
- **Needs**: Clear visualizations, ability to see timing, step-through operations
- **Technical Level**: Moderate programming experience, limited hardware knowledge

#### Persona 2: Retro-Computing Hobbyist (Bob)
- **Background**: Software engineer with interest in historical computers
- **Goals**: Experience IBM 1130 operations, potentially run historical software
- **Needs**: Accuracy, detailed specifications, ability to load real disk images
- **Technical Level**: High technical expertise, familiar with assembly

#### Persona 3: Museum Educator (Dr. Martinez)
- **Background**: Computer history professor, runs computing museum
- **Goals**: Demonstrate 1960s computing to visitors of varying technical levels
- **Needs**: Engaging visuals, audio feedback, fast-forward capability, no installation
- **Technical Level**: High conceptual knowledge, moderate technical skills

## Functional Requirements

### Must Have (MVP)

#### FR-1: IBM 2310/2315 Disk Simulation
- **FR-1.1**: Accurate disk geometry (200 cylinders, 2 heads, 4 sectors/track)
- **FR-1.2**: Seek operations with 2-cylinder quantization
- **FR-1.3**: Read/write sector operations (321 words per sector)
- **FR-1.4**: Logical block addressing (16 blocks per sector, 20 words per block)
- **FR-1.5**: Realistic timing model (1500 RPM, 27.8us/word)
- **FR-1.6**: Device status word (busy, error, attention flags)

#### FR-2: Disk Map Visualization
- **FR-2.1**: Visual grid showing cylinders, heads, and sectors
- **FR-2.2**: Color-coding by allocation status (free/used/system)
- **FR-2.3**: Hover tooltip showing C/H/S coordinates and block number
- **FR-2.4**: Click to inspect sector/block contents (hex/decimal view)
- **FR-2.5**: Current head position indicator

#### FR-3: File Format Support
- **FR-3.1**: Load .dsk files with validation
- **FR-3.2**: Create new blank .dsk files
- **FR-3.3**: Save modified .dsk files to browser storage
- **FR-3.4**: Export .dsk files to local filesystem

#### FR-4: Timing Modes
- **FR-4.1**: Realistic mode (1x historical timing)
- **FR-4.2**: Fast mode (10x-50x speed)
- **FR-4.3**: None mode (instantaneous for testing)
- **FR-4.4**: User-selectable timing mode

#### FR-5: Audio Feedback
- **FR-5.1**: Disk seek sound with pitch proportional to distance
- **FR-5.2**: Read/write operation sounds
- **FR-5.3**: Mute/unmute control

### Should Have (Phase 2)

#### FR-6: IBM 1442 Card Reader/Punch
- **FR-6.1**: Load .deck files (80-column card format)
- **FR-6.2**: Read card operations (up to 400 cpm)
- **FR-6.3**: Punch card operations (up to 360 cpm)
- **FR-6.4**: Animated hopper, transport path, and stackers
- **FR-6.5**: EBCDIC and ASCII encoding support
- **FR-6.6**: Binary mode support

#### FR-7: IBM 2311 Disk Support
- **FR-7.1**: Multi-platter geometry (Model 11 or 12)
- **FR-7.2**: Shared actuator timing model
- **FR-7.3**: Larger capacity disk images

#### FR-8: Enhanced Visualizations
- **FR-8.1**: Seek operation timeline
- **FR-8.2**: Head/timing oscilloscope view
- **FR-8.3**: Activity log with timestamps
- **FR-8.4**: Performance statistics (seeks, reads, writes)

### Nice to Have (Future)

#### FR-9: IBM 1403 Line Printer
- **FR-9.1**: Print line operations
- **FR-9.2**: Form feed and line advance
- **FR-9.3**: Page buffer preview
- **FR-9.4**: Output to PDF/text file

#### FR-10: IBM 1133 Multiplexor
- **FR-10.1**: Device attachment simulation
- **FR-10.2**: I/O command routing
- **FR-10.3**: Multiple device coordination

#### FR-11: Historical Software Support
- **FR-11.1**: Load actual IBM 1130 DMS disk images
- **FR-11.2**: Execute simple I/O programs (requires CPU simulation)
- **FR-11.3**: Job control language (JCL) interpretation

## Non-Functional Requirements

### Performance

#### NFR-1: UI Responsiveness
- **NFR-1.1**: 60 FPS rendering in browser
- **NFR-1.2**: <= 5ms per frame processing time
- **NFR-1.3**: Smooth animations for device operations

#### NFR-2: Timing Accuracy
- **NFR-2.1**: +-1% accuracy for realistic timing mode
- **NFR-2.2**: Deterministic behavior in none mode (for testing)

### Usability

#### NFR-3: Browser Compatibility
- **NFR-3.1**: Chrome 90+
- **NFR-3.2**: Firefox 88+
- **NFR-3.3**: Safari 14+
- **NFR-3.4**: Edge 90+

#### NFR-4: Accessibility
- **NFR-4.1**: Keyboard navigation support
- **NFR-4.2**: Screen reader compatibility for status information
- **NFR-4.3**: High contrast mode support

#### NFR-5: Learning Curve
- **NFR-5.1**: New users can perform basic operations within 5 minutes
- **NFR-5.2**: Built-in tutorial or demo mode
- **NFR-5.3**: Contextual help/tooltips

### Reliability

#### NFR-6: Data Integrity
- **NFR-6.1**: All .dsk file operations preserve data integrity
- **NFR-6.2**: Validation prevents corruption
- **NFR-6.3**: Automatic backups before modifications

#### NFR-7: Error Handling
- **NFR-7.1**: Graceful handling of invalid input files
- **NFR-7.2**: Clear error messages with recovery suggestions
- **NFR-7.3**: No data loss on browser crash

### Maintainability

#### NFR-8: Code Quality
- **NFR-8.1**: 100% Rust code (no unsafe)
- **NFR-8.2**: >= 80% test coverage
- **NFR-8.3**: All public APIs documented
- **NFR-8.4**: TDD practices followed

#### NFR-9: Build & Deployment
- **NFR-9.1**: Single command build process
- **NFR-9.2**: Automated CI/CD pipeline
- **NFR-9.3**: Static site hosting (GitHub Pages compatible)

### Security

#### NFR-10: Sandboxing
- **NFR-10.1**: All code runs in WASM sandbox
- **NFR-10.2**: No network access required
- **NFR-10.3**: No server-side components

#### NFR-11: Privacy
- **NFR-11.1**: All data stored locally in browser
- **NFR-11.2**: No analytics or tracking
- **NFR-11.3**: No user data collection

## Use Cases

### UC-1: Explore Disk Geometry

**Actor**: CS Student (Sarah)
**Goal**: Understand IBM 1130 disk structure and addressing

**Preconditions**: Simulator loaded in browser

**Main Flow**:
1. Sarah clicks "Load Demo Disk" button
2. System loads a pre-configured .dsk file
3. Disk map displays with all cylinders/heads/sectors visible
4. Sarah hovers over different sectors
5. Tooltip shows C/H/S coordinates and sector number
6. Sarah clicks on sector 0 (boot sector)
7. System displays sector contents in hex viewer
8. Sarah sees sector address word and data payload

**Postconditions**: Sarah understands disk layout and addressing scheme

**Success Criteria**: Can identify any sector's location and contents

### UC-2: Observe Seek Timing

**Actor**: Systems Programmer
**Goal**: Understand disk seek behavior and timing

**Preconditions**: Simulator loaded with disk mounted

**Main Flow**:
1. User enables "Show Timing" visualization
2. User initiates seek from cylinder 0 to cylinder 100
3. System animates head movement
4. Timing graph shows seek profile (accel -> steady -> settle)
5. Audio plays with pitch varying by seek distance
6. System displays calculated seek time (7.5ms x 50 + 22.5ms = 397.5ms)
7. User compares with theoretical calculation

**Postconditions**: User understands 2-cylinder quantization and timing formula

**Success Criteria**: Can predict seek times for arbitrary cylinder distances

### UC-3: Demonstrate Card Operations

**Actor**: Museum Educator (Dr. Martinez)
**Goal**: Show visitors how punched cards worked

**Preconditions**: Simulator with card deck loaded

**Main Flow**:
1. Dr. Martinez loads HELLO.deck file
2. Card reader hopper shows stack of cards
3. Dr. Martinez clicks "Read Card"
4. Animation shows card moving through reader
5. Card content displayed as both punches and text
6. Visitors see how 80 columns encode data
7. Dr. Martinez switches to fast mode for full deck
8. System processes entire deck in seconds

**Postconditions**: Visitors understand card I/O concept

**Success Criteria**: Engaging visual demonstration of historical technology

### UC-4: Debug Disk Addressing

**Actor**: Retro-Computing Hobbyist (Bob)
**Goal**: Verify correct sector/block address calculations

**Preconditions**: Bob is writing IBM 1130 software

**Main Flow**:
1. Bob loads his custom .dsk file
2. Bob enters target cylinder/head/sector (50/1/2)
3. System calculates linear sector index
4. System highlights corresponding sector in disk map
5. Bob verifies against his address calculation
6. Bob enters block address (C=50, H=1, S=2, B=5)
7. System shows word offset within sector
8. Bob confirms his addressing logic is correct

**Postconditions**: Bob's software uses correct addressing

**Success Criteria**: Visual verification prevents addressing bugs

## Success Metrics

### Adoption Metrics
- **M-1**: 100+ GitHub stars within 6 months of release
- **M-2**: 500+ unique users in first year
- **M-3**: Featured in at least 2 educational curricula

### Engagement Metrics
- **M-4**: Average session length > 10 minutes
- **M-5**: > 50% of users complete demo/tutorial
- **M-6**: < 5% browser incompatibility issues

### Quality Metrics
- **M-7**: Zero data corruption bugs reported
- **M-8**: <= 2 critical bugs per release
- **M-9**: 95% of users rate experience as "good" or "excellent"

## Out of Scope (Not in This Release)

- Full IBM 1130 CPU simulation
- Execution of arbitrary IBM 1130 programs
- Network/multi-user features
- Mobile device support (tablet/phone)
- Real-time collaboration
- Plugin/extension system

## Dependencies & Assumptions

### Dependencies
- Rust toolchain (stable channel)
- Trunk (WASM bundler)
- Modern web browser with WASM support
- WebAudio API availability

### Assumptions
- Users have basic understanding of computer architecture
- Users have access to desktop/laptop computer
- Users can download/install Rust if building from source
- Historical accuracy is valued over performance

## Risks & Mitigations

### Risk 1: Browser Performance
**Impact**: High
**Probability**: Medium
**Mitigation**: Extensive profiling, WASM optimizations, progressive enhancement

### Risk 2: Historical Accuracy Challenges
**Impact**: Medium
**Probability**: Medium
**Mitigation**: Extensive research, consult IBM 1130 documentation, community feedback

### Risk 3: Limited User Base
**Impact**: Low
**Probability**: High
**Mitigation**: Educational outreach, documentation, social media presence

### Risk 4: Complexity Creep
**Impact**: High
**Probability**: Medium
**Mitigation**: Strict MVP scope, phased development, regular reviews

## Release Criteria

**Version 0.1.0 (MVP) Release Checklist**:
- [x] Repository setup complete
- [x] Documentation complete
- [ ] Core device traits defined
- [ ] IBM 2310 disk simulation working
- [ ] Disk map visualization functional
- [ ] .dsk file format supported
- [ ] Timing modes implemented
- [ ] Audio feedback working
- [ ] All tests passing
- [ ] User documentation complete
- [ ] Demo disk images included

## Appendix

### Related Documents
- [architecture.md](architecture.md) - System architecture
- [design.md](design.md) - Technical design decisions
- [plan.md](plan.md) - Development roadmap
- [research.md](research.md) - Historical research

### Glossary
- **CHS**: Cylinder/Head/Sector addressing scheme
- **DSW**: Device Status Word
- **DMS**: Disk Monitor System (IBM 1130 operating system)
- **WASM**: WebAssembly
- **TDD**: Test-Driven Development
