# US-002 Local CLI Scaffold And Repository Store

## Status

implemented

## Lane

normal

## Product Contract

Substrate provides the first runnable Phase 0 surface as a local CLI. The CLI can
initialize a repo-local `.substrate/` store and report store status without
network access, hosted service behavior, Git protocol emulation, or global
deduplication.

## Relevant Product Docs

- `docs/product/overview.md`
- `docs/product/storage-history.md`
- `docs/product/compatibility-shell.md`
- `docs/product/verification-query.md`
- `docs/stories/US-001-phase-0-wedge-plan.md`
- `docs/decisions/0009-phase-0-local-cli-boundary.md`

## Acceptance Criteria

- A Rust CLI package exists for the Phase 0 Substrate wedge.
- `substrate init <path>` creates a repo-local `.substrate/` directory under the target path.
- The initialized store contains enough metadata to identify the local store version and root path.
- `substrate status [path]` reports whether a target path has an initialized Substrate store.
- The CLI rejects missing commands, unknown commands, and missing command arguments with non-zero exits.
- The implementation does not add network behavior, global deduplication, Git protocol emulation, auth, authorization, or hosted service state.

## Design Notes

- Commands: `init <path>`, `status [path]`.
- Queries: status reads only the target path and `.substrate/store.toml` metadata.
- API: local CLI only; no library API is committed by this story.
- Tables: none.
- Domain rules: Phase 0 store data is repo-local under `.substrate/`; repeated init is idempotent when existing metadata is valid.
- UI surfaces: terminal output only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers CLI argument errors, store metadata parsing, and status behavior. |
| Integration | Filesystem tests prove `init` creates `.substrate/store.toml` and `status` reads it. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | `cargo run -- status .` and `cargo run -- init <fixture>` prove the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Confirm Rust as the Phase 0 CLI implementation stack in a decision record.
- Add a durable US-002 story row and update the test matrix when proof exists.

## Evidence

- `cargo fmt --check` passed.
- `cargo test` passed.
- `target/debug/deps/substrate-7a6c7f7eae37c210.exe --list` reported 6 tests.
- `target/debug/deps/substrate-7a6c7f7eae37c210.exe` passed 6 tests covering argument errors, init, idempotent init, status, and metadata round trips.
- `cargo run --quiet -- init <temp-fixture>` created `.substrate/store.toml` and reported `initialized: yes`.
- `cargo run --quiet -- status <temp-fixture>` reported `initialized: yes` with `version: 1`.
- `cargo run --quiet -- status <plain-temp-dir>` reported `initialized: no`.
- `cargo run --quiet -- init` exited `2` and reported `init requires <path>`.
