/// Dart bindings for the Breez Liquid SDK
library;

export 'src/bindings.dart';
export 'src/model.dart';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'src/frb_generated.dart';

typedef BreezLiquid = RustLibApi;
typedef BreezLiquidImpl = RustLibApiImpl;

Future<void> initialize({ExternalLibrary? dylib}) {
  return RustLib.init(externalLibrary: dylib);
}
