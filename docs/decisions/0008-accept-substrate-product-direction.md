# 0008 Accept Substrate Product Direction

Date: 2026-06-17

## Status

Accepted

## Context

The repository is a generic Harness instance with no application implementation
or baked-in product contract. `product-brief_1.md` introduces Substrate: an
agent-native code store and version-control system where intent, verification,
and provenance are first-class, while code remains a durable materialized
artifact exposed through Git-compatible projections.

The Harness source hierarchy says a user-provided spec should become smaller
living product docs, story packets, validation expectations, and decisions. The
brief itself should remain input material, not the permanent operating manual.

## Decision

Accept Substrate as the product direction for this Harness instance.

Treat `product-brief_1.md` as source input and decompose it into living product
contract files under `docs/product/`, candidate epics in `docs/stories/`, and
planned validation rows in `docs/TEST_MATRIX.md`.

Do not scaffold application code, select a runtime stack, or implement the Phase
0 wedge as part of this decision. The next selected story is the Phase 0 wedge
feasibility plan, which will choose the first implementation surface and proof
gates.

## Alternatives Considered

1. Keep `product-brief_1.md` as the living spec. Rejected because Harness
   guidance treats large specs as input material and moves current truth into
   smaller product docs, stories, decisions, and validation records.
2. Jump directly into Phase 0 implementation. Rejected because runtime stack,
   compatibility boundary, security constraints, and proof gates are still open.
3. Pre-slice all phases into story packets immediately. Rejected because the
   brief is broad and speculative; creating every possible story would produce
   fake certainty and stale backlog noise.

## Consequences

Positive:

- Future agents have stable product docs to read before implementation.
- Phase 0 can be planned against explicit product constraints and validation
  expectations.
- Risky areas such as dedup side channels, regeneration drift, and Git semantics
  are visible before code exists.

Tradeoffs:

- The repository still has no executable product implementation after this
  decision.
- External comparator claims from the brief remain unverified until a later
  research or implementation story checks current primary sources.
- The first implementation story must still choose stack, surface, and proof
  gates.

## Follow-Up

- Complete US-001: Phase 0 wedge feasibility plan.
- Verify current primary sources before making public claims about Xet,
  Jujutsu, difftastic, tree-sitter, Scalar, Sapling, or related systems.
- Record a new decision when the runtime stack, storage isolation boundary, or
  Git-compatible surface is selected.
