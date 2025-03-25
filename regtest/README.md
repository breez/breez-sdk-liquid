# Regtest environment for Breez SDK - Nodeless (Liquid Implementation)

Based on [Boltz Regtest](https://github.com/BoltzExchange/regtest) (included as a submodule in `./boltz`)

## Prerequisites

* Git submodules 
    * `git submodule update --init`
* [Docker](https://docs.docker.com/engine/install/) or [Orbstack](https://orbstack.dev/) for Apple Silicon based Macs.
When using OrbStack on macOS, set `export DOCKER_DEFAULT_PLATFORM=linux/amd64` before starting.

## Usage

Starting and stopping the regtest setup. On start up, look out for unhealthy containers, which may happen occasionally.

```bash
./start.sh
```

```bash
./stop.sh
```

To control the regtest nodes, some useful aliases are provided.

```bash
source boltz/aliases.sh
```

After setting up the aliases, you can access the following commands:

```bash
# Mine blocks on both Bitcoin and Elements chains
mine-block

# Interact with bitcoind
bitcoin-cli-sim-client getblockchaininfo

# Interact with elements
elements-cli-sim-client getblockchaininfo

# Interact with Lightning nodes
lightning-cli-sim 1 getinfo
lncli-sim 1 getinfo
```

Useful commands for trying out the SDK:

```bash
# Get a bolt11 invoice
lncli-sim 1 addinvoice <amount_sat>

# Pay a bolt11 invoice
lncli-sim 1 payinvoice --force <invoice>

# Get a Bitcoin address
bitcoin-cli-sim-client getnewaddress

# Send bitcoin to a Bitcoin address
bitcoin-cli-sim-client sendtoaddress <address> <amount_btc>

# Get a Liquid address
elements-cli-sim-client getnewaddress

# Send L-BTC to a Liquid address
elements-cli-sim-client sendtoaddress <address> <amount_lbtc>
```

See [Boltz Regtest README](https://github.com/BoltzExchange/regtest/blob/master/README.md) for more info.
