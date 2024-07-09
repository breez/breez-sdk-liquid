# Breez Liquid SDK - CLI

## Setup

You'll need a Testnet LN node to test the sending and receiving operations. A simple solution is using [Alby's testnet nodes](https://thunderhub.regtest.getalby.com). Read more about Alby's test setup [here](https://github.com/getAlby/lightning-browser-extension/wiki/Test-setup).

## Commands

Start the CLI with

```bash
cargo run
```

To set a specific network, use one of

```bash
cargo run -- --network mainnet
cargo run -- --network testnet
```

To specify a custom data directory, use

```bash
cargo run -- --data-dir temp-dir
```
