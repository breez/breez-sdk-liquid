package com.breez.breez_sdk_liquid

import io.flutter.embedding.engine.plugins.FlutterPlugin

/**
 * BreezSDKLiquidPlugin
 *
 * This is a minimal plugin stub that satisfies Flutter's plugin registration
 * requirements. The actual SDK functionality is provided via FFI bindings
 * through flutter_rust_bridge.
 *
 * Logging: SDK logs flow directly through flutter_rust_bridge's log integration.
 * Use `breezLogStream()` from the Dart API to subscribe to log events.
 */
class BreezSDKLiquidPlugin : FlutterPlugin {
    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        // No-op: FFI plugin - all functionality provided via Rust bindings
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        // No-op
    }
}
