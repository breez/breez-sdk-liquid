UNAME := $(shell uname -s)
export IPHONEOS_DEPLOYMENT_TARGET := 13.0
export CARGO_TERM_COLOR := always

# Install protobuf compiler based on OS
install-protobuf:
ifeq ($(UNAME),Linux)
	sudo apt-get update && sudo apt-get install -y protobuf-compiler
else ifeq ($(UNAME), Darwin)
	brew install protobuf
else
	echo "Unsupoprted OS. Please install protobuf compiler manually." && exit 1
endif

.PHONY: init
init: install-protobuf
	rustup target add \
	  aarch64-linux-android \
	  armv7-linux-androideabi \
	  i686-linux-android \
	  x86_64-linux-android
	rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
	cargo install cargo-ndk
	yarn install
	npx patch-package

react-native:
	yarn ubrn:build
	yarn prepare
	yarn patch-working-dir
