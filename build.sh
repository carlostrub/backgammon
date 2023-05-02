#!/bin/sh
# This script is used to build and test the project.
#
# Usage:
#  build.sh

# Static Code Analysis
echo "Running static code analysis..."
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Building the project
echo "Building project..."
cargo build -v

# Running tests
echo "Running tests..."
cargo test -v --no-fail-fast

# Checking coverage
#echo "Checking coverage..."
#cargo-tarpaulin

# Checking audit reports
echo "Checking audit reports..."
cargo-audit audit
