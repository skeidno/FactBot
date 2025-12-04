use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn Help(cx: Scope) -> Element {
    rsx! {
        div {
            style: "height:100%; overflow-y:auto; padding:24px 16px;",

            div {
                style: "max-width:1200px; margin:0 auto; display:flex; flex-direction:column; gap:24px;",

                section {
                    style: "background:linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%); border-radius:20px; padding:28px 32px; border:1px solid #60a5fa;",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#1e3a8a;",
                        "❓ 帮助中心"
                    }
                    p {
                        style: "color:#1e40af; font-size:15px; margin:0;",
                        "了解 FactBot 的各项功能和使用方法"
                    }
                }

                HelpSection {
                    title: "✈️ 航司报价查询",
                    color: "#3b82f6",
                }

                HelpSection {
                    title: "⚙️ 配置管理",
                    color: "#8b5cf6",
                }

                HelpSection {
                    title: "💾 数据存储",
                    color: "#10b981",
                }

                // 新增的验证码接口帮助部分
                HelpSection {
                    title: "🔐 验证码识别 API",
                    color: "#fbbf24",
                }

                section {
                    style: "background:linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%); border-radius:18px; padding:24px 28px; border:1px solid #86efac; text-align:center;",
                    h3 {
                        style: "font-size:18px; font-weight:700; margin:0 0 12px 0; color:#166534;",
                        "需要更多帮助？"
                    }
                    p {
                        style: "margin:0; color:#15803d; font-size:14px;",
                        "如有问题或建议，请联系技术支持团队"
                    }
                }
            }
        }
    }
}

#[component]
fn HelpSection(title: &'static str, color: &'static str) -> Element {
    let link = if title == "🔐 验证码识别 API" {
        Some("/captcha/help")
    } else {
        None
    };

    rsx! {
        section {
            style: "background:white; border-radius:18px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 6px 20px rgba(15,23,42,0.08);",
            h2 {
                style: "font-size:22px; font-weight:700; margin:0 0 20px 0; color:{color};",
                a {
                    href: "{link}",
                    "{title}"
                }
            }

            div {
                style: "display:flex; flex-direction:column; gap:14px;",

                if title.contains("航司") {
                    HelpItem { label: "选择代理分组", desc: "从配置管理页面设置的代理分组中选择，系统会随机使用分组中的一个代理" }
                    HelpItem { label: "输入 Token", desc: "配置访问航司 API 所需的认证 Token" }
                    HelpItem { label: "选择航司", desc: "支持 16 家航司，包括美国航空、韩亚航空、巴拿马航空等" }
                    HelpItem { label: "立即查询", desc: "点击查询按钮发送请求，系统会随机选择分组中的一个代理使用" }
                } else if title.contains("配置") {
                    HelpItem { label: "代理配置", desc: "支持分组管理，每个分组可添加多个代理。支持批量导入，格式：ip:port:username:password" }
                    HelpItem { label: "OTP 邮箱", desc: "配置用于接收验证码的邮箱服务，支持多个配置分组" }
                    HelpItem { label: "支付卡片", desc: "管理信用卡和礼品卡信息，支持分组批量保存" }
                    HelpItem { label: "购票人信息", desc: "保存常用购票人信息，包括姓名、邮箱、电话、护照等" }
                } else if title.contains("存储") {
                    HelpItem { label: "存储位置", desc: "所有配置数据存储在：文档/FactBot/config.db" }
                    HelpItem { label: "自动保存", desc: "点击保存按钮后数据会持久化到数据库" }
                    HelpItem { label: "自动加载", desc: "应用启动时自动从数据库加载所有配置" }
                    HelpItem { label: "数据安全", desc: "使用 SQLite 本地数据库，数据仅存储在本地" }
                } else if title.contains("验证码") {
                    p {
                        style: "margin:0; font-size:14px; color:#6b7280; line-height:1.6;",
                        "验证码识别 API 提供了多种编程语言的调用示例，点击标题进入详细页面查看。"
                    }
                }
            }
        }
    }
}

#[component]
fn HelpItem(label: &'static str, desc: &'static str) -> Element {
    rsx! {
        div {
            style: "display:flex; gap:12px; align-items:flex-start;",
            div {
                style: "width:6px; height:6px; border-radius:999px; background:#3b82f6; margin-top:8px;",
            }
            div {
                strong {
                    style: "display:block; font-size:15px; font-weight:600; color:#111827; margin-bottom:4px;",
                    "{label}"
                }
                p {
                    style: "margin:0; font-size:14px; color:#6b7280; line-height:1.6;",
                    "{desc}"
                }
            }
        }
    }
}
