use fact_bot::common::Result;
use fact_bot::common::http_task::{TaskConfig, TaskManager};
use fact_bot::db;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== 数据库系统演示 ===\n");

    // ==================== 配置数据库演示 ====================
    println!("1. 配置数据库（带密码保护）");
    println!("   位置: 用户数据目录");
    
    // 保存配置
    db::save_config("api_key", "test_key_12345")?;
    db::save_config("proxy_host", "127.0.0.1")?;
    db::save_config("proxy_port", "7890")?;
    println!("   ✓ 已保存 3 个配置项");
    
    // 读取配置
    if let Some(api_key) = db::load_config("api_key")? {
        println!("   ✓ 读取配置: api_key = {}", api_key);
    }
    
    // 删除配置
    db::delete_config("proxy_port")?;
    println!("   ✓ 已删除配置: proxy_port\n");

    // ==================== 日志数据库演示 ====================
    println!("2. 日志数据库（无密码）");
    println!("   位置: 软件运行目录/logs.db\n");
    
    // 创建任务管理器（会自动记录日志到数据库）
    let config = TaskConfig::default();
    let manager = TaskManager::new("数据库测试".to_string(), config)?;
    
    // 记录各种级别的日志
    manager.info("任务开始执行");
    manager.debug("检查系统配置");
    manager.info("连接到服务器");
    manager.warn("响应时间较长");
    manager.error("连接超时");
    manager.info("任务完成");
    
    println!("   ✓ 已记录 6 条日志到数据库\n");
    
    // 从数据库读取日志
    println!("3. 从数据库读取日志:");
    let all_logs = db::get_all_logs()?;
    println!("   总日志数: {}", all_logs.len());
    
    // 显示最近的 5 条日志
    println!("\n   最近的日志:");
    for (i, log) in all_logs.iter().take(5).enumerate() {
        println!(
            "   {}. [{}] [{}] [{}] : {}",
            i + 1,
            log.task_name,
            log.log_level,
            &log.task_uuid[..8],
            log.message
        );
    }
    
    // 按级别查询
    println!("\n4. 按级别查询日志:");
    let error_logs = db::get_logs_by_level("ERROR")?;
    println!("   错误日志数: {}", error_logs.len());
    for log in error_logs.iter() {
        println!("   - [{}] {}", log.timestamp, log.message);
    }
    
    let warn_logs = db::get_logs_by_level("WARN")?;
    println!("   警告日志数: {}", warn_logs.len());
    
    // 按任务 UUID 查询
    println!("\n5. 按任务 UUID 查询:");
    let task_logs = db::get_logs_by_uuid(manager.task_id())?;
    println!("   当前任务的日志数: {}", task_logs.len());
    
    // 统计信息
    println!("\n6. 数据库统计:");
    let total_count = db::get_log_count()?;
    println!("   数据库中总日志数: {}", total_count);
    
    println!("\n=== 演示完成 ===");
    println!("\n说明:");
    println!("- 配置数据库: 存储在用户数据目录，带密码保护");
    println!("- 日志数据库: 存储在软件运行目录，无密码");
    println!("- 日志自动保存: 调用 manager.info() 等方法时自动保存到数据库");
    println!("- 支持查询: 可按级别、UUID、时间等条件查询日志");

    Ok(())
}
