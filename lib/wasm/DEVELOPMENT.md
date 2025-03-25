# Development guide - Wasm crate
This crate is responsible for building Wasm specific bindings.

## Prerequisites
To build some dependencies you need to first install
- [Protobuf](https://protobuf.dev/installation/)
- [Emscripten](https://emscripten.org/docs/getting_started/downloads.html) for compiling to Wasm

```bash
brew install protobuf emscripten
```

On first usage you will need to run:
```bash
make init
```

## Building
```bash
make pack
```

This will generate the following artifacts:

- `pkg/package.json`
- `pkg/breez_sdk_liquid_wasm_bg.wasm`
- `pkg/breez_sdk_liquid_wasm_bg.d.wasm`
- `pkg/breez_sdk_liquid_wasm.d.ts`
- `pkg/breez_sdk_liquid_wasm.js`

## Testing
```bash
make test
```
