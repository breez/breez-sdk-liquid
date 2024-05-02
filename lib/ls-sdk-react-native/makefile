.PHONY: init
init:
	make -C ../ls-sdk-bindings init
	
.PHONY: clean
clean:
	make -C ../ls-sdk-bindings clean

all: android ios react-native

react-native:
	make -C ../ls-sdk-bindings react-native
	cp -r ../ls-sdk-bindings/bindings-react-native/android/* android/
	cp -r ../ls-sdk-bindings/bindings-react-native/ios/* ios/
	cp -r ../ls-sdk-bindings/bindings-react-native/ts/src/* src/

android-make:
	make -C ../ls-sdk-bindings android

android-copy:
	mkdir -p android/src/main/jniLibs/x86
	mkdir -p android/src/main/jniLibs/x86_64
	mkdir -p android/src/main/jniLibs/arm64-v8a
	mkdir -p android/src/main/jniLibs/armeabi-v7a
	cp ../ls-sdk-bindings/ffi/kotlin/breez_liquid_sdk/breez_liquid_sdk.kt android/src/main/java/com/breezliquidsdk/breez_liquid_sdk.kt
	cp ../ls-sdk-bindings/ffi/kotlin/jniLibs/x86/libbreez_liquid_sdk_bindings.so android/src/main/jniLibs/x86/libbreez_liquid_sdk_bindings.so
	cp ../ls-sdk-bindings/ffi/kotlin/jniLibs/x86_64/libbreez_liquid_sdk_bindings.so android/src/main/jniLibs/x86_64/libbreez_liquid_sdk_bindings.so
	cp ../ls-sdk-bindings/ffi/kotlin/jniLibs/arm64-v8a/libbreez_liquid_sdk_bindings.so android/src/main/jniLibs/arm64-v8a/libbreez_liquid_sdk_bindings.so
	cp ../ls-sdk-bindings/ffi/kotlin/jniLibs/armeabi-v7a/libbreez_liquid_sdk_bindings.so android/src/main/jniLibs/armeabi-v7a/libbreez_liquid_sdk_bindings.so

android: android-make android-copy

ios-make:
	make -C ../ls-sdk-bindings bindings-swift

ios-copy:
	rm -rf ios/bindings-swift
	cp -r ../ls-sdk-bindings/bindings-swift ios/bindings-swift
	rm -f ios/bindings-swift/Package.swift

ios: ios-make ios-copy
