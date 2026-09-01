# Claude Code Setup - Windows Installer
# Downloads and installs the latest release for Windows x64
# Usage: powershell -ExecutionPolicy Bypass -File install-windows.ps1

param(
    [string]$InstallDir = "$env:PROGRAMFILES\ClaudeCodeSetup",
    [switch]$SkipConfig
)

$ProgressPreference = 'SilentlyContinue'
$ErrorActionPreference = 'Stop'

Write-Host "🚀 Claude Code Setup - Windows Installer" -ForegroundColor Cyan
Write-Host "📦 Installing to: $InstallDir" -ForegroundColor Gray

# Fetch latest release
Write-Host "📥 Fetching latest release..." -ForegroundColor Yellow
$releases = Invoke-RestMethod -Uri "https://api.github.com/repos/Ercaner1988/claude-code-setup-rustified/releases" -Headers @{"Accept"="application/vnd.github.v3+json"}
$latest = $releases[0]
$latestTag = $latest.tag_name
Write-Host "   Found: $latestTag" -ForegroundColor Green

# Find Windows x64 asset
$winAsset = $latest.assets | Where-Object { $_.name -match "windows-x86_64" } | Select-Object -First 1
if (-not $winAsset) {
    Write-Host "❌ Windows x64 binary not found in release" -ForegroundColor Red
    exit 1
}

Write-Host "   Binary: $($winAsset.name)" -ForegroundColor Green

# Create install directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Host "✅ Created directory: $InstallDir" -ForegroundColor Green
}

# Download binary
$exePath = Join-Path $InstallDir "claude-code-setup.exe"
Write-Host "⬇️  Downloading binary..." -ForegroundColor Yellow
Invoke-WebRequest -Uri $winAsset.browser_download_url -OutFile $exePath -UseBasicParsing
Write-Host "✅ Downloaded: $exePath" -ForegroundColor Green

# Add to PATH
$pathEnv = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($pathEnv -notlike "*$InstallDir*") {
    Write-Host "🔧 Adding to user PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("PATH", "$pathEnv;$InstallDir", "User")
    $env:PATH += ";$InstallDir"
    Write-Host "✅ Added to PATH" -ForegroundColor Green
}

# Make executable
Write-Host "🔐 Setting permissions..." -ForegroundColor Yellow
icacls $exePath /grant:r "$env:USERNAME`:(F)" /inheritance:e /T | Out-Null
Write-Host "✅ Executable set" -ForegroundColor Green

# Run install command (optional config)
if (-not $SkipConfig) {
    Write-Host ""
    Write-Host "⚙️  Running setup..." -ForegroundColor Yellow
    & $exePath install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️  Setup had warnings (check above), but binary installed" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "✅ Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📝 Next steps:" -ForegroundColor Cyan
Write-Host "   1. Restart PowerShell or run: `$profile | % { . `$_ }" -ForegroundColor Gray
Write-Host "   2. Verify: claude-code-setup --version" -ForegroundColor Gray
Write-Host "   3. Configure MCP: claude-code-setup mcp-list" -ForegroundColor Gray
Write-Host ""
Write-Host "📚 Docs: https://github.com/Ercaner1988/claude-code-setup-rustified" -ForegroundColor Cyan
