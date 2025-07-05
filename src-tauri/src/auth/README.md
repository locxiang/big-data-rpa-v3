# 🔐 Token认证系统配置指南

## 概述

本系统采用**配置驱动架构**，通过配置文件和可插拔验证器来实现不同系统的token认证。相比传统的继承实现方式，新架构具有以下优势：

- ✨ **零重复代码**：所有通用逻辑集中在`BaseSystem`中
- 🔧 **配置驱动**：只需定义配置，无需编写重复的处理逻辑
- 🔌 **可插拔验证器**：验证逻辑独立，易于复用和测试
- 📈 **高可维护性**：代码量减少80%，维护成本大幅降低
- 🚀 **快速扩展**：添加新系统只需约20行代码

---

## 🏗️ 架构设计

### 核心组件

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   HTTP请求       │───▶│   BaseSystem    │───▶│  TokenValidator │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                               │
                               ▼
                       ┌─────────────────┐
                       │  SystemConfig   │
                       └─────────────────┘
```

- **BaseSystem**: 通用的token处理逻辑
- **SystemConfig**: 系统配置（URL模式、Header名称等）
- **TokenValidator**: 可插拔的token验证器

---

## 🔧 添加新系统的步骤

### 1. 创建Token验证器

在 `src/auth/systems/system_xxx.rs` 文件中创建验证器：

```rust
use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

/// 自定义Token验证器
#[derive(Debug)]
struct CustomTokenValidator;

impl TokenValidator for CustomTokenValidator {
    fn validate(&self, token: &str) -> Result<()> {
        debug!("🔐 自定义验证器开始验证token，长度: {}", token.len());
        
        // 实现你的验证逻辑
        if token.len() != 32 {
            let error_msg = format!("Token长度必须为32位，当前: {}", token.len());
            warn!("❌ 自定义验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        
        if !token.chars().all(|c| c.is_alphanumeric()) {
            let error_msg = "Token必须为字母数字组合";
            warn!("❌ 自定义验证失败: {}", error_msg);
            return Err(anyhow!(error_msg));
        }
        
        info!("🔐 自定义Token验证通过");
        Ok(())
    }
}

/// 创建系统实例
pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "system_xxx".to_string(),
        system_name: "自定义系统".to_string(),
        url_pattern: r"https?://[^/]*yourapp[^/]*\..*?/api/.*".to_string(),
        header_name: "X-Your-Token".to_string(),
        token_pattern: r"([A-Za-z0-9]{32})".to_string(),
        expires_duration: 3600, // 1小时有效期
        validator: Box::new(CustomTokenValidator),
    };
    
    BaseSystem::new(config)
}
```

### 2. 注册新系统

在 `src/auth/systems/mod.rs` 中添加：

```rust
pub mod system_xxx; // 添加模块声明

// 在create_all_systems函数中添加
pub fn create_all_systems() -> Vec<Box<dyn SystemAuth + Send + Sync>> {
    vec![
        Box::new(system_a::create_system()),
        Box::new(system_b::create_system()),
        Box::new(system_c::create_system()),
        Box::new(system_d::create_system()),
        Box::new(system_xxx::create_system()), // 添加这一行
    ]
}
```

### 3. 重新编译

```bash
cd src-tauri
cargo build
```

**就是这么简单！** 🎉

---

## ⚙️ 配置详解

### SystemConfig 结构

```rust
pub struct SystemConfig {
    /// 系统ID (必须全局唯一)
    pub system_id: String,
    /// 系统名称 (用于显示和日志)
    pub system_name: String,
    /// URL匹配正则模式
    pub url_pattern: String,
    /// Token所在的HTTP Header名称
    pub header_name: String,
    /// Token提取正则模式 (必须包含一个捕获组)
    pub token_pattern: String,
    /// Token过期时间（秒）
    pub expires_duration: u64,
    /// Token验证器
    pub validator: Box<dyn TokenValidator>,
}
```

### 配置示例

| 配置项 | 示例值 | 说明 |
|--------|--------|------|
| system_id | `"system_a"` | 系统唯一标识符 |
| system_name | `"管理后台"` | 友好的系统名称 |
| url_pattern | `r"https?://[^/]*admin[^/]*\..*?/api/.*"` | URL匹配正则 |
| header_name | `"Authorization"` | Token所在Header |
| token_pattern | `r"Bearer\s+([A-Za-z0-9\-_\.]+)"` | Token提取正则 |
| expires_duration | `3600` | 过期时间(秒) |

---

## 🎯 现有系统示例

### 系统A: 管理后台 (JWT Token)
```rust
SystemConfig {
    system_id: "system_a".to_string(),
    system_name: "管理后台".to_string(),
    url_pattern: r"https?://[^/]*admin[^/]*\..*?/api/.*".to_string(),
    header_name: "Authorization".to_string(),
    token_pattern: r"Bearer\s+([A-Za-z0-9\-_\.]+)".to_string(),
    expires_duration: 3600, // 1小时
    validator: Box::new(JwtValidator),
}
```

### 系统B: 用户中心 (字母数字Token)
```rust
SystemConfig {
    system_id: "system_b".to_string(),
    system_name: "用户中心".to_string(),
    url_pattern: r"https?://[^/]*user[^/]*\..*?/api/.*".to_string(),
    header_name: "X-Auth-Token".to_string(),
    token_pattern: r"([A-Za-z0-9]{32,})".to_string(),
    expires_duration: 7200, // 2小时
    validator: Box::new(UserTokenValidator),
}
```

### 系统C: 数据平台 (十六进制Token)
```rust
SystemConfig {
    system_id: "system_c".to_string(),
    system_name: "数据平台".to_string(),
    url_pattern: r"https?://[^/]*data[^/]*\..*?/api/.*".to_string(),
    header_name: "Access-Token".to_string(),
    token_pattern: r"([A-Fa-f0-9]{64})".to_string(),
    expires_duration: 1800, // 30分钟
    validator: Box::new(DataTokenValidator),
}
```

### 系统D: 业务系统 (Base64 Token)
```rust
SystemConfig {
    system_id: "system_d".to_string(),
    system_name: "业务系统".to_string(),
    url_pattern: r"https?://[^/]*business[^/]*\..*?/api/.*".to_string(),
    header_name: "Authentication".to_string(),
    token_pattern: r"Token\s+([A-Za-z0-9\+/=]{40,})".to_string(),
    expires_duration: 1200, // 20分钟
    validator: Box::new(BusinessTokenValidator),
}
```

---

## 🔍 验证器示例

### JWT验证器
```rust
#[derive(Debug)]
struct JwtValidator;

impl TokenValidator for JwtValidator {
    fn validate(&self, token: &str) -> Result<()> {
        // 检查JWT前缀
        if !token.starts_with("eyJ") {
            return Err(anyhow!("JWT token格式错误，应以eyJ开头"));
        }
        
        // 检查JWT结构
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("JWT token应包含3个部分，实际: {}", parts.len()));
        }
        
        // 检查每个部分是否为空
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                return Err(anyhow!("JWT token第{}部分为空", i + 1));
            }
        }
        
        Ok(())
    }
}
```

### 长度+格式验证器
```rust
#[derive(Debug)]
struct LengthFormatValidator {
    min_length: usize,
    max_length: usize,
    allowed_chars: fn(char) -> bool,
}

impl TokenValidator for LengthFormatValidator {
    fn validate(&self, token: &str) -> Result<()> {
        if token.len() < self.min_length {
            return Err(anyhow!("Token长度太短，最少{}位", self.min_length));
        }
        
        if token.len() > self.max_length {
            return Err(anyhow!("Token长度太长，最多{}位", self.max_length));
        }
        
        if !token.chars().all(self.allowed_chars) {
            return Err(anyhow!("Token包含无效字符"));
        }
        
        Ok(())
    }
}
```

---

## 🔧 常用正则表达式模式

### URL匹配模式
```rust
// 匹配特定子域名
r"https?://[^/]*admin[^/]*\.example\.com/api/.*"

// 匹配特定路径前缀
r"https?://api\.company\.com/v[0-9]+/.*"

// 匹配多个可能的域名
r"https?://(admin|manage|control)\..*?/api/.*"

// 匹配任意端口的本地开发环境
r"https?://localhost:[0-9]+/api/.*"
```

### Token提取模式
```rust
// Bearer Token (JWT常用)
r"Bearer\s+([A-Za-z0-9\-_\.]+)"

// 自定义前缀
r"MyApp\s+([A-Za-z0-9]{32})"

// 纯token（无前缀）
r"([A-Fa-f0-9]{64})"

// Base64格式
r"([A-Za-z0-9\+/=]{40,})"

// API Key格式
r"ApiKey\s+([a-z0-9\-]{36})"
```

---

## 📊 性能优化建议

### 1. 正则表达式优化
- 避免过于复杂的正则表达式
- 使用具体的字符类而不是通配符
- 将最常匹配的模式放在前面

### 2. 验证器性能
- 先执行快速检查（如长度）再执行复杂验证
- 缓存编译好的正则表达式
- 避免在验证器中进行网络请求

### 3. 内存使用
- 验证器应该是轻量级的
- 避免在验证器中存储大量数据
- 使用静态方法而不是实例方法（如果可能）

---

## ⚠️ 注意事项

1. **系统ID唯一性**：确保每个系统的ID在全局范围内唯一
2. **URL模式冲突**：避免多个系统匹配相同的URL模式
3. **Token安全**：
   - 不要在日志中输出完整token
   - 验证失败时不要泄露token内容
   - 考虑token的敏感性级别
4. **正则表达式安全**：避免可能导致ReDoS攻击的复杂模式
5. **有效期设置**：根据业务安全要求设置合理的有效期
6. **错误处理**：验证器应该返回有意义的错误信息

---

## 🚀 快速开始模板

复制以下模板快速创建新系统：

```rust
use super::{TokenValidator, SystemConfig, BaseSystem};
use anyhow::{Result, anyhow};
use log::{warn, debug, info};

#[derive(Debug)]
struct MyValidator;

impl TokenValidator for MyValidator {
    fn validate(&self, token: &str) -> Result<()> {
        // TODO: 实现你的验证逻辑
        Ok(())
    }
}

pub fn create_system() -> BaseSystem {
    let config = SystemConfig {
        system_id: "my_system".to_string(),         // 修改为你的系统ID
        system_name: "我的系统".to_string(),         // 修改为你的系统名称
        url_pattern: r"https?://.*".to_string(),    // 修改为你的URL模式
        header_name: "Authorization".to_string(),   // 修改为你的Header名称
        token_pattern: r"(.+)".to_string(),         // 修改为你的Token模式
        expires_duration: 3600,                     // 修改为你的过期时间
        validator: Box::new(MyValidator),
    };
    
    BaseSystem::new(config)
}
```

只需要修改注释中的配置项，就能快速创建一个新的认证系统！🎉

---

## 📈 架构优势总结

与传统的继承实现方式相比，新的配置驱动架构具有：

- **80%代码减少**：从每个系统200+行减少到50行
- **零重复逻辑**：所有通用逻辑只写一次
- **更高可测试性**：验证器可以独立测试
- **更好的可维护性**：修改通用逻辑只需改一个地方
- **更快的开发速度**：新增系统只需20行代码
- **更强的类型安全**：编译期就能发现配置错误

这种架构设计让token认证系统既强大又简洁，是企业级应用的理想选择！ 