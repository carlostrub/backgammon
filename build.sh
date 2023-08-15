#!/bin/sh
# This script is used to build and test the project. If you pass the coveralls
# token, it will also upload the coverage report to coveralls.io.
#
# Usage:
#  build.sh --coveralls <coveralls_token>
#

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

# Create documentation
echo "Create the documentation..."
cargo doc
rm -r /usr/local/www/backgammon/doc/backgammon;
cp -r target/doc/backgammon /usr/local/www/backgammon/doc/backgammon;

# Create README
echo "Create README.md"
cargo readme > README.md

# Checking coverage
echo "Checking coverage..."
if [ -z "$2" ]
then
    echo "No coveralls token provided. Skipping upload."
    cargo tarpaulin
else
    echo "Uploading coverage report to coveralls.io..."
    cargo tarpaulin --coveralls $2
fi


# Checking audit reports
echo "Checking audit reports..."
cargo-audit audit
