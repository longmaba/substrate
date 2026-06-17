# Substrate Product Overview

## Product Direction

Substrate is an agent-native code store and version-control system. Its core
claim is that agent-produced code should be stored around the durable intent,
constraints, context, and verification that produced it, while materialized code
remains available as a verified artifact and human-readable projection.

The working product name is Substrate.

## Users

Primary users:

- Autonomous coding agents.
- Semi-autonomous coding agents.
- Agent orchestration systems and swarms.
- CI-driven agents that generate or repair code.

Secondary users:

- Human engineers reviewing, auditing, debugging, and operating agent output.
- Existing Git, CI, and code-review tooling that must keep working through a
  compatibility shell.

## Problem Contract

Substrate exists because agent-scale coding changes the pressure on version
control. The brief identifies these product pressures:

- High-volume near-duplicate code churn can inflate storage.
- Line-based diffs and three-way merges produce noisy conflicts when agents
  regenerate structural units.
- Human-curated commit narratives do not naturally represent parallel agent
  exploration.
- Verification, provenance, and generating intent are not first-class state.
- Agents often need structural and semantic queries more than a full working
  tree checkout.

## Core Bet

For agent-authored changes, intent is product truth and code is a durable,
verified derivative. This does not mean code can be discarded. Materialized code
must stay stored, inspectable, and exportable because regeneration can drift and
humans need operational legibility.

## Product Principles

- Intent and verification are first-class versioned data.
- Text projection is mandatory, not a convenience.
- Compatibility with human Git workflows is a safety feature.
- Structural and semantic operations should be the default where available.
- Benchmarks must prove storage, merge, query, and verification claims before
  those claims become product commitments.

## Success Metrics

- Repository and transfer size versus an equivalent Git-style repository under
  agent-churn workloads.
- False-conflict rate on parallel agent edits versus line-based merge.
- Fraction of merges resolved automatically through regeneration or structural
  reconciliation.
- Time to answer structural queries such as callers, call graph, symbol
  dependencies, and similar code.
- Share of stored states that are verified green.
- Bisection latency across verified states.

## Non-Goals For Initial Work

- Do not replace GitHub, GitLab, or CI in the first slice.
- Do not make regenerated code the only stored artifact.
- Do not create a global cross-repo dedup store before isolation rules exist.
- Do not claim current parity with named external systems until references are
  verified from current primary sources.
