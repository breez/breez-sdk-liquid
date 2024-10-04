
class SDKListener: breez_sdk_liquid.EventListener {
    override fun onEvent(e: breez_sdk_liquid.SdkEvent) {
        println(e.toString());
    }
}

try {
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    var testPubkey = "MIIBazCCAR2gAwIBAgIHPWTUED8FpTAFBgMrZXAwEDEOMAwGA1UEAxMFQnJlZXowHhcNMjQxMDA0MjMxMjM0WhcNMzQxMDAyMjMxMjM0WjAlMQ4wDAYDVQQKEwVCcmVlejETMBEGA1UEAxMKQnJlZXotVGVzdDAqMAUGAytlcAMhANCD9cvfIDwcoiDKKYdT9BunHLS2/OuKzV8NS0SzqV13o4GAMH4wDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0OBBYEFNo5o+5ea0sNMlW/75VgGJCv2AcJMB8GA1UdIwQYMBaAFN6q1pJW843ndJIW/Ey2ILJrKJhrMB4GA1UdEQQXMBWBE2h5ZHJhX3lzZUBwcm90b24ubWUwBQYDK2VwA0EAILwfdmryGRMoYX5HotHXci5Z5yTG3ugwdNURG3puXSG2eSXgbmwdzGDJBFTvNsEx9sP0L7nyOaKr3W1Yzf/ECQ=="
    var config = breez_sdk_liquid.defaultConfig(breez_sdk_liquid.LiquidNetwork.TESTNET, testPubkey)
    var connectRequest = breez_sdk_liquid.ConnectRequest(config, mnemonic)
    var sdk = breez_sdk_liquid.connect(connectRequest)

    var listenerId = sdk.addEventListener(SDKListener())

    var nodeInfo = sdk.getInfo()

    sdk.removeEventListener(listenerId)

    println("$nodeInfo")
    assert(nodeInfo.pubkey.equals("03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"))
} catch (ex: Exception) {
    throw RuntimeException(ex.toString())
}
