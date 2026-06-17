# US-008 JavaScript Tree-Sitter Diff Parser

## Status

implemented

## Lane

normal

## Product Contract

Substrate can use a real parser-backed normalized diff path for JavaScript and
JSX files. The diff report still keeps line-diff counts and unsupported
fallbacks, but `.js` and `.jsx` files are parsed with tree-sitter JavaScript
before changed-node counts are produced.

## Relevant Product Docs

- `docs/product/diff-merge.md`
- `docs/stories/US-005-structural-diff-spike.md`
- `docs/stories/US-007-typescript-tree-sitter-diff-parser.md`

## Acceptance Criteria

- Add the tree-sitter JavaScript dependency explicitly for JavaScript parsing.
- `substrate diff <left> <right>` recognizes `.js` and `.jsx` files as supported parser-backed inputs.
- JavaScript changed-node counts are produced from tree-sitter parse trees, not a string-only function-block scanner.
- Formatting-only JavaScript churn reports fewer normalized changes than line changes.
- JavaScript function reordering reports no normalized changed nodes.
- JavaScript and JSX logic edits report normalized changed nodes.
- Unsupported non-Rust, non-TypeScript, and non-JavaScript files continue to increment fallback counts.
- The report includes a JavaScript supported-pair count.

## Design Notes

- Commands: `diff <left> <right>`.
- Parser: `tree-sitter` plus `tree-sitter-javascript`.
- Domain rules: parser-backed JavaScript support counts normalized named syntax-node fingerprints; Rust support remains the US-005 function-block placeholder and TypeScript remains the US-007 tree-sitter path.
- UI surfaces: terminal report only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers JavaScript parser counts, formatting churn, function reordering, logic edits, JSX edits, and fallback preservation. |
| Integration | A committed JavaScript fixture is read from disk and included in `substrate diff` reports. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | `cargo run -- diff fixtures/diff-javascript-pairs/before fixtures/diff-javascript-pairs/after` proves the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-008 story row and update the test matrix when proof exists.

## Evidence

- Added explicit dependency: `tree-sitter-javascript = "0.25.0"`.
- `cargo fmt --check` passed.
- `cargo test` passed with 23 tests.
- `cargo build` passed.
- `cargo run --quiet -- diff fixtures/diff-javascript-pairs/before fixtures/diff-javascript-pairs/after` passed and reported:
  - `pair_count: 5`
  - `rust_pair_count: 0`
  - `typescript_pair_count: 0`
  - `javascript_pair_count: 4`
  - `line_diff_changed_lines: 13`
  - `normalized_changed_node_count: 30`
  - `unsupported_file_fallback_count: 1`
  - `semantic_equivalence_claimed: no`
- Formatting-only and function-reordering JavaScript pairs reported `normalized_changed_nodes=0`.
- JavaScript and JSX logic-edit pairs reported parser-backed normalized changed nodes.
- Unsupported `.txt` pair still reported `unsupported_fallback=true`.
