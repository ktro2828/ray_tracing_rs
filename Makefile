.PHONY: all
all: test

.PHONY: build
build:
	@cargo build --all

.PHONY: test
test:
	@cargo test --all

.PHONY: doctest
doctest:
	@cargo test --doc

.PHONY: check
check:
	@cargo check --all

.PHONY: format-check
format-check:
	@cargo fmt --all -- --check

.PHONY: lint
lint:
	@cargo clippy --all -- -D clippy::dbg-marcro -D warnings

.PHONY: doc
doc:
	@cargo doc --document-private-items --open

.PHONY: examples
examples:
	@cargo run --example main
