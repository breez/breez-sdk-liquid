import breez_sdk_liquid, { BindingLiquidSdk, connect, defaultConfig, LiquidNetwork } from "../generated/breez_sdk_liquid";
import breez_sdk_liquid_nwc, { BindingNwcService } from "../generated/breez_sdk_liquid_nwc";

breez_sdk_liquid.initialize();
breez_sdk_liquid_nwc.initialize();

const nwcService = new BindingNwcService({ secretKeyHex: undefined, relayUrls: undefined });

const config = defaultConfig(LiquidNetwork.Mainnet, undefined)
const _sdk = connect({
  config,
  seed: new Array(32),
  mnemonic: undefined,
  passphrase: undefined,
}, [nwcService])


console.log(nwcService.addConnectionString("test"))
