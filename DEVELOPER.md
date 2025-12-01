# FactBot 开发者文档

## 项目概述

FactBot 是一个基于 Dioxus 0.7 的桌面应用，提供航司报价查询、配置管理和日志查看等功能。

## 技术栈

- **前端框架**: Dioxus 0.7
- **桌面应用**: dioxus-desktop
- **数据库**: SQLite (rusqlite)
- **HTTP 客户端**: 支持 TLS 指纹和浏览器模拟
- **日志系统**: 自定义 Logger + 数据库持久化

## 项目结构

```
FactBot/
├── src/
│   ├── main.rs              # 应用入口
│   ├── lib.rs               # 库入口
│   ├── db.rs                # 数据库操作
│   ├── common/              # 公共模块
│   │   ├── error.rs         # 错误处理
│   │   └── http_task/       # HTTP 任务模块
│   │       ├── config.rs    # 配置（Emulation、代理等）
│   │       ├── logger.rs    # 日志记录器
│   │       ├── task_manager.rs  # 任务管理器
│   │       └── ...
│   ├── components/          # UI 组件
│   │   └── sidebar.rs       # 侧边栏
│   └── views/               # 页面视图
│       ├── home.rs          # 首页
│       ├── config.rs        # 配置页面
│       ├── logs.rs          # 日志查看页面
│       └── ...
├── examples/                # 示例代码
├── docs/                    # 文档
├── assets/                  # 静态资源
└── Cargo.toml              # 项目配置
```

## 核心模块

### 1. 数据库系统 (src/db.rs)

#### 双数据库架构

**配置数据库 (config.db)**
- 位置：用户数据目录
- 密码保护：`CONFIG_DB_PASSWORD`
- 用途：存储应用配置、用户设置

**日志数据库 (logs.db)**
- 位置：软件运行目录
- 无密码保护
- 用途：存储任务执行日志

#### API

```rust
// 配置数据库
pub fn save_config(key: &str, value: &str) -> Result<()>
pub fn load_config(key: &str) -> Result<Option<String>>
pub fn delete_config(key: &str) -> Result<()>

// 日志数据库
pub fn save_log(task_name: &str, log_level: &str, task_uuid: &str, timestamp: &str, message: &str) -> Result<()>
pub fn get_all_logs() -> Result<Vec<LogEntry>>
pub fn get_logs_by_uuid(task_uuid: &str) -> Result<Vec<LogEntry>>
pub fn get_logs_by_level(log_level: &str) -> Result<Vec<LogEntry>>
```

### 2. HTTP 任务系统 (src/common/http_task/)

#### Emulation 浏览器指纹

支持 75+ 种浏览器指纹模拟：

```rust
pub enum Emulation {
    Chrome137,      // 默认
    Safari18,
    Firefox139,
    Edge134,
    OkHttp5,
    // ... 更多
}
```

#### TaskConfig 配置

```rust
pub struct TaskConfig {
    pub intercept_domains: Vec<String>,      // 拦截域名
    pub intercept_keywords: Vec<String>,     // 拦截关键词
    pub ip_mode: IPMode,                     // 代理模式
    pub emulation: Emulation,                // 浏览器指纹
    pub timeout: Duration,                   // 超时时间
    pub enable_cookie_management: bool,      // Cookie 管理
    pub allow_redirect: bool,                // 允许重定向
    pub verify_cert: bool,                   // 证书验证
}
```

#### TaskManager 任务管理器

```rust
let config = TaskConfig {
    emulation: Emulation::Chrome137,
    timeout: Duration::from_secs(30),
    ..Default::default()
};

let mut tm = TaskManager::new("任务名称".to_string(), config)?;
tm.info("开始执行任务");
let response = tm.get("https://example.com").await?;
```

### 3. 日志系统 (src/common/http_task/logger.rs)

#### Logger 使用

```rust
use fact_bot::common::http_task::Logger;
use uuid::Uuid;

let task_id = Uuid::new_v4().to_string();
let logger = Logger::new("任务名称".to_string(), task_id);

logger.info("信息日志");
logger.debug("调试日志");
logger.warn("警告日志");
logger.error("错误日志");

// 打印所有日志
logger.print_all();
```

#### 环境区分

- **dev 模式**：DEBUG 日志正常显示和打印
- **正式版**：DEBUG 日志只入库，不显示不打印

```bash
# dev 模式
cargo run

# 正式版
cargo run --no-default-features --features desktop
```

### 4. UI 组件 (src/views/)

#### 日志查看页面 (logs.rs)

功能：
- 任务名称筛选（必填）
- 任务 UUID 筛选（动态）
- 日志级别筛选
- 关键词搜索
- 时间排序
- 分页显示
- 日志内容截断 + 气泡提示
- 一键复制

## 开发指南

### 环境搭建

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Dioxus CLI
curl -sSL http://dioxus.dev/install.sh | sh

# 克隆项目
git clone <repository>
cd FactBot

# 安装依赖
cargo build
```

### 开发运行

```bash
# 使用 dx（推荐）
dx serve

# 或使用 cargo
cargo run
```

### 构建发布

```bash
# dev 模式
cargo build --release

# 正式版（隐藏 DEBUG 日志）
cargo build --release --no-default-features --features desktop
```

### 运行示例

```bash
# 数据库演示
cargo run --example log_database_demo

# 生成测试日志
cargo run --example generate_test_logs

# 长日志测试
cargo run --example test_long_logs

# Emulation 演示
cargo run --example emulation_demo
```



## 代码规范

### Dioxus 0.7 规范

- ✅ 使用 `#[component]` 宏
- ✅ 使用 `Signal` 而不是 `use_state`
- ✅ 不使用 `cx` 或 `Scope`
- ✅ 使用 `rsx!` 宏构建 UI
- ✅ 使用 `spawn` 处理异步任务

### 命名规范

- 组件：大驼峰 `MyComponent`
- 函数：小写下划线 `my_function`
- 常量：大写下划线 `MY_CONSTANT`
- 文件：小写下划线 `my_module.rs`

### 错误处理

使用统一的错误类型：

```rust
use crate::common::{AppError, Result};

pub fn my_function() -> Result<String> {
    // ...
    Ok("success".to_string())
}
```

## 添加新功能

### 1. 添加新的视图页面

```rust
// src/views/my_page.rs
use dioxus::prelude::*;

#[component]
pub fn MyPage() -> Element {
    rsx! {
        div { "My Page Content" }
    }
}

// src/views/mod.rs
mod my_page;
pub use my_page::MyPage;

// src/main.rs
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Sidebar)]
        #[route("/my-page")]
        MyPage {},
}
```

### 2. 添加新的数据库表

```rust
// src/db.rs
pub fn init_my_table() -> Result<Connection> {
    let conn = Connection::open(get_db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS my_table (
            id INTEGER PRIMARY KEY,
            data TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}
```

### 3. 添加新的 Emulation 类型

```rust
// src/common/http_task/config.rs
pub enum Emulation {
    // ... 现有类型
    MyNewBrowser,
}

impl Emulation {
    pub fn as_str(&self) -> &'static str {
        match self {
            // ... 现有匹配
            Emulation::MyNewBrowser => "my_new_browser",
        }
    }
}
```

## 调试技巧

### 1. 查看日志

```rust
// 在代码中添加日志
logger.debug("调试信息");
logger.info("普通信息");
```

### 2. 浏览器开发者工具

按 F12 打开开发者工具，查看：
- Console：JavaScript 错误和日志
- Network：网络请求
- Elements：DOM 结构

### 3. 数据库查看

```bash
# 使用 SQLite 工具
sqlite3 logs.db
> SELECT * FROM logs LIMIT 10;
```

## 性能优化

### 1. 日志性能

- dev 模式：所有日志保存到内存
- 正式版：DEBUG 日志不保存到内存

### 2. UI 渲染

- 使用分页减少渲染数量
- 长文本截断显示
- 按需加载数据

### 3. 数据库优化

- 使用索引加速查询
- 批量操作减少 I/O
- 定期清理旧数据

## 常见问题

### Q: 如何添加新的浏览器指纹？

A: 在 `config.rs` 的 `Emulation` 枚举中添加新变体，并在 `as_str()` 方法中添加对应的字符串。

### Q: 如何修改数据库结构？

A: 修改 `db.rs` 中的 `CREATE TABLE` 语句，注意使用 `IF NOT EXISTS` 避免冲突。

### Q: 如何调试 JavaScript 复制功能？

A: 打开浏览器开发者工具（F12），查看 Console 中的日志输出。

### Q: 如何切换环境？

A: 使用 `--no-default-features --features desktop` 构建正式版。

## 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交代码
4. 创建 Pull Request

## 许可证

[根据项目实际情况填写]

## 联系方式

- 作者：zilong
- 邮箱：liuzilong326@163.com
