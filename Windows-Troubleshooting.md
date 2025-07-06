# Windows 应用程序启动问题故障排除指南

## 问题描述
当在Windows系统上运行 Tauri 应用时，出现以下错误：
```
应用程序无法启动，因为应用程序的并行配置不正确
```

## 解决方案

### 1. 已实施的修复

我们已经对项目进行了以下修改来解决此问题：

#### 1.1 更新了 `tauri.conf.json`
- 添加了中文语言支持
- 配置了 NSIS 和 WiX 安装程序
- 使用 `offlineInstaller` 模式确保 WebView2 离线安装

#### 1.2 更新了 GitHub Actions 工作流
- 在构建过程中自动安装 Visual C++ Redistributable
- 设置了 FIPS 兼容性
- 指定了明确的构建目标 `x86_64-pc-windows-msvc`

#### 1.3 创建了 Windows Manifest 文件
- 明确指定了 Visual C++ 运行时库依赖
- 配置了系统兼容性设置
- 设置了适当的执行级别

#### 1.4 更新了 Cargo.toml
- 添加了 `windows7-compat` 特性支持

### 2. 本地构建测试

运行以下脚本进行本地构建测试：
```powershell
.\build-windows.ps1
```

### 3. 手动解决方案

如果问题仍然存在，请按以下步骤操作：

#### 3.1 检查 Visual C++ 运行时库
在目标机器上安装最新的 Visual C++ Redistributable：
1. 下载 [Microsoft Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
2. 以管理员身份运行安装程序
3. 重启计算机

#### 3.2 使用 SxsTrace 工具调试
1. 以管理员身份运行命令提示符
2. 启动跟踪：
   ```cmd
   sxstrace trace -logfile:sxstrace.etl
   ```
3. 运行出错的应用程序
4. 停止跟踪并生成报告：
   ```cmd
   sxstrace parse -logfile:sxstrace.etl -outfile:sxstrace.txt
   ```
5. 查看 `sxstrace.txt` 文件了解详细错误信息

#### 3.3 检查事件日志
1. 打开事件查看器 (eventvwr.exe)
2. 导航到 Windows 日志 > 应用程序
3. 查找与应用程序相关的错误事件

### 4. 常见问题

#### 4.1 缺少 Visual C++ 2015-2022 运行时库
**解决方案**：安装 Microsoft Visual C++ 2015-2022 Redistributable (x64)

#### 4.2 WebView2 运行时缺失
**解决方案**：我们的配置使用 `offlineInstaller` 模式，应该会自动安装 WebView2

#### 4.3 权限问题
**解决方案**：确保应用程序有足够的权限运行，可能需要以管理员身份运行安装程序

### 5. 验证修复

构建完成后，请在目标机器上：
1. 卸载之前安装的应用程序版本
2. 重启计算机
3. 安装新构建的版本
4. 测试应用程序启动

### 6. 技术支持

如果问题仍然存在，请提供以下信息：
- Windows 版本和架构
- 已安装的 Visual C++ Redistributable 版本
- SxsTrace 工具的输出
- 事件日志中的相关错误信息

## 参考资料
- [Tauri Windows 构建指南](https://tauri.app/v1/guides/building/windows/)
- [Microsoft Visual C++ Redistributable 下载](https://docs.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)
- [SxsTrace 工具使用说明](https://docs.microsoft.com/en-us/windows/win32/sbscs/using-sxstrace-exe) 