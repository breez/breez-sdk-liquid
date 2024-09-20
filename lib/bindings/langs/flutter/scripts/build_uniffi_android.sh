#!/bin/bash
cd ../..
make init android
rm -r ../../packages/flutter/android/src/main/kotlin/breez_sdk_liquid*
mkdir -p ../../packages/flutter/android/src/main/jniLibs/arm64-v8a
mkdir -p ../../packages/flutter/android/src/main/jniLibs/armeabi-v7a
mkdir -p ../../packages/flutter/android/src/main/jniLibs/x86
mkdir -p ../../packages/flutter/android/src/main/jniLibs/x86_64
cp ffi/kotlin/jniLibs/arm64-v8a/*.so ../../packages/flutter/android/src/main/jniLibs/arm64-v8a/
cp ffi/kotlin/jniLibs/armeabi-v7a/*.so ../../packages/flutter/android/src/main/jniLibs/armeabi-v7a/
cp ffi/kotlin/jniLibs/x86/*.so ../../packages/flutter/android/src/main/jniLibs/x86/
cp ffi/kotlin/jniLibs/x86_64/*.so ../../packages/flutter/android/src/main/jniLibs/x86_64/
cp -r langs/android/lib/src/main/kotlin ../../packages/flutter/android/src/main/
cp -r ffi/kotlin/breez_sdk_liquid ../../packages/flutter/android/src/main/kotlin
