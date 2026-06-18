# Substrate

Substrate is a local, agent-native code store for capturing code states with the
verification context around them. It keeps ordinary files as the way humans and
tools inspect code, while giving agents a smaller, queryable store for candidate
states, projections, verification labels, and parser-aware diffs.

Phase 0 is intentionally local-only. Substrate does not replace Git, push to
GitHub, or claim semantic equivalence between versions. Use it beside your
normal repository while testing whether agent-heavy workflows benefit from
verified states and structural diff signals.

## Use Cases

Use Substrate when you want to:

| Use case | What Substrate helps with |
| --- | --- |
| Capture an agent's work before and after a risky edit | Ingest the baseline and candidate trees as local states, then compare or project them later. |
| Review noisy regenerated code | Run parser-backed structural diffs for supported languages and fall back to text accounting elsewhere. |
| Separate candidate work from verified work | Promote a state to `verified-green` only after local verification passes. |
| Benchmark storage pressure from agent churn | Compare whole-file storage with Substrate's content-addressed store on repeat-edit fixtures. |
| Try it on an existing GitHub repo without changing Git history | Initialize a local `.substrate` store in a clone, branch copy, or disposable export. |
| Give coding agents a stable local workflow | Point agents at `skills/substrate/SKILL.md` for initialize, ingest, project, verify, diff, and benchmark tasks. |

Good first experiments:

- Capture a clean repo state, make an agent edit, ingest again, and diff the two
  directories.
- Project a stored state into a fresh output directory and compare it with the
  original tree.
- Run the included benchmark fixture to see how much repeated agent churn can be
  deduplicated locally.

## What It Does Today

- Stores local source-tree states in a `.substrate` directory.
- Projects stored states back to plain files.
- Tracks candidate and `verified-green` state labels.
- Verifies object integrity, projection stability, and benchmark completion.
- Compares files or directories with structural diff support where parsers are
  available.
- Honors root `.gitignore` patterns during ingest and skips common local-only
  directories such as `.git`, `node_modules`, `target`, `dist`, build outputs,
  and cache folders.

Parser-backed diff support currently covers:

| Language | Extensions | Diff strategy |
| --- | --- | --- |
| Rust | `.rs` | normalized function-block placeholder |
| TypeScript | `.ts`, `.tsx` | tree-sitter named syntax-node fingerprints |
| JavaScript | `.js`, `.jsx` | tree-sitter named syntax-node fingerprints |
| Python | `.py` | tree-sitter named syntax-node fingerprints |
| C# | `.cs` | tree-sitter named syntax-node fingerprints |

Unsupported files still appear in fallback text-diff accounting. Diff reports
include this guard because structural change counts are review signals, not a
behavior proof:

```text
semantic_equivalence_claimed: no
```

## Install

Prerequisites:

- Rust toolchain with Cargo for development builds.
- PowerShell on Windows, or a POSIX shell on macOS/Linux.

Install the latest packaged binary into the current repository:

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/longmaba/substrate/main/scripts/install-substrate.ps1")))
```

```bash
curl -fsSL "https://raw.githubusercontent.com/longmaba/substrate/main/scripts/install-substrate.sh" | bash
```

The installer verifies the release checksum and writes the binary to
`scripts/bin/substrate.exe` on Windows or `scripts/bin/substrate` on macOS/Linux.
Run it from the repository where you want the repo-local tool:

```powershell
.\scripts\bin\substrate.exe status .
```

```bash
./scripts/bin/substrate status .
```

Build from this repository during development:

```powershell
cargo build
cargo run -- <command>
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

Verify the state against local checks and the benchmark fixture:

```powershell
cargo run -- verify <state-id> --out .\out\verify --bench fixtures\storage-agent-churn
```

After verification passes, the state label becomes `verified-green`.

## Common Workflows

### Try Substrate Beside An Existing Repo

Start with `docs/guides/existing-github-repo.md` for the full flow. The short
version is:

1. Clone, export, or copy the repository you want to test.
2. Run `init` and `ingest` to capture a baseline state.
3. Make the agent or human edit using the repo's normal tools.
4. Run `ingest` again to capture the candidate state.
5. Run `diff` between the baseline projection and candidate tree.
6. Run the repo's own tests, then `verify` the Substrate state.

### Compare Two Trees

Compare two files or two directories:

```powershell
cargo run -- diff fixtures\diff-javascript-pairs\before fixtures\diff-javascript-pairs\after
cargo run -- diff fixtures\diff-typescript-pairs\before fixtures\diff-typescript-pairs\after
cargo run -- diff fixtures\diff-rust-pairs\before fixtures\diff-rust-pairs\after
```

The report includes line-change counts, parser-backed normalized change counts
for supported languages, fallback counts for unsupported files, and the explicit
`semantic_equivalence_claimed: no` guard.

### Benchmark Agent Churn

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

## CLI Reference

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

When using the installed repo-local binary, replace `cargo run --` with
`.\scripts\bin\substrate.exe` on Windows or `./scripts/bin/substrate` on
macOS/Linux.

## Current Limits

- Local-only Phase 0 tool: no push, pull, fetch, branch, PR, or remote sync
  commands.
- No Git protocol emulation and no GitHub API integration.
- No semantic-equivalence proof, even when normalized changed-node counts are
  zero.
- Root-scoped `.gitignore` support only; advanced nested ignore behavior is not
  yet full Git parity.
- Query-first inspection for symbols, verified states, and state provenance is
  planned but not implemented yet.

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

## Roadmap

- Add more parser-backed languages beyond Rust, TypeScript, JavaScript, JSX,
  Python, and C#.
- Add larger benchmarks against real agent-churn repositories and compare
  storage, transfer, and review-noise metrics against Git-style baselines.
- Add Git-compatible import/export workflows before attempting remote or
  protocol-level GitHub integration.
- Add query-first inspection commands for symbols, verified states, and state
  provenance.
- Add retention and compaction policy for rejected or superseded candidate
  states.

Recently completed:

- Packaged binary releases and repo-local installers are available for Windows
  x64, macOS x64/arm64, and Linux x64/arm64.
- Parser-backed diff support now includes Python and C# through tree-sitter.
- Ingest honors root `.gitignore` patterns and skips common local-only
  directories.
- Diff language support now goes through a parser registry table for supported
  extensions and parser functions.

## Development Checks

Before claiming a change is complete, run:

```powershell
cargo fmt --check
cargo test
cargo build
.\scripts\bin\harness-cli.exe story verify-all
```
