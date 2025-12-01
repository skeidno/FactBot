/// 日志数据库演示
/// 
/// 演示如何使用日志系统，包括：
/// 1. 自动初始化两个数据库（配置数据库和日志数据库）
/// 2. 使用 Logger 记录日志（自动保存到数据库）
/// 3. 从数据库查询日志
/// 
/// 运行方式：
/// ```bash
/// cargo run --example log_database_demo
/// ```

use fact_bot::common::http_task::Logger;
use fact_bot::db;
use uuid::Uuid;

fn main() {
    println!("=== 日志数据库演示 ===\n");

    // 1. 初始化数据库
    println!("1. 初始化数据库...");
    match db::initialize_databases() {
        Ok(_) => println!("   ✓ 数据库初始化成功"),
        Err(e) => {
            eprintln!("   ✗ 数据库初始化失败: {}", e);
            return;
        }
    }

    // 显示数据库路径
    println!("   - 配置数据库: {:?}", db::get_config_db_path());
    println!("   - 日志数据库: {:?}\n", db::get_log_db_path());

    // 2. 创建 Logger 并记录日志
    println!("2. 创建 Logger 并记录日志...");
    let task_id = Uuid::new_v4().to_string();
    let logger = Logger::new("演示任务".to_string(), task_id.clone());

    logger.info("任务开始执行");
    logger.debug("正在加载配置文件");
    logger.info("配置加载完成");
    logger.warn("检测到网络延迟");
    logger.info("正在重试请求...");
    logger.error("请求失败，已达到最大重试次数");
    logger.info("任务执行完成");

    println!("   ✓ 已记录 {} 条日志\n", logger.count());

    // 3. 从内存中获取日志
    println!("3. 从内存中获取日志:");
    let logs = logger.get_logs();
    for (i, log) in logs.iter().enumerate() {
        println!("   [{}] {:?} - {}", i + 1, log.level, log.message);
    }
    println!();

    // 4. 打印所有日志（带颜色）
    println!("4. 打印所有日志（带颜色）:");
    logger.print_all();
    println!();

    // 5. 从数据库查询日志
    println!("5. 从数据库查询日志:");
    
    // 5.1 查询所有日志
    match db::get_all_logs() {
        Ok(logs) => {
            println!("   - 数据库中共有 {} 条日志", logs.len());
        }
        Err(e) => eprintln!("   ✗ 查询失败: {}", e),
    }

    // 5.2 查询当前任务的日志
    match db::get_logs_by_uuid(&task_id) {
        Ok(logs) => {
            println!("   - 当前任务有 {} 条日志", logs.len());
            for log in logs.iter().take(3) {
                println!("     • [{}] {} - {}", log.log_level, log.timestamp, log.message);
            }
        }
        Err(e) => eprintln!("   ✗ 查询失败: {}", e),
    }

    // 5.3 查询错误级别的日志
    match db::get_logs_by_level("ERROR") {
        Ok(logs) => {
            println!("   - ERROR 级别日志有 {} 条", logs.len());
        }
        Err(e) => eprintln!("   ✗ 查询失败: {}", e),
    }

    // 5.4 获取日志总数
    match db::get_log_count() {
        Ok(count) => {
            println!("   - 日志总数: {}\n", count);
        }
        Err(e) => eprintln!("   ✗ 查询失败: {}\n", e),
    }

    // 6. 演示配置数据库
    println!("6. 演示配置数据库（带密码保护）:");
    
    // 保存配置
    let _ = db::save_config("app_version", "1.0.0");
    let _ = db::save_config("user_name", "测试用户");
    println!("   ✓ 已保存配置");

    // 读取配置
    match db::load_config("app_version") {
        Ok(Some(value)) => println!("   - app_version: {}", value),
        Ok(None) => println!("   - app_version: 未设置"),
        Err(e) => eprintln!("   ✗ 读取失败: {}", e),
    }

    match db::load_config("user_name") {
        Ok(Some(value)) => println!("   - user_name: {}", value),
        Ok(None) => println!("   - user_name: 未设置"),
        Err(e) => eprintln!("   ✗ 读取失败: {}", e),
    }

    println!("\n=== 演示完成 ===");
    println!("\n提示：");
    println!("- 配置数据库位于用户数据目录，带密码保护");
    println!("- 日志数据库位于软件运行目录，无密码");
    println!("- 每次调用 Logger 的方法都会自动保存到数据库");
}
