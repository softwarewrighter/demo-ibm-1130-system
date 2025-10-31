#!/usr/bin/env bash
set -euo pipefail
cd "./../crates/yew-ui"
trunk build --release
echo "Built to ./dist"
