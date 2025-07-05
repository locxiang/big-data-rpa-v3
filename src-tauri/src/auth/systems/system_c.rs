use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 数据Token验证器
#[derive(Debug)]
struct DataTokenValidator;

impl TokenValidator for DataTokenValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 数据Token验证器开始验证token，长度: {}", token.len());
        
        if token.len() != 64 {
            let error_msg = format!("数据平台token长度必须为64位，当前: {}", token.len());
            warn!("❌ 数据Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        debug!("✅ 数据Token长度检查通过 (64位)");
        
        if !token.chars().all(|c| c.is_ascii_hexdigit()) {
            let error_msg = "数据平台token必须为十六进制格式";
            warn!("❌ 数据Token验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        debug!("✅ 数据Token格式检查通过 (十六进制)");
        
        info!("🔐 数据Token验证通过");
        Ok(())
    }
}

/// 创建系统C实例
pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "system_c".to_string(),
        system_name: "数据平台".to_string(),
        url_pattern: r"https?://[^/]*data[^/]*\..*?/api/.*".to_string(),
        header_name: "Access-Token".to_string(),
        token_pattern: r"([A-Fa-f0-9]{64})".to_string(),
        expires_duration: 1800, // 30分钟
        validator: Box::new(DataTokenValidator),
    };
    
    BaseSystem::new(config)
} 