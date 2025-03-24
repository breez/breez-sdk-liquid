# Building WASM

## Prerequisites

To build some dependencies you need to first install [emscripten](https://emscripten.org/docs/getting_started/downloads.html):
```bash
brew install emscripten
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
