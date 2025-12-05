use crate::common::Result;
use directories::ProjectDirs;
use rusqlite::{Connection, params};
use std::env;
use std::fs;
use std::path::PathBuf;

/// 获取配置数据库路径（用户数据目录）
/// - Windows: C:\Users\<用户>\AppData\Roaming\FactBot\config.db
/// - macOS: ~/Library/Application Support/FactBot/config.db
/// - Linux: ~/.local/share/FactBot/config.db
pub fn get_config_db_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("", "", "FactBot")
        .ok_or_else(|| crate::common::AppError::DatabaseError("无法获取应用数据目录".to_string()))?;
    
    let data_dir = project_dirs.data_dir();
    
    // 创建数据目录
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)
            .map_err(crate::common::AppError::IoError)?;
    }
    
    Ok(data_dir.join("config.db"))
}

/// 获取日志数据库路径（软件运行目录）
pub fn get_log_db_path() -> Result<PathBuf> {
    let exe_path = env::current_exe()
        .map_err(crate::common::AppError::IoError)?;
    let exe_dir = exe_path.parent()
        .ok_or_else(|| crate::common::AppError::DatabaseError("无法获取可执行文件目录".to_string()))?;
    
    Ok(exe_dir.join("logs.db"))
}

/// 初始化配置数据库
pub fn init_config_db() -> Result<Connection> {
    let db_path = get_config_db_path()?;
    let conn = Connection::open(&db_path)?;
    
    // 创建配置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    Ok(conn)
}

/// 初始化日志数据库
pub fn init_log_db() -> Result<Connection> {
    let db_path = get_log_db_path()?;
    let conn = Connection::open(&db_path)?;
    
    // 创建日志表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_name TEXT NOT NULL,
            log_level TEXT NOT NULL,
            task_uuid TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    // 创建索引以提高查询性能
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_task_uuid ON logs(task_uuid)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_log_level ON logs(log_level)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON logs(timestamp)",
        [],
    )?;
    
    Ok(conn)
}

// ==================== 配置数据库操作 ====================

/// 保存配置
pub fn save_config(key: &str, value: &str) -> Result<()> {
    let conn = init_config_db()?;
    conn.execute(
        "INSERT OR REPLACE INTO config (key, value, updated_at) VALUES (?1, ?2, CURRENT_TIMESTAMP)",
        [key, value],
    )?;
    Ok(())
}

/// 加载配置
pub fn load_config(key: &str) -> Result<Option<String>> {
    let conn = init_config_db()?;
    let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
    let mut rows = stmt.query([key])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

/// 初始化所有数据库
pub fn initialize_databases() -> Result<()> {
    // 初始化配置数据库
    let _config_conn = init_config_db()?;
    
    // 初始化日志数据库
    let _log_conn = init_log_db()?;
    
    // 检查并标记首次启动
    if is_first_launch() {
        use chrono::Local;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let _ = save_log(
            "System",
            "INFO",
            "system-init",
            &timestamp,
            "FactBot 首次启动，数据库初始化完成"
        );
    }
    
    Ok(())
}

/// 检查是否首次启动
pub fn is_first_launch() -> bool {
    match load_config("app_launched") {
        Ok(Some(_)) => false,
        _ => {
            let _ = save_config("app_launched", "true");
            true
        }
    }
}

// ==================== 日志数据库操作 ====================

/// 日志条目结构
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub id: Option<i64>,
    pub task_name: String,
    pub log_level: String,
    pub task_uuid: String,
    pub timestamp: String,
    pub message: String,
}

/// 保存日志到数据库
pub fn save_log(
    task_name: &str,
    log_level: &str,
    task_uuid: &str,
    timestamp: &str,
    message: &str,
) -> Result<()> {
    let conn = init_log_db()?;
    conn.execute(
        "INSERT INTO logs (task_name, log_level, task_uuid, timestamp, message) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![task_name, log_level, task_uuid, timestamp, message],
    )?;
    Ok(())
}

/// 获取所有日志
pub fn get_all_logs() -> Result<Vec<LogEntry>> {
    let conn = init_log_db()?;
    let mut stmt = conn.prepare(
        "SELECT id, task_name, log_level, task_uuid, timestamp, message FROM logs ORDER BY id DESC"
    )?;
    
    let logs = stmt.query_map([], |row| {
        Ok(LogEntry {
            id: Some(row.get(0)?),
            task_name: row.get(1)?,
            log_level: row.get(2)?,
            task_uuid: row.get(3)?,
            timestamp: row.get(4)?,
            message: row.get(5)?,
        })
    })?;
    
    let mut result = Vec::new();
    for log in logs {
        result.push(log?);
    }
    
    Ok(result)
}
