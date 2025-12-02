use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use base64::Engine;

#[derive(Clone, PartialEq)]
enum CaptchaType {
    Ocr,
    Click,
    Slide,
}

#[derive(Clone, PartialEq)]
enum RecognitionMethod {
    Ddddocr,
    Custom,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct RecognitionResult {
    success: bool,
    result: String,
    #[serde(default)]
    coordinates: Option<Vec<(i32, i32)>>,
}

#[component]
pub fn Captcha() -> Element {
    let mut selected_type = use_signal(|| CaptchaType::Ocr);
    let mut selected_method = use_signal(|| RecognitionMethod::Ddddocr);
    let mut image_base64 = use_signal(String::new);
    let mut reference_base64 = use_signal(String::new);
    let mut result = use_signal(|| None::<RecognitionResult>);
    let mut is_processing = use_signal(|| false);

    // 处理主图上传
    let handle_file_upload = move |evt: Event<FormData>| {
        spawn(async move {
            let file_engine = evt.files();
            if let Some(file_data) = file_engine.first() {
                println!("主图已选择: {}", file_data.name());

                match file_data.read_bytes().await {
                    Ok(file_content) => {
                        let base64_str = base64::engine::general_purpose::STANDARD.encode(&file_content);
                        println!("主图 Base64 长度: {}", base64_str.len());
                        image_base64.set(base64_str);
                        result.set(None);
                    }
                    Err(e) => {
                        println!("读取主图失败: {:?}", e);
                    }
                }
            }
        });
    };

    // 处理参考图上传
    let handle_reference_upload = move |evt: Event<FormData>| {
        spawn(async move {
            let file_engine = evt.files();
            if let Some(file_data) = file_engine.first() {
                println!("参考图已选择: {}", file_data.name());

                match file_data.read_bytes().await {
                    Ok(file_content) => {
                        let base64_str = base64::engine::general_purpose::STANDARD.encode(&file_content);
                        println!("参考图 Base64 长度: {}", base64_str.len());
                        reference_base64.set(base64_str);
                    }
                    Err(e) => {
                        println!("读取参考图失败: {:?}", e);
                    }
                }
            }
        });
    };

    rsx! {
        div {
            style: "height:100%; overflow-y:auto; padding:24px 16px 24px 0;",

            div {
                style: "display:flex; flex-direction:column; gap:28px; max-width:1400px; margin:0 auto;",

                // 标题区
                section {
                    style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:20px; padding:28px 32px; border:1px solid #fbbf24; box-shadow:0 4px 20px rgba(251, 191, 36, 0.1);",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#78350f; letter-spacing:-0.02em;",
                        "验证码识别服务"
                    }
                    p {
                        style: "color:#92400e; font-size:15px; margin:0; line-height:1.6;",
                        "支持多种验证码类型识别，包括数英 OCR、点选、滑块等，使用 DDDDOCR 等多种识别引擎"
                    }
                }

                // 配置和上传区
                section {
                    style: "background:white; border-radius:18px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 6px 24px rgba(15,23,42,0.06);",

                    div {
                        style: "display:grid; grid-template-columns:repeat(2, 1fr); gap:20px; margin-bottom:24px;",

                        div {
                            label {
                                style: "display:block; margin-bottom:8px; font-size:14px; font-weight:600; color:#374151;",
                                "验证码类型"
                            }
                            select {
                                value: match selected_type() {
                                    CaptchaType::Ocr => "ocr",
                                    CaptchaType::Click => "click",
                                    CaptchaType::Slide => "slide",
                                },
                                onchange: move |evt| {
                                    selected_type.set(match evt.value().as_str() {
                                        "click" => CaptchaType::Click,
                                        "slide" => CaptchaType::Slide,
                                        _ => CaptchaType::Ocr,
                                    });
                                },
                                style: "width:100%; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:white; color:#111827; cursor:pointer;",
                                option { value: "ocr", "🔤 数英 OCR" }
                                option { value: "click", "👆 点选验证码" }
                                option { value: "slide", "↔️ 滑块验证码" }
                            }
                        }

                        div {
                            label {
                                style: "display:block; margin-bottom:8px; font-size:14px; font-weight:600; color:#374151;",
                                "识别方式"
                            }
                            select {
                                value: match selected_method() {
                                    RecognitionMethod::Ddddocr => "ddddocr",
                                    RecognitionMethod::Custom => "custom",
                                },
                                onchange: move |evt| {
                                    selected_method.set(match evt.value().as_str() {
                                        "custom" => RecognitionMethod::Custom,
                                        _ => RecognitionMethod::Ddddocr,
                                    });
                                },
                                style: "width:100%; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:white; color:#111827; cursor:pointer;",
                                option { value: "ddddocr", "🤖 DDDDOCR" }
                                option { value: "custom", "⚙️ 自定义引擎" }
                            }
                        }
                    }

                    // 主图上传区
                    div {
                        h3 {
                            style: "margin:0 0 12px 0; font-size:16px; font-weight:600; color:#374151;",
                            "主图（验证码图片）"
                        }
                        div {
                            style: "border:2px dashed #d1d5db; border-radius:12px; padding:24px; text-align:center; background:#f9fafb;",

                        if image_base64().is_empty() {
                            label {
                                r#for: "file-upload",
                                style: "display:block; padding:40px 20px; cursor:pointer;",
                                p {
                                    style: "margin:0 0 12px 0; font-size:48px;",
                                    "📷"
                                }
                                p {
                                    style: "margin:0 0 8px 0; font-size:16px; font-weight:600; color:#374151;",
                                    "点击上传验证码图片"
                                }
                                p {
                                    style: "margin:0; font-size:14px; color:#6b7280;",
                                    "支持 JPG、PNG、GIF 等格式"
                                }
                            }
                            input {
                                r#type: "file",
                                id: "file-upload",
                                accept: "image/png,image/jpeg,image/gif",
                                multiple: false,
                                style: "display:none;",
                                onchange: handle_file_upload,
                            }
                        } else {
                            div {
                                // 图片预览（带坐标标注）
                                div {
                                    style: "position:relative; display:inline-block; max-width:100%;",
                                    img {
                                        id: "captcha-image",
                                        src: "data:image/png;base64,{image_base64()}",
                                        style: "max-width:100%; max-height:400px; border-radius:8px; box-shadow:0 4px 12px rgba(0,0,0,0.1); display:block;",
                                    }
                                    // 点选坐标标注覆盖层
                                    if let Some(res) = result() {
                                        if res.success {
                                            if let Some(coords) = &res.coordinates {
                                                if !coords.is_empty() {
                                                    for (idx, (x, y)) in coords.iter().enumerate() {
                                                        div {
                                                            key: "{idx}",
                                                            style: "position:absolute; left:{x}px; top:{y}px; width:40px; height:40px; border:3px solid #ef4444; border-radius:50%; background:rgba(239, 68, 68, 0.3); transform:translate(-50%, -50%); pointer-events:none; z-index:10; box-shadow:0 2px 8px rgba(239, 68, 68, 0.5);",
                                                            div {
                                                                style: "position:absolute; top:50%; left:50%; transform:translate(-50%, -50%); color:white; font-weight:700; font-size:16px; text-shadow:0 1px 3px rgba(0,0,0,0.5);",
                                                                "{idx + 1}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Base64 预览
                                div {
                                    style: "margin-top:12px; padding:12px; background:#f3f4f6; border-radius:8px; text-align:left;",
                                    p {
                                        style: "margin:0 0 4px 0; font-size:13px; font-weight:600; color:#374151;",
                                        "Base64 编码（前100字符）:"
                                    }
                                    p {
                                        style: "margin:0; font-size:12px; color:#6b7280; font-family:monospace; word-break:break-all;",
                                        {format!("{}...", image_base64().chars().take(100).collect::<String>())}
                                    }
                                }

                                // 重新上传按钮
                                button {
                                    onclick: move |_| {
                                        image_base64.set(String::new());
                                        reference_base64.set(String::new());
                                        result.set(None);
                                    },
                                    style: "margin-top:12px; padding:8px 16px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#374151; font-weight:600; cursor:pointer;",
                                    "🔄 重新上传"
                                }
                            }
                        }
                        }
                    }

                    // 参考图上传区（点选和滑块需要）
                    if matches!(selected_type(), CaptchaType::Click | CaptchaType::Slide) {
                        div {
                            style: "margin-top:20px;",
                            h3 {
                                style: "margin:0 0 8px 0; font-size:16px; font-weight:600; color:#374151;",
                                "参考图（问题图）",
                                span {
                                    style: "margin-left:8px; font-size:13px; font-weight:400; color:#6b7280;",
                                    "可选"
                                }
                            }
                            p {
                                style: "margin:0 0 12px 0; font-size:13px; color:#6b7280;",
                                if matches!(selected_type(), CaptchaType::Click) {
                                    "点选验证码的问题图，例如请点击所有的猫"
                                } else {
                                    "滑块验证码的缺口图或滑块图"
                                }
                            }

                            div {
                                style: "border:2px dashed #d1d5db; border-radius:12px; padding:16px; text-align:center; background:#f9fafb;",

                                if reference_base64().is_empty() {
                                    label {
                                        r#for: "reference_upload",
                                        style: "display:block; padding:20px; cursor:pointer;",
                                        p {
                                            style: "margin:0 0 8px 0; font-size:32px;",
                                            "🖼️"
                                        }
                                        p {
                                            style: "margin:0; font-size:14px; font-weight:500; color:#374151;",
                                            "点击上传参考图(可选)"
                                        }
                                    }
                                    input {
                                        r#type: "file",
                                        id: "reference_upload",
                                        accept: "image/png,image/jpeg,image/gif",
                                        multiple: false,
                                        style: "display:none;",
                                        onchange: handle_reference_upload,
                                    }
                                } else {
                                    div {
                                        img {
                                            src: "data:image/png;base64,{reference_base64()}",
                                            style: "max-width:100%; max-height:200px; border-radius:8px; box-shadow:0 2px 8px rgba(0,0,0,0.1);",
                                        }
                                        button {
                                            onclick: move |_| {
                                                reference_base64.set(String::new());
                                            },
                                            style: "margin-top:8px; padding:6px 12px; border-radius:6px; border:1px solid #d1d5db; background:white; color:#374151; font-size:13px; cursor:pointer;",
                                            "🗑️ 移除参考图"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 识别按钮
                    button {
                        disabled: image_base64().is_empty() || is_processing(),
                        onclick: move |_| {
                            let img_base64 = image_base64();
                            let ref_base64 = reference_base64();
                            let captcha_type = match selected_type() {
                                CaptchaType::Ocr => "ocr",
                                CaptchaType::Click => "click",
                                CaptchaType::Slide => "slide",
                            };

                            spawn(async move {
                                is_processing.set(true);

                                let client = reqwest::Client::new();

                                // 构建请求体
                                let mut request_body = serde_json::json!({
                                    "image_base64": img_base64,
                                    "captcha_type": captcha_type,
                                });

                                // 如果有参考图，添加到请求中
                                if !ref_base64.is_empty() {
                                    request_body["reference_base64"] = serde_json::Value::String(ref_base64);
                                }

                                let response = client
                                    .post("http://localhost:8080/api/captcha/solve")
                                    .json(&request_body)
                                    .send()
                                    .await;

                                match response {
                                    Ok(resp) => {
                                        if let Ok(data) = resp.json::<serde_json::Value>().await {
                                            println!("收到响应: {:?}", data);

                                            let success = data.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
                                            let res_text = data.get("result")
                                                .and_then(|r| r.as_str())
                                                .unwrap_or(
                                                    data.get("error").and_then(|e| e.as_str()).unwrap_or("未知错误")
                                                )
                                                .to_string();

                                            let coords = data.get("coordinates")
                                                .and_then(|c| c.as_array())
                                                .map(|arr| {
                                                    arr.iter()
                                                        .filter_map(|v| {
                                                            v.as_array().and_then(|pair| {
                                                                if pair.len() == 2 {
                                                                    Some((
                                                                        pair[0].as_i64().unwrap_or(0) as i32,
                                                                        pair[1].as_i64().unwrap_or(0) as i32
                                                                    ))
                                                                } else {
                                                                    None
                                                                }
                                                            })
                                                        })
                                                        .collect::<Vec<_>>()
                                                });

                                            println!("识别结果: success={}, result={}, coords={:?}", success, res_text, coords);

                                            result.set(Some(RecognitionResult {
                                                success,
                                                result: res_text,
                                                coordinates: coords,
                                            }));
                                        }
                                    }
                                    Err(e) => {
                                        println!("识别请求失败: {}", e);
                                        result.set(Some(RecognitionResult {
                                            success: false,
                                            result: format!("请求失败: {}", e),
                                            coordinates: None,
                                        }));
                                    }
                                }

                                is_processing.set(false);
                            });
                        },
                        style: if image_base64().is_empty() || is_processing() {
                            "width:100%; margin-top:20px; padding:14px 24px; border-radius:12px; border:none; background:#d1d5db; color:#9ca3af; font-weight:600; cursor:not-allowed; font-size:16px;"
                        } else {
                            "width:100%; margin-top:20px; padding:14px 24px; border-radius:12px; border:none; background:linear-gradient(120deg,#f59e0b,#d97706); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(245, 158, 11, 0.3); font-size:16px; transition:all 0.2s;"
                        },
                        if is_processing() {
                            "🔄 识别中..."
                        } else {
                            "🚀 开始识别"
                        }
                    }
                }

                // 结果显示区
                if let Some(res) = result() {
                    section {
                        style: "background:white; border-radius:18px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 6px 24px rgba(15,23,42,0.06);",

                        h2 {
                            style: "margin:0 0 20px 0; font-size:20px; font-weight:700; color:#111827;",
                            "识别结果"
                        }

                        div {
                            style: if res.success {
                                "background:linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%); border-radius:12px; padding:20px; border:1px solid #10b981;"
                            } else {
                                "background:linear-gradient(135deg, #fee2e2 0%, #fecaca 100%); border-radius:12px; padding:20px; border:1px solid #ef4444;"
                            },

                            p {
                                style: if res.success {
                                    "margin:0 0 8px 0; font-size:14px; font-weight:600; color:#065f46;"
                                } else {
                                    "margin:0 0 8px 0; font-size:14px; font-weight:600; color:#991b1b;"
                                },
                                if res.success { "✓ 识别成功" } else { "✗ 识别失败" }
                            }
                            p {
                                style: if res.success {
                                    "margin:0; font-size:24px; color:#047857; font-weight:700; font-family:monospace; word-break:break-all;"
                                } else {
                                    "margin:0; font-size:16px; color:#991b1b; font-weight:600; word-break:break-all;"
                                },
                                "{res.result}"
                            }

                            if let Some(coords) = &res.coordinates {
                                div {
                                    style: "margin-top:12px; padding-top:12px; border-top:1px solid rgba(16, 185, 129, 0.3);",
                                    p {
                                        style: "margin:0 0 8px 0; font-size:13px; font-weight:600; color:#065f46;",
                                        "点选坐标:"
                                    }
                                    for (idx, (x, y)) in coords.iter().enumerate() {
                                        span {
                                            key: "{idx}",
                                            style: "display:inline-block; margin:4px 8px 4px 0; padding:4px 12px; background:rgba(16, 185, 129, 0.2); border-radius:6px; font-size:13px; color:#065f46; font-family:monospace;",
                                            "{idx + 1}: ({x}, {y})"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}