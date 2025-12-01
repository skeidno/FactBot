use crate::Route;
use dioxus::prelude::*;

/// 布局组件：顶部自定义标题栏 + 左侧窄竖条图标栏 + 右侧内容区域
#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            style: "display:flex; flex-direction:column; height:100%; min-height:100vh; width:100%; background:transparent; color:#e2e8f0; font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif; box-sizing:border-box;",

            // 顶部标题栏（含最小化/最大化/关闭）
            TitleBar {}

            // 主体区域：左侧侧边栏 + 右侧内容
            div {
                style: "display:flex; flex:1 1 0; min-height:0; background:rgba(2,6,23,0.85); border-top:1px solid rgba(15,23,42,0.9); overflow:hidden;",

                // 左侧窄侧边栏（仅图标）
                div {
                    style: "width:80px; min-width:80px; max-width:80px; background:linear-gradient(195deg,rgba(15,23,42,0.95),rgba(2,6,23,0.8)); color:#f8fafc; display:flex; flex-direction:column; align-items:center; padding:16px 0; border-right:1px solid rgba(148,163,184,0.12); gap:12px; flex-shrink:0;",

                    // 顶部 Logo - 用户头像
                    UserAvatar { size: 48 }

                    // 中间主导航图标
                    nav {
                        style: "display:flex; flex-direction:column; align-items:center; gap:10px; margin-top:12px; flex:1; width:100%; padding-inline:8px;",

                        SidebarIcon { icon: "🏠", label: "首页", to: Some(Route::Home {}) }
                        SidebarIcon { icon: "✈️", label: "航司报价", to: Some(Route::Airline {}) }
                        SidebarIcon { icon: "🔤", label: "验证码识别", to: Some(Route::Captcha {}) }
                        SidebarIcon { icon: "⚙️", label: "配置管理", to: Some(Route::ConfigView {}) }
                        SidebarIcon { icon: "📋", label: "日志查看", to: Some(Route::Logs {}) }
                    }

                    // 底部辅助图标（帮助、通知等）
                    div {
                        style: "display:flex; flex-direction:column; align-items:center; gap:10px; padding-top:12px; border-top:1px solid rgba(30,41,59,0.65); width:100%; padding-inline:8px;",
                        SidebarIcon { icon: "🔔", label: "通知", to: Some(Route::Notice {}) }
                        SidebarIcon { icon: "❓", label: "帮助", to: Some(Route::Help {}) }
                    }
                }

                // 右侧内容区域，通过 Outlet 渲染当前路由页面
                div {
                    style: "flex:1 1 0; min-width:0; width:0; padding:32px 40px; overflow:hidden; background:rgba(15,23,42,0.4); color:#e2e8f0; backdrop-filter:blur(30px); border-left:1px solid rgba(148,163,184,0.12); box-sizing:border-box;",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

/// 自定义标题栏，带最小化 / 最大化 / 关闭按钮
#[component]
fn TitleBar() -> Element {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    rsx! {
        div {
            style: "height:42px; display:flex; align-items:center; justify-content:space-between; padding:0 18px; background:linear-gradient(120deg,#020617,#020617,#0b1120); color:#e5e7eb; border-bottom:1px solid rgba(15,23,42,0.9); user-select:none; cursor:grab;",
            onpointerdown: move |_| {
                #[cfg(feature = "desktop")]
                {
                    let window = dioxus_desktop::window();
                    window.drag();
                }
            },

            // 左侧 Logo 和版本号
            div {
                style: "display:flex; align-items:center; gap:12px;",
                // Logo - 机器人图标
                RobotIcon { size: 28 }
                // 标题和版本
                div {
                    style: "display:flex; flex-direction:column; gap:2px;",
                    span {
                        style: "font-weight:700; font-size:14px; color:#f8fafc; letter-spacing:0.3px;",
                        "FactBot"
                    }
                    span {
                        style: "font-size:11px; color:#94a3b8; font-weight:500;",
                        "v{VERSION}"
                    }
                }
            }

            // 右侧窗口控制按钮
            div {
                style: "display:flex; align-items:center; gap:6px; font-size:11px;",

                // 最小化
                button {
                    style: TITLE_BTN_STYLE,
                    onpointerdown: move |evt| evt.stop_propagation(),
                    onclick: move |_| {
                        #[cfg(feature = "desktop")]
                        {
                            let window = dioxus_desktop::window();
                            window.set_minimized(true);
                        }
                    },
                    "─"
                }

                // 最大化 / 还原
                button {
                    style: TITLE_BTN_STYLE,
                    onpointerdown: move |evt| evt.stop_propagation(),
                    onclick: move |_| {
                        #[cfg(feature = "desktop")]
                        {
                            let window = dioxus_desktop::window();
                            let is_max = window.is_maximized();
                            window.set_maximized(!is_max);
                        }
                    },
                    "▢"
                }

                // 关闭
                button {
                    style: TITLE_BTN_CLOSE_STYLE,
                    onpointerdown: move |evt| evt.stop_propagation(),
                    onclick: move |_| {
                        #[cfg(feature = "desktop")]
                        {
                            let window = dioxus_desktop::window();
                            window.close();
                        }
                    },
                    "✕"
                }
            }
        }
    }
}

const TITLE_BTN_STYLE: &str = "width:28px; height:24px; border-radius:999px; border:1px solid rgba(148,163,184,0.45); background:rgba(15,23,42,0.65); color:#e5e7eb; cursor:pointer; font-size:11px; display:flex; align-items:center; justify-content:center; padding:0 0 1px 0; box-shadow:0 4px 12px rgba(2,6,23,0.55); transition:background 120ms ease;";
const TITLE_BTN_CLOSE_STYLE: &str = "width:28px; height:24px; border-radius:999px; border:1px solid rgba(248,113,113,0.9); background:linear-gradient(135deg,#fb7185,#b91c1c); color:#fef2f2; cursor:pointer; font-size:11px; display:flex; align-items:center; justify-content:center; padding:0 0 1px 0; box-shadow:0 4px 16px rgba(127,29,29,0.65);";

/// 单个图标按钮，带悬浮气泡提示
/// - 如果 `to` 为 Some，则使用 Link 进行路由跳转
/// - 如果 `to` 为 None，则渲染为普通按钮（目前作为占位，将来可以绑定事件）
#[component]
fn SidebarIcon(icon: &'static str, label: &'static str, to: Option<Route>) -> Element {
    let mut hovering = use_signal(|| false);

    // 统一的图标按钮样式
    let base_style = "width:56px; height:56px; border-radius:16px; display:flex; align-items:center;\
                      justify-content:center; font-size:26px; cursor:pointer; border:1px solid rgba(148,163,184,0.2);\
                      outline:none; background:rgba(15,23,42,0.6); color:inherit; text-decoration:none; box-shadow:0 10px 20px rgba(2,6,23,0.4);\
                      transition:transform 160ms ease, box-shadow 160ms ease, border-color 160ms ease, background 160ms ease;";

    let hover_style = if hovering() {
        "transform:translateY(-2px) scale(1.08); box-shadow:0 15px 30px rgba(79,70,229,0.5); border-color:rgba(99,102,241,0.6); background:rgba(79,70,229,0.3);"
    } else {
        ""
    };

    // 气泡提示样式
    let tooltip_style = if hovering() {
        "position:absolute; left:calc(100% + 12px); top:50%; transform:translateY(-50%); \
         background:linear-gradient(135deg,#1e293b,#0f172a); color:#f1f5f9; padding:8px 14px; \
         border-radius:10px; font-size:13px; font-weight:500; white-space:nowrap; \
         box-shadow:0 10px 25px rgba(0,0,0,0.5); border:1px solid rgba(148,163,184,0.2); \
         opacity:1; pointer-events:none; z-index:1000; \
         transition:opacity 200ms ease;"
    } else {
        "position:absolute; left:calc(100% + 12px); top:50%; transform:translateY(-50%); \
         background:linear-gradient(135deg,#1e293b,#0f172a); color:#f1f5f9; padding:8px 14px; \
         border-radius:10px; font-size:13px; font-weight:500; white-space:nowrap; \
         box-shadow:0 10px 25px rgba(0,0,0,0.5); border:1px solid rgba(148,163,184,0.2); \
         opacity:0; pointer-events:none; z-index:1000; \
         transition:opacity 200ms ease;"
    };

    match to {
        Some(route) => rsx! {
            div {
                style: "position:relative; display:flex; justify-content:center;",
                Link {
                    to: route,
                    style: "text-decoration:none; display:flex; justify-content:center;",
                    div {
                        style: "{base_style} {hover_style}",
                        onpointerenter: move |_| hovering.set(true),
                        onpointerleave: move |_| hovering.set(false),
                        "{icon}"
                    }
                }
                // 气泡提示
                div {
                    style: "{tooltip_style}",
                    "{label}"
                    // 小三角箭头
                    div {
                        style: "position:absolute; right:100%; top:50%; transform:translateY(-50%); \
                                width:0; height:0; border-top:6px solid transparent; border-bottom:6px solid transparent; \
                                border-right:6px solid #1e293b;",
                    }
                }
            }
        },
        None => rsx! {
            div {
                style: "position:relative; display:flex; justify-content:center;",
                button {
                    style: "{base_style} {hover_style} opacity:0.85;",
                    onpointerenter: move |_| hovering.set(true),
                    onpointerleave: move |_| hovering.set(false),
                    "{icon}"
                }
                // 气泡提示
                div {
                    style: "{tooltip_style}",
                    "{label}"
                    // 小三角箭头
                    div {
                        style: "position:absolute; right:100%; top:50%; transform:translateY(-50%); \
                                width:0; height:0; border-top:6px solid transparent; border-bottom:6px solid transparent; \
                                border-right:6px solid #1e293b;",
                    }
                }
            }
        },
    }
}


/// 用户头像组件 - 默认头像
#[component]
fn UserAvatar(size: u32) -> Element {
    let container_size = format!("width:{}px; height:{}px; border-radius:50%; background:linear-gradient(135deg,#6366f1,#8b5cf6); display:flex; align-items:center; justify-content:center; box-shadow:0 {}px {}px rgba(99,102,241,0.4); border:2px solid rgba(139,92,246,0.5);", 
        size, size, size / 6, size / 3);
    
    rsx! {
        div {
            style: "{container_size}",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                width: "60%",
                height: "60%",
                fill: "none",
                stroke: "#ffffff",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                
                // User icon
                path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                circle { cx: "12", cy: "7", r: "4" }
            }
        }
    }
}

/// 机器人图标组件 - 内联 SVG
#[component]
fn RobotIcon(size: u32) -> Element {
    let container_size = format!("width:{}px; height:{}px; border-radius:{}px; background:linear-gradient(135deg,#6366f1,#22d3ee); display:flex; align-items:center; justify-content:center; box-shadow:0 {}px {}px rgba(14,165,233,0.4); padding:{}px;", 
        size, size, size / 3, size / 4, size / 2, size / 8);
    
    rsx! {
        div {
            style: "{container_size}",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 128 128",
                width: "100%",
                height: "100%",
                
                defs {
                    linearGradient {
                        id: "robotGrad",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "100%",
                        stop { offset: "0%", stop_color: "#6366f1", stop_opacity: "1" }
                        stop { offset: "50%", stop_color: "#22d3ee", stop_opacity: "1" }
                        stop { offset: "100%", stop_color: "#0ea5e9", stop_opacity: "1" }
                    }
                }
                
                g {
                    transform: "translate(64, 64)",
                    
                    // Robot Head
                    rect {
                        x: "-24",
                        y: "-35",
                        width: "48",
                        height: "40",
                        rx: "8",
                        fill: "url(#robotGrad)",
                        stroke: "#22d3ee",
                        stroke_width: "3"
                    }
                    
                    // Antenna
                    line {
                        x1: "0",
                        y1: "-35",
                        x2: "0",
                        y2: "-48",
                        stroke: "#00ffaa",
                        stroke_width: "4",
                        stroke_linecap: "round"
                    }
                    circle {
                        cx: "0",
                        cy: "-52",
                        r: "5",
                        fill: "#00ffaa",
                        animate {
                            attribute_name: "opacity",
                            values: "1;0.3;1",
                            dur: "2s",
                            repeat_count: "indefinite"
                        }
                    }
                    
                    // Eyes
                    circle {
                        cx: "-10",
                        cy: "-24",
                        r: "5",
                        fill: "#00ffaa",
                        animate {
                            attribute_name: "fill",
                            values: "#00ffaa;#ffffff;#00ffaa",
                            dur: "3s",
                            repeat_count: "indefinite"
                        }
                    }
                    circle {
                        cx: "10",
                        cy: "-24",
                        r: "5",
                        fill: "#00ffaa",
                        animate {
                            attribute_name: "fill",
                            values: "#00ffaa;#ffffff;#00ffaa",
                            dur: "3s",
                            repeat_count: "indefinite"
                        }
                    }
                    
                    // Mouth
                    rect {
                        x: "-14",
                        y: "-10",
                        width: "28",
                        height: "7",
                        rx: "2",
                        fill: "rgba(15,23,42,0.9)",
                        stroke: "#22d3ee",
                        stroke_width: "2"
                    }
                    line {
                        x1: "-10",
                        y1: "-6.5",
                        x2: "-4",
                        y2: "-6.5",
                        stroke: "#00ffaa",
                        stroke_width: "2",
                        stroke_linecap: "round"
                    }
                    line {
                        x1: "-1",
                        y1: "-6.5",
                        x2: "5",
                        y2: "-6.5",
                        stroke: "#00ffaa",
                        stroke_width: "2",
                        stroke_linecap: "round"
                    }
                    line {
                        x1: "8",
                        y1: "-6.5",
                        x2: "11",
                        y2: "-6.5",
                        stroke: "#00ffaa",
                        stroke_width: "2",
                        stroke_linecap: "round"
                    }
                    
                    // Body
                    rect {
                        x: "-20",
                        y: "8",
                        width: "40",
                        height: "32",
                        rx: "6",
                        fill: "url(#robotGrad)",
                        stroke: "#22d3ee",
                        stroke_width: "3"
                    }
                    
                    // Chest Panel
                    rect {
                        x: "-12",
                        y: "14",
                        width: "24",
                        height: "20",
                        rx: "3",
                        fill: "rgba(15,23,42,0.9)",
                        stroke: "#22d3ee",
                        stroke_width: "2"
                    }
                    circle {
                        cx: "0",
                        cy: "24",
                        r: "7",
                        fill: "none",
                        stroke: "#00ffaa",
                        stroke_width: "2.5",
                        animate {
                            attribute_name: "stroke-dasharray",
                            values: "0,44;44,0;0,44",
                            dur: "4s",
                            repeat_count: "indefinite"
                        }
                    }
                    
                    // Arms
                    rect {
                        x: "-28",
                        y: "12",
                        width: "7",
                        height: "24",
                        rx: "3.5",
                        fill: "url(#robotGrad)",
                        stroke: "#22d3ee",
                        stroke_width: "2"
                    }
                    rect {
                        x: "21",
                        y: "12",
                        width: "7",
                        height: "24",
                        rx: "3.5",
                        fill: "url(#robotGrad)",
                        stroke: "#22d3ee",
                        stroke_width: "2"
                    }
                }
            }
        }
    }
}
