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

# Check if nextest is installed
if ! command -v cargo-nextest &>/dev/null; then
    echo "cargo-nextest not found. Installing..."
    cargo install cargo-nextest --locked
else
    echo "cargo-nextest already installed"
fi

# Check if cargo-deny is installed
if ! command -v cargo-deny &>/dev/null; then
    echo "cargo-deny not found. Installing..."
    cargo install cargo-deny --locked
else
    echo "cargo-deny already installed"
fi

# Check if typos is installed
if ! command -v typos &>/dev/null; then
    echo "typos not found. Installing..."
    cargo install typos-cli
else
    echo "typos already installed"
fi

# Install pre-commit hooks
echo "Installing pre-commit hooks..."
pre-commit install
