# Spec Intake: Substrate

Date: 2026-06-17

## Source

- User prompt: `please check product-brief_1.md and create a plan to work on it`
- Attached file: `product-brief_1.md`
- External reference: none verified during this intake pass

## Project Summary

Substrate is an agent-native code store and version-control system where intent,
constraints, generating context, and verification are treated as durable source
truth. Materialized code remains stored and reviewable, but is modeled as a
verified derivative that can be projected into text and Git-compatible workflows
for humans, CI, and existing tooling.

The primary users are autonomous and semi-autonomous coding agents, orchestration
systems, and CI-driven agents. Human engineers are secondary users who need
legible projections, audit trails, and emergency escape hatches.

## Candidate Product Docs

| File | Purpose | Source sections |
| --- | --- | --- |
| `docs/product/overview.md` | Product direction, target users, success metrics, and non-goals. | One-line, Problem, Core bet, Target user, Success metrics |
| `docs/product/intent-model.md` | Intent graph, provenance, materialized artifact policy, and regeneration limits. | Core bet, key capabilities 1-2, Risks |
| `docs/product/storage-history.md` | Storage, change history, deduplication, retention, and partial fetch expectations. | Key capabilities 4-6, Architecture: Storage and History, Risks |
| `docs/product/diff-merge.md` | Structural diff, semantic versioning units, regeneration merge, and conflict model. | Key capabilities 2-4, Architecture: Diff/merge |
| `docs/product/verification-query.md` | Verification-in-store, query-first access, speculative candidates, and proof shape. | Key capabilities 5-8, Success metrics |
| `docs/product/compatibility-shell.md` | Git compatibility, text projection, existing tooling, and emulation risks. | Architecture: Compatibility shell, Risks |

## Candidate Epics

| Epic | Description | Status |
| --- | --- | --- |
| E01 Product contract | Convert the brief into stable product docs, open decisions, and first validation expectations. | in_progress |
| E02 Phase 0 wedge | Explore a drop-in storage and structural-diff wedge behind a Git-compatible surface. | unsliced |
| E03 Verification and provenance | Define how states are admitted, proven, and attributed to humans or agents. | unsliced |
| E04 Intent and regeneration | Define and prototype a single-language intent graph and regeneration-based merge flow. | unsliced |
| E05 Agent search and concurrency | Model speculative candidate trees and swarm-safe semantic graph concurrency. | unsliced |

## Architecture Questions

- Runtime stack: undecided. Phase 0 should confirm only what is needed for a local CLI wedge.
- Product surfaces: local CLI selected for Phase 0; API, service, hosted Git protocol, and UI remain open.
- Storage: CDC content-addressable storage is intended, but chunking algorithm, isolation boundary, and retention policy are open.
- External providers: none required for Phase 0. Model/provider integration is deferred until intent regeneration is selected.
- Deployment target: local developer repo first; hosted service and multi-tenant operation remain open.
- Security model: open. Dedup side channels, provenance integrity, and Git compatibility trust boundaries need explicit design before high-risk work.

## Validation Shape

| Layer | Expected proof |
| --- | --- |
| Unit | Chunking, canonicalization, structural diff, symbol identity, and proof-admission rules once implemented. |
| Integration | Storage round trips, Git-compatible projection/import, verification metadata persistence, query index updates. |
| E2E | Agent or developer workflow from intent/change input through stored state, projection, and verification result. |
| Platform | CLI behavior, repository filesystem behavior, large-repo transfer/storage measurements, and Git tooling compatibility. |
| Release | Benchmark comparison against equivalent Git-style workflows, traceable provenance, and documented known gaps. |

## Open Decisions

- Confirm the Phase 0 runtime stack before source scaffolding; the interface boundary is local CLI.
- Define the first Git-compatible surface: CLI-only, local protocol shim, or remote-compatible service.
- Decide how to measure storage-size and false-conflict improvements without overfitting to synthetic workloads.
- Define isolation rules for deduplication before any global or cross-repo store exists.
- Decide how nondeterministic regeneration is represented when materialized code must remain durable.
- Verify external comparator claims before using them as current product or marketing facts.

## First Story Candidates

- US-001: Phase 0 wedge feasibility plan. Implemented.
- US-002: Local CLI scaffold and repository store.
- US-003: Deterministic storage benchmark fixture.
- US-004: Ingest and projection round trip.
- US-005: Structural diff spike for one language.
- US-006: Verified-green state admission.

## Harness Delta

- Initialized the Harness durable database for this workspace.
- Recorded this brief as a new-spec intake.
- Created living product docs from the brief instead of extending the brief as a monolithic spec.
- Added planned story and validation surfaces for the first follow-up work.
