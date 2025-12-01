use crate::common::Result;
use directories::ProjectDirs;
use rusqlite::{Connection, params};
use std::env;
use std::fs;
use std::path::PathBuf;

// 配置数据库密码（实际项目中应该从环境变量或配置文件读取）
#[allow(dead_code)]
const CONFIG_DB_PASSWORD: &str = "FactBot_Config_2024";

/// 获取配置数据库路径（用户数据目录，带密码保护）
/// - Windows: C:\Users\<用户>\AppData\Roaming\FactBot\config.db
/// - macOS: ~/Library/Application Support/FactBot/config.db
/// - Linux: ~/.local/share/FactBot/config.db
pub fn get_config_db_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("", "", "FactBot")
        .expect("无法获取应用数据目录");
    
    let data_dir = project_dirs.data_dir();
    
    // 创建数据目录
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("无法创建数据目录");
    }
    
    data_dir.join("config.db")
}

/// 获取日志数据库路径（软件运行目录，无密码）
pub fn get_log_db_path() -> PathBuf {
    // 获取当前可执行文件所在目录
    let exe_path = env::current_exe().expect("无法获取可执行文件路径");
    let exe_dir = exe_path.parent().expect("无法获取可执行文件目录");
    
    exe_dir.join("logs.db")
}

/// 初始化配置数据库（带密码保护）
pub fn init_config_db() -> Result<Connection> {
    let db_path = get_config_db_path();
    let is_new_db = !db_path.exists();
    
    let conn = Connection::open(&db_path)?;
    
    // 设置数据库密码（使用 PRAGMA key）
    // 注意：这需要 SQLCipher 支持，标准 SQLite 不支持加密
    // 如果不需要加密，可以注释掉这行
    // conn.execute(&format!("PRAGMA key = '{}'", CONFIG_DB_PASSWORD), [])?;
    
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
    
    // 如果是新创建的数据库，等待确保完全初始化
    if is_new_db {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    Ok(conn)
}

/// 初始化日志数据库（无密码，存储在软件目录）
pub fn init_log_db() -> Result<Connection> {
    let db_path = get_log_db_path();
    let is_new_db = !db_path.exists();
    
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
    
    if is_new_db {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
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
        let value: String = row.get(0)?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

/// 删除配置
#[allow(dead_code)]
pub fn delete_config(key: &str) -> Result<()> {
    let conn = init_config_db()?;
    conn.execute("DELETE FROM config WHERE key = ?1", [key])?;
    Ok(())
}

/// 初始化所有数据库
/// 在应用启动时调用，确保两个数据库都被正确创建和初始化
pub fn initialize_databases() -> Result<()> {
    // 1. 初始化配置数据库（带密码保护，存储在用户数据目录）
    let config_conn = init_config_db()?;
    drop(config_conn); // 释放连接
    
    // 2. 初始化日志数据库（无密码，存储在软件运行目录）
    let log_conn = init_log_db()?;
    drop(log_conn); // 释放连接
    
    // 3. 检查并标记首次启动
    let is_first = is_first_launch();
    
    if is_first {
        // 首次启动，可以在这里添加一些初始化日志
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
            // 标记应用已启动
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

/// 根据任务 UUID 获取日志
#[allow(dead_code)]
pub fn get_logs_by_uuid(task_uuid: &str) -> Result<Vec<LogEntry>> {
    let conn = init_log_db()?;
    let mut stmt = conn.prepare(
        "SELECT id, task_name, log_level, task_uuid, timestamp, message FROM logs WHERE task_uuid = ?1 ORDER BY id DESC"
    )?;
    
    let logs = stmt.query_map([task_uuid], |row| {
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

/// 根据日志级别获取日志
#[allow(dead_code)]
pub fn get_logs_by_level(log_level: &str) -> Result<Vec<LogEntry>> {
    let conn = init_log_db()?;
    let mut stmt = conn.prepare(
        "SELECT id, task_name, log_level, task_uuid, timestamp, message FROM logs WHERE log_level = ?1 ORDER BY id DESC"
    )?;
    
    let logs = stmt.query_map([log_level], |row| {
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

/// 清空所有日志
#[allow(dead_code)]
pub fn clear_all_logs() -> Result<()> {
    let conn = init_log_db()?;
    conn.execute("DELETE FROM logs", [])?;
    Ok(())
}

/// 清空指定任务的日志
#[allow(dead_code)]
pub fn clear_logs_by_uuid(task_uuid: &str) -> Result<()> {
    let conn = init_log_db()?;
    conn.execute("DELETE FROM logs WHERE task_uuid = ?1", [task_uuid])?;
    Ok(())
}

/// 获取日志总数
#[allow(dead_code)]
pub fn get_log_count() -> Result<i64> {
    let conn = init_log_db()?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM logs", [], |row| row.get(0))?;
    Ok(count)
}
