use crate::packet_capture::HttpRequest;
use crate::auth::{
    config::TokenStatus,
    systems::{self, SystemAuth, TokenInfo},
    events,
};
use anyhow::{Result, anyhow};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use log::{info, warn, debug, error};
use std::time::{SystemTime, UNIX_EPOCH};

// 全局token管理器实例
static TOKEN_MANAGER: OnceCell<Arc<Mutex<TokenManager>>> = OnceCell::new();

/// Token管理器
pub struct TokenManager {
    /// 各系统实例
    systems: HashMap<String, Box<dyn SystemAuth + Send + Sync>>,
}

impl TokenManager {
    fn new() -> Self {
        let mut systems = HashMap::new();
        
        info!("🔧 开始加载所有认证系统...");
        
        // 创建所有系统实例
        for system in systems::create_all_systems() {
            let system_id = system.system_id().to_string();
            let system_name = system.system_name().to_string();
            
            info!("📦 加载系统: [{}] {}", system_id, system_name);
            systems.insert(system_id.clone(), system);
            debug!("✅ 系统 [{}] 加载完成", system_id);
        }
        
        info!("🎯 所有认证系统加载完成，共 {} 个系统", systems.len());
        
        Self {
            systems,
        }
    }
    
    /// 处理传入的HTTP请求
    pub fn process_request(&mut self, request: &HttpRequest) -> Result<()> {
        let url = format!("{}://{}{}", 
                         if request.dst_port == 443 { "https" } else { "http" },
                         request.host, request.path);
        
        debug!("🔄 开始处理HTTP请求: {} {}", request.method, url);
        debug!("📋 请求详情: Headers数量={}, 源地址={}:{}, 目标={}:{}",
               request.headers.len(), request.src_ip, request.src_port,
               request.dst_ip, request.dst_port);
        
        let mut processed_count = 0;
        let mut error_count = 0;
        
        // 让每个系统自己判断是否要处理这个请求
        for (system_id, system) in self.systems.iter_mut() {
            debug!("🔍 系统 [{}] 开始检查请求", system_id);
            
            match system.process_http_request(request) {
                Ok(_) => {
                    processed_count += 1;
                    debug!("✅ 系统 [{}] 处理完成", system_id);
                }
                Err(e) => {
                    error_count += 1;
                    debug!("⚠️ 系统 [{}] 处理失败: {}", system_id, e);
                }
            }
        }
        
        debug!("📊 请求处理结果: 成功={}, 失败={}, 总计={}", 
               processed_count, error_count, self.systems.len());
        
        Ok(())
    }
    
    /// 获取所有系统的token状态
    pub fn get_all_status(&self) -> Vec<TokenStatus> {
        debug!("📈 生成所有系统状态报告...");
        let mut statuses = Vec::new();
        
        for (system_id, system) in &self.systems {
            let info = system.get_token_info();
            let status = self.convert_token_info_to_status(&info, system_id, system.system_name());
            
            debug!("📝 系统 [{}] 状态: {:?}, 有token: {}", 
                   system_id, status.status, status.has_token);
            
            statuses.push(status);
        }
        
        info!("📋 状态报告生成完成，共 {} 个系统", statuses.len());
        statuses
    }
    
    /// 获取特定系统的token
    pub fn get_system_token(&self, system_id: &str) -> Option<String> {
        debug!("🔎 查找系统 [{}] 的token", system_id);
        
        let token = self.systems.get(system_id)
            .and_then(|system| system.get_current_token())
            .map(|token| token.to_string());
        
        match &token {
            Some(t) => {
                info!("✅ 系统 [{}] token可用，长度: {}", system_id, t.len());
                debug!("🔑 Token预览: {}...{}", 
                       &t[..t.len().min(8)], 
                       &t[t.len().saturating_sub(8)..]);
            }
            None => {
                debug!("❌ 系统 [{}] token不可用", system_id);
            }
        }
        
        token
    }
    
    /// 清除特定系统的token
    pub fn clear_system_token(&mut self, system_id: &str) -> Result<()> {
        info!("🗑️ 准备清除系统 [{}] 的token", system_id);
        
        let system = self.systems.get_mut(system_id)
            .ok_or_else(|| anyhow!("未找到系统: {}", system_id))?;
        
        let system_name = system.system_name().to_string();
        system.clear_token();
        
        info!("✅ 已清除系统 [{}] ({}) 的token", system_id, system_name);
        Ok(())
    }
    
    /// 清除所有系统的token
    pub fn clear_all_tokens(&mut self) {
        info!("🗑️ 开始清除所有系统的token...");
        
        let mut cleared_count = 0;
        for (system_id, system) in self.systems.iter_mut() {
            system.clear_token();
            cleared_count += 1;
            debug!("🗑️ 已清除系统 [{}] token", system_id);
        }
        
        info!("✅ 所有系统token清除完成，共清除 {} 个", cleared_count);
    }
    
    /// 检查过期的token
    pub fn check_expired_tokens(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        debug!("⏰ 开始检查过期token，当前时间戳: {}", now);
        
        let mut expired_count = 0;
        let mut valid_count = 0;
        
        for (system_id, system) in self.systems.iter_mut() {
            let info = system.get_token_info();
            
            if info.is_valid {
                if info.is_expired() {
                    expired_count += 1;
                    warn!("⏰ 系统 [{}] token已过期，过期时间: {:?}", 
                          system_id, info.expires_at);
                    
                    // 清除过期token
                    system.clear_token();
                    
                    // 发送过期事件
                    events::emit_token_expired(
                        system_id.to_string(),
                        system.system_name().to_string(),
                        now,
                    );
                } else {
                    valid_count += 1;
                    if let Some(remaining) = info.remaining_time() {
                        debug!("✅ 系统 [{}] token有效，剩余时间: {}秒", system_id, remaining);
                    }
                }
            }
        }
        
        if expired_count > 0 {
            info!("⚠️ 发现 {} 个过期token已清除，当前有效token: {}", expired_count, valid_count);
        } else {
            debug!("✅ 所有token状态正常，有效token: {}", valid_count);
        }
    }
    
    /// 将TokenInfo转换为TokenStatus
    fn convert_token_info_to_status(&self, info: &TokenInfo, system_id: &str, system_name: &str) -> TokenStatus {
        use crate::auth::config::TokenState;
        
        let status = if !info.is_valid {
            TokenState::Waiting
        } else if info.is_expired() {
            TokenState::Expired
        } else {
            TokenState::Active
        };
        
        TokenStatus {
            system_id: system_id.to_string(),
            system_name: system_name.to_string(),
            has_token: info.token.is_some(),
            token_acquired_at: info.acquired_at,
            token_expires_at: info.expires_at,
            last_seen_url: None, // 可以根据需要添加
            status,
        }
    }
}

/// 初始化token管理器
pub fn init_token_manager() -> Result<()> {
    info!("🎮 初始化Token管理器...");
    let manager = Arc::new(Mutex::new(TokenManager::new()));
    
    TOKEN_MANAGER
        .set(manager)
        .map_err(|_| anyhow!("Token管理器已经初始化过了"))?;
    
    info!("✅ Token管理器初始化完成");
    Ok(())
}

/// 获取token管理器实例
pub fn get_token_manager() -> Option<Arc<Mutex<TokenManager>>> {
    TOKEN_MANAGER.get().cloned()
}

/// 处理传入的HTTP请求
pub fn process_incoming_request(request: &HttpRequest) -> Result<()> {
    info!("🎮 manager开始处理HTTP请求: {} {}", request.method, request.path);
    
    if let Some(manager) = get_token_manager() {
        info!("✅ Token管理器已获取，准备加锁处理...");
        let mut mgr = manager.lock().unwrap();
        info!("🔒 Token管理器加锁成功，开始调用process_request...");
        let result = mgr.process_request(request);
        
        match &result {
            Ok(_) => info!("✅ manager处理HTTP请求完成"),
            Err(e) => error!("❌ manager处理HTTP请求失败: {}", e),
        }
        
        result
    } else {
        error!("❌ Token管理器未初始化，无法处理请求");
        Err(anyhow!("Token管理器未初始化"))
    }
}

/// 获取所有系统的token状态
pub fn get_all_token_status() -> Vec<TokenStatus> {
    if let Some(manager) = get_token_manager() {
        let mgr = manager.lock().unwrap();
        mgr.get_all_status()
    } else {
        error!("❌ Token管理器未初始化，返回空状态");
        Vec::new()
    }
}

/// 获取特定系统的token
pub fn get_system_token(system_id: &str) -> Option<String> {
    if let Some(manager) = get_token_manager() {
        let mgr = manager.lock().unwrap();
        mgr.get_system_token(system_id)
    } else {
        error!("❌ Token管理器未初始化，无法获取token");
        None
    }
}

/// 清除特定系统的token
pub fn clear_system_token(system_id: &str) -> Result<()> {
    if let Some(manager) = get_token_manager() {
        let mut mgr = manager.lock().unwrap();
        mgr.clear_system_token(system_id)
    } else {
        error!("❌ Token管理器未初始化，无法清除token");
        Err(anyhow!("Token管理器未初始化"))
    }
}

/// 清除所有系统的token
pub fn clear_all_tokens() -> Result<()> {
    if let Some(manager) = get_token_manager() {
        let mut mgr = manager.lock().unwrap();
        mgr.clear_all_tokens();
        Ok(())
    } else {
        error!("❌ Token管理器未初始化，无法清除所有token");
        Err(anyhow!("Token管理器未初始化"))
    }
}

/// 启动后台任务检查过期token
pub fn start_token_expiry_checker() {
    use std::thread;
    use std::time::Duration;
    
    info!("⏰ 启动Token过期检查器后台任务...");
    
    thread::spawn(|| {
        info!("🔄 Token过期检查器已启动，每60秒检查一次");
        
        loop {
            thread::sleep(Duration::from_secs(60)); // 每分钟检查一次
            
            debug!("⏰ 执行定期token过期检查...");
            
            if let Some(manager) = get_token_manager() {
                let mut mgr = manager.lock().unwrap();
                mgr.check_expired_tokens();
            } else {
                error!("❌ Token管理器未初始化，跳过过期检查");
            }
        }
    });
    
    info!("✅ Token过期检查器启动完成");
} 