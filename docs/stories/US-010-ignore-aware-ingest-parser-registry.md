# US-010 Ignore-Aware Ingest And Parser Registry

## Status

implemented

## Lane

normal

## Product Contract

Substrate ingestion skips local-only project state before writing manifests, and
diff language support is declared through a registry table instead of an
extension-dispatch chain. This makes existing-repo onboarding safer and makes the
next parser-backed language addition smaller.

## Relevant Product Docs

- `docs/product/storage-history.md`
- `docs/product/diff-merge.md`
- `docs/guides/existing-github-repo.md`

## Acceptance Criteria

- Ingest skips `.substrate` and common local-only directories such as `.git`,
  `node_modules`, `target`, `dist`, build outputs, and cache folders.
- Ingest honors root `.gitignore` patterns for files and directories, including
  basic wildcard and negation support.
- Parser-backed diff support is declared through one registry table containing
  supported extensions and changed-node functions.
- Existing Rust, TypeScript, JavaScript, JSX, and unsupported fallback behavior
  remains unchanged.
- README and existing-repo guide move the first two TODO items out of the TODO
  list and describe the implemented behavior.

## Design Notes

- Commands: `ingest <path>` and `diff <left> <right>`.
- Ignore support is intentionally basic and root-scoped; full Git ignore parity
  remains future work.
- No new dependency was added for ignore matching.
- Parser registry is internal and static for Phase 0.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers ignored ingest paths and parser registry detection. |
| Integration | Existing fixture-backed diff and ingest/project tests continue to pass. |
| E2E | Not required; there is no hosted user flow. |
| Platform | `cargo build`, fixture diff commands, and Harness story verification prove the local CLI remains usable. |
| Release | Public repo push after verification. |

## Harness Delta

- Add a durable US-010 story row and update the test matrix when proof exists.
- Close the Harness backlog item for ignore-aware ingest if verification passes.

## Evidence

- Added ignore-aware working-tree collection with root `.gitignore` parsing and default local directory skips.
- Added a static diff language registry for Rust, TypeScript, and JavaScript parser-backed paths.
- Added regression tests for ignored ingest paths and parser registry detection.
- `cargo fmt --check` passed.
- `cargo test` passed with 25 tests.
- `cargo build` passed.
- `python C:\Users\longm\.codex\skills\.system\skill-creator\scripts\quick_validate.py skills/substrate` passed.
- JavaScript and TypeScript fixture diff commands passed.
- `scripts/bin/harness-cli.exe story verify-all` passed with 10 stories.
