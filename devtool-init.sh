#!/bin/bash

# Check if pre-commit is installed
if ! command -v pre-commit &>/dev/null; then
    echo "pre-commit not found. Installing..."
    pip3 install pre-commit
else
    echo "pre-commit already installed"
fi

# Check if rustup is installed
if ! command -v rustup &>/dev/null; then
    echo "rustup not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
else
    echo "rustup already installed"
fi

if ! command -v cargo-binstall &>/dev/null; then
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
else
    echo "cargo-binstall already installed"
fi

# Check if nextest is installed
if ! command -v cargo-nextest &>/dev/null; then
    echo "cargo-nextest not found. Installing..."
    # cargo install cargo-nextest --locked
    cargo binstall -y cargo-nextest
else
    echo "cargo-nextest already installed"
fi

# Check if cargo-deny is installed
if ! command -v cargo-deny &>/dev/null; then
    echo "cargo-deny not found. Installing..."
    # cargo install cargo-deny --locked
    cargo binstall -y cargo-deny

else
    echo "cargo-deny already installed"
fi

# Check if typos is installed
if ! command -v typos &>/dev/null; then
    echo "typos not found. Installing..."
    cargo binstall -y typos-cli
    # cargo install typos-cli
else
    echo "typos already installed"
fi

# Check if sqlx-cli is installed
if ! command -v sqlx &>/dev/null; then
    echo "sqlx-cli not found. Installing..."
    cargo install sqlx-cli --no-default-features --features native-tls,postgres --locked
else
    echo "sqlx-cli already installed"
fi

# Check if sqlx-cli is installed, rerun when src code changed: `cargo watch -c -w src -x run`
if ! command -v cargo-watch &>/dev/null; then
    echo "cargo-watch not found. Installing..."
    # cargo install cargo-watch
    cargo binstall -y cargo-watch
else
    echo "cargo-watch already installed"
fi

# Install pre-commit hooks
echo "Installing pre-commit hooks..."
pre-commit install
