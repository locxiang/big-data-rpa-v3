#[cfg(target_os = "macos")]
use log::{info};
use std::path::Path;

/// 检查是否安装了ChmodBPF（仅macOS）
#[cfg(target_os = "macos")]
pub fn has_chmodbpf() -> bool {
    // 检查ChmodBPF服务是否存在
    info!("检查ChmodBPF服务是否已安装...");
    let chmodbpf_path = Path::new("/Library/LaunchDaemons/org.wireshark.ChmodBPF.plist");
    if !chmodbpf_path.exists() {
        return false;
    }

    // 打印ChmodBPF服务状态日志
    info!("ChmodBPF服务文件存在，检查服务状态...");
    
    
    return true;
        
}

/// 在Windows上，总是返回true，因为我们不需要特殊权限
#[cfg(target_os = "windows")]
pub fn has_chmodbpf() -> bool {
    true
}

/// 在其他平台上，默认返回false
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn has_chmodbpf() -> bool {
    false
}

 