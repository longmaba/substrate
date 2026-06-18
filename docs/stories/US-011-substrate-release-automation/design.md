# Design

## Domain Model

Release artifacts are platform-labeled Substrate binaries plus sibling checksum
files. The asset labels are stable public identifiers used by the installers:

- `substrate-macos-arm64`
- `substrate-macos-x64`
- `substrate-linux-x64`
- `substrate-linux-arm64`
- `substrate-windows-x64.exe`

## Application Flow

1. Maintainer pushes a tag matching `v*`.
2. GitHub Actions runs formatting, tests, and a normal build.
3. Matrix jobs build release binaries for each supported platform.
4. Each build job uploads the binary and `.sha256` as workflow artifacts.
5. The release job creates or updates the GitHub Release and uploads the final
   assets with `gh release`.
6. Installers resolve the latest release or an explicitly requested tag,
   download the matching asset and checksum, verify SHA-256, and copy the binary
   into `scripts/bin/` in the current repository.

## Interface Contract

User-facing installer commands:

```bash
curl -fsSL "https://raw.githubusercontent.com/longmaba/substrate/main/scripts/install-substrate.sh" | bash
```

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/longmaba/substrate/main/scripts/install-substrate.ps1")))
```

Installed command paths:

- macOS/Linux: `./scripts/bin/substrate <command>`
- Windows: `.\scripts\bin\substrate.exe <command>`

Overrides:

- `SUBSTRATE_RELEASE_TAG` selects a tag.
- `SUBSTRATE_BASE_URL` selects an alternate artifact directory for local smoke
  tests.
- PowerShell equivalents are `-ReleaseTag` and `-BaseUrl`.

## Data Model

No persistent Substrate data model changes. Installers create or update only the
repo-local binary under `scripts/bin/`.

## UI / Platform Impact

This is a CLI and release-platform change. Supported platforms are Windows x64,
macOS x64/arm64, and Linux x64/arm64. Unsupported platforms fail before download
with a clear error.

## Observability

GitHub Actions job logs are the release proof surface. Local Harness traces and
story evidence record implementation and validation commands.

## Alternatives Considered

1. Global install to PATH. Rejected to keep installation repo-local and avoid
   mutating shell profiles.
2. Cargo-only install. Rejected because this story exists to remove the Rust
   toolchain requirement for normal users.
3. Published-release event trigger. Rejected in favor of tag-driven release
   generation as the source of truth.
