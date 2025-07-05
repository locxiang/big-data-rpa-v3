pub mod packet_capture;
pub mod admin_utils;
pub mod auth;

pub mod commands {
    use crate::packet_capture;
    use crate::admin_utils;
    use crate::auth;
    use tauri::ipc::Channel;

    #[tauri::command]
    pub fn greet(name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
    
    // 获取捕获状态
    #[tauri::command]
    pub fn get_capture_status() -> packet_capture::CaptureStatus {
        // 打印日志显示当前捕获状态
        println!("获取捕获状态: 正在返回当前状态");
        let status = packet_capture::get_capture_status();
        println!("捕获状态: {:?}", status);
        status
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
        admin_utils::has_chmodbpf()
    }
    
    // 获取网络设备列表
    #[tauri::command]
    pub fn get_network_devices() -> Result<Vec<packet_capture::NetworkDevice>, String> {
        packet_capture::get_network_devices().map_err(|e| e.to_string())
    }
    
    // 创建数据包详情窗口
    #[tauri::command]
    pub async fn create_packet_window(
        app: tauri::AppHandle,
        title: String,
        url: String,
        width: f64,
        height: f64,
    ) -> Result<(), String> {
        println!("创建数据包详情窗口: {}", url);
        // 创建新窗口
        tauri::WebviewWindowBuilder::new(
            &app,
            &url,
            tauri::WebviewUrl::App(url.clone().into())
        )
        .title(&title)
        .inner_size(width, height)
        .center()
        .resizable(false)
        .minimizable(true)
        .maximizable(false)
        // .always_on_top(true) // 设置窗口始终在最前面
        .build()
        .map_err(|e| e.to_string())?;
            
        Ok(())
    }
    
    // 聚焦数据包详情窗口
    #[tauri::command]
    pub async fn focus_packet_window(
        _app: tauri::AppHandle,
        packet_id: String,
    ) -> Result<(), String> {
        let window_label = format!("packet-detail-{}", packet_id);
        println!("聚焦数据包详情窗口: {}", window_label);
        // 简化版本：总是返回成功
        Ok(())
    }

    // === Auth系统相关命令 ===
    
    // 获取所有系统的token状态
    #[tauri::command]
    pub fn get_all_token_status() -> Vec<auth::TokenStatus> {
        auth::get_all_token_status()
    }
    
    // 获取特定系统的token
    #[tauri::command]
    pub fn get_system_token(system_id: String) -> Option<String> {
        auth::get_system_token(&system_id)
    }
    
    // 清除特定系统的token
    #[tauri::command]
    pub fn clear_system_token(system_id: String) -> Result<(), String> {
        auth::manager::clear_system_token(&system_id).map_err(|e| e.to_string())
    }
    
    // 清除所有系统的token
    #[tauri::command]
    pub fn clear_all_tokens() -> Result<(), String> {
        auth::manager::clear_all_tokens().map_err(|e| e.to_string())
    }
    
    // 设置Token事件通道
    #[tauri::command]
    pub fn set_token_event_channel(channel: Channel<auth::TokenEvent>) -> Result<(), String> {
        auth::events::set_token_event_channel(channel).map_err(|e| e.to_string())
    }
    
    // 获取Token事件历史
    #[tauri::command]
    pub fn get_token_event_history() -> Vec<auth::TokenEvent> {
        auth::events::get_event_history()
    }
}