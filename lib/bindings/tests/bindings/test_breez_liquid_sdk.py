import breez_liquid_sdk

def test():
    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    sdk = breez_liquid_sdk.connect(mnemonic, None, breez_liquid_sdk.Network.LIQUID_TESTNET)
    req = breez_liquid_sdk.GetInfoRequest(with_scan = False)
    node_info = sdk.get_info(req)
    print(node_info)
    assert node_info.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"

test()