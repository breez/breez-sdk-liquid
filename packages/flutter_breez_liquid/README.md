# flutter_breez_liquid

[![pub package](https://img.shields.io/pub/v/breez_sdk_liquid.svg)](https://pub.dev/packages/breez_sdk_liquid)

## Table of contents
- [Platform Support](#platform-support)
- [Requirements](#requirements)
- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [License](#license)

## Platform Support

| Android | iOS | MacOS | Web | Linux | Windows |
| :-----: | :-: | :---: | :-: | :---: | :----: |
|   ✅    | ✅  |  ✅   | ❎  |  ✅(Partial)   |   ❎   |

## Requirements

- Flutter >=3.3.0
- Dart ^3.8.1
- iOS >=13.0
- MacOS >=15.0
- Android `compileSdkVersion` 36
- Java 17
- Android Gradle Plugin >=7.3.0

## Description

Flutter bindings for the [Breez SDK - Nodeless (*Liquid Implementation*)](https://sdk-doc-liquid.breez.technology/)

## Installation
To use this plugin, add `flutter_breez_liquid` as a [dependency in your pubspec.yaml file](https://flutter.dev/docs/development/platform-integration/platform-channels).

## Usage

To start using this package first import it in your Dart file.

```dart
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';
```
Call `await FlutterBreezLiquid.init();` to initialize Breez SDK - Nodeless (*Liquid Implementation*), preferably on `main.dart`:

```dart
import 'package:flutter_breez_liquid/flutter_breez_liquid.dart';

Future<void> main() async {
  await FlutterBreezLiquid.init();
  ...
}
```

Please refer to Dart examples on our official documentation for more information on features & capabilities of the Breez SDK - Nodeless (*Liquid Implementation*).

## Documentation

- [Official Breez SDK - Nodeless (*Liquid Implementation*) documentation](https://sdk-doc-liquid.breez.technology/)

## License

Dual-licensed under Apache 2.0 and MIT.