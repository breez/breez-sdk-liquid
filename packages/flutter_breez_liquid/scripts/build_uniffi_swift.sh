#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../.."
LIB_DIR="$ROOT/lib/bindings"

log() {
  echo -e "\033[1;34m[INFO]\033[0m $*"
}

log "Building iOS universal and macOS (Darwin) universal libraries..."
(
  cd "$LIB_DIR"
  make build-ios-universal build-darwin-universal
)
log "✅ Universal iOS/macOS build completed successfully."
