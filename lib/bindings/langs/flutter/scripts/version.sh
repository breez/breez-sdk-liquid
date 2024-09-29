#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR/../../../../.."
TAG_NAME=`awk '/^version: /{print $2}' $ROOT/packages/flutter/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="version = '$TAG_NAME' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/ios/breez_sdk_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/ios/flutter_breez_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/ios/flutter_breez_liquid.podspec.production
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/macos/flutter_breez_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/macos/flutter_breez_liquid.podspec.production
rm $ROOT/packages/flutter/macos/*.bak $ROOT/packages/flutter/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(TagName \"v$TAG_NAME\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" $ROOT/packages/flutter/$CMAKE_PLATFORM/CMakeLists.txt
    rm $ROOT/packages/flutter/$CMAKE_PLATFORM/*.bak
done

GRADLE_HEADER="version '$TAG_NAME' \/\/ generated; do not edit"
sed -i.bak "1 s/.*/$GRADLE_HEADER/" $ROOT/packages/flutter/android/build.gradle
sed -i.bak "1 s/.*/$GRADLE_HEADER/" $ROOT/packages/flutter/android/build.gradle.production
rm $ROOT/packages/flutter/android/*.bak

git add $ROOT/packages/flutter/