use axum::{
    routing::get,
    Router,
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

/// 健康检查路由
pub fn routes() -> Router {
    Router::new()
        .route("/", get(health_check))
}

/// 健康检查接口
async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "FactBot API",
            "version": env!("CARGO_PKG_VERSION"),
        }))
    )
}
