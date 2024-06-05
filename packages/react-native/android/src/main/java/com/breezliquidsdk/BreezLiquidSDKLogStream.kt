package com.breezliquidsdk

import breez_liquid_sdk.LogEntry
import breez_liquid_sdk.Logger
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezLiquidSDKLogger(
    private val emitter: RCTDeviceEventEmitter,
) : Logger {
    companion object {
        var emitterName = "breezLiquidSdkLog"
    }

    override fun log(l: LogEntry) {
        emitter.emit(emitterName, readableMapOf(l))
    }
}
