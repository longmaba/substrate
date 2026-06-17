# US-007 TypeScript Tree-Sitter Diff Parser

## Status

implemented

## Lane

normal

## Product Contract

Substrate can use a real parser-backed normalized diff path for TypeScript files.
The diff report still keeps line-diff counts and unsupported fallbacks, but `.ts`
and `.tsx` files are parsed with tree-sitter TypeScript before changed-node
counts are produced.

## Relevant Product Docs

- `docs/product/diff-merge.md`
- `docs/stories/US-005-structural-diff-spike.md`

## Acceptance Criteria

- Add tree-sitter dependencies explicitly for TypeScript parsing.
- `substrate diff <left> <right>` recognizes `.ts` and `.tsx` files as supported parser-backed inputs.
- TypeScript changed-node counts are produced from tree-sitter parse trees, not a string-only function-block scanner.
- Formatting-only TypeScript churn reports fewer normalized changes than line changes.
- TypeScript logic edits report normalized changed nodes.
- Unsupported non-Rust and non-TypeScript files continue to increment fallback counts.
- The report includes a TypeScript supported-pair count.

## Design Notes

- Commands: `diff <left> <right>`.
- Parser: `tree-sitter` plus `tree-sitter-typescript`.
- Domain rules: parser-backed TypeScript support counts normalized named syntax-node fingerprints; Rust support remains the US-005 function-block placeholder.
- UI surfaces: terminal report only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers TypeScript parser counts, formatting churn, logic edits, and fallback preservation. |
| Integration | A committed TypeScript fixture is read from disk and included in `substrate diff` reports. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | `cargo run -- diff fixtures/diff-typescript-pairs/before fixtures/diff-typescript-pairs/after` proves the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-007 story row and update the test matrix when proof exists.

## Evidence

- Added explicit dependencies: `tree-sitter = "0.25"` and `tree-sitter-typescript = "0.23"`.
- `cargo fmt --check` passed.
- `cargo test` passed with 21 tests.
- `cargo build` passed.
- `cargo run --quiet -- diff fixtures/diff-typescript-pairs/before fixtures/diff-typescript-pairs/after` passed and reported:
  - `pair_count: 5`
  - `rust_pair_count: 0`
  - `typescript_pair_count: 4`
  - `line_diff_changed_lines: 13`
  - `normalized_changed_node_count: 24`
  - `unsupported_file_fallback_count: 1`
  - `semantic_equivalence_claimed: no`
- Formatting-only and function-reordering TypeScript pairs reported `normalized_changed_nodes=0`.
- TypeScript logic-edit pairs reported parser-backed normalized changed nodes.
- Unsupported `.txt` pair still reported `unsupported_fallback=true`.
