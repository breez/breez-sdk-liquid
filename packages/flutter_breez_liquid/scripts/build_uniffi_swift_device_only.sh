#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Device-only iOS build script
# Builds only aarch64-apple-ios target for faster CI builds
# when testing exclusively on physical devices (e.g., TestFlight)
#
# This saves ~80% of Rust compilation time by skipping:
# - x86_64-apple-ios (Intel simulator)
# - aarch64-apple-ios-sim (ARM simulator)
# - aarch64-apple-darwin (macOS ARM)
# - x86_64-apple-darwin (macOS Intel)
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../.."
LIB_DIR="$ROOT/lib/bindings"

log() {
  echo -e "\033[1;34m[INFO]\033[0m $*"
}

log "Building iOS device-only (aarch64-apple-ios)..."
(
  cd "$LIB_DIR"
  rustup target add aarch64-apple-ios
  cargo build --release --target aarch64-apple-ios
)
log "✅ iOS device-only build completed."
