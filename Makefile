# Please keep at the top.
SHELL := /usr/bin/env bash
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

#######
# API #
#######

.PHONY: api-build api-coverage

# Build the api
api-build:
	cd api && cargo build --release

# Test the application with coverage enabled
api-coverage:
	cd api && cargo tarpaulin --exclude-files src/main.rs

# Start the application
api-start:
	docker-compose up -d redis
	RUST_LOG=info ./api/target/release/sabi

# Test the application
api-test:
	cd api && cargo test

# Hot reload
api-watch: install
	docker-compose up -d redis
	cd api && cargo watch -x run

#############
# Container #
#############

.PHONY: container-redis

# Attach to the redis container
container-redis:
	docker-compose up -d redis
	docker-compose exec redis sh

##########
# Global #
##########
.PHONY: install

# Install cargo tools
install:
	cargo install cargo-tarpaulin cargo-watch grcov
