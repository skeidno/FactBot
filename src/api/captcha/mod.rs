use axum::{
    routing::post,
    Router,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

/// 验证码识别路由
pub fn routes() -> Router {
    Router::new()
        .route("/solve", post(solve_captcha))
}

#[derive(Debug, Deserialize)]
struct CaptchaSolveRequest {
    #[allow(dead_code)]
    image_base64: String,
    #[serde(default)]
    captcha_type: String,
}

#[derive(Debug, Serialize)]
struct CaptchaSolveResponse {
    success: bool,
    message: String,
    result: Option<String>,
}

/// 验证码识别接口
async fn solve_captcha(
    Json(payload): Json<CaptchaSolveRequest>,
) -> (StatusCode, Json<CaptchaSolveResponse>) {
    // TODO: 实现实际的验证码识别逻辑
    println!("收到验证码识别请求，类型: {}", payload.captcha_type);
    
    (
        StatusCode::OK,
        Json(CaptchaSolveResponse {
            success: true,
            message: "识别成功".to_string(),
            result: Some("ABC123".to_string()),
        })
    )
}
