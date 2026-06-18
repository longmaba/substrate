# US-011 Substrate Release Automation

## Current Behavior

Substrate can be built locally with Cargo, but the repo has no GitHub release
workflow, no packaged binary assets, and no one-line installer for users who do
not already have a Rust toolchain.

## Target Behavior

Pushing a `v*` tag builds and publishes checksum-protected Substrate binaries
for Windows x64, macOS x64/arm64, and Linux x64/arm64. Users can install the
right binary into their current repository with a one-line shell or PowerShell
installer.

## Affected Users

- Human engineers trying Substrate in an existing repository.
- Coding agents that need a repo-local `substrate` command without compiling
  from source.
- Maintainers publishing Substrate releases.

## Affected Product Docs

- `README.md`
- `scripts/README.md`
- `docs/product/compatibility-shell.md`

## Non-Goals

- Global PATH installation.
- Running `substrate init` during install.
- Git protocol compatibility or hosted Git integration.
- Release automation for non-CLI artifacts.
