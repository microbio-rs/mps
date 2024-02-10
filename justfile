#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

alias r := ready
alias c := coverage

# When ready, run the same CI commands
ready:
  git diff --exit-code --quiet
  just fmt
  just check
  just test
  just lint
  git status

# --no-vcs-ignores: cargo-watch has a bug loading all .gitignores, including the ones listed in .gitignore
# use .ignore file getting the ignore list
# Run `cargo watch`
watch command:
  cargo watch --no-vcs-ignores -x '{{command}}'

# Format all files
fmt:
  cargo fmt
  taplo format

# Run cargo check
check:
  cargo ck

# Run all the tests
test:
  cargo test

# Lint the whole project
lint:
  cargo lint -- --deny warnings

# Run all the conformance tests.
coverage:
  cargo coverage

# Get code coverage
codecov:
  cargo codecov --html

# Upgrade all Rust dependencies
upgrade:
  cargo upgrade --incompatible
