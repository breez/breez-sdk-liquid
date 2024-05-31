package com.breezliquidsdk

import breez_liquid_sdk.LogEntry
import breez_liquid_sdk.LogStream
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezLiquidSDKLogStream(private val emitter: RCTDeviceEventEmitter) : LogStream {
    companion object {
        var emitterName = "breezLiquidSdkLog"
    }

    override fun log(l: LogEntry) {
        emitter.emit(emitterName, readableMapOf(l))
    }
}