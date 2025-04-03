# Development guide - Core crate

## Prerequisites
To compile the Core crate you will first need to install:
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

## Testing
To run the regular test suite:
```bash
make test
```
This comprises the following make tasks:
```bash
make cargo-test wasm-test
```

### End-to-end tests
To run end-to-end tests, regtest has to be initialized. See [regtest/README.md](../../regtest/README.md) for prerequisites. Starting and stopping the regtest environment can be done using:
```bash
make regtest-start
make regtest-stop
```

To run the end-to-end tests:
```bash
make regtest-test
```
This comprises the following make tasks:
```bash
make cargo-regtest-test wasm-regtest-test
```