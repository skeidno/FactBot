use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use base64::Engine;
use crate::db::load_config;

#[derive(Clone, PartialEq)]
enum CaptchaType {
    Ocr,
    Click,
    Slide,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct CaptchaModule {
    id: usize,
    name: String,
    module_type: String,
    supported_types: Vec<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
struct RecognitionResult {
    success: bool,
    result: String,
    #[serde(default)]
    coordinates: Option<Vec<(i32, i32)>>,
}

#[component]
pub fn Captcha() -> Element {
    let modules = use_signal(|| {
        load_config("captcha_modules")
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<CaptchaModule>>(&json).ok())
            .unwrap_or_else(|| vec![
                CaptchaModule {
                    id: 1,
                    name: "DDDDOCR".to_string(),
                    module_type: "ddddocr".to_string(),
                    supported_types: vec!["ocr".to_string(), "click".to_string(), "slide".to_string()],
                }
            ])
    });

    rsx! {
        div {
            style: "height:100%; overflow-y:auto; padding:24px 16px 24px 0;",

            div {
                style: "display:flex; flex-direction:column; gap:28px; max-width:1400px; margin:0 auto;",

                section {
                    style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:20px; padding:28px 32px; border:1px solid #fbbf24; box-shadow:0 4px 20px rgba(251, 191, 36, 0.1);",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#78350f; letter-spacing:-0.02em;",
                        "验证码识别服务"
                    }
                    p {
                        style: "color:#92400e; font-size:15px; margin:0; line-height:1.6;",
                        "支持多种验证码类型识别，包括英数验证码、点选、滑块等，使用不同的识别引擎模块"
                    }
                }

                ModuleSection { modules }
            }
        }
    }
}

#[component]
fn ModuleSection(modules: Signal<Vec<CaptchaModule>>) -> Element {
    let mut selected_module = use_signal(|| 0usize);

    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:24px;",
                div {
                    h2 {
                        style: "font-size:22px; font-weight:700; margin:0 0 6px 0; color:#111827; letter-spacing:-0.02em;",
                        "验证码识别模块"
                    }
                    p {
                        style: "margin:0; color:#6b7280; font-size:14px;",
                        "选择不同的识别引擎模块，每个模块支持不同类型的验证码"
                    }
                }
            }

            div {
                style: "display:flex; gap:12px; margin-bottom:24px; flex-wrap:wrap;",
                for (index, module) in modules().iter().enumerate() {
                    {
                        let is_selected = selected_module() == index;
                        let btn_style = if is_selected {
                            "padding:10px 18px; border-radius:10px; background:#f59e0b; color:white; font-weight:600; cursor:pointer; border:1px solid #f59e0b;"
                        } else {
                            "padding:10px 18px; border-radius:10px; background:white; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #d1d5db;"
                        };
                        rsx! {
                            button {
                                key: "{module.id}",
                                style: "{btn_style}",
                                onclick: move |_| selected_module.set(index),
                                "{module.name}"
                            }
                        }
                    }
                }
            }

            if let Some(module) = modules().get(selected_module()) {
                ModuleEditor {
                    module: module.clone(),
                }
            }
        }
    }
}

#[component]
fn ModuleEditor(module: CaptchaModule) -> Element {
    let selected_type = use_signal(|| CaptchaType::Ocr);
    let image_base64 = use_signal(String::new);
    let reference_base64 = use_signal(String::new);
    let result = use_signal(|| None::<RecognitionResult>);
    let is_processing = use_signal(|| false);

    rsx! {
        div {
            style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:16px; padding:24px; border:1px solid #fbbf24;",
            
            div {
                style: "margin-bottom:20px;",
                h3 {
                    style: "margin:0 0 8px 0; font-size:20px; font-weight:700; color:#374151;",
                    "🤖 {module.name} 模块"
                }
                p {
                    style: "margin:0; font-size:14px; color:#6b7280;",
                    "支持的验证码类型: "
                    {module.supported_types.iter().map(|t| match t.as_str() {
                        "ocr" => "英数验证码",
                        "click" => "点选验证码",
                        "slide" => "滑块验证码",
                        _ => t.as_str()
                    }).collect::<Vec<_>>().join("、")}
                }
            }

            RecognizeArea {
                selected_type,
                image_base64,
                reference_base64,
                result,
                is_processing,
                module_id: module.id,
                supported_types: module.supported_types.clone()
            }
        }
    }
}

#[component]
fn RecognizeArea(
    selected_type: Signal<CaptchaType>,
    image_base64: Signal<String>,
    reference_base64: Signal<String>,
    result: Signal<Option<RecognitionResult>>,
    is_processing: Signal<bool>,
    module_id: usize,
    supported_types: Vec<String>
) -> Element {
    let handle_file_upload = move |evt: Event<FormData>| {
        spawn(async move {
            let file_engine = evt.files();
            if let Some(file_data) = file_engine.first() {
                match file_data.read_bytes().await {
                    Ok(file_content) => {
                        let base64_str = base64::engine::general_purpose::STANDARD.encode(&file_content);
                        image_base64.set(base64_str);
                        result.set(None);
                    }
                    Err(_) => {}
                }
            }
        });
    };

    let handle_reference_upload = move |evt: Event<FormData>| {
        spawn(async move {
            let file_engine = evt.files();
            if let Some(file_data) = file_engine.first() {
                match file_data.read_bytes().await {
                    Ok(file_content) => {
                        let base64_str = base64::engine::general_purpose::STANDARD.encode(&file_content);
                        reference_base64.set(base64_str);
                    }
                    Err(_) => {}
                }
            }
        });
    };

    rsx! {
        div {
            style: "background:white; border-radius:12px; padding:20px; border:1px solid #e5e7eb;",
            
            h3 {
                style: "margin:0 0 16px 0; font-size:18px; font-weight:600; color:#374151;",
                "🔍 验证码识别"
            }

            div {
                style: "margin-bottom:16px;",
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
                    style: "width:100%; max-width:300px; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:white; cursor:pointer;",
                    if supported_types.contains(&"ocr".to_string()) {
                        option { value: "ocr", "🔤 英数验证码" }
                    }
                    if supported_types.contains(&"click".to_string()) {
                        option { value: "click", "👆 点选验证码" }
                    }
                    if supported_types.contains(&"slide".to_string()) {
                        option { value: "slide", "↔️ 滑块验证码" }
                    }
                }
            }

            ImageUploadArea {
                image_base64,
                result,
                module_id,
                handle_file_upload
            }

            if matches!(selected_type(), CaptchaType::Click | CaptchaType::Slide) {
                ReferenceImageArea {
                    reference_base64,
                    module_id,
                    handle_reference_upload
                }
            }

            RecognizeButton {
                image_base64,
                reference_base64,
                selected_type,
                is_processing,
                result
            }

            if let Some(res) = result() {
                ResultDisplay { result: res }
            }
        }
    }
}

#[component]
fn ImageUploadArea(
    image_base64: Signal<String>,
    result: Signal<Option<RecognitionResult>>,
    module_id: usize,
    handle_file_upload: EventHandler<Event<FormData>>
) -> Element {
    rsx! {
        div {
            style: "border:2px dashed #d1d5db; border-radius:12px; padding:20px; text-align:center; background:#f9fafb; margin-bottom:16px;",

            if image_base64().is_empty() {
                label {
                    r#for: "file-upload-{module_id}",
                    style: "display:block; padding:30px 20px; cursor:pointer;",
                    p {
                        style: "margin:0 0 8px 0; font-size:40px;",
                        "📷"
                    }
                    p {
                        style: "margin:0 0 6px 0; font-size:15px; font-weight:600; color:#374151;",
                        "点击上传验证码图片"
                    }
                    p {
                        style: "margin:0; font-size:13px; color:#6b7280;",
                        "支持 JPG、PNG、GIF 等格式"
                    }
                }
                input {
                    r#type: "file",
                    id: "file-upload-{module_id}",
                    accept: "image/png,image/jpeg,image/gif",
                    multiple: false,
                    style: "display:none;",
                    onchange: handle_file_upload,
                }
            } else {
                div {
                    div {
                        style: "position:relative; display:inline-block; max-width:100%;",
                        img {
                            src: "data:image/png;base64,{image_base64()}",
                            style: "max-width:100%; max-height:350px; border-radius:8px; box-shadow:0 4px 12px rgba(0,0,0,0.1); display:block;",
                        }
                        if let Some(res) = result() {
                            if res.success {
                                if let Some(coords) = &res.coordinates {
                                    for (idx, (x, y)) in coords.iter().enumerate() {
                                        div {
                                            key: "{idx}",
                                            style: "position:absolute; left:{x}px; top:{y}px; width:40px; height:40px; border:3px solid #ef4444; border-radius:50%; background:rgba(239, 68, 68, 0.3); transform:translate(-50%, -50%); pointer-events:none; z-index:10;",
                                            div {
                                                style: "position:absolute; top:50%; left:50%; transform:translate(-50%, -50%); color:white; font-weight:700; font-size:16px;",
                                                "{idx + 1}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    button {
                        onclick: move |_| {
                            image_base64.set(String::new());
                            result.set(None);
                        },
                        style: "margin-top:12px; padding:8px 16px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#374151; font-weight:600; cursor:pointer;",
                        "🔄 重新上传"
                    }
                }
            }
        }
    }
}

#[component]
fn ReferenceImageArea(
    reference_base64: Signal<String>,
    module_id: usize,
    handle_reference_upload: EventHandler<Event<FormData>>
) -> Element {
    rsx! {
        div {
            style: "border:2px dashed #d1d5db; border-radius:12px; padding:16px; text-align:center; background:#f9fafb; margin-bottom:16px;",
            p {
                style: "margin:0 0 8px 0; font-size:13px; font-weight:600; color:#374151;",
                "参考图（可选）"
            }

            if reference_base64().is_empty() {
                label {
                    r#for: "reference-upload-{module_id}",
                    style: "display:block; padding:16px; cursor:pointer;",
                    p {
                        style: "margin:0 0 6px 0; font-size:28px;",
                        "🖼️"
                    }
                    p {
                        style: "margin:0; font-size:13px; font-weight:500; color:#374151;",
                        "点击上传参考图"
                    }
                }
                input {
                    r#type: "file",
                    id: "reference-upload-{module_id}",
                    accept: "image/png,image/jpeg,image/gif",
                    multiple: false,
                    style: "display:none;",
                    onchange: handle_reference_upload,
                }
            } else {
                div {
                    img {
                        src: "data:image/png;base64,{reference_base64()}",
                        style: "max-width:100%; max-height:150px; border-radius:8px; box-shadow:0 2px 8px rgba(0,0,0,0.1);",
                    }
                    button {
                        onclick: move |_| reference_base64.set(String::new()),
                        style: "margin-top:8px; padding:6px 12px; border-radius:6px; border:1px solid #d1d5db; background:white; color:#374151; font-size:13px; cursor:pointer;",
                        "🗑️ 移除"
                    }
                }
            }
        }
    }
}

#[component]
fn RecognizeButton(
    image_base64: Signal<String>,
    reference_base64: Signal<String>,
    selected_type: Signal<CaptchaType>,
    is_processing: Signal<bool>,
    result: Signal<Option<RecognitionResult>>
) -> Element {
    rsx! {
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
                    let mut request_body = serde_json::json!({
                        "image_base64": img_base64,
                        "captcha_type": captcha_type,
                    });

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

                                result.set(Some(RecognitionResult {
                                    success,
                                    result: res_text,
                                    coordinates: coords,
                                }));
                            }
                        }
                        Err(e) => {
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
                "width:100%; padding:12px 24px; border-radius:10px; border:none; background:#d1d5db; color:#9ca3af; font-weight:600; cursor:not-allowed; font-size:15px;"
            } else {
                "width:100%; padding:12px 24px; border-radius:10px; border:none; background:linear-gradient(120deg,#f59e0b,#d97706); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(245, 158, 11, 0.3); font-size:15px;"
            },
            if is_processing() {
                "🔄 识别中..."
            } else {
                "🚀 开始识别"
            }
        }
    }
}

#[component]
fn ResultDisplay(result: RecognitionResult) -> Element {
    rsx! {
        div {
            style: if result.success {
                "margin-top:16px; background:linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%); border-radius:10px; padding:16px; border:1px solid #10b981;"
            } else {
                "margin-top:16px; background:linear-gradient(135deg, #fee2e2 0%, #fecaca 100%); border-radius:10px; padding:16px; border:1px solid #ef4444;"
            },

            p {
                style: if result.success {
                    "margin:0 0 6px 0; font-size:13px; font-weight:600; color:#065f46;"
                } else {
                    "margin:0 0 6px 0; font-size:13px; font-weight:600; color:#991b1b;"
                },
                if result.success { "✓ 识别成功" } else { "✗ 识别失败" }
            }
            p {
                style: if result.success {
                    "margin:0; font-size:20px; color:#047857; font-weight:700; font-family:monospace; word-break:break-all;"
                } else {
                    "margin:0; font-size:15px; color:#991b1b; font-weight:600; word-break:break-all;"
                },
                "{result.result}"
            }

            if let Some(coords) = &result.coordinates {
                div {
                    style: "margin-top:10px; padding-top:10px; border-top:1px solid rgba(16, 185, 129, 0.3);",
                    p {
                        style: "margin:0 0 6px 0; font-size:12px; font-weight:600; color:#065f46;",
                        "点选坐标:"
                    }
                    for (idx, (x, y)) in coords.iter().enumerate() {
                        span {
                            key: "{idx}",
                            style: "display:inline-block; margin:3px 6px 3px 0; padding:3px 10px; background:rgba(16, 185, 129, 0.2); border-radius:6px; font-size:12px; color:#065f46; font-family:monospace;",
                            "{idx + 1}: ({x}, {y})"
                        }
                    }
                }
            }
        }
    }
}
