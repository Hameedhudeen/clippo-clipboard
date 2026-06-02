param(
    [string]$Configuration = "Release",
    [string]$Runtime = "win-x64",
    [string]$Version = "0.1.1.0"
)

$ErrorActionPreference = "Stop"

if ([System.Environment]::OSVersion.Platform -ne [System.PlatformID]::Win32NT) {
    Write-Error "Windows MSIX packaging requires Windows."
}

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$Project = Join-Path $RepoRoot "apps/windows/Clippo.Windows.csproj"
$ManifestTemplate = Join-Path $RepoRoot "packaging/windows/AppxManifest.xml"
$PublishDir = Join-Path $RepoRoot "dist/windows/publish"
$PackageRoot = Join-Path $RepoRoot "dist/windows/msix-root"
$OutputDir = Join-Path $RepoRoot "dist/windows"
$OutputPackage = Join-Path $OutputDir "Clippo_$Version.msix"

if (-not (Get-Command dotnet -ErrorAction SilentlyContinue)) {
    Write-Error "dotnet is not installed. Install the .NET desktop workload before packaging Clippo for Windows."
}

if (-not (Get-Command makeappx.exe -ErrorAction SilentlyContinue)) {
    Write-Error "makeappx.exe is not available. Install the Windows SDK and run this script from a Developer PowerShell."
}

Remove-Item -Recurse -Force $PublishDir, $PackageRoot -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $PublishDir, $PackageRoot, $OutputDir | Out-Null

dotnet publish $Project `
    -c $Configuration `
    -r $Runtime `
    --self-contained true `
    -p:PublishSingleFile=false `
    -o $PublishDir

Copy-Item -Recurse -Force (Join-Path $PublishDir "*") $PackageRoot
New-Item -ItemType Directory -Force (Join-Path $PackageRoot "Assets") | Out-Null

$Manifest = Get-Content $ManifestTemplate -Raw
$Manifest = $Manifest.Replace('Version="0.1.1.0"', "Version=`"$Version`"")
Set-Content -Path (Join-Path $PackageRoot "AppxManifest.xml") -Value $Manifest -Encoding UTF8

foreach ($Asset in @("StoreLogo.png", "Square44x44Logo.png", "Square150x150Logo.png")) {
    $SourceAsset = Join-Path $RepoRoot "packaging/windows/Assets/$Asset"
    if (-not (Test-Path $SourceAsset)) {
        Write-Error "Missing Windows package asset $SourceAsset. Add branded PNG assets before creating MSIX artifacts."
    }
    Copy-Item -Force $SourceAsset (Join-Path $PackageRoot "Assets/$Asset")
}

if (Test-Path $OutputPackage) {
    Remove-Item -Force $OutputPackage
}

makeappx.exe pack /d $PackageRoot /p $OutputPackage /o

Write-Host "Created $OutputPackage"
Write-Host "Signing is intentionally handled by the release workflow or local certificate setup."
