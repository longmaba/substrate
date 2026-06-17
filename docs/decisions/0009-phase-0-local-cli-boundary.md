# 0009 Phase 0 Local CLI Boundary

Date: 2026-06-17

## Status

Accepted

## Context

US-001 required the first Phase 0 wedge to name exactly one primary surface,
define benchmark proof, scope Git-compatible behavior, and avoid global
deduplication before isolation rules exist.

The product direction needs an implementation surface small enough to validate
storage, projection, and diff behavior without committing to hosted service,
remote protocol, or complete Git emulation semantics.

## Decision

Phase 0 will use a local CLI as its only primary surface.

The wedge will operate on local fixture repositories, store data in a repo-local
`.substrate/` directory, project materialized text back to a working tree, and
report storage and diff-noise benchmark results. Compatibility is through text
projection and local inspection, not remote Git protocol emulation.

Global and cross-repo deduplication are explicitly deferred. Phase 0 storage is
repo-local only.

Rust is the default candidate stack for the first implementation story because
the wedge is filesystem and storage heavy and this repository already uses a
Rust Harness CLI operationally. The implementation story must still confirm the
stack before adding source scaffolding.

## Alternatives Considered

1. Hosted service first. Rejected because it would add deployment, tenancy,
   auth, authorization, network, and operational concerns before storage and
   projection behavior are proven.
2. Remote Git protocol shim first. Rejected because exact protocol and object
   semantics would dominate the first slice and obscure the storage/diff wedge.
3. Repository helper library first. Rejected because agents and humans need an
   executable workflow and measurable proof surface before library integration.
4. Global deduplication first. Rejected because dedup side-channel boundaries
   are unresolved and should be treated as a later high-risk topic.

## Consequences

Positive:

- The first implementation can be verified locally with deterministic fixtures.
- Storage, projection, and diff benchmarks can be measured before service or
  protocol complexity is introduced.
- Existing tools can inspect projected text output without needing native
  Substrate integration.
- Security risk is reduced by forbidding cross-repo deduplication in Phase 0.

Tradeoffs:

- Phase 0 will not prove remote Git compatibility.
- Hosted multi-agent workflows remain conceptual until a later surface is
  selected.
- Rust remains a candidate, not a final stack, until the next implementation
  story confirms scaffolding.

## Follow-Up

- Create US-002 for local CLI scaffold and repo-local store setup.
- Record another decision if US-002 confirms Rust or chooses a different stack.
- Treat any global deduplication or hosted storage proposal as high-risk until
  isolation and side-channel rules are designed.
