# Breez-SDK Liquid POC

## Setup
In order for the test to work, an instance of a running lightning testnet node has to be used. For the sake of semplicity, I suggest using [Alby's testnet nodes](https://github.com/getAlby/lightning-browser-extension/wiki/Test-setup) through ThunderHub (in particular LND-2, as LND-1 does not seem to be well-connected).

## Commands
You can run an instance of the cli with `cargo run`, optionally followed by `--data-dir` to specify which directory to use for caching (command history and mnemonic). 

Currently the cli supports the `receive` (reverse submarine swap), `send` (normal submarine swap), `get-balance` (get the liquid wallet's balance) and `get-address` (get the first fungible address, useful for funding via faucets). 

## Configuration
In order to run tests, you can execute `cargo test -- --nocapture --test-threads 1`. This is due to the fact that currently tests require some degree of interaction (e.g. adding the funding invoice) in order to work, and thus should be run with a single thread (sequentially).
