
build:
	cargo build
lint:
	cargo fmt
	cargo clippy -- -D warnings
test:
	cargo test
	cargo fmt
	cargo clippy -- -D warnings
