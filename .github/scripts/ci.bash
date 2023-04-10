#!/usr/bin/env bash
# Script for running check on your rust projects.
set -e
set -x
set -u

declare -A FEATURES
FEATURES=(
    "bling"
    "compact"
    "mini"
    "high"
)
declare -A KEYMAPS
KEYMAPS=(
    "keymap_basic"
    "keymap_borisfaure"
)


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
    for FEAT in "${FEATURES[@]}"
    do
        for KEYMAP in "${KEYMAPS[@]}"
        do
            cargo clippy --no-default-features --features "$FEAT,$KEYMAP" -- -D warnings
        done
    done
}

run_check() {
    cargo check
    for FEAT in "${FEATURES[@]}"
    do
        for KEYMAP in "${KEYMAPS[@]}"
        do
            cargo check --no-default-features --features "$FEAT,$KEYMAP"
        done
    done
}

run_test() {
    cargo test
    for FEAT in "${FEATURES[@]}"
    do
        for KEYMAP in "${KEYMAPS[@]}"
        do
            cargo test --no-default-features --features "$FEAT,$KEYMAP"
        done
    done
}

run_build() {
    cargo build
    for FEAT in "${FEATURES[@]}"
    do
        for KEYMAP in "${KEYMAPS[@]}"
        do
            cargo build --no-default-features --features "$FEAT,$KEYMAP"
        done
    done
}

run_build_release() {
    cargo build --release
    for FEAT in "${FEATURES[@]}"
    do
        for KEYMAP in "${KEYMAPS[@]}"
        do
            cargo build --release --no-default-features --features "$FEAT,$KEYMAP"
        done
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
