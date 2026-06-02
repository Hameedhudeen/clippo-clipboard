param(
    [string]$Configuration = "Release",
    [string]$Runtime = "win-x64",
    [string]$Version = "0.1.2"
)

$ErrorActionPreference = "Stop"

if ([System.Environment]::OSVersion.Platform -ne [System.PlatformID]::Win32NT) {
    Write-Error "Windows zip packaging requires Windows."
}

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$Project = Join-Path $RepoRoot "apps/windows/Clippo.Windows.csproj"
$PublishDir = Join-Path $RepoRoot "dist/windows/clippo-$Version-windows-x64"
$OutputDir = Join-Path $RepoRoot "dist/windows"
$ArchivePath = Join-Path $OutputDir "Clippo-$Version-windows-x64-alpha.zip"

if (-not (Get-Command dotnet -ErrorAction SilentlyContinue)) {
    Write-Error "dotnet is not installed. Install the .NET desktop workload before packaging Clippo for Windows."
}

Remove-Item -Recurse -Force $PublishDir -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $PublishDir, $OutputDir | Out-Null

dotnet publish $Project `
    -c $Configuration `
    -r $Runtime `
    --self-contained true `
    -p:PublishSingleFile=false `
    -o $PublishDir

Copy-Item -Force (Join-Path $RepoRoot "README.md") (Join-Path $PublishDir "README.md")
Copy-Item -Force (Join-Path $RepoRoot "LICENSE") (Join-Path $PublishDir "LICENSE")

if (Test-Path $ArchivePath) {
    Remove-Item -Force $ArchivePath
}

Compress-Archive -Path (Join-Path $PublishDir "*") -DestinationPath $ArchivePath -Force
Write-Host "Created $ArchivePath"
