import breez_liquid_sdk


class SDKListener(breez_liquid_sdk.EventListener):
   def on_event(self, event):
      print(event)


def test():
    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    config = breez_liquid_sdk.default_config(breez_liquid_sdk.Network.TESTNET)
    connect_request = breez_liquid_sdk.ConnectRequest(config=config, mnemonic=mnemonic)
    sdk = breez_liquid_sdk.connect(connect_request)

    listener_id = sdk.add_event_listener(SDKListener())

    node_info = sdk.get_info()

    sdk.remove_event_listener(listener_id)

    print(node_info)
    assert node_info.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"

test()