init:
	rustup update
	rustup component add rustfmt

build:
	cargo build --release
