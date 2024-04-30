try {
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    var sdk = breez_liquid_sdk.connect(mnemonic, null, breez_liquid_sdk.Network.LIQUID_TESTNET)
    var nodeInfo = sdk.getInfo(false)
    println("$nodeInfo")
    assert(nodeInfo.pubkey.equals("03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"))
} catch (ex: Exception) {
    throw RuntimeException(ex.toString())
}