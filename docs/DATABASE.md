# 数据库系统说明

FactBot 使用两个独立的 SQLite 数据库来管理不同类型的数据。

## 1. 配置数据库 (config.db)

### 位置
- **Windows**: `C:\Users\<用户>\AppData\Roaming\FactBot\config.db`
- **macOS**: `~/Library/Application Support/FactBot/config.db`
- **Linux**: `~/.local/share/FactBot/config.db`

### 特性
- ✅ **密码保护**：使用密码 `FactBot_Config_2024` 保护（需要 SQLCipher 支持）
- ✅ **用户数据目录**：存储在系统标准的用户数据目录
- ✅ **持久化配置**：存储应用配置、用户设置等

### 表结构
```sql
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
)
```

### API 使用
```rust
use fact_bot::db;

// 保存配置
db::save_config("key", "value")?;

// 读取配置
let value = db::load_config("key")?;

// 删除配置
db::delete_config("key")?;
```

## 2. 日志数据库 (logs.db)

### 位置
- **所有平台**：软件运行目录下的 `logs.db`
- 例如：`F:\project\Rust\FactBot\target\debug\logs.db`

### 特性
- ✅ **无密码**：方便查看和调试
- ✅ **软件目录**：与可执行文件在同一目录
- ✅ **自动记录**：Logger 的每次调用都会自动保存到数据库

### 表结构
```sql
CREATE TABLE logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_name TEXT NOT NULL,
    log_level TEXT NOT NULL,
    task_uuid TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
)

-- 索引
CREATE INDEX idx_task_uuid ON logs(task_uuid);
CREATE INDEX idx_log_level ON logs(log_level);
CREATE INDEX idx_timestamp ON logs(timestamp);
```

### API 使用

#### 使用 Logger（推荐）
```rust
use fact_bot::common::http_task::Logger;
use uuid::Uuid;

// 创建 Logger
let task_id = Uuid::new_v4().to_string();
let logger = Logger::new("任务名称".to_string(), task_id);

// 记录日志（自动保存到数据库）
logger.info("信息日志");
logger.debug("调试日志");
logger.warn("警告日志");
logger.error("错误日志");

// 打印日志（带颜色）
logger.print_all();
```

#### 直接操作数据库
```rust
use fact_bot::db;

// 保存日志
db::save_log("任务名", "INFO", "task-uuid", "2024-01-01 12:00:00", "消息")?;

// 查询所有日志
let logs = db::get_all_logs()?;

// 按任务 UUID 查询
let logs = db::get_logs_by_uuid("task-uuid")?;

// 按日志级别查询
let logs = db::get_logs_by_level("ERROR")?;

// 获取日志总数
let count = db::get_log_count()?;

// 清空日志
db::clear_all_logs()?;
db::clear_logs_by_uuid("task-uuid")?;
```

## 3. 初始化

应用启动时会自动初始化两个数据库：

```rust
use fact_bot::db;

// 初始化所有数据库
db::initialize_databases()?;
```

在 `main.rs` 的 `App` 组件中已经自动调用：
```rust
#[component]
fn App() -> Element {
    let _ = db::initialize_databases();
    // ...
}
```

## 4. 日志格式

Logger 输出格式：
```
[任务名称] [日志级别] [任务ID前8位] [时间] : 消息内容
```

示例：
```
[HTTP请求] [INFO] [23e5202d] [2024-01-01 12:00:00] : 请求成功
[HTTP请求] [WARN] [23e5202d] [2024-01-01 12:00:01] : 网络延迟
[HTTP请求] [ERROR] [23e5202d] [2024-01-01 12:00:02] : 请求失败
```

颜色：
- **INFO** = 白色
- **WARN** = 黄色
- **ERROR** = 红色
- **DEBUG** = 蓝色

## 5. 运行演示

查看完整的数据库使用示例：

```bash
cargo run --example log_database_demo
```

这个演示会展示：
- 数据库初始化
- Logger 使用
- 日志查询
- 配置管理
