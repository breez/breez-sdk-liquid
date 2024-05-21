.PHONY: init
init:
	make -C ../../ init

darwin:
	make -C ../../ python-darwin
	cp ../../ffi/python/breez_liquid_sdk.py src/breez_liquid_sdk
	cp ../../ffi/python/libbreez_liquid_sdk_bindings.dylib src/breez_liquid_sdk

linux:
	make -C ../../ python-linux
	cp ../../ffi/python/breez_liquid_sdk.py src/breez_liquid_sdk
	cp ../../ffi/python/libbreez_liquid_sdk_bindings.so src/breez_liquid_sdk
