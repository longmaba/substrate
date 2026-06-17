# Phase 0 Benchmark Results

This benchmark page records measured fixture output for the current local
Substrate wedge. Use it for demos, README claims, and regression comparisons.

## Storage Churn Fixture

Command:

```powershell
cargo run --quiet -- bench fixtures\storage-agent-churn
```

Measured output on this workspace:

```text
fixture: \\?\P:\projects\substrate\fixtures\storage-agent-churn
revision_count: 25
file_count: 50
whole_file_baseline_bytes: 117950
substrate_stored_bytes: 10094
chunk_size_bytes: 128
chunk_count: 950
unique_chunk_count: 80
dedup_ratio: 11.6852
ingest_time_ms: 9
```

Summary:

| Metric | Value |
| --- | ---: |
| Whole-file baseline bytes | 117,950 |
| Substrate stored bytes | 10,094 |
| Bytes avoided | 107,856 |
| Storage reduction | 91.4% |
| Dedup ratio | 11.6852x |
| Local ingest time | 9 ms |

Token-equivalent estimate:

| Estimate | Value |
| --- | ---: |
| Whole-file token-equivalent bytes, 4 bytes/token | 29,488 |
| Substrate token-equivalent bytes, 4 bytes/token | 2,524 |
| Token-equivalent bytes avoided | 26,964 |

This token estimate is intentionally labeled as an estimate. It divides bytes by
4 to provide a rough, tokenizer-neutral planning number. It is not a model
tokenizer run and not a billing measurement.

## Structural Diff Fixtures

JavaScript and JSX command:

```powershell
cargo run --quiet -- diff fixtures\diff-javascript-pairs\before fixtures\diff-javascript-pairs\after
```

Measured output:

```text
pair_count: 5
rust_pair_count: 0
typescript_pair_count: 0
javascript_pair_count: 4
line_diff_changed_lines: 13
normalized_changed_node_count: 30
unsupported_file_fallback_count: 1
semantic_equivalence_claimed: no
```

Important review-noise signals:

- Formatting-only JavaScript churn: `line_changed_lines=6`, `normalized_changed_nodes=0`.
- Function reordering JavaScript churn: `line_changed_lines=4`, `normalized_changed_nodes=0`.
- Logic-edit JavaScript churn: parser-backed normalized nodes are non-zero.
- JSX edit churn: parser-backed normalized nodes are non-zero.

TypeScript command:

```powershell
cargo run --quiet -- diff fixtures\diff-typescript-pairs\before fixtures\diff-typescript-pairs\after
```

Measured output:

```text
pair_count: 5
rust_pair_count: 0
typescript_pair_count: 4
javascript_pair_count: 0
line_diff_changed_lines: 13
normalized_changed_node_count: 24
unsupported_file_fallback_count: 1
semantic_equivalence_claimed: no
```

Important review-noise signals:

- Formatting-only TypeScript churn: `line_changed_lines=6`, `normalized_changed_nodes=0`.
- Function reordering TypeScript churn: `line_changed_lines=4`, `normalized_changed_nodes=0`.
- Logic-edit TypeScript churn: parser-backed normalized nodes are non-zero.

## How To Talk About These Numbers

Use these claims:

- "On the included agent-churn fixture, Substrate stores 91.4% fewer bytes than
  the whole-file baseline."
- "The fixture shows an 11.69x dedup ratio with a 9 ms local ingest run."
- "Formatting-only and function-reordering JS/TS examples drop to zero
  normalized changed nodes while line diffs still show changed lines."

Do not use these claims yet:

- "Substrate is faster than Git."
- "Substrate saves 91.4% of all real-world repository storage."
- "Substrate saves 91.4% of model tokens."
- "Substrate proves semantic equivalence."

The current proof is fixture-level evidence for the Phase 0 wedge. Broader
claims need larger benchmarks and comparisons against named external systems.
