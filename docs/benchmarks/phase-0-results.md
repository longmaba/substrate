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
substrate_delta_stored_bytes: 6566
delta_dedup_ratio: 17.9638
delta_encoding: sorted-unique-chunk-prefix-suffix-experiment
```

Summary:

| Metric | Value |
| --- | ---: |
| Whole-file baseline bytes | 117,950 |
| Substrate stored bytes | 10,094 |
| Experimental delta stored bytes | 6,566 |
| Bytes avoided | 107,856 |
| Storage reduction | 91.4% |
| Dedup ratio | 11.6852x |
| Experimental delta dedup ratio | 17.9638x |
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

The delta metric is an experiment over sorted unique chunks. It keeps the raw
Phase 0 chunk metric intact, then estimates how many payload bytes remain if a
chunk can be reconstructed from the previous sorted unique chunk plus only its
changed middle bytes. It does not use zlib, does not emulate Git packfiles, and
does not prove a production storage format.

## Medium Git Comparison Fixture

The untracked local medium fixture was used to compare raw Substrate chunks, the
experimental delta estimate, Git loose objects during active work, and packed
Git after explicit maintenance.

Command:

```powershell
cargo run --quiet -- bench bench-medium\fixture
```

Measured Substrate output on this workspace:

```text
revision_count: 40
file_count: 120
whole_file_baseline_bytes: 415872
substrate_stored_bytes: 36818
chunk_size_bytes: 128
chunk_count: 3296
unique_chunk_count: 307
dedup_ratio: 11.2953
ingest_time_ms: 26
substrate_delta_stored_bytes: 23845
delta_dedup_ratio: 17.4406
delta_encoding: sorted-unique-chunk-prefix-suffix-experiment
```

Comparison summary:

| Strategy | Bytes | Notes |
| --- | ---: | --- |
| Whole-file baseline | 415,872 | Every revision file retained in full. |
| Git loose | 88,124 | Active-work Git object state before GC. |
| Raw Substrate chunks | 36,818 | Phase 0 unique 128-byte chunk payloads. |
| Git packed | 28,866 | After explicit aggressive repack with delta + zlib. |
| Experimental Substrate delta | 23,845 | Sorted unique chunk prefix/suffix payload estimate. |

Packed Git is 1.28x smaller than raw Substrate chunks on this fixture, so broad
"Substrate beats Git storage" language is still incorrect. The experimental
delta estimate is 1.21x smaller than packed Git on the same fixture, which is
evidence that a delta-oriented Substrate storage mode is worth pursuing. It is
not a production storage claim until the format, metadata overhead, projection
path, and broader benchmarks are implemented.

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
- "The experimental sorted-chunk delta estimate reduces the included fixture
  from 10,094 raw chunk bytes to 6,566 estimated payload bytes."
- "Formatting-only and function-reordering JS/TS examples drop to zero
  normalized changed nodes while line diffs still show changed lines."

Do not use these claims yet:

- "Substrate is faster than Git."
- "Substrate raw chunks beat packed Git."
- "The experimental delta estimate is a production storage format."
- "Substrate saves 91.4% of all real-world repository storage."
- "Substrate saves 91.4% of model tokens."
- "Substrate proves semantic equivalence."

The current proof is fixture-level evidence for the Phase 0 wedge. Broader
claims need larger benchmarks and comparisons against named external systems.
