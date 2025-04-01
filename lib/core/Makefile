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
	cargo clippy --all-targets -- -D warnings

cargo-test:
	cargo test

check-regtest:
	@if ! docker compose -f ../../regtest/docker-compose.yml ps | grep -q "Up"; then \
		echo "Error: Regtest setup is not running. Please run 'make regtest-start' first."; \
		exit 1; \
	fi

regtest-start:
	cd ../../regtest && ./start.sh

regtest-stop:
	cd ../../regtest && ./stop.sh

REGTEST_TESTS ?= regtest
cargo-regtest-test: check-regtest
	$(REGTEST_PREFIX) cargo test $(REGTEST_TESTS) --features regtest

wasm-clippy:
	$(CLANG_PREFIX) cargo clippy --all-targets --target=wasm32-unknown-unknown -- -D warnings

BROWSER ?= firefox

wasm-test: test-browser test-node

test-node:
	$(CLANG_PREFIX) wasm-pack test --node

test-browser:
	$(CLANG_PREFIX) wasm-pack test --headless --$(BROWSER) --features browser-tests

test-chrome:
	BROWSER=chrome $(MAKE) test-browser

test-safari:
	BROWSER=safari $(MAKE) test-browser

wasm-regtest-test: wasm-regtest-test-browser wasm-regtest-test-node

wasm-regtest-test-node: check-regtest
	$(CLANG_PREFIX) $(REGTEST_PREFIX) wasm-pack test --node --features regtest -- $(REGTEST_TESTS) --nocapture

wasm-regtest-test-browser: check-regtest
	$(CLANG_PREFIX) $(REGTEST_PREFIX) WASM_BINDGEN_TEST_TIMEOUT=500 wasm-pack test $(if $(NO_HEADLESS), ,--headless) --$(BROWSER) --features regtest,browser-tests -- $(REGTEST_TESTS)

wasm-regtest-test-chrome:
	BROWSER=chrome $(MAKE) wasm-regtest-test-browser

wasm-regtest-test-safari:
	BROWSER=safari $(MAKE) wasm-regtest-test-browser
