use fact_bot::common::Result;
use fact_bot::common::http_task::{LoggerLevel, TaskConfig, TaskManager};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== TaskManager 日志演示 ===\n");

    // 创建任务管理器
    let config = TaskConfig::default();
    let mut manager = TaskManager::new("网页抓取".to_string(), config)?;

    println!("1. 任务管理器已创建，日志已记录但不会自动打印\n");

    // 执行一些操作（会记录日志）
    manager.info("开始执行任务");
    manager.debug("检查代理配置");
    manager.info("准备发起请求");

    // 模拟一些警告
    manager.warn("代理响应较慢");
    manager.warn("重试次数较多");

    // 模拟错误
    manager.error("连接超时");

    println!("2. 查看日志统计:");
    println!("   总日志数: {}", manager.logger().count());
    println!("   错误数: {}", manager.logger().get_logs_by_level(LoggerLevel::Error).len());
    println!("   警告数: {}", manager.logger().get_logs_by_level(LoggerLevel::Warn).len());
    println!();

    println!("3. 只打印错误日志:");
    manager.logger().print_level(LoggerLevel::Error);
    println!();

    println!("4. 只打印警告日志:");
    manager.logger().print_level(LoggerLevel::Warn);
    println!();

    println!("5. 打印所有日志（带颜色）:");
    manager.logger().print_all();
    println!();

    println!("6. 获取日志详情:");
    let logs = manager.logger().get_logs();
    for (i, log) in logs.iter().enumerate() {
        println!("   {}. [{:?}] {} - {}", i + 1, log.level, log.timestamp, log.message);
    }
    println!();

    println!("7. 清空日志:");
    manager.logger().clear();
    println!("   清空后日志数: {}", manager.logger().count());

    println!("\n=== 演示完成 ===");
    println!("\n说明:");
    println!("- 日志不会自动打印，完全由你控制");
    println!("- 可以随时查看、过滤、打印日志");
    println!("- 日志带颜色：Info=白色, Warn=黄色, Error=红色, Debug=蓝色");
    println!("- 格式：[任务名称] [任务类型] [任务id] [时间] : 日志内容");

    Ok(())
}
