#!/bin/bash

CURR_VERSION=breez_liquid-v`awk '/^version: /{print $2}' ../../../packages/flutter/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" ../../../packages/flutter/ios/flutter_breez_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" ../../../packages/flutter/macos/flutter_breez_liquid.podspec
rm ../../../packages/flutter/macos/*.bak ../../../packages/flutter/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" ../../../packages/flutter/$CMAKE_PLATFORM/CMakeLists.txt
    rm ../../../packages/flutter/$CMAKE_PLATFORM/*.bak
done

git add ../../../packages/flutter/