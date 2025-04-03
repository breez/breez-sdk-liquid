UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	CLANG_PREFIX += AR=$(shell brew --prefix llvm)/bin/llvm-ar CC=$(shell brew --prefix llvm)/bin/clang
endif

init:
	cargo install wasm-pack
	rustup target add wasm32-unknown-unknown

clippy:
	$(CLANG_PREFIX) cargo clippy --all-targets --target=wasm32-unknown-unknown -- -D warnings

build: build-bundle build-deno build-node build-web

build-bundle:
	$(CLANG_PREFIX) wasm-pack build --target bundler --release --out-dir pkg/bundle

build-deno:
	$(CLANG_PREFIX) wasm-pack build --target deno --release --out-dir pkg/deno

build-node:
	$(CLANG_PREFIX) wasm-pack build --target nodejs --release --out-dir pkg/node

build-web:
	$(CLANG_PREFIX) wasm-pack build --target web --release --out-dir pkg/web

test:
	$(CLANG_PREFIX) wasm-pack test --headless --firefox

test-chrome:
	$(CLANG_PREFIX) wasm-pack test --headless --chrome

test-safari:
	$(CLANG_PREFIX) wasm-pack test --headless --safari
