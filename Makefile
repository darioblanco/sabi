# Please keep at the top.
SHELL := /usr/bin/env bash
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

# Build the application
build:
	cargo build --release

# Test the application with coverage enabled
coverage:
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
	grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore "tests/*" --ignore "src/main.rs" -o coverage/html
	grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore "tests/*" --ignore "src/main.rs" -o coverage/tests.lcov
	rm -rf **/*.profraw

# Install cargo tools
install:
	cargo install cargo-watch grcov

# Run the application (alias for start)
run: start

# Start the application
start:
	RUST_LOG=info ./target/release/senjin

# Test the application
test:
	cargo test

# Hot reload
watch:	watch.sh
	./watch.sh

.PHONY: build coverage install run start test watch
