import ls_sdk

def test():
    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    sdk = ls_sdk.init(mnemonic, None, ls_sdk.Network.LIQUID_TESTNET)
    node_info = sdk.get_info(False)
    print(node_info)
    assert node_info.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"

test()