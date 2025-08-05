# Breez SDK Nodeless - Wasm NodeJs Memory Usage tool

- Tracks heap, external, and RSS memory after every round
- Runs N create/destroy cycles (defaults: 20 rounds × 120 instances)
- Forces GC and waits for native threads to exit
- Saves heap‑snapshots (initial, after round‑0, after final round)

## Prerequisites

Copy the `example.env` file to `.env` and set the BREEZ_API_KEY environment variable.

## Build

If you are running from a local Wasm package, build the Wasm package first in the [Wasm package](../../) directory.

```bash
cd ..
make build
```

Install the dependencies

```bash
npm i
```

## Run

```bash
npm run memory-test [rounds] [instances]
```
