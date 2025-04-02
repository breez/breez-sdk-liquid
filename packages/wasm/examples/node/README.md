# Breez SDK Nodeless - Wasm NodeJs Example

## Prerequisites
Copy the `example.env` file to `.env` and set the BREEZ_API_KEY and MNEMONIC environment variables.

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
npm run cli
```