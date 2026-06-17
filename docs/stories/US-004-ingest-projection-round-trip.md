# US-004 Ingest And Projection Round Trip

## Status

implemented

## Lane

normal

## Product Contract

Substrate can ingest supported text files from an initialized local working tree,
store their bytes in the repo-local `.substrate/` store with content-addressed
object records, write a state manifest, and project that state back to a text
tree byte-for-byte.

## Relevant Product Docs

- `docs/product/storage-history.md`
- `docs/product/compatibility-shell.md`
- `docs/product/verification-query.md`
- `docs/stories/US-001-phase-0-wedge-plan.md`
- `docs/stories/US-002-local-cli-scaffold-repository-store.md`

## Acceptance Criteria

- `substrate ingest <path>` requires an initialized `.substrate/` store under the target path.
- Ingest skips the `.substrate/` directory itself.
- Ingest stores supported text file bytes as content-addressed local objects.
- Ingest writes a deterministic manifest that maps relative file paths to content ids and byte lengths.
- `substrate project <state-id> --out <path>` recreates the manifest's supported text files under the output path.
- Ingest followed by projection is byte-for-byte stable for supported text files.
- The implementation remains local-only and does not add network access, global deduplication, hosted service behavior, or Git protocol emulation.

## Design Notes

- Commands: `ingest <path>`, `project <state-id> --out <path>`.
- Queries: projection reads `.substrate/states/<state-id>.manifest` and `.substrate/objects/<content-id>`.
- API: no library API is committed by this story.
- Tables: none.
- Domain rules: content ids are deterministic local hashes of file bytes; state ids are deterministic hashes of manifest content; supported text files exclude files containing NUL bytes.
- UI surfaces: terminal output only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers manifest serialization, text-file filtering, and state id behavior. |
| Integration | Filesystem tests prove ingest writes objects/manifests and projection recreates text files byte-for-byte. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | `cargo run -- init`, `cargo run -- ingest`, and `cargo run -- project --out` prove the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-004 story row and update the test matrix when proof exists.

## Evidence

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo build` passed and refreshed `target/debug/substrate.exe` for CLI checks.
- `target/debug/deps/substrate-7a6c7f7eae37c210.exe --list` reported 12 tests.
- `target/debug/deps/substrate-7a6c7f7eae37c210.exe` passed 12 tests.
- CLI round trip from a temp repo passed:
  - `substrate init .` created `.substrate/store.toml`.
  - `substrate ingest .` reported `ingested: yes`, `file_count: 2`, `object_count: 2`, `skipped_binary_count: 0`, and state id `sa4ba53278cd8bb4e` for that temp fixture.
  - `substrate project <state-id> --out <temp-out>` reported `projected: yes` and `file_count: 2`.
  - Byte comparison for `src/main.rs` returned `main_equal=True`.
  - Byte comparison for `docs/note.txt` returned `note_equal=True`.
