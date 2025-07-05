use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 业务Token验证器
#[derive(Debug)]
struct BusinessTokenValidator;

impl TokenValidator for BusinessTokenValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 业务Token验证器开始验证token，长度: {}", token.len());
        
        if !token.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=') {
            let error_msg = "业务系统token必须为Base64格式";
            warn!("❌ 业务Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        debug!("✅ 业务Token格式检查通过 (Base64)");
        
        if token.len() < 40 {
            let error_msg = format!("业务系统token长度必须至少40位，当前: {}", token.len());
            warn!("❌ 业务Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        debug!("✅ 业务Token长度检查通过 (>= 40位)");
        
        info!("🔐 业务Token验证通过");
        Ok(())
    }
}

/// 创建系统D实例
pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "system_d".to_string(),
        system_name: "业务系统".to_string(),
        url_pattern: r"https?://[^/]*business[^/]*\..*?/api/.*".to_string(),
        header_name: "Authentication".to_string(),
        token_pattern: r"Token\s+([A-Za-z0-9\+/=]{40,})".to_string(),
        expires_duration: 1200, // 20分钟
        validator: Box::new(BusinessTokenValidator),
    };
    
    BaseSystem::new(config)
} 