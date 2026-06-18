# 0011 Substrate Repo-Local Binary Releases

Date: 2026-06-18

## Status

Accepted

## Context

Substrate is currently a Rust CLI that users can build with Cargo. That is
acceptable for development, but it blocks normal repository onboarding because
users need a Rust toolchain before they can run `substrate` against a project.
The README already identified packaged binary releases as a product gap.

The requested distribution surface is cross-platform binary automation and a
one-line installer that lets a user install Substrate for use inside a repo.

## Decision

Substrate releases are tag-driven. Pushing a tag matching `v*` runs the release
workflow, verifies the crate, builds platform-specific release binaries, writes
`.sha256` checksums, and creates or updates the matching GitHub Release.

The supported release asset labels are:

- `substrate-macos-arm64`
- `substrate-macos-x64`
- `substrate-linux-x64`
- `substrate-linux-arm64`
- `substrate-windows-x64.exe`

Each binary has a sibling `.sha256` file. The installer treats these names as a
public contract.

The one-line installers install repo-locally instead of globally:

- macOS/Linux: `scripts/bin/substrate`
- Windows: `scripts/bin/substrate.exe`

The installer downloads from the latest release by default, verifies the
checksum before installing, and supports tag/base URL overrides for pinned and
local smoke-test installs. It does not run `substrate init` or modify product
state.

## Alternatives Considered

1. Require users to build with Cargo. Rejected because install friction is the
   gap this story closes.
2. Install a global `substrate` binary on PATH. Rejected because the requested
   workflow is repo usage, and repo-local tools avoid mutating the user's shell
   profile.
3. React only to manually published GitHub Releases. Rejected because tags are
   a clearer source of truth for reproducible binary generation.

## Consequences

Positive:

- Users can install Substrate without Rust.
- Release assets have stable names that installers can resolve without extra
  metadata.
- Repo-local installation keeps the tool easy to inspect and remove.

Tradeoffs:

- GitHub-hosted runners and release permissions become part of release proof.
- Unsupported platforms need explicit installer errors.
- Changing asset names requires coordinated workflow and installer changes.
