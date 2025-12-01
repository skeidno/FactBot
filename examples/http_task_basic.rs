use fact_bot::common::Result;
use fact_bot::common::http_task::{IPMode, ProxyConfig, TaskConfig, TaskManager};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== HTTP Task Manager 基本使用示例 ===\n");
    
    // 创建配置
    let mut config = TaskConfig::default();
    config.enable_cookie_management = true;
    config.intercept_domains = vec!["blocked.com".to_string()];
    
    // 配置 IP 池
    let proxies = vec![
        ProxyConfig {
            host: "proxy1.example.com".to_string(),
            port: 8080,
            username: None,
            password: None,
        },
        ProxyConfig {
            host: "proxy2.example.com".to_string(),
            port: 8080,
            username: None,
            password: None,
        },
    ];
    config.ip_mode = IPMode::Pool(proxies);
    
    // 创建 TaskManager（需要提供任务名称）
    let mut task_manager = TaskManager::new("示例任务".to_string(), config)?;
    
    println!("任务名称: {}", task_manager.task_name());
    println!("任务 ID: {}", task_manager.task_id());
    println!("当前代理: {:?}\n", task_manager.get_ip());
    
    // 切换代理
    task_manager.switch_next()?;
    println!("切换后的代理: {:?}\n", task_manager.get_ip());
    
    // 发起请求（注意：这是 mock 实现）
    println!("发起 GET 请求...");
    match task_manager.get("https://example.com").await {
        Ok(response) => println!("请求成功: {:?}\n", response),
        Err(e) => println!("请求失败: {}\n", e),
    }
    
    // 尝试访问被拦截的域名
    println!("尝试访问被拦截的域名...");
    match task_manager.get("https://blocked.com/path").await {
        Ok(_) => println!("不应该到达这里"),
        Err(e) => println!("请求被拦截（符合预期）: {}\n", e),
    }
    
    println!("=== 示例完成 ===");
    Ok(())
}
