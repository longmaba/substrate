# Validation

## Proof Strategy

Local proof must show that the Rust crate still builds and tests, the release
packaging script creates the expected current-platform artifact and checksum,
and the installer can verify and install that artifact from a local artifact
directory. GitHub-hosted proof is required after pushing a real `v*` release
tag.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Existing Rust unit tests continue to pass with no CLI behavior regression. |
| Integration | Packaging script builds release binary, emits stable asset name, and writes `.sha256`. |
| E2E | Not applicable; this story has no browser or interactive product flow. |
| Platform | Windows local packaging and PowerShell installer smoke; GitHub workflow matrix covers all supported platforms on tag push. |
| Performance | No performance claim. |
| Logs/Audit | GitHub Actions logs and Harness trace provide release/implementation evidence. |

## Fixtures

- Current repository root.
- Locally generated `dist/` artifact directory from the release packaging script.

## Commands

```text
cargo fmt --check
cargo test
cargo build
.\scripts\build-substrate-release.ps1
.\scripts\install-substrate.ps1 -BaseUrl dist -InstallDir <temp-dir>
.\scripts\verify-substrate-release.ps1
.\scripts\bin\harness-cli.exe story verify US-011
.\scripts\bin\harness-cli.exe story verify-all
```

After a release tag is pushed:

```text
gh release view <tag>
gh release download <tag> --pattern "substrate-*" --dir <temp-dir>
```

## Acceptance Evidence

- `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify-substrate-release.ps1`: passed; ran `cargo fmt --check`, `cargo test` with 25 passing tests, `cargo build`, Windows release packaging, checksum-verified install from local `dist/`, and `substrate status .` smoke.
- `.\scripts\bin\harness-cli.exe story verify US-011`: passed.
- `.\scripts\bin\harness-cli.exe story verify-all`: passed; 11 stories verified, 11 passed, 0 failed, 0 skipped.
- `git diff --check`: passed.
- `bash -n scripts/build-substrate-release.sh scripts/install-substrate.sh`: passed.
- No `security-scan` provider is registered in the Harness tool registry; skipped cleanly.
- Not yet proven: live GitHub release creation/upload on a pushed `v*` tag.
