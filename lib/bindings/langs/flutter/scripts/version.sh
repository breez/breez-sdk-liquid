#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../../../.."
TAG_NAME=`awk '/^version: /{print $2}' $ROOT/packages/flutter/pubspec.yaml`

# Update Flutter plugin to use the same Dart plugin version
# Ensure the ref exists in the breez-sdk-liquid-dart repository before updating
if git ls-remote --exit-code --tags https://github.com/breez/breez-sdk-liquid-dart "v$TAG_NAME"; then
    # Update only breez_liquid's ref in pubspec.yaml
    sed -i.bak -E "/breez_liquid:/,/ref:/s|(ref: ).*|\1$TAG_NAME|" "$ROOT/packages/flutter/pubspec.yaml"
    rm "$ROOT/packages/flutter/pubspec.yaml.bak"
else
    echo "Error: Git ref $TAG_NAME does not exist in breez-sdk-liquid-dart!"
    exit 1
fi

# iOS & macOS
APPLE_HEADER="version = '$TAG_NAME' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/ios/flutter_breez_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/ios/flutter_breez_liquid.podspec.production
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/macos/flutter_breez_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/macos/flutter_breez_liquid.podspec.production
rm $ROOT/packages/flutter/macos/*.bak $ROOT/packages/flutter/ios/*.bak

# Android (Gradle)
GRADLE_HEADER="version '$TAG_NAME' \/\/ generated; do not edit"
sed -i.bak "1 s/.*/$GRADLE_HEADER/" $ROOT/packages/flutter/android/build.gradle
sed -i.bak "1 s/.*/$GRADLE_HEADER/" $ROOT/packages/flutter/android/build.gradle.production
rm $ROOT/packages/flutter/android/*.bak

# Commit changes
git add $ROOT/packages/flutter/