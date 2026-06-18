# US-013 Live Release Verification

## Status

implemented

## Lane

normal

## Product Contract

Substrate release automation has repeatable live proof against a published
GitHub Release. The proof verifies that the tag-driven workflow published the
expected asset contract, that every downloaded binary matches its checksum, and
that the current-platform installer can install from the downloaded release
assets.

## Relevant Product Docs

- `docs/product/compatibility-shell.md`
- `docs/stories/US-011-substrate-release-automation/validation.md`
- `docs/decisions/0011-substrate-repo-local-binary-releases.md`

## Acceptance Criteria

- Register a present deploy-verification provider for live GitHub Release
  inspection.
- Add a repeatable verification command for a real `v*` GitHub Release tag.
- Verify the release has exactly the supported release binary assets and
  matching `.sha256` files.
- Download the assets from GitHub and verify every binary against its checksum.
- Verify downloaded file hashes against GitHub asset digests when GitHub returns
  digest metadata.
- Smoke-install the current platform binary from the downloaded release assets.
- Update durable proof records and close the release-verification backlog item.

## Design Notes

- Commands: `scripts/verify-substrate-github-release.ps1`.
- External tool: `gh`, registered as the `deploy-verification` provider.
- Domain rules: supported release asset names remain the public installer
  contract from decision `0011-substrate-repo-local-binary-releases`.
- UI surfaces: terminal report only.

## Validation

When updating durable proof status, use numeric booleans:
`scripts/bin/harness-cli story update --id US-013 --unit 1 --integration 1 --e2e 0 --platform 1`.

| Layer | Expected proof |
| --- | --- |
| Unit | PowerShell verifier enforces asset-name, checksum, and digest validation rules. |
| Integration | `gh release view` and `gh release download` inspect and download the published GitHub Release. |
| E2E | Not required; there is no browser or hosted user flow. |
| Platform | Installer smoke installs the Windows release asset from downloaded release files and runs `substrate status .`. |
| Release | `scripts/verify-substrate-github-release.ps1` verifies `v0.1.3` on GitHub. |

## Harness Delta

- Registered `gh` as a present `deploy-verification` provider.
- Added durable US-013 story proof.
- Closed backlog item #2 after live release proof passed.

## Evidence

- `scripts/bin/harness-cli.exe tool check --name gh` passed and reported
  `gh` present for `deploy-verification`.
- `scripts/verify-substrate-github-release.ps1` passed against
  `https://github.com/longmaba/substrate/releases/tag/v0.1.3`, published at
  `2026-06-18T16:31:57Z`, with `assets_verified: 10`.
- The live verifier installed the downloaded Windows asset to a temporary
  directory and `substrate status .` returned `initialized: no` for the current
  repository root.
