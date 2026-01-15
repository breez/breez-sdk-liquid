# Variables
UNAME := $(shell uname)
CURRENT_DIR := $(shell pwd)
SOURCES=$(sort $(wildcard ./src/*.rs ./src/**/*.rs))
FFI_DIR := ./ffi
TARGET_DIR := ../target
BIN_NAME := libbreez_sdk_liquid_bindings
export IPHONEOS_DEPLOYMENT_TARGET := 13.0
TARGET ?= aarch64-unknown-linux-gnu

ifeq ($(UNAME), Darwin)
	CLANG_PREFIX += AR=$(shell brew --prefix llvm)/bin/llvm-ar CC=$(shell brew --prefix llvm)/bin/clang
	BIN_EXT := dylib
else ifeq ($(UNAME), Linux)
	BIN_EXT := so
endif

BIN_PATH := $(RELEASE_DIR)/$(BIN_NAME).$(BIN_EXT)
INSTALL_PREFIX := CARGO_TARGET_DIR="$(TARGET_DIR)"

# Global
build-release:
	cargo build --release

build-release-target-%: install-target-%
	cargo build --release --target $*

install-bindgen-gobley:
	$(INSTALL_PREFIX) cargo install gobley-uniffi-bindgen --git https://github.com/breez/gobley --rev 72753910f54a157de2f84b78dc6f78415e28d4f2

install-bindgen-cs:
	$(INSTALL_PREFIX) cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.9.1+v0.28.3

install-bindgen-go:
	$(INSTALL_PREFIX) cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.3.0+v0.28.3

install-lipo:
	$(INSTALL_PREFIX) cargo install cargo-lipo 

install-target-%:
	$(CLANG_PREFIX) rustup target add $*

.PHONY: clean
clean:
	cargo clean
	rm -rf ffi
	rm -rf kmp

.PHONY: test
test:
	cargo test

bindings-all-%: \
	bindings-android \
	bindings-kotlin-multiplatform \
	bindings-swift \
	csharp-% \
	golang-% \
	python-%

## Android 
check-ndk:
	@if [ ! -d "${ANDROID_NDK_HOME}" ] ; then \
		echo "Error: Please, set the ANDROID_NDK_HOME env variable to point to your NDK folder" ; \
		exit 1 ; \
	fi
	cargo install cargo-ndk

build-ndk-release-target-%: check-ndk install-target-%
	cargo ndk -t $* -o $(FFI_DIR)/kotlin/jniLibs build --release --link-libcxx-shared

android: install-bindgen-gobley \
	build-ndk-release-target-aarch64-linux-android \
	build-ndk-release-target-armv7-linux-androideabi \
	build-ndk-release-target-i686-linux-android \
	build-ndk-release-target-x86_64-linux-android
	gobley-uniffi-bindgen -c ./uniffi.toml -o ffi/kotlin src/breez_sdk_liquid.udl

bindings-android: android
	cp -r $(FFI_DIR)/kotlin/jniLibs langs/android/lib/src/main
	cp -r $(FFI_DIR)/kotlin/main/kotlin/breez_sdk_liquid langs/android/lib/src/main/kotlin/
	cd langs/android && ./gradlew assemble
	mkdir -p $(FFI_DIR)/android
	cp langs/android/lib/build/outputs/aar/lib-release.aar $(FFI_DIR)/android

## Kotlin
KMP_BASE := langs/kotlin-multiplatform/breez-sdk-liquid-kmp/src
KOTLIN_PACKAGE := kotlin/breez_sdk_liquid

build-kotlin-bindgen-only: install-bindgen-gobley
	gobley-uniffi-bindgen -o $(FFI_DIR)/kotlin \
		-c ./uniffi.toml \
		./src/breez_sdk_liquid.udl

build-kotlin-multiplatform-bindgen-only: install-bindgen-gobley
	gobley-uniffi-bindgen -o $(FFI_DIR)/kmp \
		-c ./uniffi.kotlin-multiplatform.toml \
		./src/breez_sdk_liquid.udl

build-kotlin-multiplatform: build-kotlin-multiplatform-bindgen-only \
	build-ios-universal \
	build-ndk-release-target-aarch64-linux-android \
	build-ndk-release-target-armv7-linux-androideabi \
	build-ndk-release-target-i686-linux-android \
	build-ndk-release-target-x86_64-linux-android

bindings-kotlin-multiplatform: build-kotlin-multiplatform
	mkdir -p $(KMP_BASE)/lib/ios-arm64 $(KMP_BASE)/lib/ios-simulator-arm64 $(KMP_BASE)/lib/ios-simulator-x64
	cp -r $(FFI_DIR)/kmp/* $(KMP_BASE)/
	rm -f $(FFI_DIR)/kotlin/jniLibs/**/*liquid.so
	cp -r $(FFI_DIR)/kotlin/jniLibs/ $(KMP_BASE)/androidMain/jniLibs/
	cp $(TARGET_DIR)/aarch64-apple-ios/release/$(BIN_NAME).a $(KMP_BASE)/lib/ios-arm64
	cp $(TARGET_DIR)/aarch64-apple-ios-sim/release/$(BIN_NAME).a $(KMP_BASE)/lib/ios-simulator-arm64
	cp $(TARGET_DIR)/x86_64-apple-ios/release/$(BIN_NAME).a $(KMP_BASE)/lib/ios-simulator-x64
	cd langs/kotlin-multiplatform && ./gradlew :breez-sdk-liquid-kmp:assemble

## Apple
build-ios-universal: install-lipo \
	build-release-target-aarch64-apple-ios \
	build-release-target-x86_64-apple-ios \
	build-release-target-aarch64-apple-ios-sim
	mkdir -p $(TARGET_DIR)/ios-universal/release
	mkdir -p $(TARGET_DIR)/ios-universal-sim/release
	# build universal lib for arm device and x86 sim
	lipo -create -output $(TARGET_DIR)/ios-universal/release/$(BIN_NAME).a $(TARGET_DIR)/aarch64-apple-ios/release/$(BIN_NAME).a $(TARGET_DIR)/x86_64-apple-ios/release/$(BIN_NAME).a
	# build universal lib for arm sim and x86 sim
	lipo -create -output $(TARGET_DIR)/ios-universal-sim/release/$(BIN_NAME).a $(TARGET_DIR)/aarch64-apple-ios-sim/release/$(BIN_NAME).a $(TARGET_DIR)/x86_64-apple-ios/release/$(BIN_NAME).a

build-darwin-universal: install-lipo build-aarch64-apple-darwin build-x86_64-apple-darwin
	mkdir -p $(TARGET_DIR)/darwin-universal/release
	lipo -create -output $(TARGET_DIR)/darwin-universal/release/$(BIN_NAME).dylib $(TARGET_DIR)/aarch64-apple-darwin/release/$(BIN_NAME).dylib $(TARGET_DIR)/x86_64-apple-darwin/release/$(BIN_NAME).dylib
	lipo -create -output $(TARGET_DIR)/darwin-universal/release/$(BIN_NAME).a $(TARGET_DIR)/aarch64-apple-darwin/release/$(BIN_NAME).a $(TARGET_DIR)/x86_64-apple-darwin/release/$(BIN_NAME).a

build-aarch64-apple-darwin: install-target-aarch64-apple-darwin
	cargo lipo --release --targets aarch64-apple-darwin

build-x86_64-apple-darwin: install-target-x86_64-apple-darwin
	cargo lipo --release --targets x86_64-apple-darwin

## Swift
swift-ios: build-ios-universal
	cargo run --bin uniffi-bindgen generate src/breez_sdk_liquid.udl -l swift -o ffi/swift-ios
	cp $(TARGET_DIR)/ios-universal/release/libbreez_sdk_liquid_bindings.a ffi/swift-ios
	cd ffi/swift-ios && "swiftc" "-emit-module" "-module-name" "breez_sdk_liquid_bindings" "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-ios/breez_sdk_liquidFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_liquid_bindings" breez_sdk_liquid.swift

swift-darwin: build-darwin-universal
	cargo run --bin uniffi-bindgen generate src/breez_sdk_liquid.udl -l swift -o ffi/swift-darwin
	cp $(TARGET_DIR)/darwin-universal/release/libbreez_sdk_liquid_bindings.dylib ffi/swift-darwin
	cd ffi/swift-darwin && "swiftc" "-emit-module" "-module-name" "breez_sdk_liquid_bindings" "-Xcc" "-fmodule-map-file=$(CURRENT_DIR)/ffi/swift-darwin/breez_sdk_liquidFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_liquid_bindings" breez_sdk_liquid.swift

build-ios-framework: build-darwin-universal build-ios-universal
	mkdir -p langs/swift/Sources/BreezSDKLiquid
	cargo run --bin uniffi-bindgen generate src/breez_sdk_liquid.udl --no-format --language swift -o langs/swift/Sources/BreezSDKLiquid
	mv langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquid.swift langs/swift/Sources/BreezSDKLiquid/BreezSDKLiquid.swift
	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64/breez_sdk_liquidFFI.framework/Headers
	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdk_liquidFFI.framework/Headers
	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/macos-arm64_x86_64/breez_sdk_liquidFFI.framework/Headers
	cp $(TARGET_DIR)/aarch64-apple-ios/release/libbreez_sdk_liquid_bindings.a langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI
	cp $(TARGET_DIR)/ios-universal-sim/release/libbreez_sdk_liquid_bindings.a langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI
	cp $(TARGET_DIR)/darwin-universal/release/libbreez_sdk_liquid_bindings.a langs/swift/breez_sdk_liquidFFI.xcframework/macos-arm64_x86_64/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI
	mkdir -p langs/flutter/breez_sdk_liquidFFI/include
	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/flutter/breez_sdk_liquidFFI/include/
	rm langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h
	rm langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.modulemap

bindings-swift: build-ios-framework

# Others
csharp-darwin: install-bindgen-cs build-darwin-universal
	uniffi-bindgen-cs src/breez_sdk_liquid.udl -o ffi/csharp -c ./uniffi.toml
	cp ../target/darwin-universal/release/libbreez_sdk_liquid_bindings.dylib ffi/csharp

csharp-linux: install-bindgen-cs $(SOURCES)
	cargo build --release --target $(TARGET)
	uniffi-bindgen-cs src/breez_sdk_liquid.udl -o ffi/csharp -c ./uniffi.toml
	cp ../target/$(TARGET)/release/libbreez_sdk_liquid_bindings.so ffi/csharp

## Go
golang-darwin: install-bindgen-go build-darwin-universal
	uniffi-bindgen-go src/breez_sdk_liquid.udl -o ffi/golang -c ./uniffi.toml
	cp ../target/darwin-universal/release/libbreez_sdk_liquid_bindings.dylib ffi/golang
	cp -r ffi/golang/breez_sdk_liquid tests/bindings/golang/

golang-linux: install-bindgen-go $(SOURCES)
	cargo build --release --target $(TARGET)
	uniffi-bindgen-go src/breez_sdk_liquid.udl -o ffi/golang -c ./uniffi.toml
	cp ../target/$(TARGET)/release/libbreez_sdk_liquid_bindings.so ffi/golang
	cp -r ffi/golang/breez_sdk_liquid tests/bindings/golang/

## Python
python-linux: $(SOURCES)
	cargo build --release --target $(TARGET)
	cargo run --bin uniffi-bindgen generate src/breez_sdk_liquid.udl --no-format --language python -o ffi/python
	cp ../target/$(TARGET)/release/libbreez_sdk_liquid_bindings.so ffi/python

python-darwin: build-darwin-universal
	cargo run --bin uniffi-bindgen generate src/breez_sdk_liquid.udl --no-format --language python -o ffi/python
	cp ../target/darwin-universal/release/libbreez_sdk_liquid_bindings.dylib ffi/python
