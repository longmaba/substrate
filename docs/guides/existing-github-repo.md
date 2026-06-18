# Start With An Existing GitHub Repo

This guide shows how to try Substrate beside an existing GitHub repository.
Phase 0 is local-only: it does not push to GitHub, rewrite Git history, or
replace your remote. Keep using GitHub as the collaboration surface while using
Substrate to capture, verify, benchmark, and structurally diff local states.

## 1. Clone Or Prepare A Clean Source Tree

Clone the repository as usual:

```powershell
git clone https://github.com/<owner>/<repo>.git
cd <repo>
```

For a disposable trial, a source export or temporary working-tree copy is still
useful:

```powershell
git archive --format=tar HEAD | tar -xf - -C ..\repo-substrate-copy
cd ..\repo-substrate-copy
```

Ingestion now skips `.substrate`, honors root `.gitignore` patterns, and skips
common local-only directories such as `.git`, `node_modules`, `target`, `dist`,
`build`, `coverage`, `.cache`, `.next`, and `.turbo`.

## 2. Initialize Substrate

From the source tree:

```powershell
cargo run --manifest-path P:\projects\substrate\Cargo.toml -- init .
cargo run --manifest-path P:\projects\substrate\Cargo.toml -- status .
```

This creates `.substrate/` in the target tree. The store is local to that copy.
It is safe to delete the copy when you are done experimenting.

## 3. Ingest A Candidate State

```powershell
cargo run --manifest-path P:\projects\substrate\Cargo.toml -- ingest .
```

Record the printed `state_id`. New ingested states start as `candidate`.

Inspect the state label and metadata:

```powershell
cargo run --manifest-path P:\projects\substrate\Cargo.toml -- state <state-id>
```

## 4. Verify And Project

Project a state back to ordinary files:

```powershell
cargo run --manifest-path P:\projects\substrate\Cargo.toml -- project <state-id> --out ..\projected-state
```

Verify the state with local gates and the included benchmark fixture:

```powershell
cargo run --manifest-path P:\projects\substrate\Cargo.toml -- verify <state-id> --out ..\verified-state --bench P:\projects\substrate\fixtures\storage-agent-churn
```

Verification currently checks manifest parsing, object integrity, projection
stability, and benchmark completion. Passing verification promotes the state to
`verified-green`.

## 5. Compare Two Local Repo States

Use two directories, worktrees, exported commits, or branch copies:

```powershell
cargo run --manifest-path P:\projects\substrate\Cargo.toml -- diff ..\repo-before ..\repo-after
```

Supported parser-backed diff paths:

| Files | Behavior |
| --- | --- |
| `.rs` | normalized Rust function-block comparison |
| `.ts`, `.tsx` | tree-sitter TypeScript named-node fingerprints |
| `.js`, `.jsx` | tree-sitter JavaScript named-node fingerprints |
| `.py` | tree-sitter Python named-node fingerprints |
| `.cs` | tree-sitter C# named-node fingerprints |
| anything else | text-diff fallback accounting |

The output keeps `semantic_equivalence_claimed: no`. Treat normalized node
counts as review-noise signals, not proof that behavior is identical.

## Recommended Agent Workflow

When an agent is working in an existing repo:

1. Create or use a clean source copy.
2. Run `init`, then `ingest` before a risky edit to capture the baseline.
3. Make the edit with the normal project toolchain.
4. Run `ingest` again to capture the candidate.
5. Run `diff` between the baseline projection and candidate tree.
6. Run the repo's own tests, then `verify` the Substrate state.
7. Only describe a state as `verified-green` after Substrate reports that label.

## Current Limits

- No GitHub API integration.
- No Git protocol emulation.
- `.gitignore` support is intentionally basic and root-scoped; advanced Git
  ignore semantics such as nested ignore files and full glob parity are not yet
  implemented.
- No remote push, pull, fetch, branch, or PR commands.
- No semantic-equivalence proof, even when normalized changed nodes are zero.

These limits are intentional for the Phase 0 wedge. The goal is to prove local
storage, projection, verification, and structural diff value before expanding
the compatibility surface.
