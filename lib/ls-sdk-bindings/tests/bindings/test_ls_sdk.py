import ls_sdk

def test():
    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    sdk = ls_sdk.init(mnemonic, None, ls_sdk.Network.LIQUID_TESTNET)
    node_info = sdk.get_info(False)
    print(node_info)

test()