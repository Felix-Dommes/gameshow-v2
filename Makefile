build:
	cd ./vue && \
	make
	cargo build

run: build
	cargo run
