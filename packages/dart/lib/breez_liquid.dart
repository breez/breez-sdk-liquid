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

Future<void> initialize({ExternalLibrary? dylib}) async {
  try {
    dylib ??= await _loadPlatformSpecificLibrary();
    _log.info(
      dylib != null
          ? 'Initializing RustLib with the provided library'
          : 'Initializing RustLib with the default ExternalLibrary',
    );
    await RustLib.init(externalLibrary: dylib);
  } catch (e, stacktrace) {
    _log.severe('Initialization failed: $e', e, stacktrace);
    rethrow;
  }
}

Future<ExternalLibrary?> _loadPlatformSpecificLibrary() async {
  switch (Platform.operatingSystem) {
    case 'android':
    case 'linux':
      return _loadAndroidLinuxLibrary();
    case 'ios':
    case 'macos':
      return _loadIOSMacOSLibrary();
    default:
      _log.severe('${Platform.operatingSystem} is not yet supported!');
      throw UnsupportedPlatform('${Platform.operatingSystem} is not supported!');
  }
}

Future<ExternalLibrary?> _loadAndroidLinuxLibrary() async {
  try {
    _log.info('Attempting to load $libName for Android/Linux');
    return ExternalLibrary.open(libName);
  } catch (e) {
    _log.warning('Failed to load $libName for Android/Linux: $e');
    return null;
  }
}

Future<ExternalLibrary?> _loadIOSMacOSLibrary() async {
  try {
    _log.info('Attempting to use iOS/MacOS framework');
    return ExternalLibrary.open("$iosLibName.framework/$iosLibName");
  } catch (e) {
    _log.warning('iOS/MacOS framework not found, attempting fallback to ExternalLibrary.process: $e');
    try {
      return ExternalLibrary.process(iKnowHowToUseIt: true);
    } catch (e) {
      _log.warning('Failed to initialize ExternalLibrary.process for iOS/MacOS: $e');
      return null;
    }
  }
}
