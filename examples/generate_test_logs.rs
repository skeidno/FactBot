/// 生成测试日志数据
/// 用于测试日志查看页面的筛选功能

use fact_bot::common::http_task::Logger;
use uuid::Uuid;

fn main() {
    println!("=== 生成测试日志数据 ===\n");

    // 初始化数据库
    let _ = fact_bot::db::initialize_databases();

    // 任务1: HTTP请求任务
    println!("1. 生成 HTTP请求 任务日志...");
    for i in 1..=3 {
        let task_id = Uuid::new_v4().to_string();
        let logger = Logger::new("HTTP请求".to_string(), task_id.clone());
        
        logger.info(&format!("开始执行第 {} 次请求", i));
        logger.debug("正在解析 URL");
        logger.debug("正在建立连接");
        logger.info("连接成功");
        logger.warn("检测到网络延迟 200ms");
        logger.info("正在发送请求数据");
        
        if i % 2 == 0 {
            logger.error("请求超时，连接被重置");
            logger.info("准备重试...");
        } else {
            logger.info("请求成功，状态码 200");
            logger.debug("正在解析响应数据");
            logger.info("任务完成");
        }
    }
    println!("   ✓ 已生成 3 个 HTTP请求 任务");

    // 任务2: 数据库操作
    println!("2. 生成 数据库操作 任务日志...");
    for i in 1..=2 {
        let task_id = Uuid::new_v4().to_string();
        let logger = Logger::new("数据库操作".to_string(), task_id.clone());
        
        logger.info(&format!("开始数据库操作 #{}", i));
        logger.debug("正在连接数据库");
        logger.info("数据库连接成功");
        logger.debug("执行 SQL 查询");
        logger.info("查询返回 150 条记录");
        logger.debug("正在处理数据");
        logger.info("数据处理完成");
        logger.info("关闭数据库连接");
    }
    println!("   ✓ 已生成 2 个 数据库操作 任务");

    // 任务3: 文件处理
    println!("3. 生成 文件处理 任务日志...");
    for i in 1..=2 {
        let task_id = Uuid::new_v4().to_string();
        let logger = Logger::new("文件处理".to_string(), task_id.clone());
        
        logger.info(&format!("开始处理文件 file_{}.txt", i));
        logger.debug("正在读取文件");
        logger.info("文件大小: 2.5 MB");
        logger.debug("正在解析内容");
        
        if i == 2 {
            logger.warn("文件编码不是 UTF-8，尝试自动检测");
            logger.info("检测到编码: GBK");
        }
        
        logger.info("文件处理完成");
    }
    println!("   ✓ 已生成 2 个 文件处理 任务");

    // 任务4: 航司查询
    println!("4. 生成 航司查询 任务日志...");
    for i in 1..=4 {
        let task_id = Uuid::new_v4().to_string();
        let logger = Logger::new("航司查询".to_string(), task_id.clone());
        
        logger.info(&format!("查询航班信息 - 查询 #{}", i));
        logger.debug("正在构建查询参数");
        logger.info("出发地: 北京, 目的地: 上海");
        logger.debug("正在发送查询请求");
        
        if i == 3 {
            logger.error("航司 API 返回错误: 429 Too Many Requests");
            logger.warn("触发限流，等待 5 秒后重试");
        } else {
            logger.info("查询成功，找到 12 个航班");
            logger.debug("正在解析航班数据");
            logger.info("数据解析完成");
        }
    }
    println!("   ✓ 已生成 4 个 航司查询 任务");

    // 任务5: 系统监控
    println!("5. 生成 系统监控 任务日志...");
    let task_id = Uuid::new_v4().to_string();
    let logger = Logger::new("系统监控".to_string(), task_id.clone());
    
    logger.info("系统监控启动");
    logger.debug("正在收集系统信息");
    logger.info("CPU 使用率: 45%");
    logger.info("内存使用率: 62%");
    logger.warn("磁盘空间不足，剩余 5GB");
    logger.info("网络状态: 正常");
    logger.debug("监控数据已记录");
    println!("   ✓ 已生成 1 个 系统监控 任务");

    println!("\n=== 测试数据生成完成 ===");
    println!("共生成 5 种任务类型，12 个任务实例");
    println!("现在可以启动应用查看日志页面了！");
}
