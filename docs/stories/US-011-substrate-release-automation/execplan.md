# Exec Plan

## Goal

Provide automated cross-platform Substrate binary releases and checksum-verified
repo-local installers.

## Scope

In scope:

- Release packaging scripts for the Substrate CLI.
- Tag-triggered GitHub Actions release workflow.
- Shell and PowerShell installers that install into `scripts/bin/`.
- README, script docs, story matrix, and decision records.

Out of scope:

- Global PATH installation.
- `substrate init` during install.
- Git-compatible remote protocol behavior.
- Hosted package managers such as Homebrew, winget, apt, or npm.

## Risk Classification

Risk flags:

- External systems: GitHub Actions and GitHub Releases.
- Public contracts: release asset names and installer commands.
- Cross-platform: Windows, macOS, and Linux binaries.
- Weak proof: first release workflow cannot be fully proven until a tag is
  pushed on GitHub.

Hard gates:

- External provider behavior.

Lane: high-risk.

## Work Phases

1. Add release artifact builders and checksum generation.
2. Add tag-triggered GitHub Actions workflow.
3. Add shell and PowerShell installers.
4. Update docs, story packet, matrix, and decision record.
5. Validate formatting, tests, build, packaging, checksum install smoke, and
   Harness story verification.
6. Record Harness trace and remaining release proof gaps.

## Stop Conditions

Pause for human confirmation if:

- Installation scope changes from repo-local to global.
- Supported platforms change after implementation starts.
- Release proof requirements need to be weakened.
- The workflow needs secrets or third-party release actions.
