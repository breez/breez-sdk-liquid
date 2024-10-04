import breez_sdk_liquid

class SDKListener: EventListener {
    func onEvent(e: SdkEvent) {
        print("Received event ", e);
    }
}

let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
let testPubkey = "MIIBazCCAR2gAwIBAgIHPWTUED8FpTAFBgMrZXAwEDEOMAwGA1UEAxMFQnJlZXowHhcNMjQxMDA0MjMxMjM0WhcNMzQxMDAyMjMxMjM0WjAlMQ4wDAYDVQQKEwVCcmVlejETMBEGA1UEAxMKQnJlZXotVGVzdDAqMAUGAytlcAMhANCD9cvfIDwcoiDKKYdT9BunHLS2/OuKzV8NS0SzqV13o4GAMH4wDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0OBBYEFNo5o+5ea0sNMlW/75VgGJCv2AcJMB8GA1UdIwQYMBaAFN6q1pJW843ndJIW/Ey2ILJrKJhrMB4GA1UdEQQXMBWBE2h5ZHJhX3lzZUBwcm90b24ubWUwBQYDK2VwA0EAILwfdmryGRMoYX5HotHXci5Z5yTG3ugwdNURG3puXSG2eSXgbmwdzGDJBFTvNsEx9sP0L7nyOaKr3W1Yzf/ECQ==";
let config = breez_sdk_liquid.defaultConfig(network: .testnet, breezApiKey: testPubkey);
let connectRequest = breez_sdk_liquid.ConnectRequest(config: config, mnemonic: mnemonic);
let sdk = try breez_sdk_liquid.connect(req: connectRequest);

let listenerId = try sdk.addEventListener(listener: SDKListener());

let nodeInfo = try sdk.getInfo();

try sdk.removeEventListener(id: listenerId);

print(nodeInfo);
assert(nodeInfo.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494", "nodeInfo.pubkey");
