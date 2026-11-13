.PHONY: lint test build check run
lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
test:
	cargo test
build:
	cargo build --release
check: lint test
run:
	cargo run --release
