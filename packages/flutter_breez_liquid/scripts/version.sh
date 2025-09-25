#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../.."

log() {
  echo -e "\033[1;34m[INFO]\033[0m $*"
}

# Get version from workspace Cargo.toml
TAG_NAME=$(awk -F' = ' '/^version =/{gsub(/"/,"",$2); print $2}' "$ROOT/lib/Cargo.toml")

log "Detected version: $TAG_NAME"

# Update Flutter plugin pubspec.yaml ref
log "Updating pubspec.yaml ref..."
sed -i.bak -E "/flutter_breez_liquid:/,/ref:/s|(ref: ).*|\1v$TAG_NAME|" \
  "$ROOT/packages/flutter_breez_liquid/pubspec.yaml"
rm "$ROOT/packages/flutter_breez_liquid/pubspec.yaml.bak"

# iOS & macOS podspec
APPLE_HEADER="version = '$TAG_NAME' # generated; do not edit"
for platform in ios macos; do
  log "Updating $platform podspec..."
  sed -i.bak "1s|.*|$APPLE_HEADER|" \
    "$ROOT/packages/flutter_breez_liquid/$platform/flutter_breez_liquid.podspec"
  rm "$ROOT/packages/flutter_breez_liquid/$platform/"*.bak
done

# Android Gradle
GRADLE_HEADER="version '$TAG_NAME' // generated; do not edit"
log "Updating Android Gradle build.gradle..."
sed -i.bak "1s|.*|$GRADLE_HEADER|" \
  "$ROOT/packages/flutter_breez_liquid/android/build.gradle"
rm "$ROOT/packages/flutter_breez_liquid/android/"*.bak

# Plugin Rust crate Cargo.toml
log "Updating plugin Cargo.toml..."
sed -i.bak -E "s/^version = \".*\"/version = \"$TAG_NAME\"/" \
  "$ROOT/packages/flutter_breez_liquid/rust/Cargo.toml"
rm "$ROOT/packages/flutter_breez_liquid/rust/Cargo.toml.bak"

# Stage changes for commit
log "Staging updated files for git..."
git add "$ROOT/packages/flutter_breez_liquid/"

log "✅ Version bump to $TAG_NAME completed successfully."
