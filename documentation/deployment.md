# Deployment Guide

This document describes how to deploy the IBM 1130 Simulator web UI to GitHub Pages.

## GitHub Pages Configuration

The live demo is hosted via GitHub Pages using the **main branch, `/pages-demo` directory** approach.

### Initial Setup

1. **Enable GitHub Pages** in your repository settings:
   - Go to Settings → Pages
   - Source: Deploy from a branch
   - Branch: `main`
   - Folder: `/pages-demo`
   - Click Save

2. The live demo will be available at:
   ```
   https://softwarewrighter.github.io/demo-ibm-1130-system/
   ```

## Building for Production

### Prerequisites

```bash
# Install Rust toolchain
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Build Command

```bash
# From repository root
cd crates/yew-ui

# Build release version (outputs to ../../pages-demo)
trunk build --release
```

This will:
- Compile Rust code to WASM (optimized, with LTO)
- Generate HTML, CSS, and JavaScript files
- Output everything to `/pages-demo` directory
- Set public URL to `/demo-ibm-1130-system/` (for GitHub Pages)

### Build Output

The `/pages-demo` directory contains:
```
pages-demo/
  index.html              # Main HTML file
  yew-ui-*.wasm          # WebAssembly binary (optimized)
  yew-ui-*.js            # JavaScript glue code
  static/
    styles.css           # Application styles
```

### Deployment Process

1. **Build the application**:
   ```bash
   cd crates/yew-ui
   trunk build --release
   ```

2. **Verify build output**:
   ```bash
   ls -lh ../../pages-demo/
   ```

3. **Test locally** (optional):
   ```bash
   # Serve from pages-demo directory
   cd ../../pages-demo
   python3 -m http.server 8000
   # Open http://localhost:8000/demo-ibm-1130-system/ in browser
   ```

4. **Commit and push**:
   ```bash
   git add pages-demo/
   git commit -m "docs: Update WASM build for deployment"
   git push origin main
   ```

5. **Wait for GitHub Pages to deploy** (usually < 1 minute)
   - Check deployment status: Settings → Pages
   - Visit: https://softwarewrighter.github.io/demo-ibm-1130-system/

## Directory Structure

```
demo-ibm-1130-system/
  documentation/          # Project documentation (Markdown)
  pages-demo/             # GitHub Pages - WASM build output
    index.html
    *.wasm
    *.js
    static/
  crates/
    yew-ui/               # Source code
      src/
      static/
      Trunk.toml          # Build configuration
```

**Note**: `/pages-demo` contains auto-generated build artifacts. Don't edit these files manually.

## Trunk Configuration

The `crates/yew-ui/Trunk.toml` file configures the build:

```toml
[build]
target = "index.html"
dist = "../../pages-demo"
public_url = "/demo-ibm-1130-system/"

[watch]
ignore = ["../../pages-demo"]

[serve]
port = 1130
open = false
```

Key settings:
- `dist = "../../pages-demo"` - Output directory for GitHub Pages
- `public_url = "/demo-ibm-1130-system/"` - URL path for assets on GitHub Pages
- `port = 1130` - Local development server port (tribute to IBM 1130!)

## Development vs Production

### Development Mode
```bash
cd crates/yew-ui
trunk serve
```
- Runs on http://localhost:1130
- Hot reload enabled
- Unoptimized WASM (faster builds)
- Source maps included
- Outputs to temporary directory

### Production Mode
```bash
cd crates/yew-ui
trunk build --release
```
- Optimized WASM with LTO
- Minified JavaScript
- No source maps
- Outputs to `/docs` for deployment
- Ready for GitHub Pages

## Updating the Live Demo

Whenever you make changes to the UI:

1. Make code changes in `crates/yew-ui/src/`
2. Test locally with `trunk serve`
3. Build for production with `trunk build --release`
4. Commit the updated `/pages-demo` directory
5. Push to GitHub
6. GitHub Pages automatically deploys the new version

## Troubleshooting

### 404 Errors on GitHub Pages

If assets fail to load, verify:
- `public_url` in `Trunk.toml` matches your repository name
- Repository name: `demo-ibm-1130-system`
- Public URL: `/demo-ibm-1130-system/`

### WASM Not Loading

Check browser console for errors. Common issues:
- MIME type errors: GitHub Pages should serve `.wasm` with correct type
- CORS issues: Should not occur with GitHub Pages
- Path issues: Verify `public_url` is correct

### Build Fails

```bash
# Clean build cache
cargo clean

# Rebuild
cd crates/yew-ui
trunk build --release
```

### Large WASM File Size

Current optimizations:
- Release mode with LTO
- `wasm-opt` (if installed)
- Code splitting (planned)

To further reduce size:
```bash
# Install wasm-opt
cargo install wasm-opt

# Manually optimize
wasm-opt -Oz pages-demo/*.wasm -o pages-demo/*.wasm
```

## Performance

### Build Times
- Clean build: ~20-30 seconds
- Incremental build: ~2-5 seconds
- Release build: ~20-30 seconds

### Output Size
- WASM binary: ~200-500 KB (compressed)
- JavaScript: ~50-100 KB
- HTML + CSS: < 20 KB
- **Total**: ~300-600 KB (gzipped)

### Load Times
- First visit: < 2 seconds
- Cached: < 500ms
- WASM parse/compile: < 500ms

## CI/CD Alternative

If you prefer automated deployments, you can use GitHub Actions instead:

1. Don't track `/pages-demo` in git
2. Create `.github/workflows/deploy.yml` with Trunk build step
3. Deploy to `gh-pages` branch
4. Configure GitHub Pages to use `gh-pages` branch

This approach keeps build artifacts out of your main branch but requires CI/CD setup.

## References

- [Trunk Documentation](https://trunkrs.dev/)
- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [Yew Deployment Guide](https://yew.rs/docs/deployment)
- [WASM Optimization Guide](https://rustwasm.github.io/book/reference/code-size.html)
