# UI Testing Guide

This document describes how to test the IBM 1130 disk simulator web UI using Playwright via the MCP (Model Context Protocol) integration.

## Overview

The Yew/WASM UI can be tested using Playwright, a browser automation framework. The tests verify:
- UI renders correctly
- Visual elements are present and styled properly
- File upload functionality works
- Disk visualization displays correct geometry
- Interactive features (hover tooltips, sector highlighting) function

## Prerequisites

### Install Trunk
Trunk is required to build and serve the WASM application:

```bash
cargo install trunk
```

### Start Development Server
Navigate to the yew-ui crate and start the server:

```bash
cd crates/yew-ui
trunk serve
```

This will:
- Build the WASM bundle
- Start a server on http://localhost:1130
- Watch for file changes and rebuild automatically

## Testing with MCP/Playwright

### Available Playwright Tools

The MCP integration provides these Playwright tools:

- `mcp__playwright__playwright_navigate`: Navigate to a URL
- `mcp__playwright__playwright_screenshot`: Capture screenshots
- `mcp__playwright__playwright_get_visible_text`: Get all visible text
- `mcp__playwright__playwright_get_visible_html`: Get page HTML
- `mcp__playwright__playwright_click`: Click elements
- `mcp__playwright__playwright_fill`: Fill form inputs
- `mcp__playwright__playwright_upload_file`: Upload files to input[type=file]
- `mcp__playwright__playwright_hover`: Hover over elements
- `mcp__playwright__playwright_close`: Close browser

### Basic Test Flow

1. **Navigate to Application**
   ```
   Navigate to http://localhost:1130
   ```

2. **Take Initial Screenshot**
   ```
   Screenshot the landing page to verify initial render
   ```

3. **Verify Page Content**
   ```
   Get visible text to confirm:
   - Header: "IBM 1130 Disk & I/O Simulator"
   - Section title: "Disk Map (2315 Cartridge)"
   - Geometry info: "Cylinders: 200", "Heads: 2", etc.
   - Status: "Loaded: No (blank disk)"
   ```

4. **Test File Upload (if .dsk file available)**
   ```
   - Click file input
   - Upload a .dsk file
   - Verify geometry updates
   - Verify grid renders with sectors
   - Take screenshot showing loaded disk
   ```

5. **Test Hover Interaction**
   ```
   - Hover over sector elements
   - Verify tooltip appears with C/H/S coordinates
   - Take screenshot of hover state
   ```

6. **Verify Visual Styling**
   ```
   Get HTML to confirm:
   - CSS classes applied correctly
   - Grid structure matches expected layout
   - Color-coded sectors (free/used/system classes)
   ```

## Manual Test Scenarios

### Scenario 1: Initial Load
**Expected**: Blank disk with default IBM2315 geometry

- Cylinders: 200
- Heads: 2
- Sectors/Track: 4
- Words/Sector: 321
- Total Words: 513,600
- Loaded: No (blank disk)
- All sectors should be gray (free)

### Scenario 2: Load Valid .dsk File
**Expected**: Geometry from file, sectors colored based on content

1. Click "Load .dsk file" input
2. Select a valid .dsk file (e.g., from `crates/fixtures/data/disks/`)
3. Verify:
   - No error message appears
   - Geometry matches file
   - "Loaded: Yes" displays
   - Grid shows blue sectors where data exists
   - Hover shows correct C/H/S coordinates

### Scenario 3: Load Invalid File
**Expected**: Error message displays

1. Select a non-.dsk file
2. Verify error: "Please select a .dsk file"

1. Select a .dsk with invalid format
2. Verify error: "Error loading file: ..."

### Scenario 4: Grid Visualization
**Expected**: Properly formatted grid

- Header row shows "Cyl" and "H0", "H1"
- Row labels show cylinder numbers (0, 10, 20, ..., 190 for 2315)
- Each head cell contains 4 sectors (for 2315)
- Sectors are 16x16px squares with 2px margin
- Hover scales sector to 1.3x with shadow

### Scenario 5: Responsive Design
**Expected**: Layout adapts to viewport

- Desktop (>1024px): 2-column grid (disk map + controls)
- Tablet (768-1024px): Single column
- Mobile (<768px): Smaller sectors (12x12px), compact layout

## Test Files

Sample .dsk files for testing are located in:
```
crates/fixtures/data/disks/
```

You can create test files using the core-sim library:

```rust
use core_sim::disk::{Geometry, file_io};

let geo = Geometry::IBM2315;
let mut data = vec![0u16; geo.total_words()];

// Write some test data
data[0] = 0x1234;  // Cylinder 0, head 0, sector 0, word 0

file_io::write_dsk_file("test.dsk", &geo, &data)?;
```

## Automated Test Script

Here's a complete test sequence using MCP/Playwright:

```
1. Navigate: http://localhost:1130
2. Screenshot: "01-initial-load.png"
3. Get visible text: verify header and geometry
4. Get HTML: verify grid structure
5. Screenshot: "02-blank-disk-grid.png"
6. (If .dsk available) Upload file
7. (If .dsk available) Screenshot: "03-loaded-disk.png"
8. (If .dsk available) Get visible text: verify "Loaded: Yes"
9. Hover over sector: .sector (first one)
10. Screenshot: "04-hover-tooltip.png"
11. Close browser
```

## CI/CD Integration

For continuous integration, tests can be automated using:

1. **GitHub Actions Workflow**:
   ```yaml
   - name: Build WASM UI
     run: |
       cd crates/yew-ui
       trunk build

   - name: Serve and Test
     run: |
       trunk serve &
       sleep 5  # Wait for server
       # Run playwright tests
       npx playwright test
   ```

2. **Playwright Test Framework**:
   Create `crates/yew-ui/e2e/disk-map.spec.ts`:
   ```typescript
   import { test, expect } from '@playwright/test';

   test('disk map loads with default geometry', async ({ page }) => {
     await page.goto('http://localhost:1130');

     await expect(page.locator('h1')).toContainText('IBM 1130');
     await expect(page.locator('.disk-info')).toContainText('Cylinders: 200');
     await expect(page.locator('.disk-info')).toContainText('Heads: 2');

     await page.screenshot({ path: 'screenshots/initial-load.png' });
   });

   test('file upload shows error for wrong extension', async ({ page }) => {
     await page.goto('http://localhost:1130');

     const fileInput = page.locator('input[type="file"]');
     await fileInput.setInputFiles('test.txt');

     await expect(page.locator('.error-message'))
       .toContainText('Please select a .dsk file');
   });
   ```

## Troubleshooting

### Server Not Running
If tests fail with connection errors:
```bash
# Check if trunk is running
curl http://localhost:1130

# If not, start it
cd crates/yew-ui
trunk serve
```

### WASM Build Errors
If the UI doesn't load:
```bash
# Clean and rebuild
cargo clean
cd crates/yew-ui
trunk build --release
```

### Browser Issues
Playwright requires browser binaries:
```bash
# Install browsers
npx playwright install
```

### CORS Errors
If file upload fails due to CORS, ensure trunk serves with correct headers.
Trunk.toml should have:
```toml
[serve]
port = 1130
```

## Performance Testing

### Load Time Metrics
- Initial page load: < 2s
- WASM module load: < 1s
- File parse (2MB .dsk): < 500ms
- Grid render (1600 sectors): < 100ms

### Responsiveness
- Hover feedback: < 16ms (60fps)
- File selection response: Immediate
- Error message display: Immediate

## Accessibility Testing

Using Playwright, verify:
- Keyboard navigation works (Tab through elements)
- Focus indicators visible (outline on focus)
- Color contrast meets WCAG AA (4.5:1 minimum)
- Screen reader compatibility (ARIA labels)

```
Test: Tab navigation
1. Navigate to page
2. Press Tab repeatedly
3. Verify focus moves through: file input -> sectors -> tooltip
4. Verify visible focus indicators (2px blue outline)
```

## Visual Regression Testing

Take baseline screenshots and compare on changes:

```bash
# Capture baseline
playwright screenshot --name baseline-initial.png

# After changes, capture and compare
playwright screenshot --name current-initial.png
pixelmatch baseline-initial.png current-initial.png diff.png
```

## References

- [Playwright Documentation](https://playwright.dev/)
- [Trunk Documentation](https://trunkrs.dev/)
- [Yew Testing Guide](https://yew.rs/docs/advanced-topics/testing)
- [WASM Testing Best Practices](https://rustwasm.github.io/book/reference/debugging.html)
