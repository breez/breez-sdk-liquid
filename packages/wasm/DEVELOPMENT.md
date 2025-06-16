# Development guide - Wasm package

When developing, it can be useful to work with a locally built version of the Breez Liquid SDK instead of relying on what is published already.
To do this, you first need to build the Breez Liquid SDK bindings locally and then point the examples to make use of the locally built Breez Liquid SDK Wasm package.

All the following commands can be run in the `packages/wasm` directory.

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

## Build
To build the Wasm code run:
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

## Testing with the examples

To test locally built bindings in the examples, the npm dependencies need to be updated to use the local package.
```json
    "@breeztech/breez-sdk-liquid": "file:../../",
```

## Debugging
```bash
make build-dev
```

Each can be built separately with `make build-bundle-dev`, `make build-deno-dev`, `make build-node-dev` or `make build-web-dev`.

### Chrome - Web
- Install [Chrome DevTools C++ DWARF debugging](https://chromewebstore.google.com/detail/cc++-devtools-support-dwa/pdcpmagijalfljmkmjngeonclgbbannb) extension

1. Open Chrome DevTools
2. Go to "Sources" tab
3. Open "file://<your repo path to rs file>"
4. Set a breakpoint
5. Reload the page and verify the DevTools debugger breaks as expected.

### VSCode - Node
- Install [VSCode WebAssembly DWARF Debugging](https://marketplace.visualstudio.com/items?itemName=ms-vscode.wasm-dwarf-debugging) extension

https://code.visualstudio.com/docs/nodejs/nodejs-debugging#_debugging-webassembly
1. Enable `Debug: Toggle Auto Attach` from the command palette
2. Run the node application with `--inspect`, e.g. `node --inspect src/cli.js`
