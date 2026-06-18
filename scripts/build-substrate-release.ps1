param(
    [string]$Target = "",
    [string]$OutDir = "dist"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Get-PlatformLabelForTarget {
    param([string]$Target)

    switch ($Target) {
        "x86_64-unknown-linux-gnu" { return "linux-x64" }
        "aarch64-unknown-linux-gnu" { return "linux-arm64" }
        "x86_64-apple-darwin" { return "macos-x64" }
        "aarch64-apple-darwin" { return "macos-arm64" }
        "x86_64-pc-windows-msvc" { return "windows-x64" }
        "x86_64-pc-windows-gnu" { return "windows-x64" }
        default { throw "unsupported cargo target: $Target" }
    }
}

function Get-PlatformLabelForHost {
    $os = [System.Runtime.InteropServices.RuntimeInformation]::OSDescription
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()

    if ([System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Windows)) {
        if ($arch -eq "X64") { return "windows-x64" }
    }
    if ([System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Linux)) {
        if ($arch -eq "X64") { return "linux-x64" }
        if ($arch -eq "Arm64") { return "linux-arm64" }
    }
    if ([System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::OSX)) {
        if ($arch -eq "X64") { return "macos-x64" }
        if ($arch -eq "Arm64") { return "macos-arm64" }
    }

    throw "unsupported host platform: $os/$arch"
}

if ($Target) {
    $platformLabel = Get-PlatformLabelForTarget -Target $Target
    cargo build --release --target $Target
    $binaryDir = Join-Path "target" (Join-Path $Target "release")
} else {
    $platformLabel = Get-PlatformLabelForHost
    cargo build --release
    $binaryDir = Join-Path "target" "release"
}

$binaryName = "substrate"
$artifactName = "substrate-$platformLabel"
if ($platformLabel -eq "windows-x64") {
    $binaryName = "substrate.exe"
    $artifactName = "$artifactName.exe"
}

$binaryPath = Join-Path $binaryDir $binaryName
if (!(Test-Path -LiteralPath $binaryPath -PathType Leaf)) {
    throw "expected release binary not found: $binaryPath"
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$artifactPath = Join-Path $OutDir $artifactName
Copy-Item -LiteralPath $binaryPath -Destination $artifactPath -Force

$hash = (Get-FileHash -LiteralPath $artifactPath -Algorithm SHA256).Hash.ToLowerInvariant()
Set-Content -LiteralPath "$artifactPath.sha256" -Value "$hash  $artifactName" -Encoding ascii

Write-Output "artifact: $artifactPath"
Write-Output "checksum: $artifactPath.sha256"
