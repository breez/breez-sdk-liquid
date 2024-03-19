# breez-sdk-liquid-cli

## Setup

You'll need a Testnet LN node to test the sending and receiving operations. A simple solution is using [Alby's testnet nodes](https://thunderhub.regtest.getalby.com). Read more about Alby's test setup [here](https://github.com/getAlby/lightning-browser-extension/wiki/Test-setup).

## Commands

Start the CLI with

```bash
cargo run
```

To specify a custom data directory, use

```bash
cargo run -- --data_dir temp-dir
```

To set a custom log level, use

```bash
RUST_LOG=info cargo run
```
