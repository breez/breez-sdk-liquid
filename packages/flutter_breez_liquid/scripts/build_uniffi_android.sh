#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../.."
LIB_DIR="$ROOT/lib/bindings"
PLUGIN_DIR="$ROOT/packages/flutter_breez_liquid/android"
JNI_DIR="$PLUGIN_DIR/src/main/jniLibs"
SRC_DIR="$PLUGIN_DIR/src/main/kotlin"

ARCHS=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

log() {
  echo -e "\033[1;34m[INFO]\033[0m $*"
}

# Init Rust/FFI build
if [[ ${SKIP_BUILD:-0} -ne 1 ]]; then
  log "Initializing Rust/FFI build..."
  (
    cd "$LIB_DIR"
    make android
  )
fi

# Clean existing Kotlin bindings
log "Cleaning old Kotlin bindings..."
rm -rf "$SRC_DIR"/breez_sdk_liquid*

# Copy JNI libs
for arch in "${ARCHS[@]}"; do
  log "Copying JNI libs for $arch..."
  mkdir -p "$JNI_DIR/$arch"
  cp "$LIB_DIR/ffi/kotlin/jniLibs/$arch/"*.so "$JNI_DIR/$arch/"
done

# Copy Kotlin sources
log "Copying Kotlin sources..."
cp -r "$LIB_DIR/langs/android/lib/src/main/kotlin" "$PLUGIN_DIR/src/main/"
cp -r "$LIB_DIR/ffi/kotlin/main/kotlin/breez_sdk_liquid" "$SRC_DIR/"

log "✅ Android plugin setup completed successfully."
