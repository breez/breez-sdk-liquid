#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# WARNING: UniFFI bindings must be built via:
# ./build_uniffi_swift.sh
# before running this script.
# ============================================================

echo "WARNING: Run ./build_uniffi_swift.sh before this script."

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../.."
LIB_DIR="$ROOT/lib/bindings"
IOS_DIR="$ROOT/packages/flutter_breez_liquid/ios"
MACOS_DIR="$ROOT/packages/flutter_breez_liquid/macos"

FRAMEWORK_NAME="breez_sdk_liquidFFI.xcframework"
SOURCES_DIR="Sources"

log() {
  echo -e "\033[1;34m[INFO]\033[0m $*"
}

# Init Rust/FFI build for iOS framework
log "Building iOS/macOS frameworks..."
(
  cd "$LIB_DIR"
  make build-ios-framework
)

# Clean existing frameworks and sources
log "Cleaning old iOS/macOS frameworks and sources..."
rm -rf "$IOS_DIR/Frameworks/$FRAMEWORK_NAME" "$IOS_DIR/$SOURCES_DIR"
rm -rf "$MACOS_DIR/Frameworks/$FRAMEWORK_NAME" "$MACOS_DIR/$SOURCES_DIR"

# Copy newly built frameworks
log "Copying new iOS framework..."
cp -r "$LIB_DIR/langs/swift/$FRAMEWORK_NAME" "$IOS_DIR/Frameworks/"

log "Copying new macOS framework..."
cp -r "$LIB_DIR/langs/swift/$FRAMEWORK_NAME" "$MACOS_DIR/Frameworks/"

# Copy Swift sources
log "Copying Swift sources to iOS..."
cp -r "$LIB_DIR/langs/swift/$SOURCES_DIR" "$IOS_DIR/"

log "Copying Swift sources to macOS..."
cp -r "$LIB_DIR/langs/swift/$SOURCES_DIR" "$MACOS_DIR/"

log "✅ iOS/macOS plugin setup completed successfully."
