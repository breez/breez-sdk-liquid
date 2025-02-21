all: fmt codegen clippy

fmt:
	cd lib && cargo fmt
	cd cli && cargo fmt

clippy:
	cd lib && cargo clippy -- -D warnings
	cd lib && cargo clippy --tests -- -D warnings
	cd cli && cargo clippy -- -D warnings

test:
	cd lib && cargo test

codegen: flutter-codegen react-native-codegen

flutter-codegen:
	cd lib/bindings/langs/flutter && just gen

react-native-codegen:
	make -C ./packages/react-native react-native
