use axum::Router;
use std::net::SocketAddr;

mod health;
mod airline;
pub mod captcha;
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
        // 验证码识别服务（直接访问，兼容浏览器）
        .nest("/api", captcha::direct_routes())
        // 注册机服务
        .nest("/api/register", register::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Web API 服务器启动在 http://{}", addr);
    println!("📋 可用的 API 路由:");
    println!("   - GET  /health                    健康检查");
    println!("   - POST /api/airline/query         航司报价查询");
    println!("   - POST /api/captcha/solve         验证码识别（7种类型）");
    println!("   - POST /api/register/create       注册账号");
    println!();
    println!("📖 验证码识别支持的类型:");
    println!("   • ocr                - 英数验证码");
    println!("   • ocr_old            - 旧版 OCR 模型");
    println!("   • ocr_probability    - 概率 OCR（带置信度）");
    println!("   • detection          - 目标检测（点选）");
    println!("   • slide_match        - 滑块匹配");
    println!("   • slide_match_simple - 简化滑块匹配");
    println!("   • slide_comparison   - 滑块比对");
    println!();
    println!("📚 详细文档: docs/验证码识别功能说明.md | docs/API使用教程.md");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
