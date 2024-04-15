SOURCES=$(sort $(wildcard ./src/*.rs ./src/**/*.rs))

TARGET ?= x86_64-unknown-linux-gnu

all: python kotlin swift

python: $(SOURCES)
	cargo build --release --target $(TARGET)
	cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/ls_sdk.udl --no-format --language python -o ffi/python

kotlin: $(SOURCES)
	cargo build --release --target $(TARGET)
	cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/ls_sdk.udl --no-format --language kotlin -o ffi/kotlin

swift: $(SOURCES)
	cargo build --release --target $(TARGET)
	cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/ls_sdk.udl --no-format --language swift -o ffi/swift
