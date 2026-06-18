param(
    [string]$Repository = $env:SUBSTRATE_REPOSITORY,
    [string]$ReleaseTag = $env:SUBSTRATE_RELEASE_TAG,
    [string]$BaseUrl = $env:SUBSTRATE_BASE_URL,
    [string]$InstallDir = "scripts/bin"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (!$Repository) {
    $Repository = "longmaba/substrate"
}

function Get-HostArchitecture {
    $runtimeInfo = [System.Runtime.InteropServices.RuntimeInformation]
    $osArchitecture = $runtimeInfo.GetProperty("OSArchitecture")
    if ($null -ne $osArchitecture) {
        return $osArchitecture.GetValue($null).ToString()
    }

    $machineArchitecture = $env:PROCESSOR_ARCHITEW6432
    if (!$machineArchitecture) {
        $machineArchitecture = $env:PROCESSOR_ARCHITECTURE
    }

    switch ($machineArchitecture) {
        "AMD64" { return "X64" }
        "ARM64" { return "Arm64" }
        "x86" { return "X86" }
        default {
            if ([System.Environment]::Is64BitOperatingSystem) { return "X64" }
            return $machineArchitecture
        }
    }
}

function Get-PlatformLabel {
    $arch = Get-HostArchitecture

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

    $os = [System.Runtime.InteropServices.RuntimeInformation]::OSDescription
    throw "unsupported platform: $os/$arch"
}

function Join-SourcePath {
    param(
        [string]$Base,
        [string]$Name
    )

    if ($Base -match '^https?://') {
        return "$($Base.TrimEnd('/'))/$Name"
    }
    if ($Base -match '^file://') {
        return "$($Base.TrimEnd('/'))/$Name"
    }
    return Join-Path $Base $Name
}

function Copy-Or-Download {
    param(
        [string]$Source,
        [string]$Destination
    )

    if ($Source -match '^https?://') {
        Invoke-WebRequest -Uri $Source -OutFile $Destination -UseBasicParsing
        return
    }

    if ($Source -match '^file://') {
        $sourcePath = ([Uri]$Source).LocalPath
        Copy-Item -LiteralPath $sourcePath -Destination $Destination -Force
        return
    }

    if (Test-Path -LiteralPath $Source -PathType Leaf) {
        Copy-Item -LiteralPath $Source -Destination $Destination -Force
        return
    }

    throw "unsupported download source: $Source"
}

$label = Get-PlatformLabel
$assetName = "substrate-$label"
$installName = "substrate"
if ($label -eq "windows-x64") {
    $assetName = "$assetName.exe"
    $installName = "substrate.exe"
}

if ($BaseUrl) {
    $assetSource = Join-SourcePath -Base $BaseUrl -Name $assetName
    $checksumSource = Join-SourcePath -Base $BaseUrl -Name "$assetName.sha256"
} elseif ($ReleaseTag) {
    $assetSource = "https://github.com/$Repository/releases/download/$ReleaseTag/$assetName"
    $checksumSource = "$assetSource.sha256"
} else {
    $assetSource = "https://github.com/$Repository/releases/latest/download/$assetName"
    $checksumSource = "$assetSource.sha256"
}

$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("substrate-install-" + [guid]::NewGuid().ToString("n"))
New-Item -ItemType Directory -Force -Path $tempRoot | Out-Null

try {
    $assetPath = Join-Path $tempRoot $assetName
    $checksumPath = Join-Path $tempRoot "$assetName.sha256"

    Copy-Or-Download -Source $assetSource -Destination $assetPath
    Copy-Or-Download -Source $checksumSource -Destination $checksumPath

    $expected = (((Get-Content -LiteralPath $checksumPath -Raw).Trim() -split '\s+')[0]).ToLowerInvariant()
    $actual = (Get-FileHash -LiteralPath $assetPath -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($expected -ne $actual) {
        throw "checksum mismatch for $assetName`nexpected: $expected`nactual:   $actual"
    }

    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    $installPath = Join-Path $InstallDir $installName
    Copy-Item -LiteralPath $assetPath -Destination $installPath -Force

    Write-Output "installed: $installPath"
    if ([System.IO.Path]::IsPathRooted($installPath)) {
        Write-Output "run: $installPath <command>"
    } else {
        Write-Output "run: .\$installPath <command>"
    }
} finally {
    Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
