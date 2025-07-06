# Windows 部署说明

## 系统要求

### 最低系统要求
- **操作系统**: Windows 10 (版本 1903 或更高) 或 Windows 11
- **架构**: x86-64 (64位)
- **内存**: 至少 4GB RAM
- **磁盘空间**: 至少 500MB 可用空间
- **网络**: 需要网络连接以进行包捕获功能

### 权限要求
- **管理员权限**: 必须以管理员身份运行（用于网络包捕获功能）
- **网络权限**: 应用需要访问网络适配器

## 安装步骤

### 1. 下载应用程序
从 GitHub Release 页面下载最新版本的 Windows 安装包：
- `big-data-rpa-v3_x.x.x_x64-setup.exe` (推荐，包含安装程序)
- `big-data-rpa-v3_x.x.x_x64.msi` (MSI 安装包)

### 2. 运行安装程序
1. **右键点击** 下载的安装包
2. 选择 **"以管理员身份运行"**
3. 如果出现 Windows 安全警告，点击 **"更多信息"** → **"仍要运行"**
4. 按照安装向导完成安装

### 3. 首次运行
1. 在开始菜单中找到 **"数字重庆业务数据巡查自动化系统"**
2. **右键点击** → **"以管理员身份运行"**
3. 如果提示安装额外组件，请点击 **"是"**

## 依赖组件

### 自动安装的组件
安装程序会自动检查并安装以下组件：
- **Visual C++ 2015-2022 Redistributable (x64)**
- **Microsoft Edge WebView2 Runtime**

### 手动安装的组件（如果需要）
如果自动安装失败，请手动下载并安装：

#### Visual C++ Redistributable
- 下载地址: https://aka.ms/vs/17/release/vc_redist.x64.exe
- 运行并完成安装

#### WebView2 Runtime
- 下载地址: https://go.microsoft.com/fwlink/p/?LinkId=2124703
- 运行并完成安装

#### Npcap（网络包捕获）
- 下载地址: https://npcap.com/dist/npcap-1.75.exe
- **重要**: 安装时请勾选 **"Install Npcap in WinPcap API-compatible Mode"**
- 需要重启计算机

## 故障排除

### 常见问题及解决方案

#### 1. "应用程序无法启动，因为应用程序的并行配置不正确"
**原因**: 缺少 Visual C++ 运行时库或版本不匹配

**解决方案**:
1. 卸载所有现有的 Visual C++ Redistributable
2. 从官网下载最新的 Visual C++ 2015-2022 Redistributable (x64)
3. 以管理员身份安装
4. 重启计算机

#### 2. "找不到 wpcap.dll" 或网络捕获功能不工作
**原因**: 未安装 Npcap 或安装配置不正确

**解决方案**:
1. 下载并安装 Npcap
2. 安装时确保勾选 **"WinPcap API-compatible Mode"**
3. 重启计算机
4. 确保以管理员身份运行应用程序

#### 3. "此应用无法在你的电脑上运行"
**原因**: 系统版本过旧或架构不匹配

**解决方案**:
1. 确认系统是 64位 Windows 10 (1903+) 或 Windows 11
2. 更新 Windows 到最新版本
3. 检查是否有 Windows 更新待安装

#### 4. WebView2 相关错误
**原因**: WebView2 运行时未正确安装

**解决方案**:
1. 手动下载并安装 WebView2 Runtime
2. 确保网络连接正常
3. 重启应用程序

#### 5. 防火墙/杀毒软件阻止
**原因**: 安全软件误报

**解决方案**:
1. 将应用程序添加到防火墙白名单
2. 在杀毒软件中添加信任
3. 临时关闭实时保护进行安装

### 高级故障排除

#### 检查系统兼容性
```cmd
# 在命令提示符中运行
systeminfo | findstr /B /C:"OS Name" /C:"System Type"
```

#### 检查 .NET Framework
```cmd
# 检查 .NET Framework 版本
reg query "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\NET Framework Setup\NDP\v4\Full\" /v Release
```

#### 清理临时文件
```cmd
# 清理系统临时文件
%windir%\system32\cleanmgr.exe /sagerun:1
```

## 卸载

### 标准卸载
1. 打开 **"设置"** → **"应用"**
2. 找到 **"数字重庆业务数据巡查自动化系统"**
3. 点击 **"卸载"**

### 完全清理
如果需要完全清理所有相关文件：
1. 运行标准卸载
2. 删除用户数据文件夹: `%APPDATA%\com.big-data-rpa-v3`
3. 删除应用程序数据: `%LOCALAPPDATA%\com.big-data-rpa-v3`

## 技术支持

如果遇到其他问题，请提供以下信息：
- Windows 版本和构建号
- 错误消息的完整截图
- 事件查看器中的相关错误日志
- 是否以管理员身份运行

### 日志文件位置
- 应用日志: `%APPDATA%\com.big-data-rpa-v3\logs\`
- 系统日志: 事件查看器 → Windows 日志 → 应用程序

### 联系方式
- GitHub Issues: [项目地址]/issues
- 邮箱: [技术支持邮箱]

---

**最后更新**: 2024年12月
**版本**: 适用于 v0.5.0 及以上版本 