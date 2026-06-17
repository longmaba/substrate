# US-005 Structural Diff Spike

## Status

implemented

## Lane

normal

## Product Contract

Substrate can run a local diff-noise spike for Rust source pairs and report line
diff noise beside a normalized structural placeholder. The spike selects Rust as
the first language, measures consistently on committed fixtures, reports
unsupported fallbacks, and does not claim semantic equivalence.

## Relevant Product Docs

- `docs/product/diff-merge.md`
- `docs/product/compatibility-shell.md`
- `docs/stories/US-001-phase-0-wedge-plan.md`

## Acceptance Criteria

- A committed fixture contains at least 10 paired before/after examples.
- Fixture pairs include formatting-only churn, function reordering, localized logic edits, and whole-function regeneration.
- `substrate diff <left> <right>` reports line-diff changed line count.
- The same report includes normalized changed-node count for supported Rust files.
- Unsupported-file fallback count is reported explicitly.
- The report includes per-pair notes or classifications from the fixture when available.
- The implementation stays local-only and does not add network access, hosted service behavior, or Git protocol emulation.
- The story does not claim semantic equivalence from normalized structural similarity.

## Design Notes

- Commands: `diff <left> <right>`.
- Queries: none.
- API: no library API is committed by this story.
- Tables: none.
- Domain rules: Rust `.rs` files are supported by a dependency-free normalized function-block comparator; non-Rust files fall back to line-diff-only accounting.
- UI surfaces: terminal report only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers line-diff counts, normalized Rust function-block counts, fallback accounting, and report fields. |
| Integration | The committed fixture is read from disk and produces a consistent report across at least 10 pairs. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | `cargo run -- diff fixtures/diff-rust-pairs/before fixtures/diff-rust-pairs/after` proves the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-005 story row and update the test matrix when proof exists.

## Evidence

- `fixtures/diff-rust-pairs` contains 10 Rust before/after pairs plus one unsupported text fallback pair.
- `fixtures/diff-rust-pairs/NOTES.tsv` records classifications and notes for every pair.
- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo build` passed.
- `cargo run --quiet -- diff fixtures/diff-rust-pairs/before fixtures/diff-rust-pairs/after` passed and reported:
  - `pair_count: 11`
  - `rust_pair_count: 10`
  - `line_diff_changed_lines: 45`
  - `normalized_changed_node_count: 5`
  - `unsupported_file_fallback_count: 1`
  - `semantic_equivalence_claimed: no`
- Formatting-only and function-reordering Rust pairs reported `normalized_changed_nodes=0` while still showing line churn.
- Localized logic and whole-function regeneration pairs reported normalized changed nodes.
- `pair-11-unsupported.txt` reported `unsupported_fallback=true`.
