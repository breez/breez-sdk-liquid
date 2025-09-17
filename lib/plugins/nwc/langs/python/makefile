TARGET ?= aarch64-unknown-linux-gnu
FFI_DIR = ../../ffi
TARGET_DIR = ../../../../target/

.PHONY: init
init:
	make -C ../../ init

darwin:
	make -C ../../ python-darwin
	cp $(FFI_DIR)/python/{breez_sdk_liquid.py,breez_sdk_liquid_nwc.py} src/breez_sdk_liquid_nwc
	cp $(TARGET_DIR)/darwin-universal/release/libbreez_sdk_liquid_nwc.dylib src/breez_sdk_liquid_nwc

linux:
	make -C ../../ python-linux
	cp $(FFI_DIR)/python/{breez_sdk_liquid.py,breez_sdk_liquid_nwc.py} src/breez_sdk_liquid_nwc
	cp $(TARGET_DIR)/$(TARGET)/release/libbreez_sdk_liquid_nwc.so src/breez_sdk_liquid_nwc
