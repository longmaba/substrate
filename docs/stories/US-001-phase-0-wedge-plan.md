# US-001 Phase 0 Wedge Feasibility Plan

Date: 2026-06-17

## Decision Summary

Phase 0 should build a local CLI wedge for Substrate.

The first implementation should prove that Substrate can ingest a normal source
tree, store materialized files in a repo-local content-addressed store with
sub-file chunking, emit a working-tree text projection, and report storage and
diff-noise measurements against a Git-style whole-file baseline.

This is intentionally not a hosted service, remote Git protocol replacement, or
global deduplication system.

## Primary Surface

Exactly one primary surface is selected for Phase 0: a local CLI.

Working command shape for the implementation stories:

```text
substrate init <path>
substrate ingest <path>
substrate status
substrate project --out <path>
substrate diff <left-state> <right-state>
substrate bench <fixture-path>
```

The command names are planning targets, not final API commitments. The first
implementation story may rename them if it records the change in the story and
keeps the same behavioral coverage.

## Phase 0 Scope

In scope:

- Repo-local store under `.substrate/` in a disposable fixture repository.
- Content-addressed materialized file records.
- Content-defined or rolling chunk boundaries for text file payloads.
- State manifests that map file paths to stored content and verification state.
- Text projection back to a working directory.
- A structural-diff experiment for one selected language or a clearly delimited
  structural placeholder if parser selection is deferred to the spike story.
- Benchmark reporting for storage size and diff-noise behavior.
- Local-only verification metadata for candidate and verified states.

Out of scope:

- Remote Git protocol compatibility.
- Exact Git object identity or SHA emulation.
- Reflog, submodules, Git LFS, sparse checkout, and hosted review integration.
- Global or cross-repo deduplication.
- Multi-tenant storage, auth, authorization, or hosted APIs.
- Model-driven regeneration or provider integration.
- Query index, embeddings, or code property graph beyond benchmark metadata.

## Git-Compatible Behavior

Phase 0 must provide compatibility through projection, not protocol emulation.

In scope:

- Ingest files from a normal working tree.
- Preserve relative paths and file bytes for supported text files.
- Emit a materialized text tree that ordinary tools can inspect.
- Produce human-readable diff output from stored states.
- Keep enough manifest data to trace projected files back to stored state ids.

Deferred:

- `git clone`, `git fetch`, `git push`, and remote negotiation.
- Exact commit, tree, and blob SHA compatibility.
- Git hooks, attributes, ignore semantics, and index behavior.
- Submodules and LFS.
- Hosted pull request or code review integrations.

## Benchmarks

### Storage Benchmark

Goal: measure whether sub-file chunking reduces stored bytes on agent-churn-like
fixtures compared with a whole-file snapshot baseline.

Fixture shape:

- A small source tree with repeated boilerplate and generated-looking files.
- A sequence of at least 25 synthetic revisions.
- Each revision changes a small section of one or more large text files while
  leaving most content unchanged.
- The fixture must be deterministic and committed as test data in the first
  implementation story.

Required report fields:

- Whole-file baseline bytes.
- Substrate stored bytes.
- Chunk count.
- Unique chunk count.
- Dedup ratio.
- Time to ingest the fixture on the local machine.

Acceptance target for the first benchmark story: produce the report reliably.
Do not set a reduction target until the benchmark is running and fixture quality
is reviewed.

### Diff-Noise Benchmark

Goal: measure whether structural or normalized diff output reduces review noise
for regenerated code compared with line diff output.

Fixture shape:

- At least 10 paired before/after examples.
- Examples include formatting-only churn, function reordering, localized logic
  edits, and whole-function regeneration.
- The first language must be selected by the structural diff spike story.

Required report fields:

- Line-diff changed line count.
- Structural or normalized changed-node count.
- Unsupported-file fallback count.
- False-conflict or noise classification notes for each example.

Acceptance target for the first diff story: produce consistent measurements and
explicitly report fallbacks. Do not claim semantic equivalence from structural
similarity.

## Verified-Green Gates

A Phase 0 state may be called `verified-green` only when all applicable local
gates pass for that state:

- Manifest parse and referential-integrity checks pass.
- Stored content hashes match the manifest.
- Projection recreates the expected text tree for supported files.
- Ingest followed by projection is byte-for-byte stable for supported files.
- The selected benchmark command completes and writes a report.
- The story's configured verification command exits successfully.

If any gate is missing, the state can be recorded as `candidate` but not
`verified-green`.

## Security And Isolation

Phase 0 must stay repo-local.

- Store data under the local fixture or workspace `.substrate/` directory.
- Do not deduplicate across repositories, users, machines, or tenants.
- Do not add network access.
- Do not store secrets, credentials, or external provider payloads.
- Treat dedup side channels as an open security topic for a later high-risk
  story.

## Runtime And Architecture Direction

The first implementation should prefer a compiled local CLI with explicit file
parsing at command boundaries. Rust is the default candidate because this repo
already distributes a Rust Harness CLI and the Phase 0 wedge is filesystem and
storage heavy, but the implementation story must confirm the stack before
creating source scaffolding.

Suggested architecture boundaries:

- Domain: states, content ids, chunks, manifests, verification status.
- Application: ingest, project, diff, benchmark, verify commands.
- Infrastructure: filesystem store, hashing, chunking, report writers.
- Interface: CLI argument parsing and command output.

## Follow-Up Stories

Create these in order when implementation starts:

| Story | Title | Lane | Purpose |
| --- | --- | --- | --- |
| US-002 | Local CLI scaffold and repository store | normal | Add the CLI skeleton, repo-local `.substrate/` store, and status command. |
| US-003 | Deterministic storage benchmark fixture | normal | Add whole-file baseline and sub-file storage measurement fixture. |
| US-004 | Ingest and projection round trip | normal | Store supported text files and project them back byte-for-byte. |
| US-005 | Structural diff spike | normal | Select one language and compare structural or normalized diff output to line diff. |
| US-006 | Verified-green state admission | normal | Persist verification metadata and enforce candidate versus verified-green labels. |

Do not create hosted service, remote protocol, or global dedup stories until the
local CLI wedge has produced benchmark evidence.

## Acceptance Check

This plan satisfies US-001 when:

- The selected primary surface is local CLI.
- Storage and diff-noise benchmarks are defined with measurable report fields.
- Git-compatible projection is scoped and protocol emulation is deferred.
- Verified-green admission gates are defined.
- Dedup isolation is repo-local with global dedup deferred.
- Follow-up implementation stories are listed in sequence.
