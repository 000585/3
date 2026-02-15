# build_and_run.ps1
Write-Host "🔧 Building CHUDO Core v0.6.1..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Build successful!" -ForegroundColor Green
    Write-Host "`n🚀 Running CHUDO Core..." -ForegroundColor Cyan
    cargo run --release
} else {
    Write-Host "❌ Build failed!" -ForegroundColor Red
}
