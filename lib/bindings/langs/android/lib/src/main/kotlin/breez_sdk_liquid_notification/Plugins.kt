import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.BindingNwcService
import breez_sdk_liquid.NwcConfig

class Plugins {
  public var nwc: BindingNwcService? = null

  constructor()

  fun init(liquidSDK: BindingLiquidSdk, configs: PluginConfigs) {
    configs.nwc?.let { nwcConfig -> this.nwc = liquidSDK.useNwcPlugin(nwcConfig) }
  }

  fun stop() {
    nwc = null;
  }
}

abstract class PluginConfigs {
  public var nwc: NwcConfig? = null
}
