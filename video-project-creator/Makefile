.PHONY: build install

build:
	cargo build --release

install: build
	install -m 755 target/release/cvp ~/bin/cvp
