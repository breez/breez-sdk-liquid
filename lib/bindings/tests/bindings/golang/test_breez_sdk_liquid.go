package main

import (
	"log"

	"example.org/golang/breez_sdk_liquid"
)

func main() {
	mnemonic := "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
	testPubkey := "MIIBazCCAR2gAwIBAgIHPWTUED8FpTAFBgMrZXAwEDEOMAwGA1UEAxMFQnJlZXowHhcNMjQxMDA0MjMxMjM0WhcNMzQxMDAyMjMxMjM0WjAlMQ4wDAYDVQQKEwVCcmVlejETMBEGA1UEAxMKQnJlZXotVGVzdDAqMAUGAytlcAMhANCD9cvfIDwcoiDKKYdT9BunHLS2/OuKzV8NS0SzqV13o4GAMH4wDgYDVR0PAQH/BAQDAgWgMAwGA1UdEwEB/wQCMAAwHQYDVR0OBBYEFNo5o+5ea0sNMlW/75VgGJCv2AcJMB8GA1UdIwQYMBaAFN6q1pJW843ndJIW/Ey2ILJrKJhrMB4GA1UdEQQXMBWBE2h5ZHJhX3lzZUBwcm90b24ubWUwBQYDK2VwA0EAILwfdmryGRMoYX5HotHXci5Z5yTG3ugwdNURG3puXSG2eSXgbmwdzGDJBFTvNsEx9sP0L7nyOaKr3W1Yzf/ECQ=="

	config := breez_sdk_liquid.DefaultConfig(breez_sdk_liquid.LiquidNetworkTestnet, testPubkey)

	sdk, err := breez_sdk_liquid.Connect(breez_sdk_liquid.ConnectRequest{
		Config:   config,
		Mnemonic: mnemonic,
	})

	if err != nil {
		log.Fatalf("Connect failed: %#v", err)
	}

	info, err := sdk.GetInfo()

	if err != nil {
		log.Fatalf("GetInfo failed: %#v", err)
	}

	log.Print(info.Pubkey)
}
