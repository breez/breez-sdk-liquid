# breez-sdk-liquid

## Prerequisites

Your system must have the sqlite3 development files installed:

```bash
# On Debian
sudo apt install libsqlite3-dev
```
## Features

### Backup/Restore
The wallet provides the ability to `backup` and `restore` ongoing swaps via the corresponding methods:
```rust
let mnemonic = "...";
let data_dir = None;
let network = Network::Liquid;
let breez_wallet = Wallet::connect(mnemonic, data_dir, network)?;

breez_wallet.backup()?;  // Backs up the pending swaps under `{data_dir}/backup{-testnet}.sql`. Overwrites previous versions.
let backup_path = None;  // Can also be Some(String), a path pointing to the database. Default is `{data_dir}/backup{-testnet}.sql`
breez_wallet.restore(backup_path)?;   // Restores the specified backup
```

## Tests
In order to run tests, you can execute `cargo test -- --nocapture --test-threads 1`. This is due to the fact that currently tests require some degree of interaction (e.g. adding the funding invoice) in order to work, and thus should be run with a single thread (sequentially).
