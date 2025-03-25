UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	CLANG_PREFIX += AR=$(shell brew --prefix llvm)/bin/llvm-ar CC=$(shell brew --prefix llvm)/bin/clang
endif

init:
	cargo install wasm-pack
	rustup target add wasm32-unknown-unknown

clippy:
	$(CLANG_PREFIX) cargo clippy --target=wasm32-unknown-unknown -- -D warnings

pack:
	$(CLANG_PREFIX) wasm-pack build --weak-refs --target web --scope @breeztech

test:
	$(CLANG_PREFIX) wasm-pack test --headless --firefox

test-chrome:
	$(CLANG_PREFIX) wasm-pack test --headless --chrome

test-safari:
	$(CLANG_PREFIX) wasm-pack test --headless --safari
