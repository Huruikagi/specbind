[CmdletBinding()]
param(
    [string]$Version,
    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'SpecBind\bin')
)

$ErrorActionPreference = 'Stop'
$Repository = 'Huruikagi/specbind'

if ($env:OS -ne 'Windows_NT') {
    throw 'install.ps1 supports Windows only.'
}
if ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -ne
    [System.Runtime.InteropServices.Architecture]::X64) {
    throw 'install.ps1 supports Windows x64 only.'
}

if ([string]::IsNullOrWhiteSpace($Version)) {
    $Headers = @{
        Accept = 'application/vnd.github+json'
        'X-GitHub-Api-Version' = '2022-11-28'
    }
    $Release = Invoke-RestMethod `
        -Uri "https://api.github.com/repos/$Repository/releases/latest" `
        -Headers $Headers
    $Tag = $Release.tag_name
    if ([string]::IsNullOrWhiteSpace($Tag)) {
        throw 'Could not resolve the latest stable SpecBind release.'
    }
} elseif ($Version.StartsWith('v')) {
    $Tag = $Version
} else {
    $Tag = "v$Version"
}

if ($Tag -notmatch '^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$') {
    throw "Unsupported release version: $Tag"
}

$Archive = "specbind-$Tag-x86_64-pc-windows-msvc.zip"
$BaseUrl = "https://github.com/$Repository/releases/download/$Tag"
$TemporaryDir = Join-Path ([System.IO.Path]::GetTempPath()) `
    "specbind-install-$([Guid]::NewGuid())"

try {
    New-Item -ItemType Directory -Path $TemporaryDir | Out-Null
    $ArchivePath = Join-Path $TemporaryDir $Archive
    $ChecksumsPath = Join-Path $TemporaryDir 'SHA256SUMS'
    Invoke-WebRequest -Uri "$BaseUrl/$Archive" -OutFile $ArchivePath
    Invoke-WebRequest -Uri "$BaseUrl/SHA256SUMS" -OutFile $ChecksumsPath

    $ChecksumLine = Get-Content -LiteralPath $ChecksumsPath |
        Where-Object { $_ -match "^([0-9a-fA-F]{64})\s+$([regex]::Escape($Archive))$" } |
        Select-Object -First 1
    if ($null -eq $ChecksumLine) {
        throw "SHA256SUMS has no entry for $Archive."
    }
    $Expected = ([regex]::Match($ChecksumLine, '^[0-9a-fA-F]{64}')).Value
    $Actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $ArchivePath).Hash
    if ($Actual -ne $Expected) {
        throw "Checksum verification failed for $Archive."
    }

    $ExtractDir = Join-Path $TemporaryDir 'archive'
    Expand-Archive -LiteralPath $ArchivePath -DestinationPath $ExtractDir
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    $Destination = Join-Path $InstallDir 'specbind.exe'
    Copy-Item -Force -LiteralPath (Join-Path $ExtractDir 'specbind.exe') `
        -Destination $Destination

    $ActualVersion = & $Destination --version
    $ExpectedVersion = "specbind $($Tag.Substring(1))"
    if ($ActualVersion -ne $ExpectedVersion) {
        throw "Installed binary reports '$ActualVersion'; expected '$ExpectedVersion'."
    }

    Write-Output "Installed $ActualVersion to $Destination"
    $PathEntries = $env:Path -split ';'
    if ($InstallDir -notin $PathEntries) {
        Write-Output 'Add it to this PowerShell session with:'
        Write-Output "  `$env:Path = `"$InstallDir;`$env:Path`""
    }
} finally {
    if (Test-Path -LiteralPath $TemporaryDir) {
        Remove-Item -Recurse -Force -LiteralPath $TemporaryDir
    }
}
