#!/usr/bin/env bash
# Script for running check on your rust projects.
set -e
set -x
set -u

run_doc() {
    rustup component add rust-docs
    cargo doc
}

run_fmt() {
    rustup component add rustfmt
    cargo fmt --check
}

run_clippy() {
    rustup component add clippy-preview
    cargo clippy -- -D warnings
}

declare -A FEATURES
FEATURES=(
    "bling"
    "compact"
    "mini"
    "high"
)

run_check() {
    cargo check
    for FEAT in "${FEATURES[@]}"
    do
        cargo check --no-default-features --features "$FEAT"
    done
}

run_test() {
    cargo test
    for FEAT in "${FEATURES[@]}"
    do
        cargo test --no-default-features --features "$FEAT"
    done
}

run_build() {
    cargo build
    for FEAT in "${FEATURES[@]}"
    do
        cargo build --no-default-features --features "$FEAT"
    done
}

run_build_release() {
    cargo build --release
    for FEAT in "${FEATURES[@]}"
    do
        cargo build --release --no-default-features --features "$FEAT"
    done
}

case $1 in
    doc)
        run_doc
        ;;
    fmt)
        run_fmt
        ;;
    check)
        run_check
        ;;
    clippy)
        run_clippy
        ;;
    test)
        run_test
        ;;
    build)
        run_build
        ;;
    build-release)
        run_build_release
        ;;
esac
