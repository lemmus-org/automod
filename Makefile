build:
	cargo build

check: lint
	cargo fmt -- --check

format:
	cargo fmt

install:
	cargo install

lint:
	cargo clippy -- -D warnings

run:
	cargo run
