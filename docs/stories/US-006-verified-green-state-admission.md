# US-006 Verified-Green State Admission

## Status

implemented

## Lane

normal

## Product Contract

Substrate persists local verification metadata for ingested states and only marks
a state `verified-green` after the applicable Phase 0 local gates pass. States
created by ingest start as `candidate`; admission upgrades them only after
manifest integrity, stored object checks, projection stability, benchmark
completion, and configured local verification proof are recorded.

## Relevant Product Docs

- `docs/product/verification-query.md`
- `docs/product/storage-history.md`
- `docs/stories/US-001-phase-0-wedge-plan.md`
- `docs/stories/US-004-ingest-projection-round-trip.md`

## Acceptance Criteria

- Ingest persists `candidate` metadata for each new state.
- `substrate state <state-id>` reports the persisted label and verification metadata.
- `substrate verify <state-id> --out <path> --bench <fixture-path>` runs the Phase 0 local gates for that state.
- Verification checks manifest parse, stored-object referential integrity, content id and byte length integrity, projection output, ingest/projection byte-for-byte stability against the current working tree, and benchmark completion.
- A state is marked `verified-green` only when every required gate passes.
- Failed verification keeps or returns the state label to `candidate` and records failed gates.
- The implementation stays repo-local and does not add network access, global deduplication, hosted service behavior, auth, authorization, or Git protocol emulation.

## Design Notes

- Commands: `state <state-id>`, `verify <state-id> --out <path> --bench <fixture-path>`.
- Queries: `state` reads `.substrate/verification/<state-id>.txt`.
- API: no library API is committed by this story.
- Tables: none.
- Domain rules: `candidate` is the default state label after ingest; `verified-green` is admitted only after all required gates pass.
- UI surfaces: terminal output only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers verification metadata parsing, candidate creation, gate pass/fail behavior, and state report output. |
| Integration | Filesystem tests prove ingest writes candidate metadata and verify upgrades only valid states. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | `cargo run -- init`, `ingest`, `state`, and `verify --out --bench` prove the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-006 story row and update the test matrix when proof exists.

## Evidence

- `cargo fmt --check` passed.
- `cargo test` passed with 19 tests.
- `cargo build` passed.
- CLI positive admission path passed from a temp repo:
  - `substrate init .` created the local store.
  - `substrate ingest .` wrote state `s94e45b2701237128` as `candidate` with `file_count: 1` and `object_count: 1`.
  - `substrate state s94e45b2701237128` reported `label: candidate` before verification.
  - `substrate verify s94e45b2701237128 --out <temp-out> --bench fixtures/storage-agent-churn` reported `verified: true` and `label: verified-green`.
  - Passing gates were `manifest_parse`, `object_integrity`, `projection_stability`, and `benchmark_completion`.
  - `substrate state s94e45b2701237128` reported `label: verified-green` after verification.
- CLI negative admission path passed from a temp repo:
  - After tampering with the stored object, `substrate verify ...` reported `verified: false` and exited `1`.
  - Failed gates included `object_integrity=false` and `projection_stability=false`.
  - `substrate state <state-id>` reported `label: candidate` after failed verification.
