# Compatibility Shell

## Contract

Substrate must expose a Git-compatible shell so existing human tooling keeps
working while the underlying store evolves toward intent, structure, and query
first workflows.

Compatibility is a product safety requirement. Text projection must be available
for review, CI, emergency debugging, export, and migration.

## Required Capabilities

The compatibility shell should eventually provide:

- Materialized working-tree projection.
- Text diff output for human review.
- Import and export paths for Git repositories.
- CI-friendly checkout or projection behavior.
- Traceable links from projected files back to stored intent and verification.

## Emulation Risks

The brief calls out areas that need explicit design before implementation claims
compatibility:

- Exact object identity and SHA expectations.
- Reflog-like recovery semantics.
- Submodule behavior.
- Git LFS behavior.
- Remote protocol expectations.
- Human review workflows on hosted Git platforms.

## Phase 0 Boundary

The first implementation slice uses a local CLI as the compatibility surface.
Compatibility is through ingesting a working tree, storing local state, and
projecting materialized text back to disk. Full remote protocol emulation is a
later product decision unless selected explicitly.
