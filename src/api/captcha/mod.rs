use axum::{http::StatusCode, routing::post, Json, Router};
use base64::Engine;
use serde::{Deserialize, Serialize};

/// 验证码识别路由
pub fn routes() -> Router {
    Router::new().route("/solve", post(solve_captcha))
}

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
        "click" => recognize_click(&image_bytes, reference_bytes.as_deref()).await,
        "slide" => recognize_slide(&image_bytes, reference_bytes.as_deref()).await,
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

    // 创建 OCR 实例
    let ocr = match ddddocr::ddddocr_classification() {
        Ok(ocr) => ocr,
        Err(e) => {
            eprintln!("初始化 OCR 模型失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("初始化 OCR 模型失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    // 进行识别
    match ocr.classification(image_bytes) {
        Ok(result) => {
            println!("OCR 识别成功: {}", result);
            CaptchaSolveResponse {
                success: true,
                message: Some("识别成功".to_string()),
                result: Some(result),
                error: None,
                coordinates: None,
            }
        }
        Err(e) => {
            eprintln!("OCR 识别失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("OCR 识别失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

/// 点选验证码识别
async fn recognize_click(image_bytes: &[u8], reference_bytes: Option<&[u8]>) -> CaptchaSolveResponse {
    println!(
        "开始点选识别，主图大小: {} bytes, 参考图: {}",
        image_bytes.len(),
        if reference_bytes.is_some() { "有" } else { "无" }
    );

    // 创建检测实例
    let det = match ddddocr::ddddocr_detection() {
        Ok(det) => det,
        Err(e) => {
            eprintln!("初始化检测模型失败: {:?}", e);
            return CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("初始化检测模型失败: {:?}", e)),
                coordinates: None,
            };
        }
    };

    // 进行检测
    match det.detection(image_bytes) {
        Ok(boxes) => {
            println!("检测到 {} 个目标", boxes.len());

            // 转换坐标格式：计算每个框的中心点
            let coordinates: Vec<Vec<i32>> = boxes
                .iter()
                .map(|bbox| {
                    let center_x = ((bbox.x1 + bbox.x2) / 2) as i32;
                    let center_y = ((bbox.y1 + bbox.y2) / 2) as i32;
                    vec![center_x, center_y]
                })
                .collect();

            CaptchaSolveResponse {
                success: true,
                message: Some("识别成功".to_string()),
                result: Some(format!("检测到 {} 个目标", coordinates.len())),
                error: None,
                coordinates: Some(coordinates),
            }
        }
        Err(e) => {
            eprintln!("点选识别失败: {:?}", e);
            CaptchaSolveResponse {
                success: false,
                message: None,
                result: None,
                error: Some(format!("点选识别失败: {:?}", e)),
                coordinates: None,
            }
        }
    }
}

/// 滑块验证码识别
async fn recognize_slide(image_bytes: &[u8], reference_bytes: Option<&[u8]>) -> CaptchaSolveResponse {
    println!(
        "开始滑块识别，主图大小: {} bytes, 参考图: {}",
        image_bytes.len(),
        if reference_bytes.is_some() { "有" } else { "无" }
    );

    if reference_bytes.is_none() {
        return CaptchaSolveResponse {
            success: false,
            message: None,
            result: None,
            error: Some("滑块识别需要同时提供背景图和滑块图（参考图）".to_string()),
            coordinates: None,
        };
    }

    // 滑块识别功能暂未实现，等待ddddocr库支持
    let _ = reference_bytes.unwrap(); // 消除未使用警告
    
    CaptchaSolveResponse {
        success: false,
        message: None,
        result: None,
        error: Some("滑块识别功能暂未实现，等待ddddocr库更新".to_string()),
        coordinates: None,
    }
}
