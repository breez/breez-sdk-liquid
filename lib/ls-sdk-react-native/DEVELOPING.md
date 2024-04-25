# Setting up a development environment

The Liquid Swap SDK React Native plugin consumes the underlying Liquid Swap SDK from the following sources:

-   For iOS: The Liquid Swap SDK Swift bindings are integrated via CocoaPods.
-   For Android: The Liquid Swap SDK Android bindings are integrated via Jitpack.

When developing, it can be useful to work with a locally built version of the Liquid Swap SDK instead of relying on what is published already on CocoaPods / Jitpack.
To do this, you first need to build the Liquid Swap SDK bindings locally and then point the plugin to make use of the locally built Liquid Swap SDK bindings.

All the following commands can be run in the `lib/ls-sdk-react-native` directory.

## Prerequisites

Set the ANDROID_NDK_HOME env variable to your SDK home directory:
```
export ANDROID_NDK_HOME=<your android ndk directory>
```

To lint the result of the code generation ktlint, swiftformat and tslint need to be installed:
```bash
brew install kotlin ktlint swiftformat
yarn global add tslint typescript
```

On first usage you will need to run:
```bash
make init
```

## Building the bindings

Then to build and copy the Kotlin, Swift and React Native bindings into the React Native package run:
```bash
make all
```

This will generate the following artifacts:

- iOS
	- `ios/LiquidSwapSDKMapper.swift`
	- `ios/LiquidSwapSDK.m`
	- `ios/LiquidSwapSDK.swift`
	- `ios/bindings-swift/ls_sdkFFI.xcframework`
	- `ios/bindings-swift/Sources/LiquidSwapSDK/LiquidSwapSDK.swift`
- Android
	- `android/src/main/java/com/lssdk/ls_sdk.kt`
	- `android/src/main/java/com/lssdk/LiquidSwapSDKMapper.kt`
	- `android/src/main/java/com/lssdk/LiquidSwapSDKModule.kt`
	- `android/src/main/jniLibs/arm64-v8a/libls_sdk_bindings.so`
	- `android/src/main/jniLibs/armeabi-v7a/libls_sdk_bindings.so`
	- `android/src/main/jniLibs/x86/libls_sdk_bindings.so`
	- `android/src/main/jniLibs/x86_64/libls_sdk_bindings.so`
- Typescript
	- `src/index.ts`

### Building for one platform only

You can also build for Android or iOS only, in that case run:
```bash
make android react-native
```
or
```bash
make ios react-native
```

## Using the locally built bindings

To use the locally built bindings instead of integrating them remotely, make the following changes:

- For iOS:
	- Rename the podspec files in `lib/ls-sdk-react-native/`:
		- Rename `ls_sdk.podspec` to `ls_sdk.podspec.prod`
		- Rename `LiquidSwapSDK.podspec.dev` to `LiquidSwapSDK.podspec`
- For Android:
	- Comment out the following line from the dependencies section in `lib/ls-sdk-react-native/android/build.gradle`:
		- `implementation("com.github.breez:breez-sdk-liquid:${getVersionFromNpmPackage()}") { exclude group:"net.java.dev.jna" }`

Reinstall the dependencies in the example project and run it.
It will now use the locally built bindings.

## Testing with the example app

To test locally built bindings in the example app, the npm dependencies need to be updated to use the local package.
In `lib/ls-sdk-react-native/example/package.json` replace the current version with `file:../`:
```json
    "@breeztech/react-native-liquid-swap-sdk": "file:../",
```

Run the npm/yarn install to download dependences for both the react-native-liquid-swap-sdk package and the example app:
```bash
yarn bootstrap
```

Finally in the `lib/ls-sdk-react-native/example/` directory start either the iOS or Android app:
```bash
yarn android
```
or for iOS:
```bash
yarn ios
```

## Troubleshooting

In case you get an error like: 
> java.lang.RuntimeException: Unable to load script. Make sure you're either running Metro (run 'npx react-native start') or that your bundle 'index.android.bundle' is packaged correctly for release. 

Then manually run `npx react-native start` in the example directory and reload the app.
