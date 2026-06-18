# Test Matrix

This file maps product behavior to proof.

Substrate product behavior has been defined at the contract level, but no
runtime behavior has been implemented yet. Do not mark a row implemented until
tests or validation evidence exist.

## Status Values

| Status | Meaning |
| --- | --- |
| planned | Accepted as intended behavior, not implemented |
| in_progress | Actively being built |
| implemented | Implemented and proof exists |
| changed | Contract changed after earlier implementation |
| retired | No longer part of the product contract |

## Matrix

| Story | Contract | Unit | Integration | E2E | Platform | Status | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| US-001 | Phase 0 wedge feasibility plan defines the smallest safe implementation slice and proof gates. | yes | yes | no | no | implemented | `docs/stories/US-001-phase-0-wedge-plan.md` |
| US-002 | Local CLI scaffold and repository store create the first runnable Phase 0 surface. | yes | yes | no | yes | implemented | `docs/stories/US-002-local-cli-scaffold-repository-store.md` |
| US-003 | Deterministic storage benchmark fixture measures whole-file baseline versus sub-file storage. | yes | yes | no | yes | implemented | `docs/stories/US-003-deterministic-storage-benchmark-fixture.md` |
| US-004 | Ingest and projection round trip stores supported text files and projects them byte-for-byte. | yes | yes | no | yes | implemented | `docs/stories/US-004-ingest-projection-round-trip.md` |
| US-005 | Structural diff spike evaluates one language and records fallback behavior for unsupported code. | yes | yes | no | yes | implemented | `docs/stories/US-005-structural-diff-spike.md` |
| US-006 | Verified-green state admission persists verification metadata and separates candidate from verified states. | yes | yes | no | yes | implemented | `docs/stories/US-006-verified-green-state-admission.md` |
| US-007 | TypeScript tree-sitter diff parser adds real parser-backed changed-node counts for TypeScript files. | yes | yes | no | yes | implemented | `docs/stories/US-007-typescript-tree-sitter-diff-parser.md` |
| US-008 | JavaScript tree-sitter diff parser adds real parser-backed changed-node counts for JavaScript and JSX files. | yes | yes | no | yes | implemented | `docs/stories/US-008-javascript-tree-sitter-diff-parser.md` |
| US-009 | User onboarding docs, repo-local agent skill, and measured benchmark guide make the Phase 0 CLI usable by humans and agents. | no | yes | no | yes | implemented | `docs/stories/US-009-user-onboarding-skill-benchmarks.md` |
| US-010 | Ignore-aware ingest and parser registry complete the first two public README TODO items. | yes | yes | no | yes | implemented | `docs/stories/US-010-ignore-aware-ingest-parser-registry.md` |
| US-011 | Packaged binary releases and repo-local installers make Substrate installable without Cargo on supported platforms. | yes | yes | no | yes | implemented | `docs/stories/US-011-substrate-release-automation/`; cargo fmt --check; cargo test; cargo build; release packaging script; local installer smoke; harness-cli story verify-all |
| US-012 | Python and C# tree-sitter diff parsers add parser-backed changed-node counts for `.py` and `.cs` files. | yes | yes | no | yes | implemented | `docs/stories/US-012-python-csharp-tree-sitter-diff-parsers.md`; cargo fmt --check; cargo test; cargo build; Python and C# fixture diff commands; harness-cli story verify-all |
| US-013 | Live release verification proves the published GitHub Release asset and checksum contract. | yes | yes | no | yes | implemented | `docs/stories/US-013-live-release-verification.md`; `scripts/verify-substrate-github-release.ps1`; `harness-cli story verify US-013`; `harness-cli story verify-all` |

## Evidence Rules

- Unit proof covers pure domain and application rules.
- Integration proof covers backend enforcement, data integrity, provider
  behavior, jobs, or service contracts.
- E2E proof covers user-visible browser flows.
- Platform proof covers only shell, deployment, mobile, desktop, or runtime
  behavior that cannot be proven in lower layers.
- A story can be implemented without every proof column if the story packet
  explains why.
