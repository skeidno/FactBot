# Emulation 浏览器指纹配置

## 概述

FactBot 支持多种浏览器指纹模拟（Emulation），可以模拟不同浏览器的 TLS 指纹和 HTTP 特征，用于绕过反爬虫检测。

## 支持的浏览器

### Chrome 系列（29 个版本）

```rust
Emulation::Chrome100  // Chrome 100
Emulation::Chrome101  // Chrome 101
...
Emulation::Chrome137  // Chrome 137（默认）
```

**完整列表**：
- Chrome 100, 101, 104-110, 114, 116-120, 123-137

### Safari 系列（19 个版本）

```rust
Emulation::Safari18           // Safari 18（桌面版）
Emulation::SafariIos18_1_1    // Safari iOS 18.1.1
Emulation::SafariIPad18       // Safari iPad 18
```

**完整列表**：
- Safari 15.3, 15.5, 15.6.1, 16, 16.5, 17.0, 17.2.1, 17.4.1, 17.5, 18, 18.2, 18.3, 18.3.1, 18.5
- Safari iOS 16.5, 17.2, 17.4.1, 18.1.1
- Safari iPad 18

### Firefox 系列（10 个版本）

```rust
Emulation::Firefox139          // Firefox 139
Emulation::FirefoxPrivate136   // Firefox 隐私模式 136
Emulation::FirefoxAndroid135   // Firefox Android 135
```

**完整列表**：
- Firefox 109, 117, 128, 133, 135, 136, 139
- Firefox Private 135, 136
- Firefox Android 135

### Edge 系列（5 个版本）

```rust
Emulation::Edge134  // Edge 134
```

**完整列表**：
- Edge 101, 122, 127, 131, 134

### Opera 系列（4 个版本）

```rust
Emulation::Opera119  // Opera 119
```

**完整列表**：
- Opera 116, 117, 118, 119

### OkHttp 系列（8 个版本）

用于模拟 Android 应用的 HTTP 客户端：

```rust
Emulation::OkHttp5  // OkHttp 5
```

**完整列表**：
- OkHttp 3.9, 3.11, 3.13, 3.14
- OkHttp 4.9, 4.10, 4.12
- OkHttp 5

## 使用方式

### 基本配置

```rust
use fact_bot::common::http_task::{Emulation, TaskConfig, TaskManager};
use std::time::Duration;

// 创建配置
let config = TaskConfig {
    emulation: Emulation::Chrome137,  // 选择浏览器指纹
    timeout: Duration::from_secs(30),
    enable_cookie_management: true,
    allow_redirect: true,
    verify_cert: true,
    ..Default::default()
};

// 创建任务管理器
let tm = TaskManager::new("我的任务".to_string(), config)?;
```

### 配合代理使用

```rust
use fact_bot::common::http_task::{Emulation, IPMode, ProxyConfig, TaskConfig};

let proxy = ProxyConfig {
    host: "127.0.0.1".to_string(),
    port: 7890,
    username: Some("user".to_string()),
    password: Some("pass".to_string()),
};

let config = TaskConfig {
    emulation: Emulation::Safari18,
    ip_mode: IPMode::Fixed(proxy),
    timeout: Duration::from_secs(60),
    enable_cookie_management: true,
    verify_cert: false,  // 使用代理时可能需要禁用证书验证
    ..Default::default()
};
```

### 移动端模拟

```rust
// 模拟 iOS Safari
let config = TaskConfig {
    emulation: Emulation::SafariIos18_1_1,
    ..Default::default()
};

// 模拟 Android OkHttp
let config = TaskConfig {
    emulation: Emulation::OkHttp5,
    ..Default::default()
};

// 模拟 Android Firefox
let config = TaskConfig {
    emulation: Emulation::FirefoxAndroid135,
    ..Default::default()
};
```

## TaskConfig 完整选项

```rust
pub struct TaskConfig {
    // 拦截器配置
    pub intercept_domains: Vec<String>,      // 拦截的域名列表
    pub intercept_keywords: Vec<String>,     // 拦截的关键词列表
    
    // 代理配置
    pub ip_mode: IPMode,                     // 代理模式（固定/池/无）
    
    // 浏览器指纹
    pub emulation: Emulation,                // 浏览器模拟类型
    
    // 请求配置
    pub timeout: Duration,                   // 请求超时时间
    pub enable_cookie_management: bool,      // 是否启用 Cookie 管理
    pub allow_redirect: bool,                // 是否允许重定向
    pub verify_cert: bool,                   // 是否验证 SSL 证书
}
```

## 实际使用示例

### 示例 1: 爬取需要反爬的网站

```rust
// 使用最新的 Chrome 指纹
let config = TaskConfig {
    emulation: Emulation::Chrome137,
    timeout: Duration::from_secs(30),
    enable_cookie_management: true,
    allow_redirect: true,
    ..Default::default()
};

let mut tm = TaskManager::new("爬虫任务".to_string(), config)?;
let response = tm.get("https://example.com").await?;
```

### 示例 2: 模拟移动端访问

```rust
// 模拟 iPhone Safari
let config = TaskConfig {
    emulation: Emulation::SafariIos18_1_1,
    timeout: Duration::from_secs(30),
    ..Default::default()
};

let mut tm = TaskManager::new("移动端任务".to_string(), config)?;
```

### 示例 3: 使用代理池

```rust
let proxies = vec![
    ProxyConfig {
        host: "proxy1.example.com".to_string(),
        port: 8080,
        username: None,
        password: None,
    },
    ProxyConfig {
        host: "proxy2.example.com".to_string(),
        port: 8080,
        username: None,
        password: None,
    },
];

let config = TaskConfig {
    emulation: Emulation::Firefox139,
    ip_mode: IPMode::Pool(proxies),
    timeout: Duration::from_secs(60),
    verify_cert: false,
    ..Default::default()
};

let mut tm = TaskManager::new("代理池任务".to_string(), config)?;
```

## 选择建议

### 通用爬虫
- **推荐**: `Chrome137`, `Chrome136`, `Chrome135`
- **原因**: 最新版本，兼容性好，检测率低

### 移动端爬虫
- **iOS**: `SafariIos18_1_1`, `SafariIos17_4_1`
- **Android**: `OkHttp5`, `FirefoxAndroid135`

### 特定场景
- **需要隐私**: `FirefoxPrivate136`
- **企业环境**: `Edge134`
- **特殊网站**: 根据目标网站的主流用户选择

## 注意事项

1. **版本选择**: 选择较新但不是最新的版本，避免过于突出
2. **User-Agent**: Emulation 会自动设置匹配的 User-Agent
3. **TLS 指纹**: 每个 Emulation 都有对应的 TLS 指纹
4. **证书验证**: 使用代理时可能需要禁用证书验证
5. **Cookie 管理**: 需要保持会话时启用 Cookie 管理

## 集成 wreq::Client

实际使用时需要集成 `wreq` crate：

```rust
// 在 TaskManager 中实现
pub fn build_client(&self) -> Result<wreq::Client> {
    let mut builder = wreq::Client::builder();
    
    // 配置代理
    if let Some(proxy) = self.get_ip() {
        builder = builder.proxy(wreq::Proxy::all(proxy.http())?);
    }
    
    // 配置 Emulation
    builder = builder
        .timeout(self.config.timeout)
        .cookie_store(self.config.enable_cookie_management)
        .emulation(self.config.emulation.into());  // 需要实现 Into trait
    
    // 配置重定向
    builder = builder.redirect(if self.config.allow_redirect {
        wreq::redirect::Policy::default()
    } else {
        wreq::redirect::Policy::none()
    });
    
    // 配置证书验证
    if !self.config.verify_cert {
        builder = builder.cert_verification(false);
    }
    
    builder.build()
}
```

## 运行演示

```bash
cargo run --example emulation_demo
```

这会展示所有可用的 Emulation 类型和配置示例。

## 总结

- ✅ 支持 75+ 种浏览器指纹
- ✅ 覆盖主流浏览器和移动端
- ✅ 灵活的配置选项
- ✅ 易于集成和使用
- ✅ 支持代理和 Cookie 管理

根据你的需求选择合适的 Emulation 类型，提高爬虫的成功率！
