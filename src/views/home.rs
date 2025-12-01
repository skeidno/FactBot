use crate::Route;
use dioxus::prelude::*;

/// 欢迎页：突出产品定位，并引导用户进入航司报价页面
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            style: "height:100%; display:flex; flex-direction:column; align-items:center; justify-content:center; padding:48px; box-sizing:border-box; overflow:auto;",
            
            // 内容卡片
            div {
                style: "max-width:720px; width:100%; background:linear-gradient(135deg, rgba(30,41,59,0.95), rgba(15,23,42,0.95)); border-radius:28px; padding:48px; box-shadow:0 25px 60px rgba(0,0,0,0.4), 0 0 0 1px rgba(148,163,184,0.1); text-align:center; backdrop-filter:blur(20px);",
                
                HeaderLogo {}
                
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
                        to: Route::ConfigView {},
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

/// Header Logo 组件 - 内联 SVG
#[component]
fn HeaderLogo() -> Element {
    rsx! {
        div {
            style: "max-width:500px; width:100%; margin:0 auto 16px;",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 800 200",
                width: "100%",
                height: "auto",
                style: "shape-rendering: geometricPrecision;",
                
                defs {
                    linearGradient {
                        id: "robotGradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "100%",
                        stop { offset: "0%", style: "stop-color:#6366f1;stop-opacity:1" }
                        stop { offset: "50%", style: "stop-color:#22d3ee;stop-opacity:1" }
                        stop { offset: "100%", style: "stop-color:#0ea5e9;stop-opacity:1" }
                    }
                    linearGradient {
                        id: "textGradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "0%",
                        stop { offset: "0%", style: "stop-color:#f8fafc;stop-opacity:1" }
                        stop { offset: "100%", style: "stop-color:#cbd5e1;stop-opacity:1" }
                    }
                    linearGradient {
                        id: "accentGradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "0%",
                        stop { offset: "0%", style: "stop-color:#00ffaa;stop-opacity:1" }
                        stop { offset: "100%", style: "stop-color:#22d3ee;stop-opacity:1" }
                    }
                    filter {
                        id: "glow",
                        feGaussianBlur { std_deviation: "2", result: "coloredBlur" }
                        feMerge {
                            feMergeNode { _in: "coloredBlur" }
                            feMergeNode { _in: "SourceGraphic" }
                        }
                    }
                }
                
                rect { width: "800", height: "200", fill: "transparent" }
                
                g {
                    transform: "translate(100, 100)",
                    
                    rect { x: "-35", y: "-50", width: "70", height: "55", rx: "10", fill: "url(#robotGradient)", stroke: "#22d3ee", stroke_width: "2.5" }
                    
                    line { x1: "0", y1: "-50", x2: "0", y2: "-68", stroke: "#00ffaa", stroke_width: "3.5", stroke_linecap: "round" }
                    circle {
                        cx: "0",
                        cy: "-72",
                        r: "6",
                        fill: "#00ffaa",
                        animate { attribute_name: "opacity", values: "1;0.3;1", dur: "2s", repeat_count: "indefinite" }
                    }
                    
                    circle {
                        cx: "-14",
                        cy: "-32",
                        r: "7",
                        fill: "#00ffaa",
                        animate { attribute_name: "fill", values: "#00ffaa;#ffffff;#00ffaa", dur: "3s", repeat_count: "indefinite" }
                    }
                    circle {
                        cx: "14",
                        cy: "-32",
                        r: "7",
                        fill: "#00ffaa",
                        animate { attribute_name: "fill", values: "#00ffaa;#ffffff;#00ffaa", dur: "3s", repeat_count: "indefinite" }
                    }
                    
                    rect { x: "-20", y: "-13", width: "40", height: "9", rx: "2.5", fill: "rgba(15,23,42,0.8)", stroke: "#22d3ee", stroke_width: "1.5" }
                    line { x1: "-14", y1: "-8.5", x2: "-7", y2: "-8.5", stroke: "#00ffaa", stroke_width: "2.5", stroke_linecap: "round" }
                    line { x1: "-2", y1: "-8.5", x2: "5", y2: "-8.5", stroke: "#00ffaa", stroke_width: "2.5", stroke_linecap: "round" }
                    line { x1: "10", y1: "-8.5", x2: "16", y2: "-8.5", stroke: "#00ffaa", stroke_width: "2.5", stroke_linecap: "round" }
                    
                    rect { x: "-30", y: "10", width: "60", height: "48", rx: "8", fill: "url(#robotGradient)", stroke: "#22d3ee", stroke_width: "2.5" }
                    
                    rect { x: "-18", y: "18", width: "36", height: "30", rx: "4", fill: "rgba(15,23,42,0.8)", stroke: "#22d3ee", stroke_width: "1.5" }
                    circle {
                        cx: "0",
                        cy: "33",
                        r: "10",
                        fill: "none",
                        stroke: "url(#accentGradient)",
                        stroke_width: "2.5",
                        animate { attribute_name: "stroke-dasharray", values: "0,63;63,0;0,63", dur: "4s", repeat_count: "indefinite" }
                    }
                    
                    rect { x: "-42", y: "14", width: "10", height: "36", rx: "5", fill: "url(#robotGradient)", stroke: "#22d3ee", stroke_width: "1.5" }
                    rect { x: "32", y: "14", width: "10", height: "36", rx: "5", fill: "url(#robotGradient)", stroke: "#22d3ee", stroke_width: "1.5" }
                    
                    circle { cx: "-8", cy: "25", r: "2", fill: "#00ffaa", opacity: "0.6" }
                    circle { cx: "8", cy: "25", r: "2", fill: "#00ffaa", opacity: "0.6" }
                }
                
                text {
                    x: "200",
                    y: "110",
                    font_family: "'Segoe UI', Arial, sans-serif",
                    font_size: "56",
                    font_weight: "bold",
                    fill: "url(#textGradient)",
                    "FactBot"
                }
                
                text {
                    x: "200",
                    y: "145",
                    font_family: "'Segoe UI', Arial, sans-serif",
                    font_size: "20",
                    fill: "#22d3ee",
                    opacity: "0.9",
                    "数据运维助手 · Data Operations Assistant"
                }
                
                g {
                    opacity: "0.4",
                    line {
                        x1: "200",
                        y1: "160",
                        x2: "280",
                        y2: "160",
                        stroke: "url(#accentGradient)",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        animate { attribute_name: "x2", values: "200;280;200", dur: "3s", repeat_count: "indefinite" }
                    }
                    line {
                        x1: "290",
                        y1: "160",
                        x2: "350",
                        y2: "160",
                        stroke: "url(#accentGradient)",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        animate { attribute_name: "x2", values: "290;350;290", dur: "3s", begin: "0.5s", repeat_count: "indefinite" }
                    }
                    line {
                        x1: "360",
                        y1: "160",
                        x2: "420",
                        y2: "160",
                        stroke: "url(#accentGradient)",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        animate { attribute_name: "x2", values: "360;420;360", dur: "3s", begin: "1s", repeat_count: "indefinite" }
                    }
                }
                
                g {
                    opacity: "0.3",
                    fill: "#22d3ee",
                    circle {
                        cx: "280",
                        cy: "160",
                        r: "3",
                        animate { attribute_name: "opacity", values: "0.3;1;0.3", dur: "3s", repeat_count: "indefinite" }
                    }
                    circle {
                        cx: "350",
                        cy: "160",
                        r: "3",
                        animate { attribute_name: "opacity", values: "0.3;1;0.3", dur: "3s", begin: "0.5s", repeat_count: "indefinite" }
                    }
                    circle {
                        cx: "420",
                        cy: "160",
                        r: "3",
                        animate { attribute_name: "opacity", values: "0.3;1;0.3", dur: "3s", begin: "1s", repeat_count: "indefinite" }
                    }
                }
            }
        }
    }
}
