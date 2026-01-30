package com.breez.breez_sdk_liquid

import io.flutter.embedding.engine.plugins.FlutterPlugin

/* BreezSDKPlugin */
class BreezSDKLiquidPlugin : FlutterPlugin {
    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        // No-op: FFI plugin - all functionality provided via Rust bindings
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        // No-op
    }
}
