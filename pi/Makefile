.PHONY: all
all: clean build test

build:
	wasm-pack build --target web

test:
	cargo test

clean:
	rm -rf pkg