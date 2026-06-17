---
name: substrate
description: Use this skill whenever a user wants an agent to use Substrate, initialize or inspect a .substrate store, ingest/project/verify code states, run structural diffs, benchmark agent-churn storage, or try Substrate on an existing GitHub repository. Trigger for Substrate CLI usage, agent-native code store workflows, verified-green state checks, parser-backed diff reports, and local repo onboarding.
---

# Substrate

Use this skill to operate the local Substrate Phase 0 CLI safely and honestly.
Substrate is a local agent-native code-store wedge: it initializes a repo-local
`.substrate` store, ingests source trees as candidate states, projects states
back to files, verifies states, runs structural diffs, and benchmarks storage
churn.

## Ground Rules

- Work from the repository root unless the user gives a target path.
- Prefer `cargo run -- <command>` inside the Substrate repo during development.
- When controlling Substrate from another repository, use `cargo run --manifest-path <substrate>/Cargo.toml -- <command>`.
- Treat Phase 0 as local-only: no GitHub API, no Git protocol emulation, no remote push/pull/fetch.
- Do not claim semantic equivalence. The diff report intentionally says `semantic_equivalence_claimed: no`.
- Before using an existing repository, warn that ingestion skips `.substrate` but does not yet honor `.gitignore`.

## Command Map

```text
init <path>                                      create a local .substrate store
status [path]                                   inspect store initialization
ingest <path>                                   capture a candidate state
state <state-id>                                inspect candidate/verified metadata
project <state-id> --out <path>                 materialize a state as files
verify <state-id> --out <path> --bench <path>   run local gates and promote if passing
diff <left> <right>                             compare files or directories
bench <fixture-path>                            run storage benchmark fixture
```

## Standard Workflow

1. Initialize the target tree:
   ```powershell
   cargo run -- init .
   cargo run -- status .
   ```
2. Ingest the current tree and save the printed `state_id`:
   ```powershell
   cargo run -- ingest .
   ```
3. Inspect the state:
   ```powershell
   cargo run -- state <state-id>
   ```
4. Project it if the user needs ordinary files:
   ```powershell
   cargo run -- project <state-id> --out .\out\projected
   ```
5. Verify it before calling it green:
   ```powershell
   cargo run -- verify <state-id> --out .\out\verify --bench fixtures\storage-agent-churn
   ```

Only say `verified-green` after `state <state-id>` or `verify` reports that label.

## Existing GitHub Repo Workflow

Use this when the user asks to try Substrate on an existing repo:

1. Clone or export the repository with normal Git tools.
2. Prefer a clean source export or temporary copy that excludes `.git`,
   dependencies, caches, and build outputs.
3. From that clean tree, run Substrate with an explicit manifest path:
   ```powershell
   cargo run --manifest-path P:\projects\substrate\Cargo.toml -- init .
   cargo run --manifest-path P:\projects\substrate\Cargo.toml -- ingest .
   ```
4. Use `project`, `state`, and `verify` for state inspection.
5. Compare two local copies, worktrees, or exported commits with `diff`:
   ```powershell
   cargo run --manifest-path P:\projects\substrate\Cargo.toml -- diff ..\repo-before ..\repo-after
   ```

If the user needs a human-facing guide, point to `docs/guides/existing-github-repo.md`.

## Structural Diff Workflow

Run:

```powershell
cargo run -- diff <left> <right>
```

Supported parser-backed inputs:

| Files | Behavior |
| --- | --- |
| `.rs` | normalized Rust function-block comparison |
| `.ts`, `.tsx` | tree-sitter TypeScript named-node fingerprints |
| `.js`, `.jsx` | tree-sitter JavaScript named-node fingerprints |
| anything else | text-diff fallback accounting |

Interpret the result as review-noise evidence. Formatting-only and reorder
examples can show zero normalized changed nodes even when line counts change,
but this is not proof of equivalent runtime behavior.

## Benchmark Workflow

Run:

```powershell
cargo run -- bench fixtures\storage-agent-churn
```

Use measured numbers from `docs/benchmarks/phase-0-results.md`. Current fixture
headline: 117,950 whole-file baseline bytes versus 10,094 Substrate stored
bytes, a 91.4% reduction and 11.6852x dedup ratio on the included fixture.

When discussing token savings, say "token-equivalent estimate" and explain that
the number divides bytes by 4. Do not present it as a tokenizer run or billing
measurement.

## Verification Before Reporting

For Substrate repo changes, run:

```powershell
cargo fmt --check
cargo test
cargo build
.\scripts\bin\harness-cli.exe story verify-all
```

For documentation-only usage support, also run the example `bench` and `diff`
commands whose outputs are cited.
