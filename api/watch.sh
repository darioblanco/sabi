#!/bin/sh

# watch.sh

if ! command -v cargo-watch > /dev/null; then
    echo "Installing cargo-watch..."
    cargo install --version 7.6.1 cargo-watch
fi

echo "Running cargo-watch..."
cargo watch -x run
