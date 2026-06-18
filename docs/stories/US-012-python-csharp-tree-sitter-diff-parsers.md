# US-012 Python And C# Tree-Sitter Diff Parsers

## Status

implemented

## Lane

normal

## Product Contract

Substrate can use parser-backed normalized diff paths for Python and C# files.
The diff report still keeps line-diff counts and unsupported fallbacks, but
`.py` and `.cs` files are parsed with tree-sitter before changed-node counts are
produced.

## Relevant Product Docs

- `docs/product/diff-merge.md`
- `docs/stories/US-005-structural-diff-spike.md`
- `docs/stories/US-010-ignore-aware-ingest-parser-registry.md`

## Acceptance Criteria

- Add tree-sitter dependencies explicitly for Python and C# parsing.
- `substrate diff <left> <right>` recognizes `.py` and `.cs` files as supported parser-backed inputs.
- Python and C# changed-node counts are produced from tree-sitter parse trees.
- Formatting-only Python and C# churn reports fewer normalized changes than line changes.
- Python and C# logic edits report normalized changed nodes.
- Unsupported files continue to increment fallback counts.
- The report includes Python and C# supported-pair counts.

## Design Notes

- Commands: `diff <left> <right>`.
- Parsers: `tree-sitter-python` and `tree-sitter-c-sharp`.
- Domain rules: parser-backed Python and C# support counts normalized named syntax-node fingerprints; Rust support remains the US-005 function-block placeholder.
- UI surfaces: terminal report only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | `cargo test` covers parser counts, formatting churn, logic edits, fallback preservation, and registry detection. |
| Integration | Committed Python and C# fixtures are read from disk and included in `substrate diff` reports. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | Fixture `cargo run -- diff` commands prove the shell surface works locally. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-012 story row and update the test matrix when proof exists.

## Evidence

- Added explicit dependencies: `tree-sitter-python = "0.25.0"` and `tree-sitter-c-sharp = "0.23.5"`.
- Added parser registry entries and report counters for `.py` and `.cs` files.
- Added committed Python and C# fixture directories with unsupported fallback pairs.
- Formatting-only and function-reordering Python and C# pairs reported `normalized_changed_nodes=0`.
- Python and C# logic-edit pairs reported parser-backed normalized changed nodes.
- Unsupported `.txt` pairs still reported `unsupported_fallback=true`.
- `cargo fmt --check` passed.
- `cargo test` passed with 29 tests.
- `cargo build` passed.
- `cargo run --quiet -- diff fixtures/diff-python-pairs/before fixtures/diff-python-pairs/after` passed and reported `python_pair_count: 4`, `normalized_changed_node_count: 22`, and `unsupported_file_fallback_count: 1`.
- `cargo run --quiet -- diff fixtures/diff-csharp-pairs/before fixtures/diff-csharp-pairs/after` passed and reported `csharp_pair_count: 4`, `normalized_changed_node_count: 24`, and `unsupported_file_fallback_count: 1`.
- `scripts/bin/harness-cli.exe story verify-all` passed with 12 stories verified.
