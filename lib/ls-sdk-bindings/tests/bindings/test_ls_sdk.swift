import ls_sdk

let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
let sdk = try ls_sdk.`init`(mnemonic: mnemonic, dataDir: nil, network: .liquidTestnet);
let nodeInfo = try sdk.getInfo(withScan: false);
print(nodeInfo);
assert(nodeInfo.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494", "nodeInfo.pubkey");