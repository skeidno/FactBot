use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum CodeLanguage {
    Curl,
    Python,
    JavaScript,
    Go,
    Java,
    Rust,
}

#[component]
pub fn CaptchaHelp() -> Element {
    let selected_lang = use_signal(|| CodeLanguage::Curl);
    
    rsx! {
        script {
            r#"
            // 复制代码到剪贴板
            function copyCodeText(text) {{
                if (navigator.clipboard && navigator.clipboard.writeText) {{
                    navigator.clipboard.writeText(text)
                        .then(() => {{
                            console.log('复制成功');
                        }})
                        .catch(err => {{
                            console.error('复制失败:', err);
                            fallbackCopyCode(text);
                        }});
                }} else {{
                    fallbackCopyCode(text);
                }}
            }}
            
            function fallbackCopyCode(text) {{
                const textarea = document.createElement('textarea');
                textarea.value = text;
                textarea.style.position = 'fixed';
                textarea.style.opacity = '0';
                document.body.appendChild(textarea);
                textarea.select();
                try {{
                    document.execCommand('copy');
                    console.log('使用降级方案复制成功');
                }} catch (err) {{
                    console.error('降级方案也失败:', err);
                }}
                document.body.removeChild(textarea);
            }}
            
            // 监听所有复制按钮点击
            document.addEventListener('click', function(e) {{
                if (e.target.hasAttribute('data-copy-code')) {{
                    const text = e.target.getAttribute('data-copy-code');
                    copyCodeText(text);
                }} else if (e.target.hasAttribute('data-copy-code-id')) {{
                    const codeId = e.target.getAttribute('data-copy-code-id');
                    const codeElement = document.getElementById(codeId);
                    if (codeElement) {{
                        const text = codeElement.textContent || codeElement.innerText;
                        copyCodeText(text);
                    }}
                }}
            }});
            "#
        }
        
        div {
            style: "height:100%; overflow-y:auto; padding:24px 16px; scrollbar-width:thin; scrollbar-color:#cbd5e1 transparent;",
            
            div {
                style: "max-width:1400px; margin:0 auto; display:flex; flex-direction:column; gap:24px;",

                // 标题
                section {
                    style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:20px; padding:28px 32px; border:1px solid #fbbf24;",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#78350f;",
                        "🔐 验证码识别 API 文档"
                    }
                    p {
                        style: "color:#92400e; font-size:15px; margin:0;",
                        "支持 ddddocr + AntiCAP 双引擎，提供多种编程语言调用示例"
                    }
                }

                // 编程语言选择
                section {
                    style: "background:white; border-radius:18px; padding:24px 32px; border:1px solid #e5e7eb; box-shadow:0 6px 20px rgba(15,23,42,0.08);",
                    h2 {
                        style: "font-size:20px; font-weight:700; margin:0 0 16px 0; color:#111827;",
                        "选择编程语言"
                    }
                    div {
                        style: "display:flex; gap:12px; flex-wrap:wrap;",
                        LanguageButton { selected_lang, lang: CodeLanguage::Curl, label: "cURL" }
                        LanguageButton { selected_lang, lang: CodeLanguage::Python, label: "Python" }
                        LanguageButton { selected_lang, lang: CodeLanguage::JavaScript, label: "JavaScript" }
                        LanguageButton { selected_lang, lang: CodeLanguage::Go, label: "Go" }
                        LanguageButton { selected_lang, lang: CodeLanguage::Java, label: "Java" }
                        LanguageButton { selected_lang, lang: CodeLanguage::Rust, label: "Rust" }
                    }
                }

                // ddddocr 引擎
                EngineSection {
                    code_lang: selected_lang(),
                    engine: "ddddocr",
                }

                // AntiCAP 引擎
                EngineSection {
                    code_lang: selected_lang(),
                    engine: "AntiCAP",
                }
            }
        }
    }
}

#[component]
fn LanguageButton(mut selected_lang: Signal<CodeLanguage>, lang: CodeLanguage, label: &'static str) -> Element {
    let is_selected = selected_lang() == lang;
    
    rsx! {
        button {
            onclick: move |_| selected_lang.set(lang),
            style: if is_selected {
                "padding:10px 20px; border-radius:8px; background:#3b82f6; color:white; font-weight:600; border:none; cursor:pointer; font-size:14px;"
            } else {
                "padding:10px 20px; border-radius:8px; background:#f9fafb; color:#374151; font-weight:500; border:1px solid #e5e7eb; cursor:pointer; font-size:14px;"
            },
            "{label}"
        }
    }
}

#[component]
fn EngineSection(code_lang: CodeLanguage, engine: &'static str) -> Element {
    let is_ddddocr = engine == "ddddocr";
    
    rsx! {
        section {
            style: "background:white; border-radius:18px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 6px 20px rgba(15,23,42,0.08);",
            
            h2 {
                style: "font-size:22px; font-weight:700; margin:0 0 20px 0; color:#3b82f6;",
                {if is_ddddocr { "🔹 ddddocr 引擎" } else { "🔸 AntiCAP 引擎" }}
            }

            div {
                style: "display:grid; grid-template-columns:repeat(auto-fill, minmax(350px, 1fr)); gap:20px;",
                
                if is_ddddocr {
                    CaptchaTypeCard {
                        code_lang,
                        title: "普通验证码 (Text CAPTCHA)",
                        desc: "识别英数字验证码，支持标准、旧版、Beta 三种模式",
                        endpoint: "/api/captcha/ddddocr/ocr",
                        json_body: r#"{{"image": "data:image/png;base64,iVBORw0KG...", "type": "normal"}}"#,
                        mode_params: Some(vec![
                            ("normal", "标准模式 - 默认模式，适用于大多数验证码"),
                            ("old", "旧版模式 - 使用旧版 OCR 模型"),
                            ("beta", "Beta 模式 - 概率 OCR，返回置信度信息"),
                        ]),
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "目标检测 (Object Detection)",
                        desc: "检测图片中的目标物体，返回边界框和中心点坐标",
                        endpoint: "/api/captcha/ddddocr/det",
                        json_body: r#"{{"image": "data:image/png;base64,iVBORw0KG..."}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "滑块验证码 (Slider CAPTCHA)",
                        desc: "识别滑块位置，需要提供滑块图和背景图",
                        endpoint: "/api/captcha/ddddocr/slide",
                        json_body: r#"{{"target": "data:image/png;base64,...", "background": "data:image/png;base64,..."}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "点选验证码 (Click CAPTCHA)",
                        desc: "识别需要点击的目标位置，返回坐标列表",
                        endpoint: "/api/captcha/ddddocr/click",
                        json_body: r#"{{"image": "data:image/png;base64,..."}}"#,
                        mode_params: None,
                    }
                } else {
                    CaptchaTypeCard {
                        code_lang,
                        title: "通用 OCR",
                        desc: "识别通用文字验证码",
                        endpoint: "/api/captcha/anticap/ocr",
                        json_body: r#"{{"image": "data:image/png;base64,...", "type": "ocr"}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "算术验证码",
                        desc: "识别并计算算术表达式",
                        endpoint: "/api/captcha/anticap/ocr",
                        json_body: r#"{{"image": "data:image/png;base64,...", "type": "math"}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "图标检测",
                        desc: "检测图片中的图标位置",
                        endpoint: "/api/captcha/anticap/ocr",
                        json_body: r#"{{"image": "data:image/png;base64,...", "type": "detection_icon"}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "文字检测",
                        desc: "检测图片中的文字位置",
                        endpoint: "/api/captcha/anticap/ocr",
                        json_body: r#"{{"image": "data:image/png;base64,...", "type": "detection_text"}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "单图旋转",
                        desc: "识别图片需要旋转的角度",
                        endpoint: "/api/captcha/anticap/ocr",
                        json_body: r#"{{"image": "data:image/png;base64,...", "type": "single_rotate"}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "缺口滑块",
                        desc: "识别缺口滑块的位置",
                        endpoint: "/api/captcha/anticap/slide",
                        json_body: r#"{{"target": "data:image/png;base64,...", "background": "data:image/png;base64,...", "mode": "match"}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "阴影滑块",
                        desc: "识别阴影滑块的位置",
                        endpoint: "/api/captcha/anticap/slide",
                        json_body: r#"{{"target": "data:image/png;base64,...", "background": "data:image/png;base64,...", "mode": "comparison"}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "双图旋转",
                        desc: "识别内外圆图的旋转角度",
                        endpoint: "/api/captcha/anticap/rotate",
                        json_body: r#"{{"inside": "data:image/png;base64,...", "outside": "data:image/png;base64,..."}}"#,
                        mode_params: None,
                    }
                    CaptchaTypeCard {
                        code_lang,
                        title: "图片相似度",
                        desc: "计算两张图片的相似度",
                        endpoint: "/api/captcha/anticap/compare",
                        json_body: r#"{{"image1": "data:image/png;base64,...", "image2": "data:image/png;base64,..."}}"#,
                        mode_params: None,
                    }
                }
            }
        }
    }
}

#[component]
fn CaptchaTypeCard(
    code_lang: CodeLanguage,
    title: &'static str,
    desc: &'static str,
    endpoint: &'static str,
    json_body: &'static str,
    mode_params: Option<Vec<(&'static str, &'static str)>>,
) -> Element {
    let mut show_code = use_signal(|| false);
    let mut copy_feedback = use_signal(|| false);
    
    rsx! {
        div {
            style: "background:#f9fafb; border-radius:12px; padding:20px; border:1px solid #e5e7eb;",
            
            h3 {
                style: "font-size:16px; font-weight:700; margin:0 0 8px 0; color:#111827;",
                "{title}"
            }
            
            p {
                style: "font-size:13px; color:#6b7280; margin:0 0 12px 0; line-height:1.5;",
                "{desc}"
            }
            
            // 模式参数说明
            if let Some(modes) = mode_params {
                div {
                    style: "background:#eff6ff; border-left:3px solid #3b82f6; border-radius:4px; padding:12px; margin-bottom:12px;",
                    div {
                        style: "font-size:12px; font-weight:600; color:#1e40af; margin-bottom:8px;",
                        "📌 模式参数说明："
                    }
                    div {
                        style: "display:flex; flex-direction:column; gap:6px;",
                        for (mode_value, mode_desc) in modes {
                            div {
                                style: "display:flex; gap:8px; align-items:flex-start;",
                                code {
                                    style: "background:#dbeafe; color:#1e3a8a; padding:2px 6px; border-radius:3px; font-size:11px; font-weight:600; font-family:monospace; min-width:60px; text-align:center;",
                                    "\"{mode_value}\""
                                }
                                span {
                                    style: "font-size:12px; color:#475569; line-height:1.4; flex:1;",
                                    "{mode_desc}"
                                }
                            }
                        }
                    }
                }
            }
            
            div {
                style: "background:#1f2937; border-radius:6px; padding:8px 12px; margin-bottom:12px;",
                code {
                    style: "font-size:12px; color:#10b981; font-family:monospace;",
                    "POST {endpoint}"
                }
            }
            
            div {
                style: "display:flex; gap:8px;",
                button {
                    onclick: move |_| show_code.set(!show_code()),
                    style: "flex:1; padding:8px; border-radius:6px; background:#3b82f6; color:white; font-size:13px; font-weight:600; border:none; cursor:pointer; transition:background 0.2s;",
                    onmouseenter: move |_| {},
                    onmouseleave: move |_| {},
                    {if show_code() { "隐藏代码" } else { "查看代码" }}
                }
            }
            
            if show_code() {
                {
                    let code_example = generate_code_example(code_lang, endpoint, json_body);
                    // 生成唯一的 ID 用于定位代码元素
                    let code_id = format!("code-{}-{}", title.replace(" ", "-").replace("(", "").replace(")", "").replace("/", "-"), code_lang as u8);
                    rsx! {
                        div {
                            style: "position:relative; margin-top:12px;",
                            pre {
                                id: "{code_id}",
                                style: "margin:0; padding:12px 50px 12px 12px; background:#1f2937; border-radius:6px; overflow-x:auto; font-size:11px; color:#e5e7eb; font-family:monospace; line-height:1.5; white-space:pre-wrap; word-wrap:break-word;",
                                "{code_example}"
                            }
                            button {
                                "data-copy-code-id": "{code_id}",
                                onclick: move |_| {
                                    copy_feedback.set(true);
                                    // 2秒后重置反馈
                                    spawn(async move {
                                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                                        copy_feedback.set(false);
                                    });
                                },
                                style: "position:absolute; top:8px; right:8px; padding:6px 12px; border-radius:6px; background:rgba(59,130,246,0.9); color:white; font-size:12px; font-weight:600; border:none; cursor:pointer; transition:all 0.2s; backdrop-filter:blur(4px);",
                                onmouseenter: move |_| {},
                                onmouseleave: move |_| {},
                                {if copy_feedback() { "✓ 已复制" } else { "📋 复制" }}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn generate_code_example(lang: CodeLanguage, endpoint: &str, json_body: &str) -> String {
    let url = format!("http://localhost:8080{}", endpoint);
    
    match lang {
        CodeLanguage::Curl => format!(
            r#"curl -X POST '{}' \
  -H 'Content-Type: application/json' \
  -d '{}'"#,
            url, json_body
        ),
        CodeLanguage::Python => format!(
            r#"import requests

response = requests.post(
    '{}',
    json={}
)
result = response.json()
print(result)"#,
            url, json_body
        ),
        CodeLanguage::JavaScript => format!(
            r#"const response = await fetch('{}', {{{{
  method: 'POST',
  headers: {{{{ 'Content-Type': 'application/json' }}}},
  body: JSON.stringify({})
}}}});
const result = await response.json();
console.log(result);"#,
            url, json_body
        ),
        CodeLanguage::Go => format!(
            r#"package main

import (
    "bytes"
    "encoding/json"
    "net/http"
)

func main() {{{{
    jsonData := []byte(`{}`)
    resp, _ := http.Post("{}", "application/json", bytes.NewBuffer(jsonData))
    defer resp.Body.Close()
    
    var result map[string]interface{{{{}}}}
    json.NewDecoder(resp.Body).Decode(&result)
}}}}"#,
            json_body, url
        ),
        CodeLanguage::Java => format!(
            r#"import java.net.http.*;
import java.net.URI;

HttpClient client = HttpClient.newHttpClient();
HttpRequest request = HttpRequest.newBuilder()
    .uri(URI.create("{}"))
    .header("Content-Type", "application/json")
    .POST(HttpRequest.BodyPublishers.ofString("{}"))
    .build();

HttpResponse<String> response = client.send(request, 
    HttpResponse.BodyHandlers.ofString());
System.out.println(response.body());"#,
            url, json_body.replace("\"", "\\\"")
        ),
        CodeLanguage::Rust => format!(
            r#"use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{{{
    let client = reqwest::Client::new();
    let response = client
        .post("{}")
        .json(&json!({}))
        .send()
        .await?;
    
    let result: serde_json::Value = response.json().await?;
    println!("{{{{:?}}}}", result);
    Ok(())
}}}}"#,
            url, json_body
        ),
    }
}


