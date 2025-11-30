use directories::ProjectDirs;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

/// 获取跨平台的数据库路径
/// - Windows: C:\Users\<用户>\AppData\Roaming\FactBot\config.db
/// - macOS: ~/Library/Application Support/FactBot/config.db
/// - Linux: ~/.local/share/FactBot/config.db
pub fn get_db_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("", "", "FactBot")
        .expect("无法获取应用数据目录");
    
    let data_dir = project_dirs.data_dir();
    
    // 创建数据目录
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("无法创建数据目录");
    }
    
    data_dir.join("config.db")
}

pub fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    let is_new_db = !db_path.exists();
    
    // println!("数据库路径: {:?}", db_path);
    let conn = Connection::open(&db_path)?;
    
    // 创建配置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    
    // 如果是新创建的数据库，等待2秒确保完全初始化
    if is_new_db {
        // println!("首次创建数据库，等待初始化...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        // println!("数据库初始化完成");
    }
    
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
