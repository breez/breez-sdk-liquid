# breez-sdk-liquid

## Prerequisites

Your system must have the sqlite3 development files installed:

```bash
# On Debian
sudo apt install libsqlite3-dev
```

## Tests
In order to run tests, you can execute `cargo test -- --nocapture --test-threads 1`. This is due to the fact that currently tests require some degree of interaction (e.g. adding the funding invoice) in order to work, and thus should be run with a single thread (sequentially).
