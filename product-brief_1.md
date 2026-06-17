# Product Brief: Agent-Native Code Store (working name: **Substrate**)

## One-line

A version control and code storage system built for AI agents as the primary user — where **intent is the source of truth and code is a regenerable, verified artifact** — with a Git-compatible shell so human tooling keeps working.

---

## Problem

AI agents now produce the majority of code changes in many codebases, and they generate at machine scale: hundreds of micro-edits, parallel candidate solutions, whole-file regenerations, and high near-duplicate churn. Git was designed in 2005 for humans writing curated commits, and it fuses five concerns into one rigid model — storage, history, diff/merge, sync, and UX. Under agent-scale churn this breaks down:

- **Storage bloat** — blob-per-file-version plus millions of near-identical generated files inflate repos.
- **Noisy line diffs and false conflicts** — agents regenerate functions; line-based 3-way merge is the wrong primitive.
- **History as human narrative** — curated commit stories don't fit swarms of agents exploring in parallel.
- **No native verification or provenance** — nothing records which agent/intent produced a change or whether it passed.
- **Checkout-optimized, not query-optimized** — agents query structure (call graphs, callers, similar code) far more than they materialize a working tree.

## The core bet

For human-authored code, the source file is canonical and the *intent* behind it is lost. For agent-authored code, the durable artifact is the **intent** (spec, constraints, prompt/context, tests) and the code is a derived build output. Leaning into that inversion is where the largest gains are — most of Git's hardest problems dissolve or change shape downstream of it.

## Target user

- **Primary:** autonomous and semi-autonomous coding agents and the orchestration layers that run them (single agents, agent swarms, CI-driven agents).
- **Secondary:** human engineers reviewing, auditing, and debugging agent output — served through projections and exports, not as the day-to-day operator.

---

## What it does (key capabilities)

1. **Intent-first model.** The intent graph (spec, constraints, generating context, tests) is versioned as primary truth; materialized code is stored alongside as a verified, cache-like derivative.
2. **Regeneration-based merge.** When two agents touch the same entity, replay both intents against the merged base and regenerate, rather than 3-way-merging text. Conflicts resolve at the level of *what was being accomplished*.
3. **Structured canonical form.** Store a typed AST / IR / semantic graph as the real artifact; emit text only as an on-demand view. Eliminates formatting churn and makes diffs structural by construction.
4. **Semantic versioning units.** Version functions, types, and contracts — not files. Fetch a symbol plus its transitive dependencies instead of checking out a tree.
5. **Verification in the store.** Only admit states that pass their contract (typecheck/tests/proof) and record the result with the state. History becomes a chain of known-green states; bisection is a primitive.
6. **VCS as a search substrate.** Natively hold a scored tree of speculative candidate implementations (tests passed, perf, size) and prune losers. Cheap because of sub-file deduplication.
7. **Query-first access.** A versioned code property graph + embeddings is the primary structure; materializing a working directory is the uncommon path.
8. **Swarm concurrency.** Node-level optimistic concurrency / CRDT-style merge on the semantic graph so hundreds of agents can edit at once without branch-then-reconcile.

## Architecture (layered)

- **Storage:** content-defined chunking (CDC) content-addressable store with global, sub-file deduplication — a one-token edit re-stores one chunk, not a file. (Proven in production by Hugging Face's Xet for large artifacts; applied here to high-redundancy source.)
- **History:** change-based, garbage-collectable, with tiered retention — recent edits hot and full-fidelity, long agent-churn runs compacted to verified checkpoints, lazy/partial fetch by default. (Conceptually adjacent to Jujutsu's change-based model.)
- **Diff/merge:** structural/AST diff as default (difftastic / tree-sitter lineage), with regeneration-based resolution layered on top.
- **Index:** code property graph, symbol table, dependency edges, and embeddings, all versioned alongside content.
- **Compatibility shell:** a Git-compatible interface and protocol on top, so GitHub/GitLab/CI and human review continue to work; text projection is always available as an escape hatch.

---

## Why now / differentiation

The building blocks exist in isolation — **Xet** (sub-file dedup storage), **Jujutsu** (change-based, Git-compatible history and undoable operations), **difftastic/tree-sitter** (structural diff), **Scalar/Sapling** (lazy fetch at monorepo scale) — but no one has combined them into a coherent system whose *primary user is an agent*. The novel contributions are (a) intent-as-source-of-truth with regeneration-based merge, (b) verification and provenance as first-class versioned properties, and (c) version control as a parallel search substrate rather than a single-timeline record.

## Risks & open questions

- **Regeneration is nondeterministic.** Model drift means "store the recipe, regenerate later" can't be the only copy — materialized artifacts stay; intent is *additional* truth.
- **Semantic equivalence is undecidable.** Dedup is safe only over normalized/canonical forms (alpha-renaming, formatting), not arbitrary behavioral equivalence.
- **Operational legibility.** A fully human-illegible store is dangerous during incidents — the text projection and Git export are non-negotiable safety features.
- **Dedup side channels.** Global cross-repo dedup can leak chunk existence; isolation boundaries need explicit design.
- **Git-semantics emulation.** Exact-SHA history, reflog, submodules, and LFS need careful emulation in the compatibility shell.

## Success metrics

- Repo + transfer size vs. equivalent Git/Git-LFS repo under agent workloads (target: order-of-magnitude reduction on high-churn repos).
- False-conflict rate on parallel agent edits vs. line-based merge.
- Fraction of merges resolved automatically via regeneration.
- Time-to-answer for structural queries (callers, call graph, similar code).
- Share of stored states that are verified-green; bisection latency.

## Suggested phasing

- **Phase 0 — wedge:** CDC storage backend + structural diff behind a Git-compatible front end. Pure drop-in win on size and conflict noise; no behavior change required.
- **Phase 1:** verification-in-store + provenance tagging (human vs. agent, which model/intent).
- **Phase 2:** intent graph + regeneration-based merge for a single language.
- **Phase 3:** speculative search tree + swarm concurrency for multi-agent orchestration.

---

*This brief synthesizes a design discussion and is a starting point, not a committed spec. Figures and comparisons to existing systems (Xet, Jujutsu, difftastic, Sapling) reflect their roles as of early 2026 and should be re-verified before planning.*
