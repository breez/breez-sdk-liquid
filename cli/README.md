# Breez SDK - *Liquid* CLI

A simple cli tool that sends commands to the sdk. It is intended to demonstrate the usage and investigate issues that are hard to debug on mobile platforms.

## Prerequisites

Before running the CLI, you need to set your Breez API key. You can request an API key [here](https://breez.technology/request-api-key/#contact-us-form-sdk).

Set the API key in your environment:

#### **Linux/macOS**
```bash
export BREEZ_API_KEY="your-api-key-here"
```
#### **Windows (PowerShell)**
```bash
$env:BREEZ_API_KEY="your-api-key-here"
```

## Run
Then start the CLI with

```bash
cargo run
```

To set a specific network, use one of

```bash
cargo run -- --network mainnet
cargo run -- --network regtest
```

> **Note:** Testnet is not currently supported. The `LiquidNetwork::Testnet` variant exists for potential future testnet4 support but attempting to connect will result in an error.

To specify a custom data directory, use

```bash
cargo run -- --data-dir temp-dir
```

## Commands

To get a full list of commands run `-h` or `<command> -h` to get more information about a command.

- **send-payment** - Send a payment directly or via a swap
- **fetch-lightning-limits** - Fetch the current limits for Send and Receive payments
- **fetch-onchain-limits** - Fetch the current limits for Onchain Send and Receive payments
- **send-onchain-payment** - Send to a Bitcoin onchain address via a swap
- **receive-payment** - Receive a payment directly or via a swap
- **buy-bitcoin** - Generates an URL to buy bitcoin from a 3rd party provider
- **list-payments** - List incoming and outgoing payments
- **get-payment** - Retrieve a payment
- **list-refundables** - List refundable chain swaps
- **prepare-refund** - Prepare a refund transaction for an incomplete swap
- **refund** - Broadcast a refund transaction for an incomplete swap
- **rescan-onchain-swaps** - Rescan onchain swaps
- **get-info** - Get the balance and general info of the current instance
- **sign-message** - Sign a message using the wallet private key
- **check-message** - Verify a message with a public key
- **sync** - Sync local data with mempool and onchain data
- **recommended-fees** - Get the recommended Bitcoin fees based on the configured mempool.space instance
- **empty-cache** - Empties the encrypted transaction cache
- **backup** - Backs up the current pending swaps
- **restore** - Retrieve a list of backups
- **disconnect** - Shuts down all background threads of this SDK instance
- **parse** - Parse a generic string to get its type and relevant metadata
- **lnurl-pay** - Pay using LNURL
- **lnurl-withdraw** - Withdraw using LNURL
- **lnurl-auth** - Auth using LNURL
- **register-webhook** - Register a webhook URL
- **unregister-webhook** - Unregister the webhook URL
- **list-fiat** - List fiat currencies
- **fetch-fiat-rates** - Fetch available fiat rates