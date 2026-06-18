# Story Backlog

This backlog is seeded from `product-brief_1.md`, which introduced Substrate as
an agent-native code store and version-control system.

Do not create every possible story packet up front. Create story packets when
the work is selected or when a product decision needs a durable place to land.

## Candidate Epics

| Epic | Description | Status |
| --- | --- | --- |
| E01 Product contract | Maintain living product docs, open decisions, and validation expectations derived from the brief. | in_progress |
| E02 Phase 0 wedge | Explore a drop-in storage and structural-diff wedge behind a Git-compatible surface. | sliced |
| E03 Verification and provenance | Define how states are admitted, proven, and attributed to humans or agents. | unsliced |
| E04 Intent and regeneration | Define and prototype a single-language intent graph and regeneration-based merge flow. | unsliced |
| E05 Agent search and concurrency | Model speculative candidate trees and swarm-safe semantic graph concurrency. | unsliced |

## Selected Story Packets

| Story | Title | Status |
| --- | --- | --- |
| US-001 | Phase 0 wedge feasibility plan | implemented |
| US-002 | Local CLI scaffold and repository store | implemented |
| US-003 | Deterministic storage benchmark fixture | implemented |
| US-004 | Ingest and projection round trip | implemented |
| US-005 | Structural diff spike | implemented |
| US-006 | Verified-green state admission | implemented |
| US-007 | TypeScript tree-sitter diff parser | implemented |
| US-008 | JavaScript tree-sitter diff parser | implemented |
| US-009 | User onboarding, agent skill, and benchmarks | implemented |
| US-010 | Ignore-aware ingest and parser registry | implemented |
| US-012 | Python and C# tree-sitter diff parsers | implemented |
