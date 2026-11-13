# qa: contract v1 e2e suite. `e2e` needs a running stack (API_URL / FRONTEND_URL).
.PHONY: lint test build e2e check
lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
test:
	cargo test
build:
	cargo build --release
e2e:
	cargo test --no-fail-fast -- --ignored
check: lint test
