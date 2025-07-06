# Windows 构建测试脚本
# 使用管理员权限运行此脚本

Write-Host "=== 数字重庆业务数据巡查自动化系统 - Windows 构建测试 ===" -ForegroundColor Green

# 检查是否以管理员身份运行
$currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Write-Host "错误: 请以管理员身份运行此脚本" -ForegroundColor Red
    Write-Host "右键点击 PowerShell 并选择 '以管理员身份运行'" -ForegroundColor Yellow
    pause
    exit 1
}

# 检查必要的工具
Write-Host "检查构建环境..." -ForegroundColor Yellow

# 检查 Node.js
try {
    $nodeVersion = node --version
    Write-Host "✓ Node.js: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Node.js 未安装" -ForegroundColor Red
    Write-Host "请安装 Node.js: https://nodejs.org/" -ForegroundColor Yellow
    pause
    exit 1
}

# 检查 pnpm
try {
    $pnpmVersion = pnpm --version
    Write-Host "✓ pnpm: v$pnpmVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ pnpm 未安装" -ForegroundColor Red
    Write-Host "运行: npm install -g pnpm" -ForegroundColor Yellow
    pause
    exit 1
}

# 检查 Rust
try {
    $rustVersion = rustc --version
    Write-Host "✓ Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Rust 未安装" -ForegroundColor Red
    Write-Host "请安装 Rust: https://rustup.rs/" -ForegroundColor Yellow
    pause
    exit 1
}

# 检查 Tauri CLI
try {
    $tauriVersion = cargo tauri --version
    Write-Host "✓ Tauri CLI: $tauriVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Tauri CLI 未安装" -ForegroundColor Red
    Write-Host "运行: cargo install tauri-cli" -ForegroundColor Yellow
    pause
    exit 1
}

# 检查 Visual Studio Build Tools
Write-Host "检查 Visual Studio Build Tools..." -ForegroundColor Yellow
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsWhere) {
    $vsInstallation = & $vsWhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if ($vsInstallation) {
        Write-Host "✓ Visual Studio Build Tools 已安装" -ForegroundColor Green
    } else {
        Write-Host "✗ Visual Studio Build Tools 未安装或配置不正确" -ForegroundColor Red
        Write-Host "请安装 Visual Studio Build Tools 或 Visual Studio Community" -ForegroundColor Yellow
    }
} else {
    Write-Host "✗ Visual Studio Installer 未找到" -ForegroundColor Red
}

# 设置环境变量
Write-Host "设置构建环境变量..." -ForegroundColor Yellow
$env:RUSTFLAGS = "-C target-feature=+crt-static"
Write-Host "✓ RUSTFLAGS 已设置" -ForegroundColor Green

# 安装依赖
Write-Host "安装前端依赖..." -ForegroundColor Yellow
try {
    pnpm install
    Write-Host "✓ 前端依赖安装完成" -ForegroundColor Green
} catch {
    Write-Host "✗ 前端依赖安装失败" -ForegroundColor Red
    Write-Host "错误: $($_.Exception.Message)" -ForegroundColor Red
    pause
    exit 1
}

# 构建前端
Write-Host "构建前端..." -ForegroundColor Yellow
try {
    pnpm build
    Write-Host "✓ 前端构建完成" -ForegroundColor Green
} catch {
    Write-Host "✗ 前端构建失败" -ForegroundColor Red
    Write-Host "错误: $($_.Exception.Message)" -ForegroundColor Red
    pause
    exit 1
}

# 构建 Tauri 应用
Write-Host "构建 Tauri 应用..." -ForegroundColor Yellow
Write-Host "这可能需要几分钟时间..." -ForegroundColor Blue

try {
    pnpm tauri build --target x86_64-pc-windows-msvc
    Write-Host "✓ Tauri 应用构建完成" -ForegroundColor Green
} catch {
    Write-Host "✗ Tauri 应用构建失败" -ForegroundColor Red
    Write-Host "错误: $($_.Exception.Message)" -ForegroundColor Red
    
    # 提供故障排除建议
    Write-Host "" -ForegroundColor Yellow
    Write-Host "故障排除建议:" -ForegroundColor Yellow
    Write-Host "1. 检查是否安装了 Visual C++ 2015-2022 Redistributable" -ForegroundColor Yellow
    Write-Host "2. 确保网络连接正常（需要下载依赖）" -ForegroundColor Yellow
    Write-Host "3. 检查防火墙设置" -ForegroundColor Yellow
    Write-Host "4. 查看详细错误日志" -ForegroundColor Yellow
    
    pause
    exit 1
}

# 检查构建产物
Write-Host "检查构建产物..." -ForegroundColor Yellow
$buildDir = "src-tauri\target\x86_64-pc-windows-msvc\release\bundle"

if (Test-Path $buildDir) {
    Write-Host "✓ 构建产物位置: $buildDir" -ForegroundColor Green
    
    # 列出构建产物
    $files = Get-ChildItem -Path $buildDir -Recurse -File
    Write-Host "构建产物列表:" -ForegroundColor Blue
    foreach ($file in $files) {
        $size = [math]::Round($file.Length / 1MB, 2)
        Write-Host "  - $($file.Name) (${size}MB)" -ForegroundColor Cyan
    }
} else {
    Write-Host "✗ 构建产物目录未找到" -ForegroundColor Red
}

# 运行基本测试
Write-Host "运行基本测试..." -ForegroundColor Yellow
$exeFile = "$buildDir\nsis\big-data-rpa-v3_0.5.0_x64-setup.exe"
if (Test-Path $exeFile) {
    Write-Host "✓ 找到安装程序: $exeFile" -ForegroundColor Green
    
    # 检查文件签名（如果有）
    try {
        $signature = Get-AuthenticodeSignature $exeFile
        if ($signature.Status -eq "Valid") {
            Write-Host "✓ 文件签名有效" -ForegroundColor Green
        } else {
            Write-Host "! 文件未签名或签名无效" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "! 无法检查文件签名" -ForegroundColor Yellow
    }
} else {
    Write-Host "✗ 安装程序未找到" -ForegroundColor Red
}

Write-Host "" -ForegroundColor White
Write-Host "=== 构建测试完成 ===" -ForegroundColor Green
Write-Host "如果所有检查都通过，您可以:" -ForegroundColor White
Write-Host "1. 在当前机器上测试安装程序" -ForegroundColor White
Write-Host "2. 将安装程序复制到目标机器进行测试" -ForegroundColor White
Write-Host "3. 查看 Windows-部署说明.md 获取部署指南" -ForegroundColor White

pause 