TARGET=swdocs

.PHONY: install build cleani

build:
	cargo build --release

clean:
	cargo clean

install: build
	cp ./target/release/$(TARGET) ~/.local/bin/

cleani: clean build install
