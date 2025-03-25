UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	CLANG_PREFIX += AR=$(shell brew --prefix llvm)/bin/llvm-ar CC=$(shell brew --prefix llvm)/bin/clang
endif

LND_MACAROON_HEX=$(shell xxd -p ../../regtest/boltz/data/lnd1/data/chain/bitcoin/regtest/admin.macaroon | tr -d '\n')
BITCOIND_COOKIE=$(shell cat ../../regtest/boltz/data/bitcoind/regtest/.cookie)
REGTEST_PREFIX = LND_MACAROON_HEX=$(LND_MACAROON_HEX) BITCOIND_COOKIE=$(BITCOIND_COOKIE)

init:
	cargo install wasm-pack

clippy: cargo-clippy wasm-clippy

test: cargo-test wasm-test

regtest-test: cargo-regtest-test wasm-regtest-test

cargo-clippy:
	cargo clippy -- -D warnings

cargo-test:
	cargo test

REGTEST_TESTS ?= regtest
cargo-regtest-test:
	$(REGTEST_PREFIX) cargo test $(REGTEST_TESTS) --features "regtest"

wasm-clippy:
	$(CLANG_PREFIX) cargo clippy --target=wasm32-unknown-unknown -- -D warnings

BROWSER ?= firefox

wasm-test:
	$(CLANG_PREFIX) wasm-pack test --headless --$(BROWSER)

test-chrome:
	BROWSER=chrome $(MAKE) wasm-test

test-safari:
	BROWSER=safari $(MAKE) wasm-test

wasm-regtest-test:
	$(CLANG_PREFIX) $(REGTEST_PREFIX) WASM_BINDGEN_TEST_TIMEOUT=500 wasm-pack test --headless --$(BROWSER) --features regtest -- $(REGTEST_TESTS)

wasm-regtest-test-chrome:
	BROWSER=chrome $(MAKE) wasm-regtest-test

wasm-regtest-test-safari:
	BROWSER=safari $(MAKE) wasm-regtest-test
