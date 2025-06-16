init:
	make -C ../../lib/wasm init

build:
	make -C ../../lib/wasm build
	rm -rf bundle deno node web
	cp -r ../../lib/wasm/pkg/bundle bundle
	cp -r ../../lib/wasm/pkg/deno deno
	cp -r ../../lib/wasm/pkg/node node
	cp -r ../../lib/wasm/pkg/web web

build-dev:
	make -C ../../lib/wasm build-dev
	rm -rf bundle deno node web
	cp -r ../../lib/wasm/pkg/bundle bundle
	cp -r ../../lib/wasm/pkg/deno deno
	cp -r ../../lib/wasm/pkg/node node
	cp -r ../../lib/wasm/pkg/web web
