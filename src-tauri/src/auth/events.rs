use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use once_cell::sync::OnceCell;
use tauri::ipc::Channel;
use log::{info, error, warn};

// 全局事件系统实例
static EVENT_SYSTEM: OnceCell<Arc<Mutex<EventSystem>>> = OnceCell::new();

/// Token事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenEvent {
    /// Token获取成功
    TokenAcquired {
        system_id: String,
        system_name: String,
        token: String,
        acquired_at: u64,
        expires_at: u64,
        source_url: String,
    },
    /// Token过期
    TokenExpired {
        system_id: String,
        system_name: String,
        expired_at: u64,
    },
    /// Token获取失败
    TokenFailed {
        system_id: String,
        system_name: String,
        error: String,
        failed_at: u64,
    },
}

/// 事件监听器类型
pub type EventListener = Box<dyn Fn(&TokenEvent) + Send + Sync>;

/// 事件系统
pub struct EventSystem {
    /// 事件监听器
    listeners: HashMap<String, Vec<EventListener>>,
    /// 前端通道
    frontend_channel: Option<Channel<TokenEvent>>,
    /// 事件历史（最近100个）
    event_history: Vec<TokenEvent>,
}

impl EventSystem {
    fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            frontend_channel: None,
            event_history: Vec::new(),
        }
    }
    
    /// 设置前端通道
    pub fn set_frontend_channel(&mut self, channel: Channel<TokenEvent>) {
        info!("🔗 设置Token事件前端通道");
        self.frontend_channel = Some(channel);
    }
    
    /// 添加事件监听器
    pub fn add_listener<F>(&mut self, event_type: &str, listener: F)
    where
        F: Fn(&TokenEvent) + Send + Sync + 'static,
    {
        self.listeners
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
        info!("📡 添加事件监听器: {}", event_type);
    }
    
    /// 发送事件
    pub fn emit(&mut self, event: TokenEvent) {
        info!("🚀 发送Token事件: {:?}", event);
        
        // 添加到历史记录
        self.event_history.push(event.clone());
        if self.event_history.len() > 100 {
            self.event_history.remove(0);
        }
        
        // 通知监听器
        let event_type = match &event {
            TokenEvent::TokenAcquired { .. } => "token_acquired",
            TokenEvent::TokenExpired { .. } => "token_expired",
            TokenEvent::TokenFailed { .. } => "token_failed",
        };
        
        if let Some(listeners) = self.listeners.get(event_type) {
            for listener in listeners {
                listener(&event);
            }
        }
        
        // 发送到前端
        if let Some(channel) = &self.frontend_channel {
            if let Err(e) = channel.send(event.clone()) {
                error!("❌ 发送Token事件到前端失败: {}", e);
            }
        }
    }
    
    /// 获取事件历史
    pub fn get_event_history(&self) -> &[TokenEvent] {
        &self.event_history
    }
}

/// 初始化事件系统
pub fn init_event_system() -> Result<()> {
    let event_system = Arc::new(Mutex::new(EventSystem::new()));
    
    // 添加默认监听器
    {
        let mut system = event_system.lock().unwrap();
        
        // Token获取成功监听器
        system.add_listener("token_acquired", |event| {
            if let TokenEvent::TokenAcquired { system_name, system_id, .. } = event {
                info!("🎉 系统 [{}] ({}) Token获取成功！", system_name, system_id);
            }
        });
        
        // Token过期监听器
        system.add_listener("token_expired", |event| {
            if let TokenEvent::TokenExpired { system_name, system_id, .. } = event {
                warn!("⏰ 系统 [{}] ({}) Token已过期", system_name, system_id);
            }
        });
        
        // Token获取失败监听器
        system.add_listener("token_failed", |event| {
            if let TokenEvent::TokenFailed { system_name, system_id, error, .. } = event {
                error!("💥 系统 [{}] ({}) Token获取失败: {}", system_name, system_id, error);
            }
        });
    }
    
    EVENT_SYSTEM
        .set(event_system)
        .map_err(|_| anyhow!("事件系统已经初始化过了"))?;
        
    info!("📡 Token事件系统初始化完成");
    Ok(())
}

/// 获取事件系统实例
pub fn get_event_system() -> Option<Arc<Mutex<EventSystem>>> {
    EVENT_SYSTEM.get().cloned()
}

/// 发送Token获取成功事件
pub fn emit_token_acquired(
    system_id: String,
    system_name: String,
    token: String,
    acquired_at: u64,
    expires_at: u64,
    source_url: String,
) {
    if let Some(event_system) = get_event_system() {
        let mut system = event_system.lock().unwrap();
        system.emit(TokenEvent::TokenAcquired {
            system_id,
            system_name,
            token,
            acquired_at,
            expires_at,
            source_url,
        });
    }
}

/// 发送Token过期事件
pub fn emit_token_expired(system_id: String, system_name: String, expired_at: u64) {
    if let Some(event_system) = get_event_system() {
        let mut system = event_system.lock().unwrap();
        system.emit(TokenEvent::TokenExpired {
            system_id,
            system_name,
            expired_at,
        });
    }
}

/// 发送Token获取失败事件
pub fn emit_token_failed(system_id: String, system_name: String, error: String, failed_at: u64) {
    if let Some(event_system) = get_event_system() {
        let mut system = event_system.lock().unwrap();
        system.emit(TokenEvent::TokenFailed {
            system_id,
            system_name,
            error,
            failed_at,
        });
    }
}

/// 设置前端事件通道
pub fn set_token_event_channel(channel: Channel<TokenEvent>) -> Result<()> {
    if let Some(event_system) = get_event_system() {
        let mut system = event_system.lock().unwrap();
        system.set_frontend_channel(channel);
        Ok(())
    } else {
        Err(anyhow!("事件系统未初始化"))
    }
}

/// 获取事件历史
pub fn get_event_history() -> Vec<TokenEvent> {
    if let Some(event_system) = get_event_system() {
        let system = event_system.lock().unwrap();
        system.get_event_history().to_vec()
    } else {
        Vec::new()
    }
} 