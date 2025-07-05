use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 用户Token验证器
#[derive(Debug)]
struct UserTokenValidator;

impl TokenValidator for UserTokenValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 用户Token验证器开始验证token，长度: {}", token.len());
        
        // 检查最小长度
        if token.len() < 32 {
            let error_msg = format!("用户中心token长度必须至少32位，当前: {}", token.len());
            warn!("❌ 用户Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        debug!("✅ 用户Token长度检查通过 (>= 32位)");
        
        // 检查字符组成
        if !token.chars().all(|c| c.is_alphanumeric()) {
            let error_msg = "用户中心token必须只包含字母和数字";
            warn!("❌ 用户Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        debug!("✅ 用户Token字符组成检查通过 (只包含字母数字)");
        
        info!("🔐 用户Token验证通过");
        Ok(())
    }
}

/// 创建系统B实例
pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "system_b".to_string(),
        system_name: "用户中心".to_string(),
        url_pattern: r"https?://[^/]*user[^/]*\..*?/api/.*".to_string(),
        header_name: "X-Auth-Token".to_string(),
        token_pattern: r"([A-Za-z0-9]{32,})".to_string(),
        expires_duration: 7200, // 2小时
        validator: Box::new(UserTokenValidator),
    };
    
    BaseSystem::new(config)
} 