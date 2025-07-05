// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::{Builder, Target, TargetKind};
use big_data_rpa_v3_lib::packet_capture;
use big_data_rpa_v3_lib::auth;
use log::{error, info};

fn main() {
    let targets = [
        Target::new(TargetKind::Stdout),
        Target::new(TargetKind::Webview),
        Target::new(TargetKind::LogDir {
            file_name: Some("app".to_string()),
        }),
    ];

    tauri::Builder::default()
        .plugin(
            Builder::new()
                .targets(targets)
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            big_data_rpa_v3_lib::commands::greet,
            big_data_rpa_v3_lib::commands::get_capture_status,
            big_data_rpa_v3_lib::commands::set_status_channel,
            big_data_rpa_v3_lib::commands::set_http_channel,
            big_data_rpa_v3_lib::commands::init_packet_capture,
            big_data_rpa_v3_lib::commands::stop_packet_capture,
            big_data_rpa_v3_lib::commands::has_chmodbpf,
            big_data_rpa_v3_lib::commands::get_network_devices,
            big_data_rpa_v3_lib::commands::create_packet_window,
            big_data_rpa_v3_lib::commands::focus_packet_window,
            // Auth系统命令
            big_data_rpa_v3_lib::commands::get_all_token_status,
            big_data_rpa_v3_lib::commands::get_system_token,
            big_data_rpa_v3_lib::commands::clear_system_token,
            big_data_rpa_v3_lib::commands::clear_all_tokens,
            big_data_rpa_v3_lib::commands::set_token_event_channel,
            big_data_rpa_v3_lib::commands::get_token_event_history
        ])
        .setup(|app| {
            // 初始化 AppHandle
            if let Err(e) = packet_capture::init_app_handle(app.handle().clone()) {
                error!("初始化 AppHandle 失败: {}", e);
            }
            
            // 初始化认证系统
            if let Err(e) = auth::init_auth_system() {
                error!("初始化认证系统失败: {}", e);
            } else {
                info!("🔐 认证系统初始化成功");
            }
            
            // 在macOS上检查是否安装了ChmodBPF
            #[cfg(target_os = "macos")]
            {
                if big_data_rpa_v3_lib::admin_utils::has_chmodbpf() {
                    info!("检测到ChmodBPF已安装，可以直接使用抓包功能");
                } else {
                    info!("未检测到ChmodBPF，抓包功能可能受限");
                }
            }
            
            // 注意：不再自动启动数据包捕获，而是由用户点击按钮触发
            info!("应用已启动，等待用户请求开始捕获");
            
            Ok(())
        })
        .on_window_event(|_event_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                info!("窗口关闭，停止数据包捕获");
                // if let Err(e) = packet_capture::stop_packet_capture() {
                //     error!("停止数据包捕获失败: {}", e);
                // }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
