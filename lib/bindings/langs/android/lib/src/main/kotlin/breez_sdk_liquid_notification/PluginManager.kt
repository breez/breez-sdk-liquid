package breez_sdk_liquid_notification

import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.BindingNwcService
import breez_sdk_liquid.NwcConfig

class PluginManager {
  companion object {
    private const val TAG = "PluginManager"
    private var nwc: BindingNwcService? = null

    internal fun nwc(
            liquidSDK: BindingLiquidSdk,
            configs: PluginConfigs,
            logger: ServiceLogger
    ): BindingNwcService? {
      synchronized(this) {
        if (nwc == null) {
          if (configs.nwc == null) return null
          logger.log(TAG, "Starting NWC service", "DEBUG")
          nwc = liquidSDK.useNwcPlugin(configs.nwc)
          logger.log(TAG, "Successfully started NWC service", "DEBUG")
        } else {
          logger.log(TAG, "Already started NWC service", "DEBUG")
        }
        return nwc
      }
    }

    internal fun shutdown(logger: ServiceLogger) {
      synchronized(this) {
        logger.log(TAG, "Shutting down the plugin manager", "DEBUG")
        nwc?.stop()
        nwc = null
        logger.log(TAG, "Successfully shut down the plugin manager", "DEBUG")
      }
    }
  }
}

class PluginConfigs(public val nwc: NwcConfig?) {}
