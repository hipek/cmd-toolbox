.PHONY: build install test

build:
	cargo build --release

install: build
	install -m 755 target/release/tvd ~/bin/tvd

test:
	cargo test
