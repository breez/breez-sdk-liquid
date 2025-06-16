# Development guide - Wasm crate
This crate is responsible for building Wasm specific bindings.

## Prerequisites
To build some dependencies you need to first install
- [Protobuf](https://protobuf.dev/installation/)
- [Emscripten](https://emscripten.org/docs/getting_started/downloads.html) for compiling to Wasm
- [Firefox](https://mozilla.org/firefox/download/) for testing Wasm

```bash
brew install protobuf emscripten
```

On first usage you will need to run:
```bash
make init
```

## Building
```bash
make build
```

This will generate the following artifacts:
* Bundle - suitable for use with bundlers like Webpack
 - `bundle/package.json`
 - `bundle/breez_sdk_liquid_wasm.d.ts`
 - `bundle/breez_sdk_liquid_wasm.js`
 - `bundle/breez_sdk_liquid_wasm_bg.js`
 - `bundle/breez_sdk_liquid_wasm_bg.wasm`
 - `bundle/breez_sdk_liquid_wasm_bg.wasm.d.ts`
* Deno - ES module for use with Deno
 - `deno/breez_sdk_liquid_wasm.d.ts`
 - `deno/breez_sdk_liquid_wasm.js`
 - `deno/breez_sdk_liquid_wasm_bg.wasm`
 - `deno/breez_sdk_liquid_wasm_bg.wasm.d.ts`
* Node - CommonJS module for use with Node.js
 - `node/package.json`
 - `node/breez_sdk_liquid_wasm.d.ts`
 - `node/breez_sdk_liquid_wasm.js`
 - `node/breez_sdk_liquid_wasm_bg.wasm`
 - `node/breez_sdk_liquid_wasm_bg.wasm.d.ts`
* Web - ES module for use in browsers
 - `web/package.json`
 - `web/breez_sdk_liquid_wasm.d.ts`
 - `web/breez_sdk_liquid_wasm.js`
 - `web/breez_sdk_liquid_wasm_bg.wasm`
 - `web/breez_sdk_liquid_wasm_bg.wasm.d.ts`

Each can be built separately with `make build-bundle`, `make build-deno`, `make build-node` or `make build-web`.

## Testing
```bash
make test
```

## Debugging
```bash
make build-dev
```

This will generate the same build artifacts but with DWARF debug information.

Each can be built separately with `make build-bundle-dev`, `make build-deno-dev`, `make build-node-dev` or `make build-web-dev`.
