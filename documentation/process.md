# Development Process

This document describes the development methodology, code quality standards, and workflow for the IBM 1130 Simulator project.

## Development Methodology

### Test-Driven Development (TDD)

All features and bug fixes **must** follow the Red/Green/Refactor TDD cycle:

1. **Red**: Write a failing test that defines the desired behavior
2. **Green**: Implement the minimum code necessary to make the test pass
3. **Refactor**: Clean up the code while keeping tests green

**Example TDD workflow**:
```rust
// 1. RED - Write failing test first
#[test]
fn test_seek_quantization() {
    let mut disk = Ibm2310::new(TimingModel::none());
    let outcome = disk.seek(5);
    assert_eq!(outcome.quantized_cyl, 4); // Seeks to even cylinder
}

// 2. GREEN - Implement feature
fn quantize_cylinder(&self, cyl: u16) -> u16 {
    (cyl / 2) * 2  // Round down to nearest even number
}

// 3. REFACTOR - Improve while tests stay green
```

### Code Organization Principles

#### Modular Architecture

**Separate crates for major components**:
- `core-sim`: Device simulation logic (platform-agnostic)
- `yew-ui`: Browser-based UI (WASM target)
- `fixtures`: Test data and sample files

**Separate modules for orthogonal concerns**:
- Each device type in its own module (`disk/`, `card/`, `printer/`, `mux/`)
- Cross-cutting concerns separated (`timing`, `audio`, `cpu_bus`, `util`)

**Short, tested, documented functions**:
- Functions should be <=30 lines when possible
- Each function should do one thing well
- Public functions must have doc comments
- Complex logic must have inline comments explaining "why"

**Example of well-structured code**:
```rust
/// Calculate seek time for 2315 disk based on cylinder distance.
///
/// The 2315 seeks in 2-cylinder increments with timing formula:
/// t = 7.5ms x N_even + 22.5ms settle
fn calculate_seek_time(&self, from: u16, to: u16) -> u64 {
    let delta = ((to as i32 - from as i32).abs() / 2) as f64;
    let time_ms = delta * 7.5 + 22.5;
    self.timing.delay_us((time_ms * 1000.0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seek_time_same_cylinder() {
        let timing = TimingModel::realistic();
        let disk = Ibm2310::new(timing);
        assert_eq!(disk.calculate_seek_time(0, 0), 22_500); // Just settle time
    }

    #[test]
    fn test_seek_time_two_cylinders() {
        let timing = TimingModel::realistic();
        let disk = Ibm2310::new(timing);
        assert_eq!(disk.calculate_seek_time(0, 2), 30_000); // 7.5ms + 22.5ms
    }
}
```

## Quality Assurance Checklist

Before **every commit**, the following quality checks **must** be performed:

### 1. Code Formatting
```bash
cargo fmt --all
```
- Uses Rust standard formatting
- **Never** skip formatting
- Ensures consistent code style across the project

### 2. Linting (Clippy)
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
- Fix **all** clippy warnings
- **Do not** use `#[allow(clippy::...)]` to disable checks
- If a clippy warning seems incorrect, refactor the code to satisfy it
- Clippy catches common bugs and enforces best practices

### 3. Verify .gitignore
- Ensure build artifacts are ignored (`/target`, `dist/`)
- Never commit IDE-specific files (`.vscode/`, `.idea/`)
- Never commit system files (`.DS_Store`)
- Check with `git status` before committing

### 4. Markdown File Encoding
**All `.md` files must use ASCII-only encoding (a subset of UTF-8).**

Specifically, only printable ASCII characters (0x20-0x7E) plus newlines (0x0A) and tabs (0x09) are allowed.

**Rationale**: While GitHub supports UTF-8/Unicode in markdown, restricting to ASCII ensures:
- Maximum compatibility across all tools and editors
- No invisible or control characters that cause display issues
- Consistent rendering across different systems and locales
- Simpler text processing and searching

**Common violations to avoid**:
- Unicode symbols: U+00D7 (multiplication), U+2264 (less/equal), U+2248 (approx), U+00B5 (micro)
- Smart quotes: U+201C, U+201D, U+2018, U+2019
- Special dashes: U+2013 (en dash), U+2014 (em dash), U+2010 (non-breaking hyphen)
- Non-breaking spaces and other invisible Unicode characters

**Use ASCII equivalents instead**:
- Multiplication: `x` instead of U+00D7
- Less than or equal: `<=` instead of U+2264
- Approximately: `~=` instead of U+2248
- Microseconds: `us` instead of U+00B5 (mu)
- Arrow: `->` instead of U+2192
- Element of: `in` instead of U+2208
- Not equal: `!=` instead of U+2260
- Regular ASCII quotes, dashes, and spaces only

**Automated enforcement**:
The test suite includes `test_markdown_files_are_ascii_only()` which automatically fails the build if any .md file contains non-ASCII characters. Run with:
```bash
cargo test --all
```

**Manual verification**:
```bash
# Check all .md files for non-ASCII characters
find . -name "*.md" -exec perl -ne 'print "$ARGV:$.: $_" if /[^\x00-\x7F]/' {} +

# Or use the provided script
./scripts/check-md-encoding.sh
```

### 5. Update Documentation
- Update doc comments for any changed public APIs
- Update `docs/status.md` if feature status changes
- Update `CLAUDE.md` if architecture changes
- Update `README.md` if user-facing changes occur

### 6. Run All Tests
```bash
cargo test --all
```
- **All tests must pass** before committing
- **Never** disable tests with `#[ignore]` without documenting why
- Add new tests for any new functionality
- Maintain or improve code coverage

### 7. Verify Build
```bash
# Native build
cargo build --all

# WASM build (if UI changes)
cd crates/yew-ui
trunk build --release
```

## Commit Standards

### Commit Messages

Use detailed, descriptive commit messages following this format:

```
<type>: <short summary (50 chars or less)>

<detailed explanation of what changed and why (wrap at 72 chars)>

- Bullet points for multiple changes
- Reference issue numbers if applicable (#123)
- Explain trade-offs or design decisions
```

**Commit types**:
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code restructuring without behavior change
- `test`: Adding or updating tests
- `docs`: Documentation changes
- `chore`: Build, dependencies, tooling

**Example good commit**:
```
feat: implement 2-cylinder seek quantization for IBM 2310

The IBM 2315 cartridge drive seeks in increments of 2 cylinders
due to mechanical constraints. This commit adds quantization logic
to round all seek targets down to the nearest even cylinder.

- Added quantize_cylinder() method to Ibm2310
- Updated seek timing calculation for quantized moves
- Added tests for odd/even cylinder seek behavior

Refs: docs/ibm_1130_disk_i_o_simulator_starter_docs.md (seek specs)
```

**Example bad commit** (avoid):
```
fixed stuff
```

### Push Policy

**Always push commits** after they pass quality checks:

```bash
git push origin <branch-name>
```

**Reasons to push frequently**:
1. **Backup**: Protects work against local machine failure
2. **Collaboration**: Makes changes visible to team members
3. **CI/CD**: Triggers automated testing on other platforms
4. **History**: Creates recoverable checkpoints

## Testing Guidelines

### Test Organization

Tests should be co-located with the code they test:

```rust
// src/disk/ibm2310.rs

impl Ibm2310 {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_one() {
        // Test implementation
    }
}
```

### Test Categories

1. **Unit tests**: Test individual functions in isolation
2. **Integration tests**: Test interactions between modules
3. **Timing tests**: Use `TimingModel::none()` for deterministic results
4. **WASM tests**: Use `wasm-bindgen-test` for browser-specific code

### Test Naming

Use descriptive test names that explain what is being tested:

```rust
#[test]
fn test_read_sector_with_invalid_address_returns_error() { }

#[test]
fn test_geometry_total_sectors_calculation() { }

#[test]
fn test_block_addr_conversion_to_sector_index() { }
```

## Code Review Guidelines

When reviewing code (or self-reviewing before commit):

- [ ] Does it follow TDD (tests exist and pass)?
- [ ] Is it modular (appropriate crate/module/function boundaries)?
- [ ] Are functions short and focused?
- [ ] Are public APIs documented?
- [ ] Does it pass all quality checks (fmt, clippy, tests)?
- [ ] Is the commit message clear and detailed?
- [ ] Has documentation been updated?
- [ ] Are there any hardcoded values that should be constants?
- [ ] Are error cases handled properly?
- [ ] Is the code testable (minimal dependencies, clear inputs/outputs)?

## Rust Edition

This project uses **Rust 2024 edition** as specified in all `Cargo.toml` files:

```toml
[workspace.package]
edition = "2024"
```

All crates inherit this edition from the workspace. When adding new features, ensure compatibility with Rust 2024 edition requirements.

## Dependencies

- Keep dependencies minimal and well-justified
- Document why each dependency is needed
- Prefer standard library solutions when possible
- Pin versions for reproducible builds
- Regularly audit dependencies for security issues

## Markdown File Encoding (CRITICAL - ASCII ONLY)

**ALL markdown (.md) files MUST use ASCII-only encoding**

This is a STRICT REQUIREMENT enforced by automated tests:

### Prohibited Characters

**NEVER use these in markdown files:**
- Unicode arrows (use -> <- <-> instead)
- Unicode bullets (use - or * instead)
- Unicode box-drawing characters (use +|- instead)
- Unicode checkmarks/crosses (use [X] [ ] instead)
- Unicode stars (use [*] instead)
- Emojis or Unicode symbols
- APL special characters (use comments or ASCII equivalents)
- Smart quotes/dashes (use " " ' - -- instead)
- Accented characters (use plain e, u, etc. instead)
- Degree symbols (use 'deg' instead)
- Math symbols (use <=  >=  != instead)
- Greek letters (use 'mu' 'pi' etc. instead)

### Why ASCII-Only?

1. **Compatibility:** Works on ALL platforms, editors, and terminals
2. **No Encoding Issues:** Prevents mojibake and display problems
3. **Copy-Paste Safe:** Works across different systems
4. **Testing:** Project has automated tests that FAIL on non-ASCII
5. **History:** Plain text survives format changes better

### Testing

Before committing, ALWAYS run:
```bash
cargo test --all
```

The test `test_markdown_files_are_ascii_only` will FAIL if non-ASCII characters are found.

### Quick Reference

**Arrows:** -> <- <->
**Bullets:** - or *
**Boxes:** +--+ | |
**Status:** [X] [ ] [OK] [!]
**Stars:** *  [*]

If unsure, stick to letters, numbers, and basic punctuation: -.,;:!?()[]{}

## Continuous Improvement

This process document is living documentation. Improve it when:
- You discover a better practice
- A quality issue slips through
- The team agrees on a new standard
- Tools or workflows change

Always commit process improvements with clear rationale.
