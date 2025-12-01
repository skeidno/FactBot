# HTTP Task Manager

一个功能完整的 HTTP 任务管理模块，提供网络拦截、IP 管理、日志记录、Cookie 管理等功能。

## 功能特性

- ✅ **任务管理**：自动生成唯一 UUID，完整的任务生命周期管理
- ✅ **网络拦截器**：支持域名和 URL 关键词拦截
- ✅ **IP 管理**：支持固定 IP、IP 池轮询、无代理三种模式
- ✅ **日志系统**：四个日志等级（Normal、Warn、Error、Debug）
- ✅ **Cookie 管理**：自动记录、更新和附加 Cookie
- ✅ **请求执行**：严格的执行流程保证各模块协同工作

## 快速开始

### 基本使用

```rust
use fact_bot::common::Result;  // 使用全局 Result<T>
use fact_bot::common::http_task::{TaskConfig, TaskManager, LogLevel};

#[tokio::main]
async fn main() -> Result<()> {  // 简化的 Result，无需指定错误类型
    // 创建配置
    let mut config = TaskConfig::default();
    config.log_level = LogLevel::Debug;
    config.enable_cookie_management = true;
    
    // 创建 TaskManager
    let mut task_manager = TaskManager::new(config)?;
    
    // 发起请求
    let response = task_manager.get("https://example.com").await?;
    
    Ok(())
}
```

### 全局错误系统

项目使用统一的错误处理系统，所有模块共享 `AppError` 和 `Result<T>`：

```rust
use fact_bot::common::{AppError, Result};

// 所有函数使用统一的 Result<T>
pub fn my_function() -> Result<String> {
    // 错误会自动转换
    let data = std::fs::read_to_string("file.txt")?;  // IO 错误自动转换
    let json: serde_json::Value = serde_json::from_str(&data)?;  // JSON 错误自动转换
    Ok(json.to_string())
}
```

查看完整文档：[错误处理指南](../../../docs/ERROR_HANDLING.md)

### 配置网络拦截器

```rust
let mut config = TaskConfig::default();
config.intercept_domains = vec!["blocked.com".to_string()];
config.intercept_keywords = vec!["malware".to_string()];

let mut task_manager = TaskManager::new(config)?;

// 这个请求会被拦截
match task_manager.get("https://blocked.com/path").await {
    Err(e) => println!("Blocked: {}", e),
    _ => {}
}
```

### 配置 IP 池

```rust
use fact_bot::common::http_task::{IPMode, ProxyConfig};

let proxies = vec![
    ProxyConfig {
        host: "proxy1.example.com".to_string(),
        port: 8080,
        username: Some("user".to_string()),
        password: Some("pass".to_string()),
    },
    ProxyConfig {
        host: "proxy2.example.com".to_string(),
        port: 8080,
        username: None,
        password: None,
    },
];

let mut config = TaskConfig::default();
config.ip_mode = IPMode::Pool(proxies);

let mut task_manager = TaskManager::new(config)?;

// 查看当前代理
println!("Current proxy: {:?}", task_manager.get_ip());

// 切换到下一个代理
task_manager.switch_next()?;
```

### 启用 Cookie 管理

```rust
let mut config = TaskConfig::default();
config.enable_cookie_management = true;

let mut task_manager = TaskManager::new(config)?;

// Cookie 会自动管理
task_manager.get("https://example.com/login").await?;
task_manager.get("https://example.com/dashboard").await?; // 自动带上 Cookie
```

## API 文档

### TaskManager

主要的任务管理器接口。

#### 方法

- `new(config: TaskConfig) -> Result<TaskManager>` - 创建新的任务管理器
- `task_id() -> &str` - 获取任务 ID
- `get_ip() -> Option<&ProxyConfig>` - 获取当前代理
- `switch_next() -> Result<()>` - 切换到下一个代理
- `get(url: &str) -> Result<Response>` - 发起 GET 请求
- `post_json(url: &str, body: Value) -> Result<Response>` - 发起 POST JSON 请求
- `post_form(url: &str, form: HashMap) -> Result<Response>` - 发起 POST 表单请求
- `fetch_text(url: &str) -> Result<String>` - 获取文本响应
- `fetch_json(url: &str) -> Result<Value>` - 获取 JSON 响应
- `fetch_bytes(url: &str) -> Result<Vec<u8>>` - 获取字节响应

### TaskConfig

任务配置结构。

#### 字段

- `intercept_domains: Vec<String>` - 拦截的域名列表
- `intercept_keywords: Vec<String>` - 拦截的 URL 关键词列表
- `ip_mode: IPMode` - IP 使用模式
- `log_level: LogLevel` - 日志等级
- `tls_fingerprint: Option<String>` - TLS 指纹
- `browser_fingerprint: Option<String>` - 浏览器指纹
- `user_agent: Option<String>` - 自定义 User-Agent
- `timeout: Duration` - 请求超时时间
- `enable_cookie_management: bool` - 是否启用 Cookie 管理

### IPMode

IP 使用模式枚举。

- `Fixed(ProxyConfig)` - 固定 IP 模式
- `Pool(Vec<ProxyConfig>)` - IP 池轮询模式
- `None` - 无代理模式

### LogLevel

日志等级枚举。

- `Normal` - 普通日志
- `Warn` - 警告日志
- `Error` - 错误日志
- `Debug` - 调试日志（包含所有信息）

## 请求执行流程

每个请求都按照以下顺序执行：

1. **拦截器检查** - 检查 URL 是否被拦截
2. **获取代理** - 从 IP 管理器获取当前代理
3. **配置指纹** - 应用 TLS 和浏览器指纹
4. **插入 Cookie** - 附加 CookieStore 中的 Cookie
5. **发起请求** - 执行实际的 HTTP 请求
6. **更新 Cookie** - 从响应中更新 Cookie
7. **记录日志** - 根据日志等级输出日志

## 测试

运行所有测试：

```bash
cargo test --lib http_task
```

运行示例：

```bash
cargo run --example http_task_basic
```

## 注意事项

⚠️ **当前实现使用 Mock HTTP 客户端**

当前版本使用占位符实现的 HTTP 客户端。要使用实际的网络请求，需要：

1. 实现或集成 `common::wreq::Client`
2. 在 `TaskManager::execute_request` 中替换 Mock 实现
3. 应用 TLS 指纹和浏览器指纹配置

## 架构

```
src/common/http_task/
├── mod.rs              # 模块入口
├── task_manager.rs     # TaskManager 主结构
├── interceptor.rs      # 网络拦截器
├── ip_manager.rs       # IP 管理系统
├── cookie_store.rs     # Cookie 管理
├── logger.rs           # 日志系统
├── config.rs           # 配置结构
└── types.rs            # 公共类型定义
```

## 许可证

与主项目相同
