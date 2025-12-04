use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use base64::Engine as Base64Engine;

#[derive(Clone, PartialEq)]
enum CaptchaEngine {
    Ddddocr,
    AntiCAP,
}

#[derive(Clone, PartialEq)]
enum CaptchaType {
    // ddddocr 引擎
    DdddocrOcr,           // 普通验证码（标准/旧版/Beta）
    DdddocrDet,           // 目标检测
    DdddocrSlide,         // 滑块验证码
    DdddocrClick,         // 点选验证码
    
    // AntiCAP 引擎
    AnticapOcr,           // 通用 OCR
    AnticapMath,          // 算术验证码
    AnticapDetIcon,       // 图标检测
    AnticapDetText,       // 文字检测
    AnticapRotate,        // 单图旋转
    AnticapSlideMatch,    // 缺口滑块
    AnticapSlideComp,     // 阴影滑块
    AnticapRotateDouble,  // 双图旋转
    AnticapCompare,       // 图片相似度
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
struct ApiResponse {
    success: bool,
    #[serde(default)]
    result: Option<serde_json::Value>,
    #[serde(default)]
    objects: Option<Vec<DetectionObject>>,
    #[serde(default)]
    targets: Option<Vec<ClickTarget>>,
    #[serde(default)]
    distance: Option<i32>,
    #[serde(default)]
    similarity: Option<f64>,
    #[serde(default)]
    inner_angle: Option<f64>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    detail: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
struct DetectionObject {
    id: usize,
    bbox: Vec<i32>,
    center: Vec<i32>,
    label: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
struct ClickTarget {
    id: usize,
    position: Position,
    label: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[component]
pub fn Captcha() -> Element {
    let selected_engine = use_signal(|| CaptchaEngine::Ddddocr);
    let selected_type = use_signal(|| CaptchaType::DdddocrOcr);
    let ocr_mode = use_signal(|| "normal".to_string());
    let result_text = use_signal(String::new);
    let api_response = use_signal(|| None::<ApiResponse>);
    let image_base64 = use_signal(String::new);
    let image2_base64 = use_signal(String::new);

    rsx! {
        div {
            class: "captcha-scroll-container",
            style: "height:100%; overflow-y:auto; overflow-x:hidden; padding:24px 16px 24px 0; scrollbar-width:thin; scrollbar-color:#cbd5e1 transparent;",

            div {
                style: "display:flex; flex-direction:column; gap:28px; max-width:1400px; margin:0 auto;",

                section {
                    style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:20px; padding:28px 32px; border:1px solid #fbbf24; box-shadow:0 4px 20px rgba(251, 191, 36, 0.1);",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#78350f; letter-spacing:-0.02em;",
                        "🔐 验证码识别系统"
                    }
                    p {
                        style: "color:#92400e; font-size:15px; margin:0; line-height:1.6;",
                        "集成 ddddocr + AntiCAP 双引擎，支持多种验证码类型识别"
                    }
                }

                EngineSelector { selected_engine, selected_type, result_text, api_response, image_base64, image2_base64 }
                TypeSelector { selected_engine, selected_type, ocr_mode, result_text, api_response, image_base64, image2_base64 }
                RecognitionPanel { selected_engine, selected_type, ocr_mode, result_text, api_response, image_base64, image2_base64 }
            }
        }
    }
}

#[component]
fn EngineSelector(selected_engine: Signal<CaptchaEngine>, selected_type: Signal<CaptchaType>, mut result_text: Signal<String>, mut api_response: Signal<Option<ApiResponse>>, mut image_base64: Signal<String>, mut image2_base64: Signal<String>) -> Element {
    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:24px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",

            h2 {
                style: "font-size:20px; font-weight:700; margin:0 0 16px 0; color:#111827;",
                "选择识别引擎"
            }

            div {
                style: "display:grid; grid-template-columns:1fr 1fr; gap:16px;",
                
                button {
                    style: if matches!(selected_engine(), CaptchaEngine::Ddddocr) {
                        "padding:20px; border-radius:12px; background:linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%); color:white; font-weight:600; cursor:pointer; border:none; box-shadow:0 4px 12px rgba(251, 191, 36, 0.4); transition:all 0.3s;"
                    } else {
                        "padding:20px; border-radius:12px; background:#f9fafb; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #e5e7eb; transition:all 0.3s;"
                    },
                    onclick: move |_| {
                        selected_engine.set(CaptchaEngine::Ddddocr);
                        selected_type.set(CaptchaType::DdddocrOcr);
                        result_text.set(String::new());
                        api_response.set(None);
                        image_base64.set(String::new());
                        image2_base64.set(String::new());
                    },
                    div {
                        style: "font-size:18px; font-weight:700; margin-bottom:6px;",
                        "ddddocr 引擎"
                    }
                    div {
                        style: if matches!(selected_engine(), CaptchaEngine::Ddddocr) {
                            "font-size:13px; opacity:0.9;"
                        } else {
                            "font-size:13px; color:#9ca3af;"
                        },
                        "通用识别 + 滑块 + 点选"
                    }
                }

                button {
                    style: if matches!(selected_engine(), CaptchaEngine::AntiCAP) {
                        "padding:20px; border-radius:12px; background:linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%); color:white; font-weight:600; cursor:pointer; border:none; box-shadow:0 4px 12px rgba(251, 191, 36, 0.4); transition:all 0.3s;"
                    } else {
                        "padding:20px; border-radius:12px; background:#f9fafb; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #e5e7eb; transition:all 0.3s;"
                    },
                    onclick: move |_| {
                        selected_engine.set(CaptchaEngine::AntiCAP);
                        selected_type.set(CaptchaType::AnticapOcr);
                        result_text.set(String::new());
                        api_response.set(None);
                        image_base64.set(String::new());
                        image2_base64.set(String::new());
                    },
                    div {
                        style: "font-size:18px; font-weight:700; margin-bottom:6px;",
                        "AntiCAP 引擎"
                    }
                    div {
                        style: if matches!(selected_engine(), CaptchaEngine::AntiCAP) {
                            "font-size:13px; opacity:0.9;"
                        } else {
                            "font-size:13px; color:#9ca3af;"
                        },
                        "OCR + 算术 + 检测 + 旋转"
                    }
                }
            }
        }
    }
}

#[component]
fn RecognitionPanel(selected_engine: Signal<CaptchaEngine>, selected_type: Signal<CaptchaType>, ocr_mode: Signal<String>, result_text: Signal<String>, api_response: Signal<Option<ApiResponse>>, image_base64: Signal<String>, image2_base64: Signal<String>) -> Element {
    let is_processing = use_signal(|| false);

    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:24px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",

            h3 {
                style: "margin:0 0 16px 0; font-size:18px; font-weight:700; color:#111827;",
                "📸 上传验证码图片"
            }

            ImageUploader {
                image_base64,
                image2_base64,
                selected_type,
                api_response
            }

            if !result_text().is_empty() {
                ResultDisplay { 
                    result_text: result_text(),
                    api_response: api_response()
                }
            }

            RecognizeButton {
                selected_engine,
                selected_type,
                image_base64,
                image2_base64,
                ocr_mode,
                is_processing,
                result_text,
                api_response
            }
        }
    }
}

#[component]
fn TypeSelector(selected_engine: Signal<CaptchaEngine>, selected_type: Signal<CaptchaType>, ocr_mode: Signal<String>, mut result_text: Signal<String>, mut api_response: Signal<Option<ApiResponse>>, mut image_base64: Signal<String>, mut image2_base64: Signal<String>) -> Element {
    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:24px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
            
            h3 {
                style: "margin:0 0 16px 0; font-size:18px; font-weight:700; color:#111827;",
                "{get_type_section_title(selected_engine())}"
            }

            p {
                style: "margin:0 0 16px 0; font-size:14px; color:#6b7280;",
                "{get_type_section_desc(selected_engine())}"
            }

            div {
                style: "display:grid; grid-template-columns:repeat(auto-fill, minmax(180px, 1fr)); gap:12px;",

                if matches!(selected_engine(), CaptchaEngine::Ddddocr) {
                    TypeButton { selected_type, value: CaptchaType::DdddocrOcr, label: "普通验证码", icon: "ddddocr", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::DdddocrDet, label: "目标检测", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::DdddocrSlide, label: "滑块验证码", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::DdddocrClick, label: "点选验证码", icon: "", result_text, api_response, image_base64, image2_base64 }
                } else {
                    TypeButton { selected_type, value: CaptchaType::AnticapOcr, label: "通用验证码", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapMath, label: "算术验证码", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapDetIcon, label: "图标检测", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapDetText, label: "文字检测", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapRotate, label: "单图旋转", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapSlideMatch, label: "缺口滑块", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapSlideComp, label: "阴影滑块", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapRotateDouble, label: "双图旋转", icon: "", result_text, api_response, image_base64, image2_base64 }
                    TypeButton { selected_type, value: CaptchaType::AnticapCompare, label: "图片相似度", icon: "", result_text, api_response, image_base64, image2_base64 }
                }
            }

            // OCR 模式选择（仅 ddddocr 普通验证码显示）
            if matches!(selected_engine(), CaptchaEngine::Ddddocr) && matches!(selected_type(), CaptchaType::DdddocrOcr) {
                div {
                    style: "margin-top:20px; padding-top:20px; border-top:1px solid #e5e7eb;",
                    p {
                        style: "margin:0 0 12px 0; font-size:14px; font-weight:600; color:#374151;",
                        "识别模式"
                    }
                    div {
                        style: "display:flex; gap:12px;",
                        
                        button {
                            style: if ocr_mode() == "normal" {
                                "padding:10px 16px; border-radius:8px; background:#fbbf24; color:white; font-weight:600; cursor:pointer; border:none; font-size:13px;"
                            } else {
                                "padding:10px 16px; border-radius:8px; background:#f9fafb; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #e5e7eb; font-size:13px;"
                            },
                            onclick: move |_| ocr_mode.set("normal".to_string()),
                            "🔹 标准模式"
                        }
                        
                        button {
                            style: if ocr_mode() == "old" {
                                "padding:10px 16px; border-radius:8px; background:#fbbf24; color:white; font-weight:600; cursor:pointer; border:none; font-size:13px;"
                            } else {
                                "padding:10px 16px; border-radius:8px; background:#f9fafb; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #e5e7eb; font-size:13px;"
                            },
                            onclick: move |_| ocr_mode.set("old".to_string()),
                            "🔸 旧版模式"
                        }
                        
                        button {
                            style: if ocr_mode() == "beta" {
                                "padding:10px 16px; border-radius:8px; background:#fbbf24; color:white; font-weight:600; cursor:pointer; border:none; font-size:13px;"
                            } else {
                                "padding:10px 16px; border-radius:8px; background:#f9fafb; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #e5e7eb; font-size:13px;"
                            },
                            onclick: move |_| ocr_mode.set("beta".to_string()),
                            "⭐ Beta 高精度"
                        }
                    }
                }
            }
        }
    }
}

fn get_type_section_title(engine: CaptchaEngine) -> &'static str {
    match engine {
        CaptchaEngine::Ddddocr => "普通验证码识别",
        CaptchaEngine::AntiCAP => "AntiCAP 验证码识别",
    }
}

fn get_type_section_desc(engine: CaptchaEngine) -> &'static str {
    match engine {
        CaptchaEngine::Ddddocr => "适用于各类常规验证码，支持标准、旧版、Beta 三种识别模式",
        CaptchaEngine::AntiCAP => "OCR + 算术 + 检测 + 旋转",
    }
}

#[component]
fn TypeButton(selected_type: Signal<CaptchaType>, value: CaptchaType, label: &'static str, icon: &'static str, mut result_text: Signal<String>, mut api_response: Signal<Option<ApiResponse>>, mut image_base64: Signal<String>, mut image2_base64: Signal<String>) -> Element {
    let is_selected = selected_type() == value;
    
    rsx! {
        button {
            style: if is_selected {
                "padding:12px 16px; border-radius:10px; background:#fbbf24; color:white; font-weight:600; cursor:pointer; border:none; box-shadow:0 2px 8px rgba(251, 191, 36, 0.3); transition:all 0.2s; text-align:center; font-size:14px;"
            } else {
                "padding:12px 16px; border-radius:10px; background:white; color:#374151; font-weight:500; cursor:pointer; border:1px solid #e5e7eb; transition:all 0.2s; text-align:center; font-size:14px;"
            },
            onclick: move |_| {
                selected_type.set(value.clone());
                result_text.set(String::new());
                api_response.set(None);
                image_base64.set(String::new());
                image2_base64.set(String::new());
            },
            if !icon.is_empty() {
                span {
                    style: "display:inline-block; padding:2px 8px; background:rgba(251, 191, 36, 0.2); border-radius:4px; font-size:11px; margin-right:6px;",
                    "{icon}"
                }
            }
            "{label}"
        }
    }
}

fn needs_second_image(captcha_type: CaptchaType) -> bool {
    matches!(
        captcha_type,
        CaptchaType::DdddocrSlide
            | CaptchaType::AnticapSlideMatch
            | CaptchaType::AnticapSlideComp
            | CaptchaType::AnticapRotateDouble
            | CaptchaType::AnticapCompare
    )
}

fn get_second_image_label(captcha_type: CaptchaType) -> &'static str {
    match captcha_type {
        CaptchaType::DdddocrSlide | CaptchaType::AnticapSlideMatch | CaptchaType::AnticapSlideComp => "背景图",
        CaptchaType::AnticapRotateDouble => "外圆图",
        CaptchaType::AnticapCompare => "对比图",
        _ => "副图",
    }
}

#[component]
fn ImageUploader(image_base64: Signal<String>, image2_base64: Signal<String>, selected_type: Signal<CaptchaType>, api_response: Signal<Option<ApiResponse>>) -> Element {
    let handle_upload = move |evt: Event<FormData>| {
        spawn(async move {
            if let Some(file_data) = evt.files().first() {
                if let Ok(file_content) = file_data.read_bytes().await {
                    let base64_str = Base64Engine::encode(&base64::engine::general_purpose::STANDARD, &file_content);
                    image_base64.set(base64_str);
                }
            }
        });
    };

    let handle_upload2 = move |evt: Event<FormData>| {
        spawn(async move {
            if let Some(file_data) = evt.files().first() {
                if let Ok(file_content) = file_data.read_bytes().await {
                    let base64_str = Base64Engine::encode(&base64::engine::general_purpose::STANDARD, &file_content);
                    image2_base64.set(base64_str);
                }
            }
        });
    };

    let needs_second = needs_second_image(selected_type());

    rsx! {
        div {
            style: if needs_second {
                "display:grid; grid-template-columns:1fr 1fr; gap:16px;"
            } else {
                "display:block;"
            },

            // 主图
            div {
                if image_base64().is_empty() {
                    label {
                        r#for: "upload-main",
                        style: "display:block; padding:40px; text-align:center; cursor:pointer; border-radius:8px; background:white; border:2px dashed #d1d5db; transition:all 0.2s;",
                        p { style: "margin:0 0 8px 0; font-size:48px;", "📷" }
                        p { style: "margin:0; font-size:14px; color:#6b7280;", "点击上传主图" }
                    }
                    input {
                        r#type: "file",
                        id: "upload-main",
                        accept: "image/*",
                        style: "display:none;",
                        onchange: handle_upload,
                    }
                } else {
                    div {
                        style: "position:relative; background:white; border-radius:8px; padding:12px; border:1px solid #e5e7eb;",
                        div {
                            style: "position:relative; display:inline-block;",
                            img {
                                id: "main-image",
                                src: "data:image/png;base64,{image_base64()}",
                                style: "max-width:100%; height:auto; border-radius:6px; display:block; image-rendering:crisp-edges;",
                            }
                            // 标注层（双图旋转不显示标注，因为会叠加显示）
                            if let Some(resp) = api_response() {
                                if !matches!(selected_type(), CaptchaType::AnticapRotateDouble) {
                                    AnnotationLayer { response: resp.clone(), image_id: "main-image".to_string(), is_background: false }
                                }
                            }
                        }
                        button {
                            onclick: move |_| image_base64.set(String::new()),
                            style: "margin-top:8px; width:100%; padding:8px; border-radius:6px; border:1px solid #d1d5db; background:white; color:#374151; font-size:13px; cursor:pointer;",
                            "🗑️ 移除"
                        }
                    }
                }
            }

            // 副图（如果需要）
            if needs_second {
                div {
                    if image2_base64().is_empty() {
                        label {
                            r#for: "upload-second",
                            style: "display:block; padding:40px; text-align:center; cursor:pointer; border-radius:8px; background:white; border:2px dashed #d1d5db; transition:all 0.2s;",
                            p { style: "margin:0 0 8px 0; font-size:48px;", "🖼️" }
                            p { style: "margin:0; font-size:14px; color:#6b7280;", "{get_second_image_label(selected_type())}" }
                        }
                        input {
                            r#type: "file",
                            id: "upload-second",
                            accept: "image/*",
                            style: "display:none;",
                            onchange: handle_upload2,
                        }
                    } else {
                        // 双图旋转验证码特殊处理：叠加显示
                        if matches!(selected_type(), CaptchaType::AnticapRotateDouble) {
                            div {
                                style: "position:relative; background:white; border-radius:8px; padding:12px; border:1px solid #e5e7eb;",
                                div {
                                    style: "position:relative; display:inline-block;",
                                    // 外圆图（背景）
                                    img {
                                        id: "second-image",
                                        src: "data:image/png;base64,{image2_base64()}",
                                        style: "max-width:100%; height:auto; border-radius:6px; display:block; image-rendering:crisp-edges;",
                                    }
                                    // 内圆图（叠加在上面，根据角度旋转，等比缩放到外圆图尺寸）
                                    if let Some(resp) = api_response() {
                                        if let Some(angle) = resp.inner_angle {
                                            {
                                                // 反向旋转：API 返回的角度需要取负值
                                                let rotate_angle = -angle;
                                                rsx! {
                                                    img {
                                                        src: "data:image/png;base64,{image_base64()}",
                                                        style: "position:absolute; top:0; left:0; width:100%; height:100%; object-fit:contain; transform:rotate({rotate_angle}deg); transform-origin:center; image-rendering:crisp-edges;",
                                                    }
                                                    // 角度标签（显示原始角度）
                                                    div {
                                                        style: "position:absolute; top:10px; left:10px; background:#8b5cf6; color:white; padding:6px 12px; border-radius:6px; font-size:14px; font-weight:bold; box-shadow:0 2px 8px rgba(139, 92, 246, 0.4); z-index:10;",
                                                        "↺ {angle:.1}°"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                button {
                                    onclick: move |_| image2_base64.set(String::new()),
                                    style: "margin-top:8px; width:100%; padding:8px; border-radius:6px; border:1px solid #d1d5db; background:white; color:#374151; font-size:13px; cursor:pointer;",
                                    "🗑️ 移除"
                                }
                            }
                        } else {
                            // 其他类型：正常显示副图
                            div {
                                style: "position:relative; background:white; border-radius:8px; padding:12px; border:1px solid #e5e7eb;",
                                div {
                                    style: "position:relative; display:inline-block;",
                                    img {
                                        id: "second-image",
                                        src: "data:image/png;base64,{image2_base64()}",
                                        style: "max-width:100%; height:auto; border-radius:6px; display:block; image-rendering:crisp-edges;",
                                    }
                                    // 标注层（滑块位置显示在背景图上）
                                    if let Some(resp) = api_response() {
                                        AnnotationLayer { response: resp.clone(), image_id: "second-image".to_string(), is_background: true }
                                    }
                                }
                                button {
                                    onclick: move |_| image2_base64.set(String::new()),
                                    style: "margin-top:8px; width:100%; padding:8px; border-radius:6px; border:1px solid #d1d5db; background:white; color:#374151; font-size:13px; cursor:pointer;",
                                    "🗑️ 移除"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}



#[component]
fn RecognizeButton(
    selected_engine: Signal<CaptchaEngine>,
    selected_type: Signal<CaptchaType>,
    image_base64: Signal<String>,
    image2_base64: Signal<String>,
    ocr_mode: Signal<String>,
    mut is_processing: Signal<bool>,
    mut result_text: Signal<String>,
    mut api_response: Signal<Option<ApiResponse>>
) -> Element {
    let can_submit = !image_base64().is_empty() && !is_processing();

    rsx! {
        button {
            disabled: !can_submit,
            onclick: move |_| {
                let img1 = format!("data:image/png;base64,{}", image_base64());
                let img2 = if !image2_base64().is_empty() {
                    format!("data:image/png;base64,{}", image2_base64())
                } else {
                    String::new()
                };
                let engine = selected_engine();
                let cap_type = selected_type();
                let mode = ocr_mode();

                spawn(async move {
                    is_processing.set(true);
                    result_text.set(String::new());
                    api_response.set(None);

                    let client = reqwest::Client::new();
                    let (url, body) = build_request(engine, cap_type, &img1, &img2, &mode);

                    match client.post(&url).json(&body).send().await {
                        Ok(resp) => {
                            if let Ok(api_resp) = resp.json::<ApiResponse>().await {
                                // 调试信息 - 显示原始 API 响应
                                #[cfg(debug_assertions)]
                                {
                                    println!("=== API 原始响应 ===");
                                    println!("{}", serde_json::to_string_pretty(&api_resp).unwrap_or_else(|_| "无法序列化".to_string()));
                                    println!("==================");
                                    
                                    if let Some(ref targets) = api_resp.targets {
                                        println!("点选目标数量: {}", targets.len());
                                        for (i, t) in targets.iter().enumerate() {
                                            println!("  目标{}: ({}, {}) - {}", i+1, t.position.x, t.position.y, t.label);
                                        }
                                    }
                                    if let Some(ref objects) = api_resp.objects {
                                        println!("检测对象数量: {}", objects.len());
                                    }
                                    if let Some(distance) = api_resp.distance {
                                        println!("滑块距离: {} px", distance);
                                    }
                                    if let Some(angle) = api_resp.inner_angle {
                                        println!("旋转角度: {:.1}°", angle);
                                    }
                                }
                                
                                result_text.set(format_result(api_resp.clone()));
                                api_response.set(Some(api_resp));
                            } else {
                                result_text.set("❌ 解析响应失败".to_string());
                            }
                        }
                        Err(e) => {
                            result_text.set(format!("❌ 请求失败: {}", e));
                        }
                    }

                    is_processing.set(false);
                });
            },
            style: if can_submit {
                "width:100%; padding:16px 24px; border-radius:12px; border:none; background:linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%); color:white; font-weight:700; cursor:pointer; box-shadow:0 4px 16px rgba(251, 191, 36, 0.4); font-size:16px; transition:all 0.3s; display:flex; align-items:center; justify-content:center; gap:8px;"
            } else {
                "width:100%; padding:16px 24px; border-radius:12px; border:none; background:#e5e7eb; color:#9ca3af; font-weight:700; cursor:not-allowed; font-size:16px; display:flex; align-items:center; justify-content:center; gap:8px;"
            },
            span { style: "font-size:20px;", if is_processing() { "⏳" } else { "🚀" } }
            span { if is_processing() { "识别中..." } else { "开始识别" } }
        }
    }
}

fn build_request(engine: CaptchaEngine, cap_type: CaptchaType, img1: &str, img2: &str, mode: &str) -> (String, serde_json::Value) {
    match engine {
        CaptchaEngine::Ddddocr => match cap_type {
            CaptchaType::DdddocrOcr => (
                "http://localhost:8080/api/captcha/ddddocr/ocr".to_string(),
                serde_json::json!({"image": img1, "type": mode})
            ),
            CaptchaType::DdddocrDet => (
                "http://localhost:8080/api/captcha/ddddocr/det".to_string(),
                serde_json::json!({"image": img1})
            ),
            CaptchaType::DdddocrSlide => (
                "http://localhost:8080/api/captcha/ddddocr/slide".to_string(),
                serde_json::json!({"target": img1, "background": img2})
            ),
            CaptchaType::DdddocrClick => (
                "http://localhost:8080/api/captcha/ddddocr/click".to_string(),
                serde_json::json!({"image": img1})
            ),
            _ => ("".to_string(), serde_json::json!({}))
        },
        CaptchaEngine::AntiCAP => match cap_type {
            CaptchaType::AnticapOcr => (
                "http://localhost:8080/api/captcha/anticap/ocr".to_string(),
                serde_json::json!({"image": img1, "type": "ocr"})
            ),
            CaptchaType::AnticapMath => (
                "http://localhost:8080/api/captcha/anticap/ocr".to_string(),
                serde_json::json!({"image": img1, "type": "math"})
            ),
            CaptchaType::AnticapDetIcon => (
                "http://localhost:8080/api/captcha/anticap/ocr".to_string(),
                serde_json::json!({"image": img1, "type": "detection_icon"})
            ),
            CaptchaType::AnticapDetText => (
                "http://localhost:8080/api/captcha/anticap/ocr".to_string(),
                serde_json::json!({"image": img1, "type": "detection_text"})
            ),
            CaptchaType::AnticapRotate => (
                "http://localhost:8080/api/captcha/anticap/ocr".to_string(),
                serde_json::json!({"image": img1, "type": "single_rotate"})
            ),
            CaptchaType::AnticapSlideMatch => (
                "http://localhost:8080/api/captcha/anticap/slide".to_string(),
                serde_json::json!({"target": img1, "background": img2, "mode": "match"})
            ),
            CaptchaType::AnticapSlideComp => (
                "http://localhost:8080/api/captcha/anticap/slide".to_string(),
                serde_json::json!({"target": img1, "background": img2, "mode": "comparison"})
            ),
            CaptchaType::AnticapRotateDouble => (
                "http://localhost:8080/api/captcha/anticap/rotate".to_string(),
                serde_json::json!({"inside": img1, "outside": img2})
            ),
            CaptchaType::AnticapCompare => (
                "http://localhost:8080/api/captcha/anticap/compare".to_string(),
                serde_json::json!({"image1": img1, "image2": img2})
            ),
            _ => ("".to_string(), serde_json::json!({}))
        }
    }
}

fn format_result(resp: ApiResponse) -> String {
    if let Some(detail) = resp.detail {
        return format!("❌ {}", detail);
    }

    if !resp.success {
        return "❌ 识别失败".to_string();
    }

    let mut output = String::new();

    if let Some(result) = resp.result {
        if let Some(s) = result.as_str() {
            output.push_str(&format!("📝 结果: {}\n", s));
        } else if let Some(n) = result.as_i64() {
            output.push_str(&format!("📝 结果: {}\n", n));
        } else {
            output.push_str(&format!("📝 结果: {}\n", result));
        }
    }

    if let Some(objects) = resp.objects {
        output.push_str(&format!("\n🎯 检测到 {} 个目标:\n", objects.len()));
        for obj in objects.iter().take(5) {
            output.push_str(&format!("  • {} - 中心点: ({}, {})\n", obj.label, obj.center[0], obj.center[1]));
        }
    }

    if let Some(targets) = resp.targets {
        output.push_str(&format!("\n👆 可点击目标 {} 个:\n", targets.len()));
        for target in targets.iter().take(5) {
            output.push_str(&format!("  • {} - 位置: ({}, {})\n", target.label, target.position.x, target.position.y));
        }
    }

    if let Some(distance) = resp.distance {
        output.push_str(&format!("\n🧩 滑动距离: {} 像素\n", distance));
    }

    if let Some(similarity) = resp.similarity {
        output.push_str(&format!("\n📊 相似度: {:.2}%\n", similarity * 100.0));
    }

    if let Some(angle) = resp.inner_angle {
        output.push_str(&format!("\n🔄 旋转角度: {:.1}°\n", angle));
    }

    if let Some(desc) = resp.description {
        output.push_str(&format!("\n💡 {}\n", desc));
    }

    output
}

#[component]
fn AnnotationLayer(response: ApiResponse, image_id: String, is_background: bool) -> Element {
    let _ = (image_id, is_background); // 暂时不使用，避免警告
    
    // 不使用 viewBox，让 SVG 使用像素坐标系统
    rsx! {
        svg {
            style: "position:absolute; top:0; left:0; width:100%; height:100%; pointer-events:none;",
            
            // 绘制检测框（图标检测、文字检测）
            if let Some(objects) = &response.objects {
                for (idx, obj) in objects.iter().enumerate() {
                    g {
                        key: "{idx}",
                        // 检测框
                        rect {
                            x: "{obj.bbox[0]}",
                            y: "{obj.bbox[1]}",
                            width: "{obj.bbox[2] - obj.bbox[0]}",
                            height: "{obj.bbox[3] - obj.bbox[1]}",
                            fill: "none",
                            stroke: "#10b981",
                            stroke_width: "2",
                            rx: "2",
                        }
                        // 中心点
                        circle {
                            cx: "{obj.center[0]}",
                            cy: "{obj.center[1]}",
                            r: "4",
                            fill: "#ef4444",
                        }
                        // 标签背景
                        rect {
                            x: "{obj.bbox[0]}",
                            y: "{obj.bbox[1] - 20}",
                            width: "{(obj.label.len() as i32 + 3) * 7}",
                            height: "18",
                            fill: "#10b981",
                            rx: "3",
                        }
                        // 标签文字
                        text {
                            x: "{obj.bbox[0] + 3}",
                            y: "{obj.bbox[1] - 7}",
                            fill: "white",
                            font_size: "12",
                            font_weight: "bold",
                            "{idx + 1}: {obj.label}"
                        }
                    }
                }
            }
            
            // 绘制点击目标
            if let Some(targets) = &response.targets {
                for (idx, target) in targets.iter().enumerate() {
                    g {
                        key: "{idx}",
                        // 点击圆圈
                        circle {
                            cx: "{target.position.x}",
                            cy: "{target.position.y}",
                            r: "10",
                            fill: "rgba(239, 68, 68, 0.3)",
                            stroke: "#ef4444",
                            stroke_width: "2",
                        }
                        // 序号
                        circle {
                            cx: "{target.position.x}",
                            cy: "{target.position.y}",
                            r: "6",
                            fill: "#ef4444",
                        }
                        text {
                            x: "{target.position.x}",
                            y: "{target.position.y + 4}",
                            fill: "white",
                            font_size: "10",
                            font_weight: "bold",
                            text_anchor: "middle",
                            "{idx + 1}"
                        }
                        // 标签
                        if !target.label.is_empty() {
                            rect {
                                x: "{target.position.x - (target.label.len() as i32 * 3)}",
                                y: "{target.position.y - 25}",
                                width: "{target.label.len() as i32 * 7}",
                                height: "16",
                                fill: "#ef4444",
                                rx: "3",
                            }
                            text {
                                x: "{target.position.x}",
                                y: "{target.position.y - 13}",
                                fill: "white",
                                font_size: "11",
                                font_weight: "bold",
                                text_anchor: "middle",
                                "{target.label}"
                            }
                        }
                    }
                }
            }
            
            // 绘制滑块位置（垂直线）
            if let Some(distance) = response.distance {
                g {
                    // 虚线
                    line {
                        x1: "{distance}",
                        y1: "0",
                        x2: "{distance}",
                        y2: "100%",
                        stroke: "#3b82f6",
                        stroke_width: "3",
                        stroke_dasharray: "8,4",
                    }
                    // 半透明区域
                    rect {
                        x: "{distance - 2}",
                        y: "0",
                        width: "4",
                        height: "100%",
                        fill: "rgba(59, 130, 246, 0.2)",
                    }
                    // 标签背景
                    {
                        let label_text = format!("← {}px", distance);
                        let label_width = label_text.len() as i32 * 8 + 10;
                        rsx! {
                            rect {
                                x: "{distance + 5}",
                                y: "10",
                                width: "{label_width}",
                                height: "22",
                                fill: "#3b82f6",
                                rx: "4",
                            }
                            text {
                                x: "{distance + 10}",
                                y: "26",
                                fill: "white",
                                font_size: "14",
                                font_weight: "bold",
                                "{label_text}"
                            }
                        }
                    }
                }
            }
            
            // 绘制旋转角度指示（仅单图旋转，双图旋转用叠加显示）
            if let Some(angle) = response.inner_angle {
                if !is_background && response.distance.is_none() && response.objects.is_none() && response.targets.is_none() {
                    {
                        // API 返回的角度直接显示（逆时针为正）
                        let angle_text = format!("↺ {:.1}°", angle);
                        rsx! {
                            g {
                                // 旋转方向指示弧线（逆时针）
                                path {
                                    d: "M 240,140 A 40,40 0 0,0 200,100",
                                    fill: "none",
                                    stroke: "#8b5cf6",
                                    stroke_width: "4",
                                    stroke_linecap: "round",
                                }
                                // 箭头（逆时针方向）
                                polygon {
                                    points: "200,100 205,105 197,103",
                                    fill: "#8b5cf6",
                                }
                                // 中心点
                                circle {
                                    cx: "200",
                                    cy: "150",
                                    r: "6",
                                    fill: "#8b5cf6",
                                }
                                // 角度标签背景
                                rect {
                                    x: "165",
                                    y: "20",
                                    width: "70",
                                    height: "24",
                                    fill: "#8b5cf6",
                                    rx: "4",
                                }
                                // 角度标签文字
                                text {
                                    x: "200",
                                    y: "37",
                                    fill: "white",
                                    font_size: "14",
                                    font_weight: "bold",
                                    text_anchor: "middle",
                                    "{angle_text}"
                                }
                            }
                        }
                    }
                }
            }
            
            // 绘制旋转目标位置（AntiCAP 单图旋转的 target 坐标）
            if let Some(ref result) = response.result {
                if let Some(target_array) = result.as_array() {
                    if target_array.len() == 2 {
                        if let (Some(x), Some(y)) = (target_array[0].as_f64(), target_array[1].as_f64()) {
                            {
                                let x_i32 = x as i32;
                                let y_i32 = y as i32;
                                rsx! {
                                    g {
                                        // 目标位置十字标记
                                        line {
                                            x1: "{x_i32 - 15}",
                                            y1: "{y_i32}",
                                            x2: "{x_i32 + 15}",
                                            y2: "{y_i32}",
                                            stroke: "#ef4444",
                                            stroke_width: "3",
                                        }
                                        line {
                                            x1: "{x_i32}",
                                            y1: "{y_i32 - 15}",
                                            x2: "{x_i32}",
                                            y2: "{y_i32 + 15}",
                                            stroke: "#ef4444",
                                            stroke_width: "3",
                                        }
                                        // 目标圆圈
                                        circle {
                                            cx: "{x_i32}",
                                            cy: "{y_i32}",
                                            r: "20",
                                            fill: "none",
                                            stroke: "#ef4444",
                                            stroke_width: "2",
                                        }
                                        // 坐标标签
                                        rect {
                                            x: "{x_i32 + 25}",
                                            y: "{y_i32 - 12}",
                                            width: "100",
                                            height: "24",
                                            fill: "#ef4444",
                                            rx: "4",
                                        }
                                        text {
                                            x: "{x_i32 + 30}",
                                            y: "{y_i32 + 5}",
                                            fill: "white",
                                            font_size: "12",
                                            font_weight: "bold",
                                            "({x_i32}, {y_i32})"
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

#[component]
fn ResultDisplay(result_text: String, api_response: Option<ApiResponse>) -> Element {
    let is_success = api_response.as_ref().map(|r| r.success).unwrap_or(false);
    let mut show_raw = use_signal(|| false);
    
    rsx! {
        div {
            style: if is_success {
                "margin-bottom:20px; background:linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%); border-radius:12px; padding:20px; border:1px solid #10b981; box-shadow:0 2px 8px rgba(16, 185, 129, 0.2);"
            } else {
                "margin-bottom:20px; background:linear-gradient(135deg, #fee2e2 0%, #fecaca 100%); border-radius:12px; padding:20px; border:1px solid #ef4444; box-shadow:0 2px 8px rgba(239, 68, 68, 0.2);"
            },

            div {
                style: "display:flex; align-items:center; justify-content:space-between; margin-bottom:12px;",
                div {
                    style: "display:flex; align-items:center; gap:8px;",
                    span {
                        style: "font-size:24px;",
                        if is_success { "✅" } else { "❌" }
                    }
                    h4 {
                        style: if is_success {
                            "margin:0; font-size:16px; font-weight:700; color:#065f46;"
                        } else {
                            "margin:0; font-size:16px; font-weight:700; color:#991b1b;"
                        },
                        if is_success { "识别成功" } else { "识别失败" }
                    }
                }
                button {
                    onclick: move |_| show_raw.set(!show_raw()),
                    style: "padding:6px 12px; border-radius:6px; border:1px solid #d1d5db; background:white; color:#374151; font-size:12px; cursor:pointer;",
                    if show_raw() { "隐藏原始响应" } else { "查看原始响应" }
                }
            }

            if show_raw() {
                if let Some(ref resp) = api_response {
                    pre {
                        style: "margin:0 0 12px 0; padding:12px; background:#f9fafb; border-radius:6px; font-size:12px; color:#374151; font-family:monospace; white-space:pre-wrap; word-break:break-word; line-height:1.4; overflow-x:auto;",
                        "{serde_json::to_string_pretty(resp).unwrap_or_else(|_| \"无法序列化\".to_string())}"
                    }
                }
            }

            pre {
                style: if is_success {
                    "margin:0; font-size:14px; color:#047857; font-family:monospace; white-space:pre-wrap; word-break:break-word; line-height:1.6;"
                } else {
                    "margin:0; font-size:14px; color:#991b1b; font-family:monospace; white-space:pre-wrap; word-break:break-word; line-height:1.6;"
                },
                "{result_text}"
            }
        }
    }
}
