# 0010 Rust Phase 0 CLI Stack

Date: 2026-06-18

## Status

Accepted

## Context

US-002 begins the first executable Phase 0 wedge for Substrate. Decision 0009
selected a local CLI as the Phase 0 surface and named Rust as the default
candidate stack because the wedge is filesystem and storage heavy.

The repository already relies on a Rust Harness CLI operationally, and the first
Substrate slice needs deterministic local filesystem behavior, simple binary
execution, and no hosted service or network runtime.

## Decision

Use Rust for the Phase 0 Substrate CLI implementation.

The initial CLI remains dependency-free and uses only the Rust standard library
until a selected story needs parsing, hashing, structural diff, or CLI ergonomics
that justify adding crates. The first package exposes `init` and `status` as a
local binary surface and stores Phase 0 metadata under a repo-local
`.substrate/` directory.

## Alternatives Considered

1. Node.js CLI. Rejected because the first wedge is storage and filesystem heavy,
   and this repository already has Rust operational precedent through the
   Harness CLI.
2. Python script. Rejected because the project direction calls for a durable CLI
   wedge that can grow into storage and benchmark work without interpreter
   assumptions.
3. Add a CLI framework crate immediately. Rejected because US-002 only needs two
   commands and the repository guidance avoids new dependencies without a clear
   story-level need.

## Consequences

Positive:

- The first executable surface can be validated with `cargo test` and direct
  local CLI runs.
- Later storage, hashing, chunking, and benchmark stories can build on a compiled
  toolchain.
- The dependency surface remains minimal for the first story.

Tradeoffs:

- CLI ergonomics are intentionally basic until a later story justifies a parser
  dependency.
- The current binary is not a Git protocol shim or hosted service entrypoint.

## Follow-Up

- Add dependencies only when a selected story records the need and validation
  impact.
- Keep Phase 0 store data repo-local unless a later high-risk story designs
  isolation for broader deduplication.
