# Verification And Query

## Contract

Substrate stores verification and provenance alongside code states. Query-first
access is a primary product surface for agents; full working-tree materialization
is still required but should not be the only way to inspect the repository.

## Verification In Store

The store should eventually record:

- Which tests, typechecks, proofs, or benchmarks ran.
- Which state each result applies to.
- Whether the state is candidate, rejected, or admitted.
- Which actor or system produced the result.
- Enough evidence to support bisection and audit trails.

Known-green history is a product goal. The first implementation must define the
minimum proof required before any state is marked verified.

## Query-First Access

Agents should be able to ask structural questions without first materializing a
whole tree, including:

- What calls this symbol?
- What does this symbol depend on?
- Which candidates passed a target proof?
- Which similar implementations exist?
- Which intent produced this state?
- Which verified checkpoint introduced a behavior?

## Speculative Search

The brief frames the VCS as a search substrate for parallel agent work. Candidate
states may be ranked by verification result, performance, size, or other scoring
signals, then pruned or retained according to policy.

Open questions:

- What scores are mandatory versus optional?
- How are rejected candidates retained for audit without bloating the store?
- How do agents compare candidates produced from different intent contexts?
- What proof is required before pruning alternatives?
