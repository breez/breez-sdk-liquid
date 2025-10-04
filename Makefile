all: fmt codegen clippy

init:
	make -C ./lib/bindings init
	make -C ./lib/core init
	make -C ./lib/wasm init

fmt:
	cd lib && cargo fmt
	cd cli && cargo fmt

clippy: cargo-clippy wasm-clippy

cargo-clippy:
	cd lib/bindings && cargo clippy --all-targets -- -A deprecated -D warnings
	cd lib/core && cargo clippy --all-targets -- -D warnings
	cd lib/plugins/nwc && cargo clippy -- -D warnings
	cd cli && cargo clippy -- -D warnings

wasm-clippy:
	make -C ./lib/core wasm-clippy
	make -C ./lib/wasm clippy

test: cargo-test wasm-test

cargo-test:
	cd lib/bindings && cargo test
	cd lib/core && cargo test

wasm-test:
	make -C ./lib/core wasm-test
	make -C ./lib/wasm test

codegen: flutter-codegen react-native-codegen

flutter-codegen:
	cd packages/flutter_breez_liquid && just gen

react-native-codegen:
	make -C ./packages/react-native react-native
