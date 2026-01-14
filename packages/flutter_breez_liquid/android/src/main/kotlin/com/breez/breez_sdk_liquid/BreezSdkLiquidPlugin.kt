package com.breez.breez_sdk_liquid

import android.util.Log
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.plugin.common.EventChannel
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result

/** BreezSDKLiquidPlugin */
class BreezSDKLiquidPlugin : FlutterPlugin, MethodCallHandler, EventChannel.StreamHandler {
    companion object {
        private const val TAG = "BreezSDKLiquidPlugin"
    }

    private lateinit var channel: MethodChannel
    private var eventChannel: EventChannel? = null
    private var eventSink: EventChannel.EventSink? = null

    private val logReceiver = object : android.content.BroadcastReceiver() {
        override fun onReceive(context: android.content.Context, intent: android.content.Intent) {
            if (intent.action == "com.breez.breez_sdk_liquid.LOG") {
                val message = intent.getStringExtra("message")
                val level = intent.getStringExtra("level")
                if (message != null && level != null) {
                    val log = mapOf("level" to level, "line" to message)
                    eventSink?.success(log)
                }
            }
        }
    }

    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        Log.d(TAG, "onAttachedToEngine called")
        
        channel = MethodChannel(flutterPluginBinding.binaryMessenger, "breez_sdk_liquid")
        channel.setMethodCallHandler(this)

        eventChannel = EventChannel(flutterPluginBinding.binaryMessenger, "breez_sdk_liquid_logs")
        eventChannel?.setStreamHandler(this)
        
        val filter = android.content.IntentFilter("com.breez.breez_sdk_liquid.LOG")
        if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.TIRAMISU) {
             flutterPluginBinding.applicationContext.registerReceiver(logReceiver, filter, android.content.Context.RECEIVER_NOT_EXPORTED)
        } else {
             flutterPluginBinding.applicationContext.registerReceiver(logReceiver, filter)
        }
    }

    override fun onMethodCall(call: MethodCall, result: Result) {
        if (call.method == "getPlatformVersion") {
            result.success("Android ${android.os.Build.VERSION.RELEASE}")
        } else {
            result.notImplemented()
        }
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        Log.d(TAG, "onDetachedFromEngine called")
        channel.setMethodCallHandler(null)
        binding.applicationContext.unregisterReceiver(logReceiver)
    }

    override fun onListen(arguments: Any?, events: EventChannel.EventSink?) {
        Log.d(TAG, "onListen called")
        eventSink = events
    }

    override fun onCancel(arguments: Any?) {
        Log.d(TAG, "onCancel called")
        eventSink = null
        eventChannel = null
    }
}