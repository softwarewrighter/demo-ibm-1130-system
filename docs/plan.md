# Development Plan

## Project Phases

The IBM 1130 System Simulator development is organized into milestones, each delivering incrementally more functionality while maintaining quality standards.

## Milestone 0: Core Infrastructure (Current)

**Goal**: Establish project foundation with development standards, documentation, and basic crate structure.

**Status**: COMPLETE

### Completed Tasks
- [x] Repository initialization with Git
- [x] Cargo workspace setup (3 crates)
- [x] MIT License and copyright
- [x] README.md with project overview
- [x] Development process documentation (TDD, quality checks)
- [x] CLAUDE.md for AI assistant guidance
- [x] Markdown encoding standards and automated test
- [x] All placeholder crates building successfully
- [x] Quality check scripts (fmt, clippy, test, build)

### Artifacts
- Repository structure with crates/docs/scripts directories
- Comprehensive documentation (README, CLAUDE.md, process.md, research.md)
- Automated test for markdown encoding
- Build passing with all quality checks

## Milestone 1: Device Traits & Timing (Next)

**Goal**: Define core abstractions and timing model for device simulation.

**Duration**: 1-2 weeks

### Tasks
- [ ] Define `Device` trait with reset/poll/dsw methods
- [ ] Define `DiskDevice` trait extending Device
- [ ] Implement `TimingModel` enum (None, Realistic, Fast)
- [ ] Implement `DeviceStatusWord` struct
- [ ] Create geometry types (DiskGeometry, SectorAddr, BlockAddr)
- [ ] Write comprehensive unit tests for all traits and types
- [ ] Document all public APIs with doc comments

### Acceptance Criteria
- All traits compile and pass clippy
- >= 90% test coverage for timing logic
- Examples demonstrating trait usage
- cargo doc generates complete documentation

### Deliverables
- `crates/core-sim/src/lib.rs` - Core trait definitions
- `crates/core-sim/src/timing.rs` - Timing model implementation
- `tests/` - Comprehensive test suite

## Milestone 2: IBM 2310 Disk Simulation

**Goal**: Complete simulation of IBM 2310/2315 disk device.

**Duration**: 2-3 weeks

### Tasks
- [ ] Implement `Ibm2310` struct with DiskDevice trait
- [ ] Disk geometry (200 cyl, 2 heads, 4 sectors, 321 words)
- [ ] Seek with 2-cylinder quantization
- [ ] Read/write sector operations
- [ ] Read/write block operations
- [ ] Sector addressing validation
- [ ] Device state machine (idle -> seeking -> reading -> idle)
- [ ] Integration tests for all operations

### Acceptance Criteria
- Seek timing within 1% of IBM specifications
- All addressing modes work correctly
- State machine handles concurrent operations
- Zero crashes or panics on invalid input
- >= 85% test coverage

### Deliverables
- `crates/core-sim/src/disk/ibm2310.rs`
- `crates/core-sim/src/disk/mod.rs`
- Integration test suite

## Milestone 3: File Format Support

**Goal**: Load, create, and save .dsk disk image files.

**Duration**: 1-2 weeks

### Tasks
- [ ] Define .dsk file format specification
- [ ] Implement .dsk file parser with validation
- [ ] Implement .dsk file writer
- [ ] Header validation (magic, version, geometry)
- [ ] Round-trip tests (write then read, verify identical)
- [ ] Error handling for corrupt files
- [ ] Sample .dsk files in fixtures/

### Acceptance Criteria
- Can load valid .dsk files without errors
- Rejects invalid files with clear error messages
- Round-trip preserves all data bit-for-bit
- Handles edge cases (empty disk, full disk, etc.)

### Deliverables
- `crates/core-sim/src/formats/dsk.rs`
- `crates/fixtures/data/disks/*.dsk` - Sample files
- Property tests for round-trip validation

## Milestone 4: Web UI Foundation

**Goal**: Browser-based UI with basic disk visualization.

**Duration**: 2-3 weeks

### Tasks
- [ ] Yew application skeleton
- [ ] Trunk configuration for WASM build
- [ ] Basic disk map component (grid layout)
- [ ] Load .dsk file from browser
- [ ] Display disk geometry and status
- [ ] Color-coded sector display (free/used/system)
- [ ] Hover tooltip showing C/H/S coordinates
- [ ] Basic styling with CSS

### Acceptance Criteria
- Runs in Chrome, Firefox, Safari
- Loads and displays sample .dsk files
- Responsive UI (60 FPS)
- No console errors
- Accessible via keyboard navigation

### Deliverables
- `crates/yew-ui/src/app.rs` - Main application
- `crates/yew-ui/src/views/disk_map.rs` - Disk visualization
- `crates/yew-ui/static/styles.css` - Styling
- Working demo at https://softwarewrighter.github.io/demo-ibm-1130-system/

## Milestone 5: Interactive Disk Operations

**Goal**: User can perform disk operations and see results.

**Duration**: 2 weeks

### Tasks
- [ ] Bridge service connecting UI to core-sim
- [ ] Command dispatch (seek, read, write)
- [ ] Operation status display
- [ ] Timing visualization
- [ ] Head position animation
- [ ] Activity log with timestamps
- [ ] Error display and recovery

### Acceptance Criteria
- Can initiate seeks from UI
- Can read/write sectors and see results
- Visual feedback for all operations
- Error messages are user-friendly
- Operations can be cancelled

### Deliverables
- `crates/yew-ui/src/services/bridge.rs`
- `crates/yew-ui/src/views/console.rs`
- `crates/yew-ui/src/views/status_bar.rs`

## Milestone 6: Audio Feedback

**Goal**: Synthesized audio for disk operations.

**Duration**: 1 week

### Tasks
- [ ] WebAudio service setup
- [ ] Seek sound synthesis (pitch based on distance)
- [ ] Read/write operation sounds
- [ ] ADSR envelope implementation
- [ ] Mute/unmute control
- [ ] Volume control

### Acceptance Criteria
- Seek sounds correlate with distance
- Audio synchronized with visual feedback
- No audio glitches or pops
- Mute persists across page refreshes (localStorage)

### Deliverables
- `crates/yew-ui/src/services/audio.rs`
- `crates/core-sim/src/audio.rs` - Audio model

## Milestone 7: IndexedDB Storage

**Goal**: Persistent disk images in browser.

**Duration**: 1 week

### Tasks
- [ ] IndexedDB wrapper via gloo
- [ ] Save disk images to IndexedDB
- [ ] Load disk images from IndexedDB
- [ ] List saved disks
- [ ] Delete saved disks
- [ ] Import/export to filesystem

### Acceptance Criteria
- Disk images persist across browser sessions
- Can manage multiple disk images
- Export downloads .dsk file
- Import validates before saving

### Deliverables
- `crates/yew-ui/src/services/storage.rs`
- UI for disk management

## Milestone 8: MVP Release (v0.1.0)

**Goal**: Polished release suitable for educational use.

**Duration**: 1-2 weeks

### Tasks
- [ ] User documentation and tutorials
- [ ] Demo disk images with interesting content
- [ ] Performance optimization
- [ ] Cross-browser testing
- [ ] Accessibility audit
- [ ] Release notes and changelog
- [ ] GitHub Pages deployment

### Acceptance Criteria
- All MVP requirements from PRD.md complete
- Zero known critical bugs
- Documentation complete and accurate
- Positive feedback from beta testers
- Performance metrics met (60 FPS, < 5ms/frame)

### Deliverables
- v0.1.0 tagged release
- Published at https://softwarewrighter.github.io/demo-ibm-1130-system/
- Announcement blog post / README update

## Future Milestones (Post-MVP)

### Milestone 9: IBM 1442 Card Reader/Punch
- Card deck file format (.deck)
- Read/punch operations
- Animated card transport
- EBCDIC/ASCII encoding

### Milestone 10: IBM 2311 Disk Support
- Multi-platter geometry
- Larger capacity
- Shared actuator timing

### Milestone 11: IBM 1403 Line Printer
- Print operations
- Form feed
- PDF export

### Milestone 12: IBM 1133 Multiplexor
- Device attachment
- I/O routing
- Multi-device coordination

### Milestone 13: CPU Integration (Phase 2)
- I/O channel protocol
- Interrupt handling
- DMA simulation
- Execute simple programs

### Milestone 14: Historical Software (Phase 3)
- Load real DMS disk images
- Execute IBM 1130 programs
- Full system emulation

## Development Practices

### Every Milestone
1. **TDD**: Write tests before implementation
2. **Quality Checks**: fmt, clippy, test, build all pass
3. **Documentation**: Update docs for new features
4. **Review**: Self-review code against process.md standards
5. **Commit**: Detailed commit messages
6. **Push**: Regular pushes for backup

### Milestone Reviews
At the end of each milestone:
1. Update status.md with progress
2. Review plan.md and adjust if needed
3. Tag release if applicable
4. Retrospective: What went well? What to improve?

## Risk Management

### Technical Risks
- **Browser Performance**: Mitigate with profiling and optimization
- **WASM Limitations**: Research workarounds early
- **Audio API Compatibility**: Test cross-browser

### Schedule Risks
- **Scope Creep**: Strict adherence to milestone scope
- **Technical Unknowns**: Time-box research, spike solutions
- **Quality Issues**: Never skip tests or reviews

## Success Criteria

### By Milestone 8 (MVP)
- [ ] Educational value demonstrated
- [ ] Historical accuracy validated
- [ ] Runs smoothly in all major browsers
- [ ] Positive user feedback
- [ ] Complete test coverage
- [ ] All documentation current

## Current Status

**Active Milestone**: Milestone 0 (Complete)
**Next Milestone**: Milestone 1 (Device Traits & Timing)
**Start Date**: 2025-10-31
**Target Completion**: TBD based on dev velocity

## Related Documents

- [status.md](status.md) - Current project status
- [PRD.md](PRD.md) - Product requirements
- [architecture.md](architecture.md) - System architecture
- [design.md](design.md) - Technical design decisions
