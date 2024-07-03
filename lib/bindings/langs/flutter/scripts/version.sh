#!/bin/bash
ROOT="../../../.."
TAG_NAME=v`awk '/^version: /{print $2}' $ROOT/packages/flutter/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="tag_name = '$TAG_NAME' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/ios/flutter_breez_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" $ROOT/packages/flutter/macos/flutter_breez_liquid.podspec
rm $ROOT/packages/flutter/macos/*.bak $ROOT/packages/flutter/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(TagName \"$TAG_NAME\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" $ROOT/packages/flutter/$CMAKE_PLATFORM/CMakeLists.txt
    rm $ROOT/packages/flutter/$CMAKE_PLATFORM/*.bak
done

git add $ROOT/packages/flutter/