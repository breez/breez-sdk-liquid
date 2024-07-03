# Breez Liquid SDK plugin

[![pub package](https://img.shields.io/pub/v/breez_liquid_sdk.svg)](https://pub.dev/packages/breez_liquid_sdk)

## Table of contents
- [Platform Support](#platform-support)
- [Requirements](#requirements)
- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)

## Platform Support

| Android | iOS | MacOS | Web | Linux | Windows |
| :-----: | :-: | :---: | :-: | :---: | :----: |
|   ✅    | ❎  |  ❎   | ❎  |  ❎   |   ❎   |

## Requirements

- Flutter >=3.10.0
- Dart >=3.4.0 <4.0.0
- iOS >=12.0
- MacOS >=10.11
- Android `compileSDK` 31
- Java 1.8
- Android Gradle Plugin >=7.1.2
- Gradle wrapper >=7.4

## Description

This is a Flutter package that wraps the Dart bindings of [Breez Liquid SDK](https://github.com/breez/breez-liquid-sdk?tab=readme-ov-file#readme).

## Installation
To use this plugin, add `breez_liquid_sdk` as a [dependency in your pubspec.yaml file](https://flutter.dev/docs/development/platform-integration/platform-channels).

## Usage

To start using this package first import it in your Dart file.

```dart
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
```
Call `initialize()` to initialize Breez Liquid SDK, preferably on `main.dart`:

```dart
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart' as liquid_sdk;

void main() async {
    // Initialize library
    await liquid_sdk.initialize();
}
```

Please refer to Dart examples on Breez Liquid SDK documentation for more information on features & capabilities of the Breez Liquid SDK.

## Documentation

- [Official Breez Liquid SDK documentation](https://sdk-doc-liquid.breez.technology/)