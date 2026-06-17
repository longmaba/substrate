# Product Docs

This directory holds the living product contract for Substrate. The original
brief remains source input; these files are the current operating surface for
future stories and implementation work.

## Current Product Contract

- `spec-intake-substrate.md` - intake summary, candidate epics, validation
  shape, and open decisions from `product-brief_1.md`.
- `overview.md` - product direction, users, problem contract, principles, and
  success metrics.
- `intent-model.md` - intent graph, provenance, regeneration, and acceptance
  boundaries.
- `storage-history.md` - storage, history, retention, and deduplication risks.
- `diff-merge.md` - structural diff, semantic versioning units, merge, and
  conflict model.
- `verification-query.md` - verification-in-store, query-first access, and
  speculative search.
- `compatibility-shell.md` - Git-compatible projection and emulation risks.

## User-Facing Guides

- `../guides/existing-github-repo.md` - how to try Substrate beside an existing
  GitHub repository with current Phase 0 limits.
- `../benchmarks/phase-0-results.md` - measured fixture results and safe public
  benchmark language.

## Generic Harness Guidance

This directory started generic and mostly empty in Harness v0.

When a user provides a project spec, derive smaller product contract files here
instead of keeping one large spec as the living plan. Name files by the product
domains that actually exist in that spec, for example `overview.md`,
`billing.md`, `workflows.md`, `permissions.md`, or `api-conventions.md`.

Do not create domain files before the spec just to fill the folder. Empty
structure is healthier than fake product truth.

## Update Rule

When behavior changes:

1. Update the affected product doc.
2. Update or create the story packet.
3. Update durable proof status with `scripts/bin/harness-cli story add` or
   `scripts/bin/harness-cli story update`.
4. Record a decision if the change affects architecture, scope, risk, or a
   previously settled product rule.
