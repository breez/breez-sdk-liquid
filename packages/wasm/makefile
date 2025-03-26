init:
	make -C ../../lib/wasm init

build:
	make -C ../../lib/wasm build
	rm -rf bundle node web
	cp -r ../../lib/wasm/pkg/bundle bundle
	cp -r ../../lib/wasm/pkg/node node
	cp -r ../../lib/wasm/pkg/web web
