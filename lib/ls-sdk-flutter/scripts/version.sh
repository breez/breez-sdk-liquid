#!/bin/bash

CURR_VERSION=breez_liquid-v`awk '/^version: /{print $2}' packages/breez_liquid/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_breez_liquid/ios/flutter_breez_liquid.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_breez_liquid/macos/flutter_breez_liquid.podspec
rm packages/flutter_breez_liquid/macos/*.bak packages/flutter_breez_liquid/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/flutter_breez_liquid/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/flutter_breez_liquid/$CMAKE_PLATFORM/*.bak
done

git add packages/flutter_breez_liquid/