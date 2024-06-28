# Breez Liquid SDK plugin

[![pub package](https://img.shields.io/pub/v/breez_sdk_liquid.svg)](https://pub.dev/packages/breez_sdk_liquid)

## Table of contents
- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)

## Description

This is a Dart package that wraps the Dart bindings of [Breez Liquid SDK](https://github.com/breez/breez-liquid-sdk?tab=readme-ov-file#readme).

## Installation
To use this plugin, add `breez_sdk_liquid` as a [dependency in your pubspec.yaml file](https://flutter.dev/docs/development/platform-integration/platform-channels).

## Usage

To start using this package first import it in your Dart file.

```dart
import 'package:breez_liquid/breez_liquid.dart';
```
Call `initialize()` to initialize Breez Liquid SDK, preferably on `main.dart`:

```dart
import 'package:breez_liquid/breez_liquid.dart' as liquid_sdk;

void main() async {
    // Initialize library
    await liquid_sdk.initialize();
}
```

Please refer to Dart examples on Breez Liquid SDK documentation for more information on features & capabilities of the Breez Liquid SDK.

## Documentation

- [Official Breez Liquid SDK documentation](https://sdk-doc-liquid.breez.technology/)