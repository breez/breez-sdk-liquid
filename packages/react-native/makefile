UNAME := $(shell uname -s)
export IPHONEOS_DEPLOYMENT_TARGET := 13.0
export CARGO_TERM_COLOR := always

.PHONY: init
init: install-protobuf
	yarn install
	npx patch-package
	make -C ../../lib/bindings init

# Install protobuf compiler based on OS
install-protobuf:
ifeq ($(UNAME),Linux)
	sudo apt-get update && sudo apt-get install -y protobuf-compiler
else ifeq ($(UNAME), Darwin)
	brew install protobuf
else
	echo "Unsupoprted OS. Please install protobuf compiler manually." && exit 1
endif

react-native:
	yarn ubrn:build
	yarn prepare
	patch -p3 < patches/working-dir.patch
