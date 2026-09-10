package com.breezsdkliquid_notifications

import breez_sdk_liquid.ConnectRequest
import breez_sdk_liquid.defaultConfig
import breez_sdk_liquid.LiquidNetwork
import breez_sdk_liquid_notification.ForegroundService
import breez_sdk_liquid_notification.NotificationHelper.Companion.registerNotificationChannels

class ForegroundService : ForegroundService() {
    override fun onCreate() {
        super.onCreate()
        // Register the default notification channels
        registerNotificationChannels(applicationContext)
    }

    override fun getConnectRequest(): ConnectRequest? {
        val mnemonic = BreezMnemonicStore(applicationContext, BuildConfig.MNEMONIC_KEY_NAME).get()

        if (mnemonic.isNullOrEmpty()) {
            // Nothing has been stored yet, or the key was invalidated. Returning null makes the
            // base class skip the connection and shut down instead of trying to connect with an
            // empty mnemonic.
            logger.log(TAG, "No mnemonic available, skipping connection", "WARN")
            return null
        }

        val config = defaultConfig(LiquidNetwork.MAINNET, BuildConfig.BREEZ_API_KEY)
        config.workingDir = "${applicationContext.filesDir}/breezSdkLiquid"

        return ConnectRequest(config, mnemonic)
    }

    companion object {
        private const val TAG = "ForegroundService"
    }
}
