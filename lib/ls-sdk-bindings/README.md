To build the bindings:

```bash
# Kotlin
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/ls_sdk.udl --no-format --language kotlin -o ffi/kotlin

# Python
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/ls_sdk.udl --no-format --language python -o ffi/python

# Swift
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/ls_sdk.udl --no-format --language swift -o ffi/swift
```