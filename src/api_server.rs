use axum::{
    routing::get,
    Router,
    http::StatusCode,
};
use std::net::SocketAddr;

/// 启动 Web API 服务器
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health_check));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Web API 服务器启动在 http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// 健康检查接口
async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
