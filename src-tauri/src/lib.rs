pub mod packet_capture;
pub mod admin_utils;

pub mod commands {
    use crate::packet_capture;
    use crate::admin_utils;
    use tauri::ipc::Channel;

    #[tauri::command]
    pub fn greet(name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
    
    // 获取捕获状态
    #[tauri::command]
    pub fn get_capture_status() -> packet_capture::CaptureStatus {
        packet_capture::get_capture_status()
    }
    
    // 设置状态更新通道
    #[tauri::command]
    pub fn set_status_channel(channel: Channel<packet_capture::CaptureStatus>) -> Result<(), String> {
        packet_capture::set_status_channel(channel).map_err(|e| e.to_string())
    }
    
    // 设置 HTTP 请求通道
    #[tauri::command]
    pub fn set_http_channel(channel: Channel<packet_capture::HttpRequest>) -> Result<(), String> {
        packet_capture::set_http_channel(channel).map_err(|e| e.to_string())
    }
    
    // 初始化数据包捕获
    #[tauri::command]
    pub fn init_packet_capture() -> Result<(), String> {
        packet_capture::init_packet_capture().map_err(|e| e.to_string())
    }
    
    // 停止数据包捕获
    #[tauri::command]
    pub fn stop_packet_capture() -> Result<(), String> {
        packet_capture::stop_packet_capture().map_err(|e| e.to_string())
    }
    
    // 检查是否安装了ChmodBPF
    #[tauri::command]
    pub fn has_chmodbpf() -> bool {
        #[cfg(target_os = "macos")]
        {
            admin_utils::has_chmodbpf()
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            true // 在非macOS平台上，默认返回true，表示不需要特殊权限
        }
    }
}