package main

import (
	"log"

	"example.org/golang/breez_sdk_liquid"
)

func main() {
	mnemonic := "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
	config, err := breez_sdk_liquid.DefaultConfig(breez_sdk_liquid.LiquidNetworkTestnet, nil)
	config.SyncServiceUrl = nil

	if err != nil {
		log.Fatalf("Config creation failed: %#v", err)
	}

	connectResponse, err := breez_sdk_liquid.Connect(breez_sdk_liquid.ConnectRequest{
		Config:   config,
		Mnemonic: &mnemonic,
	}, nil)

	if err != nil {
		log.Fatalf("Connect failed: %#v", err)
	}

	sdk := connectResponse.Sdk
	info, err := sdk.GetInfo()

	if err != nil {
		log.Fatalf("GetInfo failed: %#v", err)
	}

	log.Print(info.WalletInfo.Pubkey)
}
