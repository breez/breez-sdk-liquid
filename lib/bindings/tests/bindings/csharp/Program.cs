
using Breez.Sdk.Liquid;

try
{
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    var config = BreezSdkLiquidMethods.DefaultConfig(LiquidNetwork.Testnet, null) with { syncServiceUrl = null };

    var connectReq = new ConnectRequest(config, mnemonic);
    ConnectResponse connectResponse = BreezSdkLiquidMethods.Connect(connectReq, null);
    BindingLiquidSdk sdk = connectResponse.sdk;

    GetInfoResponse? info = sdk.GetInfo();

    Console.WriteLine(info!.walletInfo.pubkey);
}
catch (Exception e)
{
    Console.WriteLine(e.Message);
}
