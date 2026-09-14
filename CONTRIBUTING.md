# Contributing

## Requirements
- Rust toolchain pinned by `rust-toolchain.toml`.
- Signed commits and tags: `git config --global commit.gpgsign true`.
- Every commit claiming green must reproduce via `cargo xt ci`.

## Setup
git config core.hooksPath .githooks

## Before push
scripts/preflight.sh

## Mastery gate
bash checks/run.sh mastery

## Release
scripts/sign-and-tag.sh vX.Y.Z
scripts/verify-release.sh vX.Y.Z

## Commit style
Conventional Commits: feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert(scope): subject.

## Claim discipline
Every feature lands with a claim, evidence, and a boundary.
