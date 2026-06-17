# Intent Model

## Contract

Substrate treats intent as a first-class versioned object. Intent includes the
specification, constraints, relevant context, tests or proofs, and provenance
needed to explain why a materialized code state exists.

Materialized code is still stored. It is not only a cache, because model drift,
provider changes, and nondeterministic generation mean later regeneration may
not reproduce the same bytes or behavior.

## Intent Graph

The product should eventually represent these relationships:

- Intent that requested or constrained a change.
- Context used to generate or evaluate the change.
- Candidate materialized artifacts produced from the intent.
- Verification results attached to candidates and admitted states.
- Provenance for the actor or system that generated the state.
- Links to affected symbols, contracts, tests, and dependencies.

## Regeneration Policy

Regeneration is a merge and repair strategy, not a promise that code can be
discarded. When two agents touch the same entity, the target model is to replay
or reconcile both intents against the merged base and regenerate a candidate
state, then admit it only after verification.

Open questions for regeneration:

- Which inputs are required to make regeneration auditable?
- How is model or toolchain drift represented?
- What is the fallback when regeneration fails or produces ambiguous output?
- Which languages or structural units are eligible in the first prototype?

## Provenance

Stored states should be able to answer:

- Was this produced by a human, agent, CI system, or mixed workflow?
- Which intent and constraints produced it?
- Which verification gates ran?
- Which artifacts were admitted, rejected, or superseded?
- What text projection was available to human tools?

## Acceptance Boundaries

Do not admit an intent-derived state as product truth only because generation
succeeded. Acceptance requires a defined proof policy for the story being built.
Until that policy exists, states can be recorded as candidates, not accepted
verified states.
