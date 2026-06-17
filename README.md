# Substrate

Substrate is a local Phase 0 wedge for an agent-native code store. It stores
code states with verification metadata, projects those states back to plain
files, and compares source trees with structural diff support where a parser is
available.

Current parser-backed diff support covers Rust, TypeScript, JavaScript, and JSX.
Unsupported files still get text-diff fallback accounting, and Substrate does
not claim semantic equivalence.

## Why It Exists

Agent-generated code creates different version-control pressure than ordinary
human commits:

- high-volume near-duplicate code churn,
- noisy line diffs from formatting and regeneration,
- many candidate states that need local verification before promotion,
- agents that need queryable state rather than only a full checkout.

Substrate keeps compatibility with ordinary files while exploring smaller,
verified, parser-aware units of code history.

## Install And Build

Prerequisites:

- Rust toolchain with Cargo.
- PowerShell on Windows, or an equivalent shell on macOS/Linux.

Build from this repository:

```powershell
cargo build
```

Run commands through Cargo during development:

```powershell
cargo run -- <command>
```

The CLI surface is intentionally small:

```text
substrate init <path>
substrate status [path]
substrate ingest <path>
substrate project <state-id> --out <path>
substrate state <state-id>
substrate verify <state-id> --out <path> --bench <fixture-path>
substrate diff <left> <right>
substrate bench <fixture-path>
```

## Quick Start

Initialize a local store in a source tree:

```powershell
cargo run -- init .
cargo run -- status .
```

Capture the current working tree as a candidate state:

```powershell
cargo run -- ingest .
```

The ingest output includes a `state_id`. Inspect that state:

```powershell
cargo run -- state <state-id>
```

Project the state back to normal files:

```powershell
cargo run -- project <state-id> --out .\out\projected
```

Verify the state against the local gates and benchmark fixture:

```powershell
cargo run -- verify <state-id> --out .\out\verify --bench fixtures\storage-agent-churn
```

After verification passes, the state label becomes `verified-green`.

## Structural Diff

Compare two files or two directories:

```powershell
cargo run -- diff fixtures\diff-javascript-pairs\before fixtures\diff-javascript-pairs\after
cargo run -- diff fixtures\diff-typescript-pairs\before fixtures\diff-typescript-pairs\after
cargo run -- diff fixtures\diff-rust-pairs\before fixtures\diff-rust-pairs\after
```

The report includes line-change counts, parser-backed normalized change counts
for supported languages, fallback counts for unsupported files, and this guard:

```text
semantic_equivalence_claimed: no
```

Supported parser-backed inputs:

| Language | Extensions | Path |
| --- | --- | --- |
| Rust | `.rs` | normalized function-block placeholder |
| TypeScript | `.ts`, `.tsx` | tree-sitter named syntax-node fingerprints |
| JavaScript | `.js`, `.jsx` | tree-sitter named syntax-node fingerprints |

## Benchmarks

Run the included storage benchmark:

```powershell
cargo run -- bench fixtures\storage-agent-churn
```

Current local fixture result:

| Metric | Value |
| --- | ---: |
| Revisions | 25 |
| Files across revisions | 50 |
| Whole-file baseline bytes | 117,950 |
| Substrate stored bytes | 10,094 |
| Bytes avoided | 107,856 |
| Storage reduction | 91.4% |
| Dedup ratio | 11.6852x |
| Local ingest time | 9 ms |

As a rough token-equivalent estimate using 4 bytes per token, this fixture is
about 29,488 token-equivalent bytes as whole files versus 2,524 token-equivalent
bytes in the Substrate store, or about 26,964 token-equivalent bytes avoided.
This is an estimate from bytes, not a tokenizer or model-billing measurement.

More detail lives in `docs/benchmarks/phase-0-results.md`.

## Use With An Existing GitHub Repo

Start with the guide in `docs/guides/existing-github-repo.md`.

Phase 0 is local-only. It does not push to GitHub, replace Git, or emulate the
Git protocol yet. Use it beside Git: clone or export a repository, initialize a
local `.substrate` store, ingest candidate states, verify them, and use `diff`
to compare local directories or worktrees.

Important current caveat: ingestion skips `.substrate`, but it does not yet
honor `.gitignore`. For clean measurements, run Substrate on a source export or
working-tree copy that excludes `.git`, dependency folders, and build outputs.

## Agent Skill

A repo-local skill for agents is available at `skills/substrate/SKILL.md`.
Agents can use it when asked to initialize, ingest, verify, diff, benchmark, or
onboard an existing repository with Substrate.

## Product Docs

- `docs/product/overview.md` explains the product direction.
- `docs/product/storage-history.md` defines the storage/history contract.
- `docs/product/diff-merge.md` defines the structural diff and merge direction.
- `docs/product/verification-query.md` defines verified states and query-first access.
- `docs/TEST_MATRIX.md` maps implemented behavior to proof.

## Development Checks

Before claiming a change is complete, run:

```powershell
cargo fmt --check
cargo test
cargo build
.\scripts\bin\harness-cli.exe story verify-all
```
