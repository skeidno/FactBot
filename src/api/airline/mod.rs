use axum::{
    routing::post,
    Router,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 航司 API 路由
pub fn routes() -> Router {
    Router::new()
        .route("/query", post(query_airline))
}

#[derive(Debug, Deserialize)]
struct AirlineQueryRequest {
    airline_code: String,
    from: String,
    to: String,
    date: String,
}

#[derive(Debug, Serialize)]
struct AirlineQueryResponse {
    success: bool,
    message: String,
    data: Option<Value>,
}

/// 航司报价查询接口
async fn query_airline(
    Json(payload): Json<AirlineQueryRequest>,
) -> (StatusCode, Json<AirlineQueryResponse>) {
    println!("收到航司查询请求: {:?}", payload);
    
    (
        StatusCode::OK,
        Json(AirlineQueryResponse {
            success: true,
            message: "查询成功".to_string(),
            data: Some(serde_json::json!({
                "airline": payload.airline_code,
                "route": format!("{} -> {}", payload.from, payload.to),
                "date": payload.date,
                "price": 1000,
            })),
        })
    )
}
