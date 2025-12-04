use axum::{http::StatusCode, routing::post, Json, Router};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Python 验证码识别服务的地址（运行在 8000 端口）
const API_BASE_URL: &str = "http://localhost:8000";

/// 验证码识别路由
pub fn routes() -> Router {
    Router::new()
        .route("/solve", post(solve_captcha))
        .route("/ddddocr/ocr", post(ddddocr_ocr))
        .route("/ddddocr/det", post(ddddocr_det))
        .route("/ddddocr/slide", post(ddddocr_slide))
        .route("/ddddocr/click", post(ddddocr_click))
        .route("/anticap/ocr", post(anticap_ocr))
        .route("/anticap/slide", post(anticap_slide))
        .route("/anticap/rotate", post(anticap_rotate))
        .route("/anticap/compare", post(anticap_compare))
}

/// 额外的路由（用于兼容直接访问）
pub fn direct_routes() -> Router {
    Router::new()
        .route("/click", post(ddddocr_click))
        .route("/ocr", post(ddddocr_ocr))
        .route("/det", post(ddddocr_det))
        .route("/slide", post(ddddocr_slide))
}

// ============ 通用请求/响应结构 ============

#[derive(Debug, Deserialize)]
struct CaptchaSolveRequest {
    image_base64: String,
    #[serde(default = "default_captcha_type")]
    captcha_type: String,
    #[serde(default)]
    reference_base64: Option<String>,
}

fn default_captcha_type() -> String {
    "ocr".to_string()
}

#[derive(Debug, Serialize)]
struct CaptchaSolveResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordinates: Option<Vec<Vec<i32>>>,
}

// ============ ddddocr 引擎请求结构 ============

#[derive(Debug, Deserialize)]
struct DdddocrOcrRequest {
    image: String,
    #[serde(default = "default_ocr_type")]
    r#type: String,
}

fn default_ocr_type() -> String {
    "normal".to_string()
}

#[derive(Debug, Deserialize)]
struct DdddocrSlideRequest {
    target: String,
    background: String,
}

#[derive(Debug, Deserialize)]
struct DdddocrClickRequest {
    image: String,
    #[serde(default)]
    question: Option<String>,
}

// API 请求结构（用于发送给 Python 服务）
#[derive(Debug, Serialize)]
struct ClickApiRequest {
    image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    question: Option<String>,
}

// ============ AntiCAP 引擎请求结构 ============

#[derive(Debug, Deserialize)]
struct AnticapOcrRequest {
    image: String,
    r#type: String, // ocr | math | detection_icon | detection_text | single_rotate
}

#[derive(Debug, Deserialize)]
struct AnticapSlideRequest {
    target: String,
    background: String,
    mode: String, // match | comparison
}

#[derive(Debug, Deserialize)]
struct AnticapRotateRequest {
    inside: String,
    outside: String,
}

#[derive(Debug, Deserialize)]
struct AnticapCompareRequest {
    image1: String,
    image2: String,
}

/// 验证码识别接口
async fn solve_captcha(
    Json(payload): Json<CaptchaSolveRequest>,
) -> (StatusCode, Json<CaptchaSolveResponse>) {
    println!(
        "收到验证码识别请求，类型: {}, 主图长度: {}, 参考图: {}",
        payload.captcha_type,
        payload.image_base64.len(),
        if payload.reference_base64.is_some() { "有" } else { "无" }
    );

    // 解码 base64 图片
    let image_bytes = match base64::engine::general_purpose::STANDARD.decode(&payload.image_base64)
    {
        Ok(bytes) => bytes,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(CaptchaSolveResponse {
                    success: false,
                    message: None,
                    result: None,
                    error: Some(format!("Base64 解码失败: {}", e)),
                    coordinates: None,
                }),
            );
        }
    };

    // 解码参考图（如果有）
    let reference_bytes = if let Some(ref_base64) = &payload.reference_base64 {
        match base64::engine::general_purpose::STANDARD.decode(ref_base64) {
            Ok(bytes) => Some(bytes),
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(CaptchaSolveResponse {
                        success: false,
                        message: None,
                        result: None,
                        error: Some(format!("参考图 Base64 解码失败: {}", e)),
                        coordinates: None,
                    }),
                );
            }
        }
    } else {
        None
    };

    // 根据类型调用不同的识别方法
    let response = match payload.captcha_type.as_str() {
        "ocr" => recognize_ocr(&image_bytes).await,
        "ocr_old" => recognize_ocr_old(&image_bytes).await,
        "ocr_probability" => recognize_ocr_probability(&image_bytes).await,
        "detection" => recognize_detection(&image_bytes).await,
        "click" => recognize_detection(&image_bytes).await, // click 是 detection 的别名
        "slide_match" => recognize_slide_match(&image_bytes, reference_bytes.as_deref()).await,
        "slide_match_simple" => recognize_slide_match_simple(&image_bytes, reference_bytes.as_deref()).await,
        "slide_comparison" => recognize_slide_comparison(&image_bytes, reference_bytes.as_deref()).await,
        "slide" => recognize_slide_match(&image_bytes, reference_bytes.as_deref()).await, // slide 是 slide_match 的别名
        _ => CaptchaSolveResponse {
            success: false,
            message: None,
            result: None,
            error: Some(format!("不支持的验证码类型: {}", payload.captcha_type)),
            coordinates: None,
        },
    };

    (StatusCode::OK, Json(response))
}

/// OCR 识别
async fn recognize_ocr(image_bytes: &[u8]) -> CaptchaSolveResponse {
    println!("开始 OCR 识别，图片大小: {} bytes", image_bytes.len());

    // 转换为 base64
    let image_base64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);

    // 调用 API
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/ocr", API_BASE_URL))
        .json(&serde_json::json!({
            "image": format!("data:image/png;base64,{}", image_base64),
            "type": "normal"
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("API 请求失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("API 请求失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    // 解析响应
    match response.json::<Value>().await {
        Ok(data) => {
            if data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                let result = data.get("result").and_then(|v| v.as_str()).unwrap_or("").to_string();
                println!("OCR 识别成功: {}", result);
                CaptchaSolveResponse {
                    success: true,
                    message: Some("识别成功".to_string()),
                    result: Some(result),
                    error: None,
                    coordinates: None,
                }
            } else {
                let error = data.get("detail").and_then(|v| v.as_str()).unwrap_or("未知错误").to_string();
                CaptchaSolveResponse {
                    success: false,
                    message: None,
                    result: None,
                    error: Some(error),
                    coordinates: None,
                }
            }
        }
        Err(e) => {
            eprintln!("解析响应失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("解析响应失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

/// 旧版 OCR 识别
async fn recognize_ocr_old(image_bytes: &[u8]) -> CaptchaSolveResponse {
    println!("开始旧版 OCR 识别，图片大小: {} bytes", image_bytes.len());

    let image_base64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);

    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/ocr", API_BASE_URL))
        .json(&serde_json::json!({
            "image": format!("data:image/png;base64,{}", image_base64),
            "type": "old"
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("API 请求失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("API 请求失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    match response.json::<Value>().await {
        Ok(data) => {
            if data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                let result = data.get("result").and_then(|v| v.as_str()).unwrap_or("").to_string();
                println!("旧版 OCR 识别成功: {}", result);
                CaptchaSolveResponse {
                    success: true,
                    message: Some("识别成功".to_string()),
                    result: Some(result),
                    error: None,
                    coordinates: None,
                }
            } else {
                let error = data.get("detail").and_then(|v| v.as_str()).unwrap_or("未知错误").to_string();
                CaptchaSolveResponse {
                    success: false,
                    message: None,
                    result: None,
                    error: Some(error),
                    coordinates: None,
                }
            }
        }
        Err(e) => {
            eprintln!("解析响应失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("解析响应失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

/// OCR 概率识别（带置信度）
async fn recognize_ocr_probability(image_bytes: &[u8]) -> CaptchaSolveResponse {
    println!("开始概率 OCR 识别，图片大小: {} bytes", image_bytes.len());

    let image_base64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);

    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/ocr", API_BASE_URL))
        .json(&serde_json::json!({
            "image": format!("data:image/png;base64,{}", image_base64),
            "type": "beta"
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("API 请求失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("API 请求失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    match response.json::<Value>().await {
        Ok(data) => {
            if data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                let result = data.get("result").and_then(|v| v.as_str()).unwrap_or("").to_string();
                println!("概率 OCR 识别成功: {}", result);
                CaptchaSolveResponse {
                    success: true,
                    message: Some("识别成功".to_string()),
                    result: Some(result),
                    error: None,
                    coordinates: None,
                }
            } else {
                let error = data.get("detail").and_then(|v| v.as_str()).unwrap_or("未知错误").to_string();
                CaptchaSolveResponse {
                    success: false,
                    message: None,
                    result: None,
                    error: Some(error),
                    coordinates: None,
                }
            }
        }
        Err(e) => {
            eprintln!("解析响应失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("解析响应失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

/// 目标检测（点选验证码）
async fn recognize_detection(image_bytes: &[u8]) -> CaptchaSolveResponse {
    println!("开始目标检测，图片大小: {} bytes", image_bytes.len());

    let image_base64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);

    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/click", API_BASE_URL))
        .json(&serde_json::json!({
            "image": format!("data:image/png;base64,{}", image_base64)
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("API 请求失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("API 请求失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    match response.json::<Value>().await {
        Ok(data) => {
            if data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                let targets = data.get("targets").and_then(|v| v.as_array());
                let coordinates: Vec<Vec<i32>> = if let Some(targets) = targets {
                    targets
                        .iter()
                        .filter_map(|target| {
                            let pos = target.get("position")?;
                            let x = pos.get("x")?.as_i64()? as i32;
                            let y = pos.get("y")?.as_i64()? as i32;
                            Some(vec![x, y])
                        })
                        .collect()
                } else {
                    vec![]
                };

                println!("检测到 {} 个目标", coordinates.len());
                CaptchaSolveResponse {
                    success: true,
                    message: Some("识别成功".to_string()),
                    result: Some(format!("检测到 {} 个目标", coordinates.len())),
                    error: None,
                    coordinates: Some(coordinates),
                }
            } else {
                let error = data.get("detail").and_then(|v| v.as_str()).unwrap_or("未知错误").to_string();
                CaptchaSolveResponse {
                    success: false,
                    message: None,
                    result: None,
                    error: Some(error),
                    coordinates: None,
                }
            }
        }
        Err(e) => {
            eprintln!("解析响应失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("解析响应失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

/// 滑块匹配
async fn recognize_slide_match(image_bytes: &[u8], background_bytes: Option<&[u8]>) -> CaptchaSolveResponse {
    println!("开始滑块匹配，滑块大小: {} bytes, 背景图: {}",
             image_bytes.len(),
             if background_bytes.is_some() { "有" } else { "无" });

    if background_bytes.is_none() {
        return CaptchaSolveResponse {
            success: false,
            message: None,
            result: None,
            error: Some("滑块匹配需要同时提供滑块图和背景图".to_string()),
            coordinates: None,
        };
    }

    let bg_bytes = background_bytes.unwrap();

    // 智能检测：如果主图比背景图大，自动交换
    let (target, background) = if image_bytes.len() > bg_bytes.len() {
        println!("⚠️  检测到图片顺序可能错误，自动交换：主图({} bytes) <-> 背景图({} bytes)",
                 image_bytes.len(), bg_bytes.len());
        (bg_bytes, image_bytes)
    } else {
        (image_bytes, bg_bytes)
    };

    let target_base64 = base64::engine::general_purpose::STANDARD.encode(target);
    let bg_base64 = base64::engine::general_purpose::STANDARD.encode(background);

    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/slide", API_BASE_URL))
        .json(&serde_json::json!({
            "target": format!("data:image/png;base64,{}", target_base64),
            "background": format!("data:image/png;base64,{}", bg_base64)
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("API 请求失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("API 请求失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    match response.json::<Value>().await {
        Ok(data) => {
            if data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                let distance = data.get("distance").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                println!("滑块匹配成功: x={}", distance);
                CaptchaSolveResponse {
                    success: true,
                    message: Some("识别成功".to_string()),
                    result: Some(format!("滑块位置: x={}, y=0", distance)),
                    error: None,
                    coordinates: Some(vec![vec![distance, 0]]),
                }
            } else {
                let error = data.get("detail").and_then(|v| v.as_str()).unwrap_or("未知错误").to_string();
                CaptchaSolveResponse {
                    success: false,
                    message: None,
                    result: None,
                    error: Some(error),
                    coordinates: None,
                }
            }
        }
        Err(e) => {
            eprintln!("解析响应失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("解析响应失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

/// 简化滑块匹配
async fn recognize_slide_match_simple(image_bytes: &[u8], background_bytes: Option<&[u8]>) -> CaptchaSolveResponse {
    // 简化版本使用相同的 API，因为 API 文档中没有区分
    recognize_slide_match(image_bytes, background_bytes).await
}

/// 滑块比对
async fn recognize_slide_comparison(image_bytes: &[u8], background_bytes: Option<&[u8]>) -> CaptchaSolveResponse {
    println!("开始滑块比对，缺口图大小: {} bytes, 完整图: {}",
             image_bytes.len(),
             if background_bytes.is_some() { "有" } else { "无" });

    if background_bytes.is_none() {
        return CaptchaSolveResponse {
            success: false,
            message: None,
            result: None,
            error: Some("滑块比对需要同时提供缺口图和完整背景图".to_string()),
            coordinates: None,
        };
    }

    let bg_bytes = background_bytes.unwrap();

    let target_base64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);
    let bg_base64 = base64::engine::general_purpose::STANDARD.encode(bg_bytes);

    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/anticap/slide", API_BASE_URL))
        .json(&serde_json::json!({
            "target": format!("data:image/png;base64,{}", target_base64),
            "background": format!("data:image/png;base64,{}", bg_base64),
            "mode": "comparison"
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("API 请求失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("API 请求失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    match response.json::<Value>().await {
        Ok(data) => {
            if data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                let result = data.get("result").and_then(|v| v.get("target")).and_then(|v| v.as_array());
                if let Some(coords) = result {
                    let x = coords.first().and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let y = coords.get(1).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    println!("滑块比对成功: x={}, y={}", x, y);
                    CaptchaSolveResponse {
                        success: true,
                        message: Some("识别成功".to_string()),
                        result: Some(format!("滑块位置: x={}, y={}", x, y)),
                        error: None,
                        coordinates: Some(vec![vec![x, y]]),
                    }
                } else {
                    CaptchaSolveResponse {
                        success: false,
                        message: None,
                        result: None,
                        error: Some("无法解析响应数据".to_string()),
                        coordinates: None,
                    }
                }
            } else {
                let error = data.get("detail").and_then(|v| v.as_str()).unwrap_or("未知错误").to_string();
                CaptchaSolveResponse {
                    success: false,
                    message: None,
                    result: None,
                    error: Some(error),
                    coordinates: None,
                }
            }
        }
        Err(e) => {
            eprintln!("解析响应失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("解析响应失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

// ============ 新增 API 端点 ============

/// ddddocr - 普通验证码识别
async fn ddddocr_ocr(Json(payload): Json<DdddocrOcrRequest>) -> (StatusCode, Json<Value>) {
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/ocr", API_BASE_URL))
        .json(&serde_json::json!({
            "image": payload.image,
            "type": payload.r#type
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}

/// ddddocr - 目标检测
async fn ddddocr_det(Json(payload): Json<serde_json::Value>) -> (StatusCode, Json<Value>) {
    let image = match payload.get("image").and_then(|v| v.as_str()) {
        Some(img) => img,
        None => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"detail": "缺少 image 参数"}))),
    };

    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/det", API_BASE_URL))
        .json(&serde_json::json!({
            "image": image
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}

/// ddddocr - 滑块验证码
async fn ddddocr_slide(Json(payload): Json<DdddocrSlideRequest>) -> (StatusCode, Json<Value>) {
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/slide", API_BASE_URL))
        .json(&serde_json::json!({
            "target": payload.target,
            "background": payload.background
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}

/// ddddocr - 点选验证码
async fn ddddocr_click(Json(payload): Json<DdddocrClickRequest>) -> (StatusCode, Json<Value>) {
    let api_request = ClickApiRequest {
        image: payload.image,
        question: payload.question,
    };
    
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/click", API_BASE_URL))
        .json(&api_request)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}

/// AntiCAP - 通用 OCR 识别
/// 支持类型: ocr, math, detection_icon, detection_text, single_rotate
async fn anticap_ocr(Json(payload): Json<AnticapOcrRequest>) -> (StatusCode, Json<Value>) {
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/anticap/ocr", API_BASE_URL))
        .json(&serde_json::json!({
            "image": payload.image,
            "type": payload.r#type
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}

/// AntiCAP - 滑块验证码
/// 支持模式: match (匹配), comparison (比对)
async fn anticap_slide(Json(payload): Json<AnticapSlideRequest>) -> (StatusCode, Json<Value>) {
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/anticap/slide", API_BASE_URL))
        .json(&serde_json::json!({
            "target": payload.target,
            "background": payload.background,
            "mode": payload.mode
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}

/// AntiCAP - 双图旋转验证码
/// 返回内外圈的旋转角度
async fn anticap_rotate(Json(payload): Json<AnticapRotateRequest>) -> (StatusCode, Json<Value>) {
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/anticap/rotate", API_BASE_URL))
        .json(&serde_json::json!({
            "inside": payload.inside,
            "outside": payload.outside
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}

/// AntiCAP - 图片相似度对比
/// 返回两张图片的相似度分数
async fn anticap_compare(Json(payload): Json<AnticapCompareRequest>) -> (StatusCode, Json<Value>) {
    let client = reqwest::Client::new();
    let response = match client
        .post(format!("{}/api/anticap/compare", API_BASE_URL))
        .json(&serde_json::json!({
            "image1": payload.image1,
            "image2": payload.image2
        }))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("API 请求失败: {:?}", e)}))),
    };

    let status = response.status();
    match response.json::<Value>().await {
        Ok(data) => (StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK), Json(data)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"detail": format!("解析响应失败: {:?}", e)}))),
    }
}
