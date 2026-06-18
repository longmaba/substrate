param(
    [string]$InstallSmokeDir = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (!$InstallSmokeDir) {
    $InstallSmokeDir = Join-Path ([System.IO.Path]::GetTempPath()) ("substrate-install-smoke-" + [guid]::NewGuid().ToString("n"))
}

try {
    cargo fmt --check
    cargo test
    cargo build

    .\scripts\build-substrate-release.ps1
    .\scripts\install-substrate.ps1 -BaseUrl dist -InstallDir $InstallSmokeDir

    $installedName = "substrate"
    if ([System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Windows)) {
        $installedName = "substrate.exe"
    }

    $installedPath = Join-Path $InstallSmokeDir $installedName
    if (!(Test-Path -LiteralPath $installedPath -PathType Leaf)) {
        throw "installer smoke did not create $installedPath"
    }

    & $installedPath status .
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
} finally {
    Remove-Item -LiteralPath $InstallSmokeDir -Recurse -Force -ErrorAction SilentlyContinue
}
