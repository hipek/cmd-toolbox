.PHONY: build install

build:
	cargo build --release

install: build
	install -m 755 cvp/target/release/cvp ~/bin/cvp
