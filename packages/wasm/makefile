init:
	make -C ../../lib/wasm init

build:
	make -C ../../lib/wasm build
	rm -rf bundle deno node web
	cp -r ../../lib/wasm/pkg/bundle bundle
	cp -r ../../lib/wasm/pkg/deno deno
	cp -r ../../lib/wasm/pkg/node node
	cp -r ../../lib/wasm/pkg/web web

build-node:
	make -C ../../lib/wasm build-node
	rm -rf node
	cp -r ../../lib/wasm/pkg/node node

build-deno:
	make -C ../../lib/wasm build-deno
	rm -rf deno
	cp -r ../../lib/wasm/pkg/deno deno

build-web:
	make -C ../../lib/wasm build-web
	rm -rf web
	cp -r ../../lib/wasm/pkg/web web

build-bundle:
	make -C ../../lib/wasm build-bundle
	rm -rf bundle
	cp -r ../../lib/wasm/pkg/bundle bundle
