# US-014 Compressed Chunk Storage Gap

## Status

implemented

## Lane

normal

## Product Contract

Substrate Phase 0 proves raw chunk deduplication, but the medium benchmark shows
that Git can still store less data after an aggressive pack step. The storage
benchmark now reports an experimental sorted unique chunk prefix/suffix delta
estimate so the gap can be measured without changing the raw Phase 0 store.

This story does not claim raw Substrate chunks beat packed Git. It records Git
packed storage as the current post-maintenance byte-efficiency comparison target
and Git loose storage as the active-work comparison target. The new delta metric
is benchmark evidence for a possible future storage mode, not a production
format.

## Relevant Product Docs

- `docs/product/storage-history.md`
- `docs/product/verification-query.md`
- `docs/benchmarks/phase-0-results.md`

## Acceptance Criteria

- The medium benchmark result is recorded as a baseline: Git packed storage is
  smaller than raw Substrate chunks after aggressive repack in the current
  synthetic agent-churn comparison.
- At least one compression or delta-storage approach is prototyped or rejected
  with measured evidence. Done: sorted unique chunk prefix/suffix delta estimate.
- The benchmark report distinguishes active-work Git loose comparisons from
  post-maintenance Git packed comparisons. Done in `docs/benchmarks/phase-0-results.md`.
- Any selected storage change preserves deterministic projection of stored
  states back to ordinary files. Done: no persisted store format or projection
  behavior changed; the new output is benchmark-only.
- Product docs state what Substrate does and does not claim against packed Git.
  Done in `docs/product/storage-history.md` and benchmark claim guidance.

## Design Notes

- Commands: extended `bench <fixture-path>` with `substrate_delta_stored_bytes`,
  `delta_dedup_ratio`, and `delta_encoding` output.
- Queries: no user-facing query command is required for the first pass.
- API: CLI output has additional experimental fields; raw `substrate_stored_bytes`
  remains unchanged.
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

Adds benchmark fields and documentation for the packed-Git comparison gap while
keeping raw Phase 0 chunk storage distinct from the experimental delta estimate.

## Evidence

Initial action item from the medium benchmark report:

- Git packed after aggressive maintenance: 28.9 KB.
- Raw Substrate unique chunks: 36.8 KB.
- Git packed is currently 1.28x smaller than raw Substrate chunks in that
  benchmark.
- Substrate Phase 0 applies neither zlib compression nor delta encoding to its
  unique chunk store.

Implementation evidence:

- `cargo run --quiet -- bench fixtures\storage-agent-churn` reported raw
  Substrate chunks at 10,094 bytes and experimental delta at 6,566 bytes.
- `cargo run --quiet -- bench bench-medium\fixture` reported raw Substrate
  chunks at 36,818 bytes and experimental delta at 23,845 bytes.
- The medium fixture comparison records Git loose at 88,124 bytes and Git packed
  at 28,866 bytes, so raw chunks still do not beat packed Git while the delta
  experiment does on this fixture.
- `cargo fmt --check` passed.
- `cargo test` passed with 30 tests.
