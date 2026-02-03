#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# Device-only iOS framework build script
# Creates xcframework using device binary as placeholder for sim/macOS
#
# WARNING: The resulting framework will only work on physical iOS devices.
# Simulator and macOS builds will fail at runtime.
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../.."
LIB_DIR="$ROOT/lib/bindings"
TARGET_DIR="$ROOT/lib/target"
BIN_NAME="libbreez_sdk_liquid_bindings"
IOS_DIR="$ROOT/packages/flutter_breez_liquid/ios"
MACOS_DIR="$ROOT/packages/flutter_breez_liquid/macos"

log() {
  echo -e "\033[1;34m[INFO]\033[0m $*"
}

log "Building iOS framework (device-only)..."

# Generate Swift bindings
cd "$LIB_DIR"
mkdir -p langs/swift/Sources/BreezSDKLiquid
cargo run --bin uniffi-bindgen generate src/breez_sdk_liquid.udl --no-format --language swift -o langs/swift/Sources/BreezSDKLiquid
mv langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquid.swift langs/swift/Sources/BreezSDKLiquid/BreezSDKLiquid.swift

# Copy headers to all framework targets
log "Copying headers..."
cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64/breez_sdk_liquidFFI.framework/Headers
cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdk_liquidFFI.framework/Headers
cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/macos-arm64_x86_64/breez_sdk_liquidFFI.framework/Headers

# Copy device binary to device framework
log "Copying device binary..."
cp "$TARGET_DIR/aarch64-apple-ios/release/$BIN_NAME.a" langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI

# Use device binary as placeholder for simulator/macOS (won't run but framework structure is valid for build)
log "Copying device binary as placeholder for simulator/macOS..."
cp "$TARGET_DIR/aarch64-apple-ios/release/$BIN_NAME.a" langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI
cp "$TARGET_DIR/aarch64-apple-ios/release/$BIN_NAME.a" langs/swift/breez_sdk_liquidFFI.xcframework/macos-arm64_x86_64/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI

# Copy header for Flutter
mkdir -p langs/flutter/breez_sdk_liquidFFI/include
cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/flutter/breez_sdk_liquidFFI/include/

FRAMEWORK_NAME="breez_sdk_liquidFFI.xcframework"
SOURCES_DIR="Sources"

# Clean existing frameworks and sources
log "Cleaning old iOS/macOS frameworks and sources..."
rm -rf "$IOS_DIR/Frameworks/$FRAMEWORK_NAME" "$IOS_DIR/$SOURCES_DIR"
rm -rf "$MACOS_DIR/Frameworks/$FRAMEWORK_NAME" "$MACOS_DIR/$SOURCES_DIR"

# Copy newly built frameworks
log "Copying new iOS framework..."
cp -r "$LIB_DIR/langs/swift/$FRAMEWORK_NAME" "$IOS_DIR/Frameworks/"

log "Copying new macOS framework..."
cp -r "$LIB_DIR/langs/swift/$FRAMEWORK_NAME" "$MACOS_DIR/Frameworks/"

# Copy Swift sources (before cleanup)
log "Copying Swift sources to iOS..."
cp -r "$LIB_DIR/langs/swift/$SOURCES_DIR" "$IOS_DIR/"

log "Copying Swift sources to macOS..."
cp -r "$LIB_DIR/langs/swift/$SOURCES_DIR" "$MACOS_DIR/"

# Cleanup generated files that aren't needed (after copying Sources)
rm langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h
rm langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.modulemap

# Copy headers to Flutter plugin Classes
log "Copying headers to Flutter plugin directories..."
cp "$LIB_DIR/langs/flutter/breez_sdk_liquidFFI/include/breez_sdk_liquidFFI.h" "$IOS_DIR/Classes/breez_sdk_liquidFFI.h"
cp "$LIB_DIR/langs/flutter/breez_sdk_liquidFFI/include/breez_sdk_liquidFFI.h" "$MACOS_DIR/Classes/breez_sdk_liquidFFI.h"

log "✅ iOS framework (device-only) completed."
