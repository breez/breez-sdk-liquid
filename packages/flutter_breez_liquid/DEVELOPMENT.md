# Development guide - Flutter package

Flutter plugin for Breez SDK - Nodeless (*Liquid Implementation*) is structured as follows:

- **Swift & Android bindings**: Provide sources and frameworks for the Notification Service Extension, which is bundled with the base plugin
- **Rust directory**: Contains a wrapper crate for the core SDK and sdk-common, along with flutter_rust_bridge bindings for Dart/Flutter integration

When developing, you can work with locally built versions of all components instead of relying on published packages.

All the following commands can be run in the `packages/flutter_breez_liquid` directory.

## Prerequisites
### Required
This guide assumes you have the following tools installed on any development machines:
- [Flutter](https://docs.flutter.dev/get-started/install)
- [rustup](https://rustup.rs)
- [just](https://github.com/casey/just?tab=readme-ov-file#installation) command runner.

## Building and setup

To install all requirements and build everything, simply run:
```bash
just setup build
```

This single command will:
- Install all necessary dependencies
- Build the Rust bindings
- Generate Flutter/Dart bindings using flutter_rust_bridge
- Set up the development environment for both iOS and Android

The command generates the following artifacts:

- **iOS/macOS**
  - `ios/Frameworks/breez_sdk_liquidFFI.xcframework` - Native iOS framework
  - `macos/Frameworks/breez_sdk_liquidFFI.xcframework` - Native macOS framework
  - `ios/Sources/` and `macos/Sources/` - Swift source bindings
  - `ios/Classes/breez_sdk_liquidFFI.h` - C headers for iOS
  - `macos/Classes/breez_sdk_liquidFFI.h` - C headers for macOS
  
- **Android**
  - `android/src/main/jniLibs/*/` - Native libraries for different architectures (arm64-v8a, armeabi-v7a, x86, x86_64)
  - `android/src/main/kotlin/breez_sdk_liquid/` - Kotlin bindings
  
- **Dart**
  - `lib/src/rust/` - Generated Dart API bindings via flutter_rust_bridge

## Available commands

You can also run individual build steps. To see all available recipes, run `just`:

```bash
just setup                   # Install required dependencies
just check-deps              # Check system dependencies
just build                   # Build UniFFI library and generate bindings
just build-uniffi            # Build UniFFI library for all platforms
just build-uniffi-android    # Build UniFFI library for Android only
just build-uniffi-swift      # Build UniFFI library for iOS/macOS only
just gen                     # Generate Dart bindings and iOS frameworks
just codegen                 # Generate Dart bindings only
just version                 # Update version across all platform files
just clean                   # Clean all build artifacts
```

## Development workflow

For iterative development:

1. Make changes to the Rust core or bindings
2. Rebuild with `just build`
3. Test changes on [Misty Breez](https://github.com/breez/misty-breez) app with `flutter run`

For faster iteration, you can build only the platform you're testing:
- `just build-uniffi-android && just codegen` for Android development
- `just build-uniffi-swift && just gen` for iOS/macOS development

## Troubleshooting

**Build issues:**
- Ensure ANDROID_NDK_HOME is set correctly
- Make sure you have the latest Flutter SDK installed
- Try `flutter clean` on [Misty Breez](https://github.com/breez/misty-breez) root directory if you encounter cache issues

**iOS specific issues:**
- Run `cd ios && pod install` on [Misty Breez](https://github.com/breez/misty-breez) root directory if CocoaPods dependencies aren't updated
- Clean Xcode build folder if encountering linking issues
- Re-add Notification Service Extension(NSE) files to Compile Sources on Build Phase of your NSE target

**Android specific issues:**
- Ensure Android SDK and NDK are properly configured
- Try `flutter clean && ./gradlew clean` and rebuild if encountering JNI issues

**flutter_rust_bridge specific issues:**
- [flutter_rust_bridge > Troubleshooting](https://cjycode.com/flutter_rust_bridge/manual/troubleshooting)