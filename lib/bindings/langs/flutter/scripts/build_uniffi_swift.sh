#!/bin/bash
cd ../..
make init bindings-swift
rm -rf ../../packages/flutter/ios/Frameworks/breez_sdk_liquidFFI.xcframework ../../packages/flutter/ios/Sources
cp -r langs/swift/breez_sdk_liquidFFI.xcframework ../../packages/flutter/ios/Frameworks/breez_sdk_liquidFFI.xcframework
cp -r langs/swift/Sources ../../packages/flutter/ios/Sources
