import breez_sdk_liquid


class SDKListener(breez_sdk_liquid.EventListener):
   def on_event(self, event):
      print(event)


def test():
    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    test_pubkey = "MIIBazCCAR2gAwIBAgIHPWTUED8FpTAFBgMrZXAwEDEOMAwGA1UEAxMFQnJlZXowHhcNMjQxMDA0MjMxMjM0WhcNMzQxMDAyMjMxMjM0WjAlMQ4wDAYDVQQKEwVCcmVlejETMBEGA1UEAxMKQnJlZXotVGVzdDAqMAUGAytlcAMhANCD9cvfIDwcoiDKKYdT9BunHLS2/OuKzV8NS0SzqV13o4GAMH4wDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0OBBYEFNo5o+5ea0sNMlW/75VgGJCv2AcJMB8GA1UdIwQYMBaAFN6q1pJW843ndJIW/Ey2ILJrKJhrMB4GA1UdEQQXMBWBE2h5ZHJhX3lzZUBwcm90b24ubWUwBQYDK2VwA0EAILwfdmryGRMoYX5HotHXci5Z5yTG3ugwdNURG3puXSG2eSXgbmwdzGDJBFTvNsEx9sP0L7nyOaKr3W1Yzf/ECQ=="
    config = breez_sdk_liquid.default_config(breez_sdk_liquid.LiquidNetwork.TESTNET, test_pubkey)
    connect_request = breez_sdk_liquid.ConnectRequest(config=config, mnemonic=mnemonic)
    sdk = breez_sdk_liquid.connect(connect_request)

    listener_id = sdk.add_event_listener(SDKListener())

    node_info = sdk.get_info()

    sdk.remove_event_listener(listener_id)

    print(node_info)
    assert node_info.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"

test()
