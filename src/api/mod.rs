use axum::Router;
use std::net::SocketAddr;

mod health;
mod airline;
mod captcha;
mod register;

/// 启动 Web API 服务器
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        // 健康检查
        .nest("/health", health::routes())
        // 航司 API 服务
        .nest("/api/airline", airline::routes())
        // 验证码识别服务
        .nest("/api/captcha", captcha::routes())
        // 注册机服务
        .nest("/api/register", register::routes());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Web API 服务器启动在 http://{}", addr);
    println!("📋 可用的 API 路由:");
    println!("   - GET  /health              健康检查");
    println!("   - POST /api/airline/query   航司报价查询");
    println!("   - POST /api/captcha/solve   验证码识别");
    println!("   - POST /api/register/create 注册账号");
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
