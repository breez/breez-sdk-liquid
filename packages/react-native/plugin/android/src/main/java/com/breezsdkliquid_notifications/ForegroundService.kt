package com.breezsdkliquid_notifications

import breez_sdk_liquid.ConnectRequest
import breez_sdk_liquid.defaultConfig
import breez_sdk_liquid.LiquidNetwork
import breez_sdk_liquid_notification.ForegroundService
import breez_sdk_liquid_notification.NotificationHelper.Companion.registerNotificationChannels
import kotlinx.coroutines.runBlocking

class ForegroundService: ForegroundService() {

    override fun onCreate() {
        super.onCreate()
        // Register the default notification channels
        registerNotificationChannels(applicationContext)
    }

    override fun getConnectRequest(): ConnectRequest {
        val secureStore = SecureStoreHelper(applicationContext)
        val apiKey = BuildConfig.BREEZ_API_KEY
	val mnemonicKey = BuildConfig.MNEMONIC_KEY_NAME

        val config = defaultConfig(LiquidNetwork.MAINNET, apiKey)
        config.workingDir = "${applicationContext.filesDir}/breezSdkLiquid"
        val mnemonic: String

        runBlocking {
            mnemonic = secureStore.getItem(mnemonicKey) ?: ""
        }

        return ConnectRequest(config, mnemonic)
    }
}
