use serde::{Deserialize, Serialize};

/// Token状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatus {
    pub system_id: String,
    pub system_name: String,
    pub has_token: bool,
    pub token_acquired_at: Option<u64>,
    pub token_expires_at: Option<u64>,
    pub last_seen_url: Option<String>,
    pub status: TokenState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenState {
    /// 等待获取
    Waiting,
    /// 已获取，有效
    Active,
    /// 已过期
    Expired,
    /// 获取失败
    Failed(String),
} 