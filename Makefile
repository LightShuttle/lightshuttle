.PHONY: all fmt clippy test ci

all: ci

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

ci: fmt-check clippy test
