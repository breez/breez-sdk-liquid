/// Dart bindings for the Breez Liquid SDK
library;

export 'src/bindings.dart';
export 'src/model.dart';
export 'src/error.dart';
export 'src/bindings/duplicates.dart';

import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:logging/logging.dart';
import 'src/frb_generated.dart';

typedef BreezLiquid = RustLibApi;
typedef BreezLiquidImpl = RustLibApiImpl;

const libName = 'libbreez_sdk_liquid_bindings.so';
const iosLibName = "breez_sdk_liquidFFI";

final _log = Logger('breez_liquid');

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
        _log.info('Use iOS framework');
        dylib = ExternalLibrary.open("$iosLibName.framework/$iosLibName");
      } catch (e) {
        _log.info('iOS framework not found, use ExternalLibrary.process');
        dylib = ExternalLibrary.process(iKnowHowToUseIt: true);
      }
    } else {
      _log.info('${Platform.operatingSystem} is not yet supported!');
      throw UnsupportedPlatform('${Platform.operatingSystem} is not yet supported!');
    }
  }
  _log.info('Return RustLib');
  return RustLib.init(externalLibrary: dylib);
}
