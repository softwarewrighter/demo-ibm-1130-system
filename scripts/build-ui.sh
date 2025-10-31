#!/bin/bash
# Build script for the IBM 1130 Simulator Web UI
#
# This script builds the WASM/Yew UI and ensures all assets are properly
# deployed to the docs/ directory for GitHub Pages.

set -e  # Exit on error

echo "🚀 Building IBM 1130 Simulator Web UI..."
echo

# Navigate to the UI directory
cd "$(dirname "$0")/../crates/yew-ui" || exit 1

# Ensure licensed-media is in static directory
if [ ! -d "static/licensed-media" ]; then
    echo "⚠️  Warning: static/licensed-media not found"
    if [ -d "../../licensed-media" ]; then
        echo "📋 Copying licensed-media from root to static directory..."
        cp -r ../../licensed-media static/
    else
        echo "❌ Error: licensed-media directory not found"
        exit 1
    fi
fi

# Build the WASM UI
echo "📦 Running trunk build --release..."
trunk build --release

echo
echo "✅ Build complete!"
echo "   Output: ../../docs/"
echo
echo "📝 Next steps:"
echo "   1. Test locally: cd crates/yew-ui && trunk serve"
echo "   2. Commit: git add -A && git commit -m \"build: Update WASM UI\""
echo "   3. Deploy: git push"
echo
