## Build

### Prerequisites
* set the ANDROID_NDK_HOME env variable to your sdk home folder
```
export ANDROID_NDK_HOME=<your android ndk directory>
```

### Building the plugin
On first usage you will need to run:
```
make init
```
Then to generate the React Native code:
```
make react-native
```

### Generated artifacts
* Android
 >* android/src/main/java/com/lssdk/LiquidSwapSDKMapper.kt
 >* android/src/main/java/com/lssdk/LiquidSwapSDKModule.kt
* iOS
 >* ios/LiquidSwapSDKMapper.swift
 >* ios/LiquidSwapSDK.m
 >* ios/LiquidSwapSDK.swift
* Typescript
 >* src/index.ts

### Publish
When publishing, make sure the following are updated:
- Update the version number in `package.json`.
- Set the published version of `@breeztech/react-native-liquid-swap-sdk` in `example/package.json`. 

Then login to npm:
```
npm login --@scope=@breeztech
```
Then publish:
```
npm publish --access public
```