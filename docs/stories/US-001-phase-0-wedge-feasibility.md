# US-001 Phase 0 Wedge Feasibility Plan

## Status

implemented

## Lane

normal

## Product Contract

Define the smallest Phase 0 implementation slice for Substrate: a drop-in wedge
that can demonstrate storage and diff value while preserving a Git-compatible
text projection. This story should produce a buildable plan, not the full
implementation.

## Relevant Product Docs

- `docs/product/overview.md`
- `docs/product/storage-history.md`
- `docs/product/diff-merge.md`
- `docs/product/compatibility-shell.md`
- `docs/product/verification-query.md`

## Acceptance Criteria

- [x] The selected Phase 0 boundary names exactly one primary surface: local CLI,
  local repository helper, protocol shim, or service.
- [x] The plan defines one measurable storage benchmark and one measurable diff or
  conflict-noise benchmark.
- [x] The plan identifies which Git-compatible behaviors are in scope and which are
  explicitly deferred.
- [x] The plan defines the minimum verification gates before any state can be called
  verified green.
- [x] The plan records open security questions around deduplication isolation rather
  than implementing global deduplication.
- [x] The plan produces follow-up story candidates for the first implementation
  slice.

## Design Notes

- Commands: local CLI is selected as the Phase 0 primary surface.
- Queries: benchmark and inspection queries are limited to storage and diff
  reports before index work.
- API: no public API is accepted by this story.
- Tables: no database schema is accepted by this story.
- Domain rules: intent, materialized artifact, verification result, and text
  projection are the minimum concepts to name.
- UI surfaces: none.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id US-001 --unit 1 --integration 1 --e2e 0 --platform 0`.

| Layer | Expected proof |
| --- | --- |
| Unit | Completed by reviewable plan assertions in `docs/stories/US-001-phase-0-wedge-plan.md`. |
| Integration | Completed by proposed integration proof paths for ingest, projection, benchmark, and verified-green gates. |
| E2E | Not required for this planning story. |
| Platform | A proposed platform proof path for local CLI or repository behavior, if selected. |
| Release | Not required. |

## Harness Delta

- This story converts the Phase 0 suggestion from `product-brief_1.md` into the
  first selected work packet.
- Implementation stories should be created only after this feasibility plan
  chooses the first surface and proof gates.
- Follow-up implementation candidates are listed in
  `docs/stories/US-001-phase-0-wedge-plan.md`.

## Evidence

- `docs/stories/US-001-phase-0-wedge-plan.md`
