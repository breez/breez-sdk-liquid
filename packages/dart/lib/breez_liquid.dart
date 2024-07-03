/// Dart bindings for the Breez Liquid SDK
library;

export 'src/bindings.dart';
export 'src/model.dart';
export 'src/error.dart';
export 'src/bindings/duplicates.dart';

import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'src/frb_generated.dart';

typedef BreezLiquid = RustLibApi;
typedef BreezLiquidImpl = RustLibApiImpl;

const libName = 'breez_liquid_sdk';

Future<void> initialize({ExternalLibrary? dylib}) {
  if (dylib == null && (Platform.isIOS || Platform.isMacOS)) {
    try {
      dylib = ExternalLibrary.open("$libName.framework/$libName");
    } catch (e) {
      dylib = ExternalLibrary.process(iKnowHowToUseIt: true);
    }
  }
  return RustLib.init(externalLibrary: dylib);
}
