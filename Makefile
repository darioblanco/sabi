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
watch:
	./watch.sh

.PHONY: build coverage install run start test watch

###################
# Docker & Podman #
###################

DOCKER_IMAGE_NAME=senjin
DOCKER_IMAGE_TAG=latest
DOCKER_IMAGE=$(DOCKER_IMAGE_NAME):$(DOCKER_IMAGE_TAG)

# Build the Docker image
docker-build:
	docker build -t $(DOCKER_IMAGE) .

# Run the Docker container
docker-run:
	docker run -p 3030:3030 $(DOCKER_IMAGE)

# Stop and remove the Docker container
docker-stop:
	docker stop $(DOCKER_IMAGE_NAME) || true && docker rm $(DOCKER_IMAGE_NAME) || true

# Push the Docker image to a registry
docker-push:
	docker push $(DOCKER_IMAGE)

# Build the Podman image
podman-build:
	podman build -t $(DOCKER_IMAGE) .

# Run the Podman container
podman-run:
	podman run -p 3030:3030 $(DOCKER_IMAGE)

# Stop and remove the Podman container
podman-stop:
	podman stop $(DOCKER_IMAGE_NAME) || true && podman rm $(DOCKER_IMAGE_NAME) || true

# Push the Podman image to a registry
podman-push:
	podman push $(DOCKER_IMAGE)

# Clean up dangling Docker images and containers
docker-cleanup:
	docker image prune -f
	docker container prune -f

# Clean up dangling Podman images and containers
podman-cleanup:
	podman image prune -f
	podman container prune -f
