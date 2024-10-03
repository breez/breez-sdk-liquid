#!/bin/bash
cd ../..
make init build-ios-framework
# Clean existing framework & Sources dirs
rm -rf ../../packages/flutter/ios/Frameworks/breez_sdk_liquidFFI.xcframework ../../packages/flutter/ios/Sources
rm -rf ../../packages/flutter/ios/Frameworks/breez_sdk_liquidFFI.xcframework ../../packages/flutter/macos/Sources
# Copy newly built framework & Sources dirs
cp -r langs/swift/breez_sdk_liquidFFI.xcframework ../../packages/flutter/ios/Frameworks/breez_sdk_liquidFFI.xcframework
cp -r langs/swift/breez_sdk_liquidFFI.xcframework ../../packages/flutter/macos/Frameworks/breez_sdk_liquidFFI.xcframework
cp -r langs/swift/Sources ../../packages/flutter/ios/Sources
cp -r langs/swift/Sources ../../packages/flutter/macos/Sources