# Please keep at the top.
SHELL := /usr/bin/env bash
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

# Build the application
build:
	cargo build --release

# Test the application with coverage enabled
coverage:
	cargo tarpaulin --exclude-files src/main.rs

# Install cargo tools
install:
	cargo install cargo-tarpaulin cargo-watch grcov

# Run the application (alias for start)
run: start

# Start the application
start:
	docker-compose up -d redis
	RUST_LOG=info ./target/release/sabi

# Test the application
test:
	cargo test

# Hot reload
watch:
	docker-compose up -d redis
	./watch.sh

.PHONY: build coverage install run start test watch

###################
# Container #
###################

# Build the Docker image
container-build:
	docker-compose build

# Run the Docker container
container-run:
	docker-compose up -d

# Stop and remove the Docker container
container-down:
	docker-compose down
