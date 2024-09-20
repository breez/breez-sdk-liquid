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

const libName = 'libbreez_sdk_liquid_bindings.so';
const iosLibName = "breez_sdk_liquidFFI";

class UnsupportedPlatform implements Exception {
  UnsupportedPlatform(String s);
}

Future<void> initialize({ExternalLibrary? dylib}) {
  if (dylib == null) {
    if (Platform.isAndroid || Platform.isLinux) {
      // On Linux the lib needs to be in LD_LIBRARY_PATH or working directory
      dylib = ExternalLibrary.open(libName);
    } else if (Platform.isIOS || Platform.isMacOS) {
      try {
        dylib = ExternalLibrary.open("$iosLibName.framework/$iosLibName");
      } catch (e) {
        dylib = ExternalLibrary.process(iKnowHowToUseIt: true);
      }
    } else {
      throw UnsupportedPlatform('${Platform.operatingSystem} is not yet supported!');
    }
  }
  return RustLib.init(externalLibrary: dylib);
}
