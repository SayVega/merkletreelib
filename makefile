.PHONY: build run test fmt check clean

build:
	cargo build

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt --all -- --check
