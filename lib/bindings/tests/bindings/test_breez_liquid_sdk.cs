
using breez_sdk_liquid.breez_sdk_liquid;

try
{
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    var config = BreezSdkLiquidMethods.DefaultConfig(LiquidNetwork.Testnet, null) with { syncServiceUrl = null };

    var connectReq = new ConnectRequest(config, mnemonic);
    ConnectResponse res = BreezSdkLiquidMethods.Connect(connectReq, null);
    BindingLiquidSdk sdk = res.sdk;

    GetInfoResponse? info = sdk.GetInfo();

    Console.WriteLine(info!.walletInfo.pubkey);
}
catch (Exception e)
{
    Console.WriteLine(e.Message);
    throw;
}
