# Technical Debt & TODO Items

## Critical: Clippy Workarounds to Remove

**++ THESE VIOLATE PROJECT STANDARDS** - See `docs/process.md` and `CLAUDE.md`

### 1. Dead Code in `crates/yew-ui/src/views/disk_map.rs`

**Current Status:** Uses `#[allow(dead_code)]` to suppress warnings
**Violations:** Lines 7, 126, 163, 181, 228, 236

**Affected Code:**
```rust
#[allow(dead_code)]  // + REMOVE THIS
pub struct DiskState { ... }

#[allow(dead_code)]  // + REMOVE THIS
fn render_disk_grid( ... ) { ... }

#[allow(dead_code)]  // + REMOVE THIS
fn render_head_sectors( ... ) { ... }

#[allow(dead_code)]  // + REMOVE THIS
fn render_sector( ... ) { ... }

#[allow(dead_code)]  // + REMOVE THIS
fn calculate_sector_index( ... ) { ... }

#[allow(dead_code)]  // + REMOVE THIS
fn parse_dsk_bytes( ... ) { ... }
```

**Root Cause:**
The original disk visualization UI was replaced with the new tabbed interface (Overview/Hardware/Reference), but the old disk_map.rs code was left in place.

**Proper Fix Options:**

**Option A: Delete Unused Code (RECOMMENDED)**
- [ ] Remove `DiskState` struct entirely
- [ ] Remove all unused rendering functions
- [ ] Keep only the minimal `DiskMap` component shell if needed for future use
- [ ] Document removal in commit message

**Option B: Feature-Gate for Future Use**
- [ ] Move old disk visualization to `#[cfg(feature = "disk-viz")]`
- [ ] Document why code is preserved
- [ ] Ensure it compiles under the feature flag
- [ ] Update `Cargo.toml` with feature definition

**Option C: Complete the Migration**
- [ ] Integrate disk visualization into Hardware tab or Demos tab
- [ ] Actually use the functions with real disk image loading
- [ ] Wire up to emulator when available

**Recommended Action:** Option A - delete the code
**Justification:** We have no current plan to use the old disk visualization. When we need disk visualization for demos, we'll build it properly integrated with the emulator.

**Estimated Effort:** 30 minutes
**Priority:** HIGH (violates core process guidelines)

---

## Build Warnings to Address

### 2. Unused Imports (if any)

**Check For:**
```bash
cargo clippy --all-targets --all-features -- -W unused-imports
```

**Fix:** Remove or gate unused imports

---

## Test Failures to Address

### 3. Missing Tests for New Code

**New Code Without Tests:**
- [ ] `crates/yew-ui/src/views/reference.rs` - No tests
- [ ] `crates/yew-ui/src/views/overview.rs` - No tests
- [ ] `crates/yew-ui/src/views/hardware.rs` - No tests
- [ ] `crates/yew-ui/src/views/header_nav.rs` - No tests

**Required Tests:**
- [ ] Unit tests for component rendering
- [ ] Integration tests for tab switching
- [ ] Accessibility tests for keyboard navigation
- [ ] Link validation tests for Reference tab

**Process Violation:**
TDD requires writing tests first, then implementation. These components were written without tests.

**Proper Fix:**
- [ ] Add `#[cfg(test)]` modules to each component file
- [ ] Test component props and rendering
- [ ] Test callbacks and state changes
- [ ] Achieve >80% code coverage

**Estimated Effort:** 4-6 hours
**Priority:** HIGH (violates TDD process)

---

## Code Quality Issues

### 4. Incomplete Documentation

**Missing Doc Comments:**
- [ ] `reference.rs` - No module-level doc comment
- [ ] `hardware.rs::HardwareDevice` - No enum doc comment
- [ ] Public functions lack `///` documentation

**Process Requirement:**
All public APIs must have doc comments (see `process.md`)

**Proper Fix:**
```rust
/// Reference documentation tab component.
///
/// Displays organized links to IBM 1130 manuals, language references,
/// and online resources. All links open in new tabs with proper
/// security attributes.
///
/// # Example
/// ```no_run
/// html! { <Reference /> }
/// ```
```

**Estimated Effort:** 1-2 hours
**Priority:** MEDIUM

---

## Process Improvements

### 5. Pre-Commit Checklist Not Followed

**What Was Skipped:**
- Tests were not written before implementation
- Clippy warnings were suppressed instead of fixed
- Code was not refactored before commit

**Required Process (from `docs/process.md`):**

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

**Action Items:**
- [ ] Re-read `docs/process.md` before every commit
- [ ] Never use `#[allow(...)]` or `#[ignore]` without explicit justification
- [ ] Write tests FIRST (Red + Green + Refactor)

---

## CLAUDE.md Updates Needed

### 6. Re-emphasize No Workarounds Policy

**Current Issue:**
CLAUDE.md doesn't explicitly forbid `#[allow(dead_code)]` and similar workarounds.

**Required Addition:**
```markdown
## Quality Standards (CRITICAL - NEVER VIOLATE)

**Clippy Warnings:**
- NEVER use `#[allow(clippy::...)]` to disable warnings
- NEVER use `#[allow(dead_code)]` to suppress unused code warnings
- NEVER use `#[allow(unused_imports)]` or similar
- EXCEPTION: Only if documented in issue with explicit approval

**Proper Responses to Warnings:**
- Dead code + Delete it or use it
- Unused imports + Remove them
- Clippy suggestions + Apply them
- If can't fix immediately + Create TODO.md entry, do NOT suppress

**Tests:**
- NEVER use `#[ignore]` without documentation
- NEVER skip `cargo test` in pre-commit
- NEVER commit failing tests

**Consequences of Violations:**
- Creates technical debt
- Violates project standards
- Requires cleanup work later
- Sets bad precedent
```

**Priority:** CRITICAL
**Estimated Effort:** 15 minutes

---

## Immediate Action Plan

### Sprint 1: Fix Critical Violations (This Week)

1. **[ ] Update CLAUDE.md** with strict no-workarounds policy
2. **[ ] Fix disk_map.rs dead code** (Option A: delete unused code)
3. **[ ] Run full clippy check** and ensure zero warnings without suppression
4. **[ ] Update docs/process.md** if needed for clarity

### Sprint 2: Add Missing Tests (Next Week)

1. **[ ] Add tests for reference.rs**
2. **[ ] Add tests for overview.rs**
3. **[ ] Add tests for hardware.rs**
4. **[ ] Add tests for header_nav.rs**
5. **[ ] Achieve >80% coverage on new code**

### Sprint 3: Documentation (Following Week)

1. **[ ] Add doc comments to all public APIs**
2. **[ ] Document module purposes**
3. **[ ] Add usage examples**

---

## Tracking

**Created:** 2025-11-01
**Last Updated:** 2025-11-01
**Next Review:** 2025-11-08

**Assigned:** (maintainer/contributor name)
**Status:** Open

---

## References

- **Process Guidelines:** `docs/process.md`
- **Project Instructions:** `CLAUDE.md`
- **Clippy Documentation:** https://rust-lang.github.io/rust-clippy/

## Lessons Learned

**Why This Happened:**
- Rushed to complete Reference tab feature
- Took shortcut to suppress warnings instead of proper fix
- Did not follow TDD (wrote code before tests)
- Ignored process guidelines under time pressure

**How to Prevent:**
- Always read `process.md` before starting work
- Plan time for tests and proper fixes
- "Fast is slow, slow is fast" - doing it right the first time is faster
- Technical debt compounds - fix issues immediately

**Acknowledgment:**
These violations were created in commit 5177a72 ("feat: Add Reference tab with comprehensive documentation links"). They must be remedied before the codebase can be considered production-ready.
