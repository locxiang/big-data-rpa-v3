# Tauri v2 Windows 本地构建脚本
Write-Host "开始构建 Tauri v2 Windows 版本..." -ForegroundColor Green

# 检查系统信息
Write-Host "检查系统信息..." -ForegroundColor Yellow
Write-Host "Windows 版本: $(Get-WmiObject -Class Win32_OperatingSystem | Select-Object -ExpandProperty Caption)" -ForegroundColor White
Write-Host "架构: $env:PROCESSOR_ARCHITECTURE" -ForegroundColor White
Write-Host "PowerShell 版本: $($PSVersionTable.PSVersion)" -ForegroundColor White

# 检查必需的工具
Write-Host "检查必需工具..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version
    Write-Host "Rust: $rustVersion" -ForegroundColor White
} catch {
    Write-Host "错误: 未找到 Rust 编译器" -ForegroundColor Red
    exit 1
}

try {
    $cargoVersion = cargo --version
    Write-Host "Cargo: $cargoVersion" -ForegroundColor White
} catch {
    Write-Host "错误: 未找到 Cargo" -ForegroundColor Red
    exit 1
}

try {
    $pnpmVersion = pnpm --version
    Write-Host "pnpm: $pnpmVersion" -ForegroundColor White
} catch {
    Write-Host "错误: 未找到 pnpm" -ForegroundColor Red
    exit 1
}

# 检查 Visual C++ 工具
Write-Host "检查 Visual C++ 工具..." -ForegroundColor Yellow
$vcInstalled = Get-Command "cl.exe" -ErrorAction SilentlyContinue
if ($vcInstalled) {
    Write-Host "Visual C++ 编译器: 已安装" -ForegroundColor White
} else {
    Write-Host "警告: 未找到 Visual C++ 编译器" -ForegroundColor Yellow
}

# 添加 Windows 构建目标
Write-Host "添加 Windows 构建目标..." -ForegroundColor Yellow
rustup target add x86_64-pc-windows-msvc

# 清理之前的构建
Write-Host "清理之前的构建..." -ForegroundColor Yellow
if (Test-Path "src-tauri/target") {
    Remove-Item "src-tauri/target" -Recurse -Force
}
if (Test-Path "dist") {
    Remove-Item "dist" -Recurse -Force
}

# 安装前端依赖
Write-Host "安装前端依赖..." -ForegroundColor Yellow
pnpm install
if ($LASTEXITCODE -ne 0) {
    Write-Host "错误: 前端依赖安装失败" -ForegroundColor Red
    exit 1
}

# 构建前端
Write-Host "构建前端..." -ForegroundColor Yellow
pnpm build
if ($LASTEXITCODE -ne 0) {
    Write-Host "错误: 前端构建失败" -ForegroundColor Red
    exit 1
}

# 构建 Tauri 应用
Write-Host "构建 Tauri 应用..." -ForegroundColor Yellow
pnpm tauri build --target x86_64-pc-windows-msvc --bundles msi,nsis --verbose
if ($LASTEXITCODE -ne 0) {
    Write-Host "错误: Tauri 应用构建失败" -ForegroundColor Red
    exit 1
}

Write-Host "构建完成！" -ForegroundColor Green
Write-Host "输出文件位置：" -ForegroundColor Cyan

# 查找生成的文件
$targetPath = "src-tauri/target/x86_64-pc-windows-msvc/release/bundle"
if (Test-Path $targetPath) {
    Get-ChildItem $targetPath -Recurse -Include "*.exe", "*.msi" | ForEach-Object {
        Write-Host "  $($_.FullName)" -ForegroundColor White
        # 检查文件大小
        $size = [Math]::Round($_.Length / 1MB, 2)
        Write-Host "    文件大小: ${size} MB" -ForegroundColor Gray
    }
} else {
    Write-Host "未找到构建输出目录" -ForegroundColor Red
}

# 生成依赖信息
Write-Host "`n生成依赖检查脚本..." -ForegroundColor Yellow
$checkScript = @"
# 依赖检查脚本
Write-Host "检查目标机器依赖..." -ForegroundColor Yellow

# 检查 Visual C++ Redistributable
`$vcRedist = Get-WmiObject -Class Win32_Product | Where-Object { `$_.Name -match "Microsoft Visual C\+\+ 2015-2022" }
if (`$vcRedist) {
    Write-Host "Visual C++ 2015-2022 Redistributable: 已安装" -ForegroundColor Green
    `$vcRedist | ForEach-Object { Write-Host "  版本: `$(`$_.Version)" -ForegroundColor White }
} else {
    Write-Host "Visual C++ 2015-2022 Redistributable: 未安装" -ForegroundColor Red
    Write-Host "请下载并安装: https://aka.ms/vs/17/release/vc_redist.x64.exe" -ForegroundColor Yellow
}

# 检查 WebView2
`$webview2 = Get-ItemProperty -Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -ErrorAction SilentlyContinue
if (`$webview2) {
    Write-Host "WebView2 Runtime: 已安装" -ForegroundColor Green
    Write-Host "  版本: `$(`$webview2.pv)" -ForegroundColor White
} else {
    Write-Host "WebView2 Runtime: 未安装" -ForegroundColor Red
    Write-Host "请下载并安装: https://go.microsoft.com/fwlink/p/?LinkId=2124703" -ForegroundColor Yellow
}

# 检查 Windows 版本
`$winVersion = Get-WmiObject -Class Win32_OperatingSystem
Write-Host "Windows 版本: `$(`$winVersion.Caption)" -ForegroundColor White
Write-Host "构建版本: `$(`$winVersion.BuildNumber)" -ForegroundColor White
"@

Set-Content -Path "check-dependencies.ps1" -Value $checkScript -Encoding UTF8

Write-Host "`n构建完成！请按以下步骤测试：" -ForegroundColor Yellow
Write-Host "1. 在目标机器上运行: .\check-dependencies.ps1" -ForegroundColor White
Write-Host "2. 安装缺失的依赖" -ForegroundColor White
Write-Host "3. 安装并测试应用程序" -ForegroundColor White 