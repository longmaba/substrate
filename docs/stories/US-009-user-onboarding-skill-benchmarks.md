# US-009 User Onboarding, Agent Skill, And Benchmarks

## Status

implemented

## Lane

normal

## Product Contract

Substrate has enough user-facing documentation for a developer or agent to run
the Phase 0 CLI, try it on an existing GitHub repository, use a repo-local agent
skill, and cite measured benchmark evidence without overclaiming.

## Relevant Product Docs

- `docs/product/overview.md`
- `docs/product/storage-history.md`
- `docs/product/diff-merge.md`
- `docs/product/verification-query.md`

## Acceptance Criteria

- Root README describes Substrate rather than the generic Harness scaffold.
- README includes installation, command usage, quickstart, structural diff,
  benchmark summary, existing-repo pointer, and agent-skill pointer.
- Existing GitHub repo guide explains the local-only Phase 0 workflow and
  current ignore-rule behavior.
- Agent skill exists as a reusable `SKILL.md` with valid frontmatter and a clear
  workflow for initialize, ingest, project, verify, diff, and benchmark tasks.
- Benchmark doc cites measured fixture results and labels token savings as an
  estimate, not a model-billing measurement.
- Durable story and test matrix records are updated.

## Design Notes

- Commands: documentation references the existing Phase 0 CLI only.
- UI surfaces: Markdown docs and repo-local skill folder.
- Non-goal: no CLI behavior change and no GitHub remote integration.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id <id> --unit 0 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | Not required; this is docs and skill packaging. |
| Integration | Documentation references existing CLI commands and measured fixture output. |
| E2E | Not required; there is no hosted user flow. |
| Platform | `cargo run -- bench`, `cargo run -- diff`, `cargo test`, `cargo build`, and skill validation prove the local surface remains usable. |
| Release | Not required for this story. |

## Harness Delta

- Add a durable US-009 story row and update the test matrix when proof exists.
- Record any onboarding friction found while writing the GitHub guide.

## Evidence

- Root README rewritten for Substrate usage.
- Added `docs/guides/existing-github-repo.md`.
- Added `docs/benchmarks/phase-0-results.md`.
- Added `skills/substrate/SKILL.md` and `skills/substrate/agents/openai.yaml`.
- `cargo run --quiet -- bench fixtures/storage-agent-churn` reported 91.4% fewer bytes than the whole-file fixture baseline and 11.6852x dedup ratio.
- `cargo run --quiet -- diff fixtures/diff-javascript-pairs/before fixtures/diff-javascript-pairs/after` reported parser-backed JavaScript counts and unsupported fallback accounting.
- `cargo run --quiet -- diff fixtures/diff-typescript-pairs/before fixtures/diff-typescript-pairs/after` reported parser-backed TypeScript counts and unsupported fallback accounting.
- `python C:\Users\longm\.codex\skills\.system\skill-creator\scripts\quick_validate.py skills/substrate` passed.
- `cargo fmt --check`, `cargo test`, and `cargo build` passed.
