# Breez Liquid SDK

To get started with the Breez Liquid SDK, follow [these examples](https://sdk-doc-liquid.breez.technology/).

## Getting Started
```rust
let mnemonic = Mnemonic::generate_in(Language::English, 12)?;

// Create the default config
let mut config = LiquidSdk::default_config(LiquidNetwork::Mainnet);

// Customize the config object according to your needs
config.working_dir = "path to an existing directory".into();

let connect_request = ConnectRequest {
    mnemonic: mnemonic.to_string(),
    config,
};
let sdk = LiquidSdk::connect(connect_request).await?;
```

## Tests
In order to run tests, you can execute `cargo test -- --nocapture --test-threads 1`. This is due to the fact that currently tests require some degree of interaction (e.g. adding the funding invoice) in order to work, and thus should be run with a single thread (sequentially).
