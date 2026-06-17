# US-003 Deterministic Storage Benchmark Fixture

## Status

implemented

## Lane

normal

## Product Contract

Substrate provides a deterministic storage benchmark fixture and CLI report that
measure whole-file snapshot bytes against a local chunked storage estimate for
agent-churn-like source revisions. The story proves report reliability and
fixture shape; it does not claim a required storage reduction target.

## Relevant Product Docs

- `docs/product/storage-history.md`
- `docs/product/overview.md`
- `docs/stories/US-001-phase-0-wedge-plan.md`
- `docs/stories/US-002-local-cli-scaffold-repository-store.md`

## Acceptance Criteria

- A committed deterministic fixture contains at least 25 synthetic revisions.
- Each revision contains a small source tree with repeated boilerplate and generated-looking text files.
- Each revision changes only a small section of one or more large text files while most content remains unchanged.
- `substrate bench <fixture-path>` reports whole-file baseline bytes, Substrate stored bytes, chunk count, unique chunk count, dedup ratio, and ingest time.
- The report is deterministic except for elapsed time.
- The benchmark stays local-only and does not add network access, global deduplication, hosted service behavior, or Git protocol emulation.

## Design Notes

- Commands: `bench <fixture-path>`.
- Queries: the command reads revision directories from the fixture path and prints aggregate metrics.
- API: no library API is committed by this story.
- Tables: none.
- Domain rules: revisions are sorted by directory name; whole-file baseline counts every file in every revision; Substrate stored bytes count unique fixed-size chunks across all revision files.
- UI surfaces: terminal report only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers fixture discovery, deterministic metric calculation, and report fields. |
| Integration | The committed fixture is read from disk and produces a non-empty benchmark report. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | `cargo run -- bench fixtures/storage-agent-churn` proves the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-003 story row and update the test matrix when proof exists.

## Evidence

- `fixtures/storage-agent-churn` contains 25 revision directories, `rev-00` through `rev-24`.
- Each revision contains `src/generated_handlers.rs` and `src/generated_schema.rs` with repeated generated-looking source and small revision-specific changes.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `target/debug/deps/substrate-7a6c7f7eae37c210.exe --list` reported 8 tests.
- `target/debug/deps/substrate-7a6c7f7eae37c210.exe` passed 8 tests.
- `cargo run --quiet -- bench fixtures/storage-agent-churn` passed and reported:
  - `revision_count: 25`
  - `file_count: 50`
  - `whole_file_baseline_bytes: 117950`
  - `substrate_stored_bytes: 10094`
  - `chunk_size_bytes: 128`
  - `chunk_count: 950`
  - `unique_chunk_count: 80`
  - `dedup_ratio: 11.6852`
  - `ingest_time_ms: 8` on this run
- `cargo run --quiet -- bench` exited non-zero and reported `bench requires <fixture-path>`.
