param(
    [string]$Repository = "longmaba/substrate",
    [string]$Tag = "v0.1.3",
    [string]$DownloadDir = "",
    [string]$InstallSmokeDir = "",
    [switch]$KeepDownloads
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$expectedBinaries = @(
    "substrate-linux-arm64",
    "substrate-linux-x64",
    "substrate-macos-arm64",
    "substrate-macos-x64",
    "substrate-windows-x64.exe"
)

function Get-GhCommand {
    $command = Get-Command gh -ErrorAction SilentlyContinue
    if ($null -eq $command) {
        throw "GitHub CLI 'gh' is required for live release verification"
    }
    return $command.Source
}

function ConvertTo-AssetMap {
    param($Assets)

    $map = @{}
    foreach ($asset in $Assets) {
        $map[$asset.name] = $asset
    }
    return $map
}

function Assert-FileHash {
    param(
        [string]$Path,
        [string]$ExpectedHash,
        [string]$Name
    )

    $actualHash = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actualHash -ne $ExpectedHash.ToLowerInvariant()) {
        throw "checksum mismatch for $Name`nexpected: $ExpectedHash`nactual:   $actualHash"
    }
    return $actualHash
}

function Test-AssetDigest {
    param(
        $Asset,
        [string]$Path
    )

    if ($null -eq $Asset.digest -or !$Asset.digest.StartsWith("sha256:")) {
        return
    }

    $expectedDigest = $Asset.digest.Substring("sha256:".Length).ToLowerInvariant()
    $actualDigest = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actualDigest -ne $expectedDigest) {
        throw "GitHub asset digest mismatch for $($Asset.name)`nexpected: $expectedDigest`nactual:   $actualDigest"
    }
}

if (!$DownloadDir) {
    $DownloadDir = Join-Path ([System.IO.Path]::GetTempPath()) ("substrate-gh-release-" + [guid]::NewGuid().ToString("n"))
}
if (!$InstallSmokeDir) {
    $InstallSmokeDir = Join-Path ([System.IO.Path]::GetTempPath()) ("substrate-gh-install-" + [guid]::NewGuid().ToString("n"))
}

$gh = Get-GhCommand
$expectedAssets = @()
foreach ($binary in $expectedBinaries) {
    $expectedAssets += $binary
    $expectedAssets += "$binary.sha256"
}

New-Item -ItemType Directory -Force -Path $DownloadDir | Out-Null

try {
    $releaseJson = & $gh release view $Tag --repo $Repository --json tagName,name,assets,url,publishedAt,targetCommitish
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
    $release = $releaseJson | ConvertFrom-Json
    if ($release.tagName -ne $Tag) {
        throw "release tag mismatch: expected $Tag, got $($release.tagName)"
    }

    $assetMap = ConvertTo-AssetMap -Assets $release.assets
    foreach ($assetName in $expectedAssets) {
        if (!$assetMap.ContainsKey($assetName)) {
            throw "release $Tag is missing expected asset: $assetName"
        }
    }

    $unexpectedAssets = @($assetMap.Keys | Where-Object { $expectedAssets -notcontains $_ } | Sort-Object)
    if ($unexpectedAssets.Count -gt 0) {
        throw "release $Tag has unexpected assets: $($unexpectedAssets -join ', ')"
    }

    & $gh release download $Tag --repo $Repository --dir $DownloadDir --clobber
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }

    foreach ($assetName in $expectedAssets) {
        $assetPath = Join-Path $DownloadDir $assetName
        if (!(Test-Path -LiteralPath $assetPath -PathType Leaf)) {
            throw "downloaded release is missing expected file: $assetPath"
        }
        Test-AssetDigest -Asset $assetMap[$assetName] -Path $assetPath
    }

    foreach ($binary in $expectedBinaries) {
        $binaryPath = Join-Path $DownloadDir $binary
        $checksumPath = Join-Path $DownloadDir "$binary.sha256"
        $checksumParts = ((Get-Content -LiteralPath $checksumPath -Raw).Trim() -split '\s+')
        if ($checksumParts.Count -lt 2) {
            throw "checksum file does not contain '<hash> <name>': $checksumPath"
        }
        if ($checksumParts[1] -ne $binary) {
            throw "checksum file name mismatch for ${binary}: found $($checksumParts[1])"
        }
        Assert-FileHash -Path $binaryPath -ExpectedHash $checksumParts[0] -Name $binary | Out-Null
    }

    .\scripts\install-substrate.ps1 -BaseUrl $DownloadDir -InstallDir $InstallSmokeDir

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

    Write-Output "release: $($release.url)"
    Write-Output "tag: $($release.tagName)"
    Write-Output "published_at: $($release.publishedAt)"
    Write-Output "assets_verified: $($expectedAssets.Count)"
    if ($KeepDownloads) {
        Write-Output "downloads: $DownloadDir"
    }
} finally {
    if (!$KeepDownloads) {
        Remove-Item -LiteralPath $DownloadDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    Remove-Item -LiteralPath $InstallSmokeDir -Recurse -Force -ErrorAction SilentlyContinue
}
