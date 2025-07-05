pub mod config;
pub mod systems;
pub mod events;
pub mod manager;

use anyhow::Result;
use crate::packet_capture::HttpRequest;
use log::{info, debug, error};

// 重新导出主要类型
pub use config::*;
pub use events::*;
pub use manager::*;
pub use systems::{SystemAuth, TokenInfo, TokenValidator, SystemConfig, BaseSystem, SystemRegistry};

// 主要的初始化函数
pub fn init_auth_system() -> Result<()> {
    info!("🚀 开始初始化Token认证系统...");
    
    // 初始化事件系统
    debug!("📡 初始化事件系统...");
    events::init_event_system()?;
    debug!("✅ 事件系统初始化完成");
    
    // 初始化token管理器
    debug!("🎮 初始化token管理器...");
    manager::init_token_manager()?;
    debug!("✅ token管理器初始化完成");
    
    // 启动后台任务
    debug!("⏰ 启动token过期检查器...");
    manager::start_token_expiry_checker();
    debug!("✅ token过期检查器启动完成");
    
    info!("🔐 Token认证系统初始化完成！已加载 {} 个系统", 
          get_all_token_status().len());
    Ok(())
}

// 处理来自抓包模块的HTTP请求
pub fn process_http_request(request: &HttpRequest) -> Result<()> {
    info!("🎯 auth模块收到HTTP请求: {} {} (来源: {}:{})", 
           request.method, request.path, request.src_ip, request.src_port);
    
    info!("🔍 开始调用manager::process_incoming_request...");
    let result = manager::process_incoming_request(request);
    
    match &result {
        Ok(_) => {
            info!("✅ auth模块处理HTTP请求成功");
        }
        Err(e) => {
            error!("❌ auth模块处理HTTP请求失败: {}", e);
        }
    }
    
    result
}

// 获取所有系统的token状态
pub fn get_all_token_status() -> Vec<TokenStatus> {
    debug!("📊 获取所有系统token状态");
    let statuses = manager::get_all_token_status();
    
    debug!("📈 系统状态统计: 总数={}, 有效={}, 过期={}, 等待={}",
           statuses.len(),
           statuses.iter().filter(|s| matches!(s.status, TokenState::Active)).count(),
           statuses.iter().filter(|s| matches!(s.status, TokenState::Expired)).count(),
           statuses.iter().filter(|s| matches!(s.status, TokenState::Waiting)).count());
    
    statuses
}

// 获取特定系统的token
pub fn get_system_token(system_id: &str) -> Option<String> {
    debug!("🔍 查询系统 [{}] 的token", system_id);
    let token = manager::get_system_token(system_id);
    
    if token.is_some() {
        debug!("✅ 系统 [{}] token可用", system_id);
    } else {
        debug!("❌ 系统 [{}] token不可用", system_id);
    }
    
    token
} 