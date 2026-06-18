# Storage And History

## Contract

Substrate should reduce agent-churn storage and transfer costs by storing code
and metadata at a finer granularity than whole-file snapshots. The brief points
to content-defined chunking, content addressing, sub-file deduplication,
change-based history, tiered retention, and lazy or partial fetch as intended
directions.

These are product goals, not yet selected implementation details.

## Storage Expectations

The storage layer should eventually support:

- Content-addressed artifacts.
- Sub-file deduplication for high-redundancy source changes.
- Durable materialized code artifacts.
- Versioned intent and verification metadata.
- Fetch by symbol, dependency set, or query result where possible.
- Text projection for compatibility surfaces.

## Benchmark Claim Boundaries

Phase 0 benchmark reports distinguish raw unique chunk storage from experimental
delta estimates. Raw chunks are the implemented local store behavior today. The
sorted unique chunk prefix/suffix delta number is benchmark evidence for a
future storage mode, not a selected production format.

Packed Git comparisons must be labeled separately from active-work Git loose
comparisons. Git packed storage applies delta compression and zlib after an
explicit maintenance step; raw Phase 0 Substrate chunks do not. Do not claim raw
Substrate chunks beat packed Git unless a current benchmark proves that exact
comparison.

## History Expectations

The history model should eventually support:

- Change-based records rather than only human-curated commit snapshots.
- Garbage collection or compaction of exploratory agent churn.
- Verified checkpoints for long-running speculative work.
- Undoable operations and recoverable projections.
- Bisection over known-green states.

## Retention Questions

- Which candidate states are retained, compacted, or pruned?
- What metadata must survive compaction for auditability?
- Which materialized artifacts are mandatory to keep even when intent is stored?
- How does retention interact with Git-compatible exports?

## Security Questions

Global or cross-repo deduplication can create side-channel risks. Before any
shared store is implemented, the product needs explicit boundaries for tenant,
repository, workspace, and local-only dedup scopes.
