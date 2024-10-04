
using Breez.Sdk.Liquid;

try
{
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    var testPubkey = "MIIBazCCAR2gAwIBAgIHPWTUED8FpTAFBgMrZXAwEDEOMAwGA1UEAxMFQnJlZXowHhcNMjQxMDA0MjMxMjM0WhcNMzQxMDAyMjMxMjM0WjAlMQ4wDAYDVQQKEwVCcmVlejETMBEGA1UEAxMKQnJlZXotVGVzdDAqMAUGAytlcAMhANCD9cvfIDwcoiDKKYdT9BunHLS2/OuKzV8NS0SzqV13o4GAMH4wDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0OBBYEFNo5o+5ea0sNMlW/75VgGJCv2AcJMB8GA1UdIwQYMBaAFN6q1pJW843ndJIW/Ey2ILJrKJhrMB4GA1UdEQQXMBWBE2h5ZHJhX3lzZUBwcm90b24ubWUwBQYDK2VwA0EAILwfdmryGRMoYX5HotHXci5Z5yTG3ugwdNURG3puXSG2eSXgbmwdzGDJBFTvNsEx9sP0L7nyOaKr3W1Yzf/ECQ==";
    var config = BreezSdkLiquidMethods.DefaultConfig(LiquidNetwork.Testnet, testPubkey);

    var connectReq = new ConnectRequest(config, mnemonic);
    BindingLiquidSdk sdk = BreezSdkLiquidMethods.Connect(connectReq);

    GetInfoResponse? info = sdk.GetInfo();

    Console.WriteLine(info!.pubkey);
}
catch (Exception e)
{
    Console.WriteLine(e.Message);
}
