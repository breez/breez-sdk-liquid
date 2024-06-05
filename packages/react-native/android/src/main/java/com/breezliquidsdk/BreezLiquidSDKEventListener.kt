package com.breezliquidsdk

import breez_liquid_sdk.EventListener
import breez_liquid_sdk.LiquidSdkEvent
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezLiquidSDKEventListener(
    private val emitter: RCTDeviceEventEmitter,
) : EventListener {
    private var id: String? = null

    fun setId(id: String) {
        this.id = id
    }

    override fun onEvent(e: LiquidSdkEvent) {
        this.id?.let {
            emitter.emit("event-$it", readableMapOf(e))
        }
    }
}
