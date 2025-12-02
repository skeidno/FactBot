use axum::{
    routing::post,
    Router,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

/// 注册机路由
pub fn routes() -> Router {
    Router::new()
        .route("/create", post(create_account))
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    platform: String,
    #[allow(dead_code)]
    email: Option<String>,
    #[allow(dead_code)]
    phone: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    use_proxy: bool,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    success: bool,
    message: String,
    account: Option<AccountInfo>,
}

#[derive(Debug, Serialize)]
struct AccountInfo {
    username: String,
    password: String,
    platform: String,
}

/// 注册账号接口
async fn create_account(
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<RegisterResponse>) {
    println!("收到注册请求，平台: {}", payload.platform);
    
    (
        StatusCode::OK,
        Json(RegisterResponse {
            success: true,
            message: "注册成功".to_string(),
            account: Some(AccountInfo {
                username: "test_user".to_string(),
                password: "test_pass".to_string(),
                platform: payload.platform,
            }),
        })
    )
}
