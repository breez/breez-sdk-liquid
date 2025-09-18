package com.breez.breez_sdk_liquid

import breez_sdk_liquid.setLogger
import kotlinx.coroutines.CoroutineScope

object SdkLogInitializer {
    private var listener: SdkLogListener? = null

    fun initializeListener(): SdkLogListener {
        if (listener == null) {
            try {
                listener = SdkLogListener()
                setLogger(listener!!)
            } catch (e: Throwable) {
                e.printStackTrace()
                listener = null
                throw e
            }
        }
        return listener!!
    }

    fun unsubscribeListener(scope: CoroutineScope) {
        listener?.unsubscribe(scope)
    }
}