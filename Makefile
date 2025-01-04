build:
	cargo build --release
install:
	cargo test
	cargo install --path .