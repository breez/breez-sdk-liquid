import breez_liquid_sdk

class SDKListener: EventListener {
    func onEvent(e: LiquidSdkEvent) {
        print("Received event ", e);
    }
}

let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
let config = breez_liquid_sdk.defaultConfig(network: .testnet);
let connectRequest = breez_liquid_sdk.ConnectRequest(config: config, mnemonic: mnemonic);
let sdk = try breez_liquid_sdk.connect(req: connectRequest);

let listenerId = try sdk.addEventListener(listener: SDKListener());

let getInfoReq = breez_liquid_sdk.GetInfoRequest(withScan: false);
let nodeInfo = try sdk.getInfo(req: getInfoReq);

try sdk.removeEventListener(id: listenerId);

print(nodeInfo);
assert(nodeInfo.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494", "nodeInfo.pubkey");