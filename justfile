#!/usr/bin/env just --justfile

_CARGO_TARGET_DIR := env_var_or_default("CARGO_TARGET_DIR", "target")

alias t := test
alias l := lint
alias c := check
alias cov := coverage-report
alias r := generate-readme
alias s := setup

default: check lint test

setup:
    cargo install cargo-readme
    cargo install grcov
    cargo install committed
    cargo install cargo-deny
    cargo install cargo-audit --features=fix
    cargo install typos-cli
    cargo install cargo-hack
#   pip install codespell

# Run all the tests
test:
    cargo test


# Check the program with all features enabled.
check:
    cargo check
    cargo hack check --feature-powerset --rust-version
    committed aurora..HEAD --no-merge-commit
    typos
    codespell --skip="target,git,_typos.toml" --ignore-words="{{justfile_directory()}}/.codespellignore"
    cargo deny check
    cargo deny check licenses
    cargo audit

@lint:
    cargo fmt --all -- --check --verbose
    cargo clippy --verbose --all-targets -- -D warnings

# Run the tests, and generate a coverage report
coverage:
    CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="{{_CARGO_TARGET_DIR}}/coverage/data/cargo-test-%p-%m.profraw" cargo test

# Generate the coverage report
coverage-report: coverage
    # Generate the report in html format using grcov
    grcov . --binary-path {{_CARGO_TARGET_DIR}}/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore "../*" -o {{_CARGO_TARGET_DIR}}/coverage/report/ --llvm --ignore "/*"

    # Open the report in the browser
    xdg-open {{_CARGO_TARGET_DIR}}/coverage/report/index.html

# Generate the readme file
generate-readme:
    cargo readme --template _readme.tpl > README.md
    sed -i "s/\*\*Note\*\*/\[!Note\]/g" README.md
    cargo depgraph --all-features --build-deps --target-deps --depth 70 | dot -Tpng > _deps.png
