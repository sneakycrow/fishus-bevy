set dotenv-load := true

# List all available commands
default:
    @just --list

dev:
    cargo run

run:
    cargo run --release

build:
    cargo build --release

# Run full CI testing suite including tests, formatting, and clippy
check: build fmt clippy test

alias t := test
# Run all unit tests with verbose output
test:
    cargo test --verbose

alias format := fmt
# Check code formatting matches rustfmt style
fmt:
    cargo fmt --all -- --check

alias lint := clippy
# Run Clippy linter and treat warnings as errors
clippy:
    cargo clippy -- -D warnings
