#!/usr/bin/env bash
set -euo pipefail
cd "./../crates/yew-ui"
if ! command -v trunk >/dev/null 2>&1; then
  echo "Installing trunk (wasm web bundler)..."
  cargo install trunk
fi
if ! rustup target list | grep -q 'wasm32-unknown-unknown (installed)'; then
  rustup target add wasm32-unknown-unknown
fi
exec trunk serve --open
