$ErrorActionPreference = 'Stop'

Write-Host "==> blink-md Installer for Windows" -ForegroundColor Blue

$Repo = "billlzzz26/blink-md"
$LatestReleaseJson = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
$Version = $LatestReleaseJson.tag_name
$Artifact = "blink-md-windows-amd64.zip"
$Url = "https://github.com/$Repo/releases/download/$Version/$Artifact"

$InstallDir = "$HOME\.blink-md\bin"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir
}

$TmpDir = [System.IO.Path]::GetTempPath()
$ZipFile = Join-Path $TmpDir $Artifact

Write-Host "==> Downloading blink-md $Version..." -ForegroundColor Blue
Invoke-WebRequest -Uri $Url -OutFile $ZipFile

Write-Host "==> Extracting..." -ForegroundColor Blue
Expand-Archive -Path $ZipFile -DestinationPath $TmpDir -Force
Move-Item -Path (Join-Path $TmpDir "blink-md.exe") -Destination (Join-Path $InstallDir "blink-md.exe") -Force

# Add to PATH for current session
$Path = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($Path -notlike "*$InstallDir*") {
    Write-Host "==> Adding to User PATH..." -ForegroundColor Blue
    [Environment]::SetEnvironmentVariable("PATH", "$Path;$InstallDir", "User")
    $env:PATH += ";$InstallDir"
}

Write-Host "==> Successfully installed blink-md!" -ForegroundColor Green
Write-Host "==> Please restart your terminal and run 'blink-md --help'" -ForegroundColor Green
