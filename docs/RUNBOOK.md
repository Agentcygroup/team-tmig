# Runbook

## Daily
    cargo xt ci

## Before push
    scripts/preflight.sh

## Mastery gate
    bash checks/run.sh mastery

## Release
    scripts/sign-and-tag.sh vX.Y.Z
    scripts/verify-release.sh vX.Y.Z

## Recover a bad push
    git revert <sha> && git push origin main
    # never force-push main (.githooks/pre-push blocks it)
