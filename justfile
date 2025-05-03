set dotenv-load := true

# List all available commands
default:
    @just --list

alias run := dev
dev:
    cargo run

build:
    cargo build

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
