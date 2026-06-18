# US-014 Compressed Chunk Storage Gap

## Status

planned

## Lane

normal

## Product Contract

Substrate Phase 0 proves raw chunk deduplication, but the medium benchmark shows
that Git can still store less data after an aggressive pack step. The next
storage story must evaluate whether compressed chunks, delta encoding, or a
combined approach can close that gap without weakening Substrate's local,
queryable, projection-safe store contract.

This story does not claim Substrate currently beats packed Git. It turns the
benchmark result into an explicit follow-up: Git packed storage is the current
comparison target for post-maintenance byte efficiency, while Git loose storage
remains the active-work comparison target.

## Relevant Product Docs

- `docs/product/storage-history.md`
- `docs/product/verification-query.md`
- `docs/benchmarks/phase-0-results.md`

## Acceptance Criteria

- The medium benchmark result is recorded as a baseline: Git packed storage is
  smaller than raw Substrate chunks after aggressive repack in the current
  synthetic agent-churn comparison.
- At least one compression or delta-storage approach is prototyped or rejected
  with measured evidence.
- The benchmark report distinguishes active-work Git loose comparisons from
  post-maintenance Git packed comparisons.
- Any selected storage change preserves deterministic projection of stored
  states back to ordinary files.
- Product docs state what Substrate does and does not claim against packed Git.

## Design Notes

- Commands: likely extend `bench` or add a storage experiment command for
  compressed chunk accounting.
- Queries: no user-facing query command is required for the first pass.
- API: CLI output may need additional fields for compressed or delta storage
  metrics if the experiment graduates into product behavior.
- Tables: no Harness schema change is expected.
- Domain rules: raw, compressed, and delta-derived storage metrics must remain
  labeled separately so benchmarks are not apples-to-oranges.
- UI surfaces: benchmark docs or generated reports only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id US-014 --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | Storage accounting tests for any new compression or delta metric. |
| Integration | Reproducible benchmark run comparing whole-file baseline, Git loose, Git packed, raw Substrate chunks, and any new Substrate storage mode. |
| E2E | Not expected unless a public workflow changes. |
| Platform | Windows PowerShell benchmark smoke; shell portability check if scripts are added. |
| Release | Not expected until the feature changes packaged CLI behavior. |

## Harness Delta

Adds a planned normal-lane story for the benchmark action item discovered while
reviewing the medium storage comparison.

## Evidence

Initial action item from the medium benchmark report:

- Git packed after aggressive maintenance: 28.9 KB.
- Raw Substrate unique chunks: 36.8 KB.
- Git packed is currently 1.28x smaller than raw Substrate chunks in that
  benchmark.
- Substrate Phase 0 applies neither zlib compression nor delta encoding to its
  unique chunk store.
