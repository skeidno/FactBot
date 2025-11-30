use directories::UserDirs;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

pub fn get_db_path() -> PathBuf {
    let user_dirs = UserDirs::new().expect("无法获取用户目录");
    let documents = user_dirs.document_dir().expect("无法获取文档目录");
    let factbot_dir = documents.join("FactBot");
    
    // 创建 FactBot 目录
    if !factbot_dir.exists() {
        fs::create_dir_all(&factbot_dir).expect("无法创建 FactBot 目录");
    }
    
    factbot_dir.join("config.db")
}

pub fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;
    
    // 创建配置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

pub fn save_config(key: &str, value: &str) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
        [key, value],
    )?;
    Ok(())
}

pub fn load_config(key: &str) -> Result<Option<String>> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
    let mut rows = stmt.query([key])?;
    
    if let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}
