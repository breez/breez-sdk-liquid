# Breez SDK Nodeless - Wasm Vite Example

## Prerequisites
Copy the `example.env` file to `.env` and set the VITE_BREEZ_API_KEY and VITE_MNEMONIC environment variables.

## Build
If you are running from a local Wasm package, build the Wasm package first in the [Wasm package](../../) directory.
```bash
cd ..
make build
```

Install the dependencies
```bash
yarn
```

Run vite build
```bash
yarn build
```

## Run
```bash
yarn preview
```