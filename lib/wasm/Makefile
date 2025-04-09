UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	CLANG_PREFIX += AR=$(shell brew --prefix llvm)/bin/llvm-ar CC=$(shell brew --prefix llvm)/bin/clang
endif

init:
	cargo install wasm-pack
	rustup target add wasm32-unknown-unknown

clippy: clippy-default clippy-browser clippy-node

clippy-default:
	$(CLANG_PREFIX) cargo clippy --all-targets --target=wasm32-unknown-unknown -- -D warnings

clippy-browser:
	$(CLANG_PREFIX) cargo clippy --all-targets --target=wasm32-unknown-unknown --features browser -- -D warnings

clippy-node: 
	$(CLANG_PREFIX) cargo clippy --all-targets --target=wasm32-unknown-unknown --features node-js -- -D warnings

build: build-bundle build-deno build-node build-web

build-bundle:
	$(CLANG_PREFIX) wasm-pack build --target bundler --release --out-dir pkg/bundle

build-deno:
	$(CLANG_PREFIX) wasm-pack build --target deno --release --out-dir pkg/deno

build-node:
	$(CLANG_PREFIX) wasm-pack build --target nodejs --release --out-dir pkg/node --features node-js

build-web:
	$(CLANG_PREFIX) wasm-pack build --target web --release --out-dir pkg/web -- features browser

test: test-firefox test-node

test-node:
	$(CLANG_PREFIX) wasm-pack test --node --features node-js

test-firefox:
	$(CLANG_PREFIX) wasm-pack test --headless --firefox --features browser

test-chrome:
	$(CLANG_PREFIX) wasm-pack test --headless --chrome --features browser

test-safari:
	$(CLANG_PREFIX) wasm-pack test --headless --safari --features browser
