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
                style: "display:flex; flex:1 1 auto; background:rgba(2,6,23,0.85); border-top:1px solid rgba(15,23,42,0.9); overflow:hidden;",

                // 左侧窄侧边栏（仅图标）
                div {
                    style: "width:80px; background:linear-gradient(195deg,rgba(15,23,42,0.95),rgba(2,6,23,0.8)); color:#f8fafc; display:flex; flex-direction:column; align-items:center; padding:16px 0; border-right:1px solid rgba(148,163,184,0.12); gap:12px;",

                    // 顶部 Logo
                    div {
                        style: "width:48px; height:48px; border-radius:16px; background:linear-gradient(135deg,#6366f1,#22d3ee); display:flex; align-items:center; justify-content:center; font-weight:700; font-size:20px; color:#0f172a; box-shadow:0 12px 30px rgba(14,165,233,0.4);",
                        "F"
                    }

                    // 中间主导航图标
                    nav {
                        style: "display:flex; flex-direction:column; align-items:center; gap:10px; margin-top:12px; flex:1; width:100%; padding-inline:8px;",

                        SidebarIcon { icon: "🏠", label: "首页", to: Some(Route::Home {}) }
                        SidebarIcon { icon: "✈️", label: "航司报价", to: Some(Route::Airline {}) }
                        SidebarIcon { icon: "⚙️", label: "配置管理", to: Some(Route::Config {}) }
                        SidebarIcon { icon: "🛰️", label: "可用服务", to: None }
                        SidebarIcon { icon: "🧰", label: "注册机", to: None }
                        SidebarIcon { icon: "🔐", label: "验证码", to: None }
                    }

                    // 底部辅助图标（帮助、设置等）
                    div {
                        style: "display:flex; flex-direction:column; align-items:center; gap:10px; padding-top:12px; border-top:1px solid rgba(30,41,59,0.65); width:100%; padding-inline:8px;",
                        SidebarIcon { icon: "❓", label: "帮助", to: None }
                        SidebarIcon { icon: "🔔", label: "通知", to: None }
                    }
                }

                // 右侧内容区域，通过 Outlet 渲染当前路由页面
                div {
                    style: "flex:1; min-width:0; padding:32px 40px; overflow:hidden; background:rgba(15,23,42,0.4); color:#e2e8f0; backdrop-filter:blur(30px); border-left:1px solid rgba(148,163,184,0.12);",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

/// 自定义标题栏，带最小化 / 最大化 / 关闭按钮
#[component]
fn TitleBar() -> Element {
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

            // 左侧标题
            div {
                style: "display:flex; align-items:center; gap:10px; font-size:13px; letter-spacing:0.4px;",
                span {
                    style: "width:10px; height:10px; border-radius:999px; background:linear-gradient(135deg,#4f46e5,#22c55e); box-shadow:0 0 12px rgba(59,130,246,0.9);",
                }
                span {
                    style: "font-weight:600; text-transform:uppercase; color:#f8fafc;",
                    "FactBot 控制台"
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
