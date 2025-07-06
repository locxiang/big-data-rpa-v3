# 目标机器依赖检查脚本 - Tauri v2 应用
# 在出现问题的 Windows 机器上运行此脚本

Write-Host "======================================" -ForegroundColor Cyan
Write-Host "Tauri v2 应用依赖检查脚本" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan

# 检查系统信息
Write-Host "`n1. 系统信息检查..." -ForegroundColor Yellow
$winVersion = Get-WmiObject -Class Win32_OperatingSystem
Write-Host "   Windows 版本: $($winVersion.Caption)" -ForegroundColor White
Write-Host "   构建版本: $($winVersion.BuildNumber)" -ForegroundColor White
Write-Host "   架构: $env:PROCESSOR_ARCHITECTURE" -ForegroundColor White
Write-Host "   Service Pack: $($winVersion.CSDVersion)" -ForegroundColor White

# 检查 Visual C++ Redistributable
Write-Host "`n2. Visual C++ Redistributable 检查..." -ForegroundColor Yellow
$vcRedist = Get-WmiObject -Class Win32_Product | Where-Object { 
    $_.Name -match "Microsoft Visual C\+\+ 2015-2022" -or 
    $_.Name -match "Microsoft Visual C\+\+ 2019" -or 
    $_.Name -match "Microsoft Visual C\+\+ 2017" -or 
    $_.Name -match "Microsoft Visual C\+\+ 2015"
}

if ($vcRedist) {
    Write-Host "   ✓ Visual C++ Redistributable 已安装:" -ForegroundColor Green
    $vcRedist | ForEach-Object { 
        Write-Host "     - $($_.Name) - 版本: $($_.Version)" -ForegroundColor White
        Write-Host "     - 安装日期: $($_.InstallDate)" -ForegroundColor Gray
    }
} else {
    Write-Host "   ✗ Visual C++ Redistributable 未安装" -ForegroundColor Red
    Write-Host "   请下载并安装: https://aka.ms/vs/17/release/vc_redist.x64.exe" -ForegroundColor Yellow
}

# 检查 WebView2
Write-Host "`n3. WebView2 Runtime 检查..." -ForegroundColor Yellow
$webview2Registry = @(
    "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
    "HKLM:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
)

$webview2Found = $false
foreach ($regPath in $webview2Registry) {
    $webview2 = Get-ItemProperty -Path $regPath -ErrorAction SilentlyContinue
    if ($webview2) {
        Write-Host "   ✓ WebView2 Runtime 已安装:" -ForegroundColor Green
        Write-Host "     - 版本: $($webview2.pv)" -ForegroundColor White
        Write-Host "     - 路径: $($webview2.name)" -ForegroundColor Gray
        $webview2Found = $true
        break
    }
}

if (-not $webview2Found) {
    Write-Host "   ✗ WebView2 Runtime 未安装" -ForegroundColor Red
    Write-Host "   请下载并安装: https://go.microsoft.com/fwlink/p/?LinkId=2124703" -ForegroundColor Yellow
}

# 检查 Windows 更新状态
Write-Host "`n4. Windows 更新检查..." -ForegroundColor Yellow
try {
    $lastUpdate = Get-HotFix | Sort-Object InstalledOn -Descending | Select-Object -First 1
    Write-Host "   最后更新: $($lastUpdate.InstalledOn)" -ForegroundColor White
    Write-Host "   更新编号: $($lastUpdate.HotFixID)" -ForegroundColor Gray
} catch {
    Write-Host "   无法获取更新信息" -ForegroundColor Yellow
}

# 检查 .NET Framework
Write-Host "`n5. .NET Framework 检查..." -ForegroundColor Yellow
$dotnetVersions = Get-ChildItem "HKLM:\SOFTWARE\Microsoft\NET Framework Setup\NDP" -Recurse | Get-ItemProperty -Name version -ErrorAction SilentlyContinue | Where-Object { $_.PSChildName -match "^v\d" }
if ($dotnetVersions) {
    Write-Host "   ✓ .NET Framework 已安装:" -ForegroundColor Green
    $dotnetVersions | ForEach-Object { 
        Write-Host "     - $($_.PSChildName): $($_.Version)" -ForegroundColor White
    }
} else {
    Write-Host "   ✗ .NET Framework 信息不可用" -ForegroundColor Yellow
}

# 检查 Windows Defender 状态
Write-Host "`n6. Windows Defender 检查..." -ForegroundColor Yellow
try {
    $defenderStatus = Get-MpComputerStatus -ErrorAction SilentlyContinue
    if ($defenderStatus) {
        Write-Host "   实时保护: $($defenderStatus.RealTimeProtectionEnabled)" -ForegroundColor White
        Write-Host "   反恶意软件: $($defenderStatus.AntimalwareEnabled)" -ForegroundColor White
    } else {
        Write-Host "   无法获取 Windows Defender 状态" -ForegroundColor Yellow
    }
} catch {
    Write-Host "   Windows Defender 服务不可用" -ForegroundColor Yellow
}

# 检查事件日志中的相关错误
Write-Host "`n7. 事件日志错误检查..." -ForegroundColor Yellow
try {
    $sxsErrors = Get-WinEvent -FilterHashtable @{LogName="System"; ID=33,59,32,35} -MaxEvents 10 -ErrorAction SilentlyContinue
    if ($sxsErrors) {
        Write-Host "   ✗ 发现 SxS (并行配置) 相关错误:" -ForegroundColor Red
        $sxsErrors | ForEach-Object {
            Write-Host "     - $($_.TimeCreated): $($_.Id) - $($_.LevelDisplayName)" -ForegroundColor White
        }
    } else {
        Write-Host "   ✓ 未发现 SxS 相关错误" -ForegroundColor Green
    }
} catch {
    Write-Host "   无法检查事件日志" -ForegroundColor Yellow
}

# 检查磁盘空间
Write-Host "`n8. 磁盘空间检查..." -ForegroundColor Yellow
$systemDrive = Get-WmiObject -Class Win32_LogicalDisk | Where-Object { $_.DriveType -eq 3 -and $_.DeviceID -eq $env:SystemDrive }
if ($systemDrive) {
    $freeSpaceGB = [Math]::Round($systemDrive.FreeSpace / 1GB, 2)
    $totalSpaceGB = [Math]::Round($systemDrive.Size / 1GB, 2)
    Write-Host "   系统盘 ($($systemDrive.DeviceID)) 可用空间: $freeSpaceGB GB / $totalSpaceGB GB" -ForegroundColor White
    if ($freeSpaceGB -lt 5) {
        Write-Host "   ⚠️ 系统盘空间不足" -ForegroundColor Yellow
    }
}

# 生成报告
Write-Host "`n======================================" -ForegroundColor Cyan
Write-Host "检查完成！" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan

Write-Host "`n建议的修复步骤:" -ForegroundColor Yellow
Write-Host "1. 如果 Visual C++ Redistributable 未安装，请安装最新版本" -ForegroundColor White
Write-Host "2. 如果 WebView2 Runtime 未安装，请安装最新版本" -ForegroundColor White
Write-Host "3. 如果发现 SxS 错误，请检查应用程序的 manifest 文件" -ForegroundColor White
Write-Host "4. 尝试以管理员身份运行应用程序安装程序" -ForegroundColor White
Write-Host "5. 临时禁用杀毒软件重新测试" -ForegroundColor White

Write-Host "`n如果问题仍然存在，请将此检查结果发送给开发者。" -ForegroundColor Yellow

# 询问是否要生成详细报告
Write-Host "`n是否要生成详细报告文件？(Y/N): " -ForegroundColor Cyan -NoNewline
$response = Read-Host
if ($response -eq "Y" -or $response -eq "y") {
    $reportFile = "system-report-$(Get-Date -Format 'yyyyMMdd-HHmmss').txt"
    $reportContent = @"
=== Tauri v2 应用系统检查报告 ===
生成时间: $(Get-Date)

系统信息:
- Windows 版本: $($winVersion.Caption)
- 构建版本: $($winVersion.BuildNumber)
- 架构: $env:PROCESSOR_ARCHITECTURE
- Service Pack: $($winVersion.CSDVersion)

Visual C++ Redistributable:
$(if ($vcRedist) { $vcRedist | ForEach-Object { "- $($_.Name) - 版本: $($_.Version)" } | Out-String } else { "- 未安装" })

WebView2 Runtime:
$(if ($webview2Found) { "- 已安装" } else { "- 未安装" })

磁盘空间:
- 系统盘可用空间: $freeSpaceGB GB / $totalSpaceGB GB

事件日志 SxS 错误:
$(if ($sxsErrors) { $sxsErrors | ForEach-Object { "- $($_.TimeCreated): $($_.Id) - $($_.LevelDisplayName)" } | Out-String } else { "- 未发现相关错误" })

"@
    Set-Content -Path $reportFile -Value $reportContent -Encoding UTF8
    Write-Host "报告已保存到: $reportFile" -ForegroundColor Green
} 