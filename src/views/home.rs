use crate::Route;
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

/// 欢迎页：突出产品定位，并引导用户进入航司报价页面
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            style: "height:100%; display:flex; flex-direction:column; align-items:center; justify-content:center; padding:48px; box-sizing:border-box; overflow:auto;",
            
            // Logo 区域 - 使用 header.svg
            div {
                style: "width:100%; max-width:800px; margin-bottom:40px; display:flex; justify-content:center;",
                img {
                    src: HEADER_SVG,
                    alt: "FactBot Logo",
                    style: "width:100%; height:auto; max-width:700px; filter:drop-shadow(0 20px 40px rgba(99,102,241,0.3));",
                }
            }
            
            // 内容卡片
            div {
                style: "max-width:720px; width:100%; background:linear-gradient(135deg, rgba(30,41,59,0.95), rgba(15,23,42,0.95)); border-radius:28px; padding:48px; box-shadow:0 25px 60px rgba(0,0,0,0.4), 0 0 0 1px rgba(148,163,184,0.1); text-align:center; backdrop-filter:blur(20px);",
                
                span {
                    style: "display:inline-flex; padding:8px 18px; border-radius:999px; background:linear-gradient(135deg,rgba(99,102,241,0.2),rgba(34,211,238,0.2)); color:#22d3ee; font-size:13px; font-weight:600; border:1px solid rgba(34,211,238,0.3);",
                    "🤖 欢迎来到 FactBot · 数据运维助手"
                }
                
                h1 {
                    style: "margin:28px 0 16px; font-size:36px; font-weight:700; background:linear-gradient(135deg,#f8fafc,#cbd5e1); -webkit-background-clip:text; -webkit-text-fill-color:transparent; background-clip:text;",
                    "集中管理多家航司报价、代理与接口配置"
                }
                
                p {
                    style: "margin:0 auto 32px; max-width:540px; color:#94a3b8; font-size:16px; line-height:1.8;",
                    "FactBot 提供统一的工作台，帮助你在一个界面完成代理配置、Token 维护以及各航司查询。通过左侧侧边栏即可随时切换到航司功能页。"
                }
                
                // 功能特性卡片
                div {
                    style: "display:grid; grid-template-columns:repeat(auto-fit, minmax(200px, 1fr)); gap:16px; margin-bottom:32px;",
                    
                    FeatureCard {
                        icon: "✈️",
                        title: "航司报价",
                        desc: "支持16家航司查询"
                    }
                    FeatureCard {
                        icon: "⚙️",
                        title: "配置管理",
                        desc: "统一管理代理和卡片"
                    }
                    FeatureCard {
                        icon: "🔒",
                        title: "安全可靠",
                        desc: "本地存储配置信息"
                    }
                }
                
                div {
                    style: "display:flex; flex-wrap:wrap; justify-content:center; gap:16px;",
                    Link {
                        to: Route::Airline {},
                        style: "padding:14px 34px; border-radius:18px; font-size:15px; font-weight:600; color:white; background:linear-gradient(135deg,#6366f1,#22d3ee); text-decoration:none; box-shadow:0 10px 25px rgba(99,102,241,0.4); transition:transform 0.2s, box-shadow 0.2s;",
                        "进入航司报价 →"
                    }
                    Link {
                        to: Route::Config {},
                        style: "padding:14px 28px; border-radius:18px; font-size:15px; font-weight:600; color:#22d3ee; background:rgba(34,211,238,0.1); border:1px solid rgba(34,211,238,0.3); text-decoration:none; transition:background 0.2s;",
                        "配置管理"
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, desc: &'static str) -> Element {
    rsx! {
        div {
            style: "padding:20px; border-radius:16px; background:rgba(15,23,42,0.6); border:1px solid rgba(148,163,184,0.1); text-align:center; transition:transform 0.2s, box-shadow 0.2s;",
            
            div {
                style: "font-size:32px; margin-bottom:8px;",
                "{icon}"
            }
            div {
                style: "font-size:15px; font-weight:600; color:#f8fafc; margin-bottom:4px;",
                "{title}"
            }
            div {
                style: "font-size:13px; color:#94a3b8;",
                "{desc}"
            }
        }
    }
}
