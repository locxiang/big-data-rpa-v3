use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 长度Token验证器
#[derive(Debug)]
struct LengthValidator;

impl TokenValidator for LengthValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 长度验证器开始验证token，长度: {}", token.len());
        
        // 检查token长度是否大于10
        if token.len() <= 10 {
            let error_msg = format!("Token长度不足，要求大于10个字符，实际: {}", token.len());
            warn!("❌ Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        debug!("✅ Token长度检查通过: {}", token.len());
        
        info!("🔐 Token验证通过");
        Ok(())
    }
}

/// 创建GitLab系统实例
pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "gitlab".to_string(),
        system_name: "GitLab".to_string(),
        url_pattern: r"192\.168\.91\.1.*".to_string(),
        header_name: "x-csrf-token".to_string(),
        token_pattern: r"(.+)".to_string(), // 直接匹配任意字符，不需要Bearer前缀
        expires_duration: 3600, // 1小时
        validator: Box::new(LengthValidator),
    };
    
    BaseSystem::new(config)
} 