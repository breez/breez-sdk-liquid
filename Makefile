all: fmt codegen clippy

fmt:
	cd lib && cargo fmt
	cd cli && cargo fmt

clippy: cargo-clippy wasm-clippy

cargo-clippy:
	cd lib/bindings && cargo clippy -- -D warnings
	cd lib/bindings && cargo clippy --tests -- -D warnings
	cd lib/core && cargo clippy -- -D warnings
	cd lib/core && cargo clippy --tests -- -D warnings
	cd cli && cargo clippy -- -D warnings

wasm-clippy:
	make -C ./lib/wasm clippy

test: cargo-test wasm-test

cargo-test:
	cd lib/bindings && cargo test
	cd lib/core && cargo test

wasm-test:
	make -C ./lib/wasm test

codegen: flutter-codegen react-native-codegen

flutter-codegen:
	cd lib/bindings/langs/flutter && just gen

react-native-codegen:
	make -C ./packages/react-native react-native
