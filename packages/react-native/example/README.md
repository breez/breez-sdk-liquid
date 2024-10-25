# Breez Liquid SDK React Native Example

## Prerequisites
You need to set your Breez API key in the relevant places before running the example.
Either Find and Replace `INSERT_YOUR_BREEZ_API_KEY` with your Breez API key, 
or replace `INSERT_YOUR_BREEZ_API_KEY` in the following files:
* `android/app/gradle.properties`
* `ios/Secrets.xconfig`

## Build

Run the npm/yarn install to download dependences:
```bash
yarn
```
or
```bash
npm i
```

## Run

### Android

```bash
yarn android
```

#### Android Troubleshooting

* Before running `yarn android`, stop any `Metro` instances that may be running.
* If you get the error
  ```
  Failed to load dynamic library 'libbreez_sdk_liquid_bindings.so': dlopen failed: cannot locate symbol "__extenddftf2"
  ```
  that is likely due to a dependency issue affecting x86_64 images. Try to run the app on a physical Android device or on a x86 image.

### iOS

```bash
yarn pods
yarn ios
```

## Development

To develop the Breez Liquid SDK alongside the React Native module and example app, please read [DEVELOPING.md](../DEVELOPING.md) for details on how to setup your development environment.
