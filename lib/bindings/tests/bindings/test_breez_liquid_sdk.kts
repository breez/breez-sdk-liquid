
class SDKListener: breez_liquid_sdk.EventListener {
    override fun onEvent(e: breez_liquid_sdk.LiquidSdkEvent) {
        println(e.toString());
    }
}

try {
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    var config = breez_liquid_sdk.defaultConfig(breez_liquid_sdk.LiquidSdkNetwork.TESTNET)
    var connectRequest = breez_liquid_sdk.ConnectRequest(config, mnemonic)
    var sdk = breez_liquid_sdk.connect(connectRequest)

    var listenerId = sdk.addEventListener(SDKListener())

    var nodeInfo = sdk.getInfo()

    sdk.removeEventListener(listenerId)

    println("$nodeInfo")
    assert(nodeInfo.pubkey.equals("03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"))
} catch (ex: Exception) {
    throw RuntimeException(ex.toString())
}