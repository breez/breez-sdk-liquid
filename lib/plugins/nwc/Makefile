## Env
TARGET ?= aarch64-unknown-linux-gnu
FFI_DIR = ./ffi
TARGET_DIR = ../../target
SHELL := /bin/bash
SOURCES = $(sort $(wildcard ./src/*.rs ./src/**/*.rs))

LIBBREEZ_NWC_LINUX = $(TARGET_DIR)/$(TARGET)/release/libbreez_sdk_liquid_nwc.so
LIBBREEZ_NWC_DARWIN = $(TARGET_DIR)/darwin-universal/release/libbreez_sdk_liquid_nwc.dylib
LIBBREEZ_NWC_DARWIN_STATIC = $(TARGET_DIR)/darwin-universal/release/libbreez_sdk_liquid_nwc.a
LIBBREEZ_NWC_IOS = $(TARGET_DIR)/ios-universal/release/libbreez_sdk_liquid_nwc.a
LIBBREEZ_NWC_IOS_SIM = $(TARGET_DIR)/ios-universal-sim/release/libbreez_sdk_liquid_nwc.a

## General

# $1 - Generator
# $2 - Library file
# $3 - Output dir
define uniffi_bindgen
	$1 \
		--library $2 \
		--out-dir $3 \
		-c uniffi.toml
	cp $2 $3
endef

# $1 - Language
# $2 - Library file
# $3 - Output dir
define cargo_uniffi_bindgen
	cargo run --bin uniffi-bindgen generate \
		--language $1 \
		--library $2 \
		--no-format \
		-o $3 \
		-c uniffi.toml
	cp $2 $3 
endef

.PHONY: init
init:
	make -C ../../bindings init
	cargo install uniffi-bindgen-go --git https://github.com/NordSecurity/uniffi-bindgen-go --tag v0.3.0+v0.28.3
	cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.9.1+v0.28.3
	cargo install gobley-uniffi-bindgen --git https://github.com/breez/gobley --rev f920c275e4f60e22080a6cd280d9562d64eb6ac9

.PHONY: clean
clean:
	cargo clean
	rm -rf ffi
	rm -rf kmp

ios-universal: $(SOURCES)		
	mkdir -p $(TARGET_DIR)/ios-universal/release
	mkdir -p $(TARGET_DIR)/ios-universal-sim/release
	cargo build --release --target aarch64-apple-ios ;\
	cargo build --release --target x86_64-apple-ios ;\
	cargo build --release --target aarch64-apple-ios-sim ;\
	# build universal lib for arm device and x86 sim
	lipo -create -output $(LIBBREEZ_NWC_IOS) \
		$(TARGET_DIR)/aarch64-apple-ios/release/libbreez_sdk_liquid_nwc.a \
		$(TARGET_DIR)/x86_64-apple-ios/release/libbreez_sdk_liquid_nwc.a
	# build universal lib for arm sim and x86 sim
	lipo -create -output $(LIBBREEZ_NWC_IOS_SIM) \
		$(TARGET_DIR)/aarch64-apple-ios-sim/release/libbreez_sdk_liquid_nwc.a \
		$(TARGET_DIR)/x86_64-apple-ios/release/libbreez_sdk_liquid_nwc.a

darwin-universal: $(SOURCES)
	mkdir -p $(TARGET_DIR)/darwin-universal/release
	cargo lipo --release --targets aarch64-apple-darwin
	cargo lipo --release --targets x86_64-apple-darwin
	lipo -create -output $(LIBBREEZ_NWC_DARWIN) \
		$(TARGET_DIR)/aarch64-apple-darwin/release/libbreez_sdk_liquid_bindings.dylib \
		$(TARGET_DIR)/x86_64-apple-darwin/release/libbreez_sdk_liquid_bindings.dylib
	lipo -create -output $(LIBBREEZ_NWC_DARWIN_STATIC) \
		../target/aarch64-apple-darwin/release/libbreez_sdk_liquid_bindings.a \
		../target/x86_64-apple-darwin/release/libbreez_sdk_liquid_bindings.a

## Python
python-darwin: darwin-universal
	$(call cargo_uniffi_bindgen, python, $(LIBBREEZ_NWC_DARWIN), $(FFI_DIR)/python)

python-linux: $(SOURCES)
	cargo build --release --target $(TARGET)
	$(call cargo_uniffi_bindgen, python, $(LIBBREEZ_NWC_LINUX), $(FFI_DIR)/python)

## Go
golang-darwin: darwin-universal
	$(call uniffi_bindgen, uniffi-bindgen-go, $(LIBBREEZ_NWC_DARWIN), $(FFI_DIR)/golang)

golang-linux: $(SOURCES)
	cargo build --release --target $(TARGET)
	$(call uniffi_bindgen, uniffi-bindgen-go, $(LIBBREEZ_NWC_LINUX), $(FFI_DIR)/golang)

## C-Sharp
csharp-darwin: darwin-universal
	$(call uniffi_bindgen, uniffi-bindgen-cs, $(LIBBREEZ_NWC_DARWIN), $(FFI_DIR)/csharp)

csharp-linux: $(SOURCES)
	cargo build --release --target $(TARGET)
	$(call uniffi_bindgen, uniffi-bindgen-cs, $(LIBBREEZ_NWC_LINUX), $(FFI_DIR)/csharp)

## Swift
bindings-swift: ios-universal darwin-universal build-ios-framework

swift-ios: ios-universal
	$(call cargo_uniffi_bindgen, swift, $(LIBBREEZ_NWC_IOS), $(FFI_DIR)/swift-ios)
	cd $(FFI_DIR)/swift-ios && "swiftc" "-emit-module" "-module-name" "breez_sdk_liquid_nwc" "-Xcc" "-fmodule-map-file=$(FFI_DIR)/swift-ios/breez_sdk_liquid_nwcFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_liquid_nwc" breez_sdk_liquid_nwc.swift

swift-darwin: darwin-universal
	$(call cargo_uniffi_bindgen, swift, $(LIBBREEZ_NWC_DARWIN), $(FFI_DIR)/swift-darwin)
	cd $(FFI_DIR)/swift-darwin && "swiftc" "-emit-module" "-module-name" "breez_sdk_liquid_nwc" "-Xcc" "-fmodule-map-file=$(FFI_DIR)/swift-darwin/breez_sdk_liquid_nwcFFI.modulemap" "-I" "."  "-L" "." "-lbreez_sdk_liquid_nwc" breez_sdk_liquid_nwc.swift

## TODO: In progress

# build-ios-framework:
# 	mkdir -p langs/swift/Sources/BreezSDKLiquidNWC
# 	cargo run --bin uniffi-bindgen generate src/breez_sdk_liquid.udl --no-format --language swift -o langs/swift/Sources/BreezSDKLiquid
# 	mv langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquid.swift langs/swift/Sources/BreezSDKLiquid/BreezSDKLiquid.swift
# 	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64/breez_sdk_liquidFFI.framework/Headers
# 	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdk_liquidFFI.framework/Headers
# 	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/swift/breez_sdk_liquidFFI.xcframework/macos-arm64_x86_64/breez_sdk_liquidFFI.framework/Headers
# 	cp ../target/aarch64-apple-ios/release/libbreez_sdk_liquid_bindings.a langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI
# 	cp ../target/ios-universal-sim/release/libbreez_sdk_liquid_bindings.a langs/swift/breez_sdk_liquidFFI.xcframework/ios-arm64_x86_64-simulator/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI
# 	cp ../target/darwin-universal/release/libbreez_sdk_liquid_bindings.a langs/swift/breez_sdk_liquidFFI.xcframework/macos-arm64_x86_64/breez_sdk_liquidFFI.framework/breez_sdk_liquidFFI
# 	cp langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h langs/flutter/breez_sdk_liquidFFI/include/
# 	rm langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.h
# 	rm langs/swift/Sources/BreezSDKLiquid/breez_sdk_liquidFFI.modulemap

## Android 
.PHONY: ndk-home
ndk-home:
	@if [ ! -d "${ANDROID_NDK_HOME}" ] ; then \
		echo "Error: Please, set the ANDROID_NDK_HOME env variable to point to your NDK folder" ; \
		exit 1 ; \
	fi

.PHONY: android
android: aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
	gobley-uniffi-bindgen -c ./uniffi.toml -o $(FFI_DIR)/kotlin --library 

aarch64-linux-android: $(SOURCES) ndk-home
	cargo ndk -t aarch64-linux-android -o ffi/kotlin/jnilibs build --release	
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so ffi/kotlin/jniLibs/arm64-v8a/

armv7-linux-androideabi: $(SOURCES) ndk-home
	cargo ndk -t armv7-linux-androideabi -o ffi/kotlin/jniLibs build --release
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so ffi/kotlin/jniLibs/armeabi-v7a/

i686-linux-android: $(SOURCES) ndk-home
	cargo ndk -t i686-linux-android -o ffi/kotlin/jniLibs build --release
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/i686-linux-android/libc++_shared.so ffi/kotlin/jniLibs/x86/

x86_64-linux-android: $(SOURCES) ndk-home
	cargo ndk -t x86_64-linux-android -o ffi/kotlin/jniLibs build --release
	cp -a $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(OS_NAME)-x86_64/sysroot/usr/lib/x86_64-linux-android/libc++_shared.so ffi/kotlin/jniLibs/x86_64/

bindings-android: android
	cp -r ffi/kotlin/jniLibs langs/android/lib/src/main
	cp -r ffi/kotlin/main/kotlin/breez_sdk_liquid langs/android/lib/src/main/kotlin/
	cd langs/android && ./gradlew assemble --debug
	mkdir -p ffi/android
	cp langs/android/lib/build/outputs/aar/lib-release.aar ffi/android
