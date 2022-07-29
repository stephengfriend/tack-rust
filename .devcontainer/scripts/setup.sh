#!/usr/bin/env bash

# Create docker group, if it doesn't exist, and add our user to it
: `sudo groupadd -f docker`
sudo usermod -aG docker $USER

# Prepare rust environment
rustup update
rustup install stable
rustup default stable
rustup component add clippy rustfmt
cargo install cargo-watch