use crate::Route;
use dioxus::prelude::*;

/// 布局组件：顶部自定义标题栏 + 左侧窄竖条图标栏 + 右侧内容区域
#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            style: "display:flex; flex-direction:column; height:100vh; background:#f5f5f5; color:#111827; font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;",

            // 顶部标题栏（含最小化/最大化/关闭）
            TitleBar {}

            // 主体区域：左侧侧边栏 + 右侧内容
            div {
                style: "display:flex; flex:1 1 auto;",

                // 左侧窄侧边栏（仅图标）
                div {
                    style: "width:72px; background:#111827; color:#e5e7eb; display:flex; flex-direction:column; align-items:center; padding:12px 0; box-shadow:2px 0 6px rgba(15,23,42,0.5);",

                    // 顶部 Logo
                    div {
                        style: "width:40px; height:40px; border-radius:16px; background:#4f46e5; display:flex; align-items:center; justify-content:center; font-weight:700; font-size:18px; color:white; margin-bottom:16px;",
                        "F"
                    }

                    // 中间主导航图标
                    nav {
                        style: "display:flex; flex-direction:column; align-items:center; gap:10px; margin-top:8px; flex:1;",

                        SidebarIcon { icon: "🏠", label: "首页", to: Some(Route::Home {}) }
                        SidebarIcon { icon: "📝", label: "博客示例", to: Some(Route::Blog { id: 1 }) }
                        SidebarIcon { icon: "✈️", label: "航司报价", to: Some(Route::Airline {}) }
                        SidebarIcon { icon: "☁️", label: "云服务（预留）", to: None }
                        SidebarIcon { icon: "🔔", label: "通知（预留）", to: None }
                    }

                    // 底部辅助图标（帮助、设置等）
                    div {
                        style: "display:flex; flex-direction:column; align-items:center; gap:10px; padding-top:12px; border-top:1px solid #1f2937;",
                        SidebarIcon { icon: "❓", label: "帮助（预留）", to: None }
                        SidebarIcon { icon: "⚙️", label: "设置（预留）", to: None }
                    }
                }

                // 右侧内容区域，通过 Outlet 渲染当前路由页面
                div {
                    style: "flex:1; min-width:0; padding:16px 24px; overflow-y:auto; background:white;",
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
            style: "height:32px; display:flex; align-items:center; justify-content:space-between; padding:0 10px 0 14px; background:linear-gradient(90deg,#020617,#020617,#111827); color:#e5e7eb;",

            // 左侧标题
            div {
                style: "display:flex; align-items:center; gap:8px; font-size:13px;",
                span {
                    style: "width:8px; height:8px; border-radius:999px; background:linear-gradient(135deg,#4f46e5,#22c55e); box-shadow:0 0 8px rgba(59,130,246,0.8);",
                }
                span {
                    "FactBot · 面板"
                }
            }

            // 右侧窗口控制按钮
            div {
                style: "display:flex; align-items:center; gap:6px; font-size:11px;",

                // 最小化
                button {
                    style: TITLE_BTN_STYLE,
                    onclick: move |_| {
                        #[cfg(feature = "desktop")]
                        {
                            let window = dioxus::desktop::use_window();
                            window.set_minimized(true);
                        }
                    },
                    "─"
                }

                // 最大化 / 还原
                button {
                    style: TITLE_BTN_STYLE,
                    onclick: move |_| {
                        #[cfg(feature = "desktop")]
                        {
                            let window = dioxus::desktop::use_window();
                            let is_max = window.is_maximized();
                            window.set_maximized(!is_max);
                        }
                    },
                    "▢"
                }

                // 关闭
                button {
                    style: TITLE_BTN_CLOSE_STYLE,
                    onclick: move |_| {
                        #[cfg(feature = "desktop")]
                        {
                            let window = dioxus::desktop::use_window();
                            window.close();
                        }
                    },
                    "✕"
                }
            }
        }
    }
}

const TITLE_BTN_STYLE: &str = "width:26px; height:22px; border-radius:999px; border:1px solid rgba(148,163,184,0.35); background:rgba(15,23,42,0.6); color:#e5e7eb; cursor:pointer; font-size:11px; display:flex; align-items:center; justify-content:center; padding:0 0 1px 0; box-shadow:0 0 0 1px rgba(15,23,42,0.5);";
const TITLE_BTN_CLOSE_STYLE: &str = "width:26px; height:22px; border-radius:999px; border:1px solid rgba(248,113,113,0.9); background:linear-gradient(135deg,#ef4444,#b91c1c); color:#fef2f2; cursor:pointer; font-size:11px; display:flex; align-items:center; justify-content:center; padding:0 0 1px 0; box-shadow:0 0 0 1px rgba(127,29,29,0.9);";

/// 单个图标按钮。
/// - 如果 `to` 为 Some，则使用 Link 进行路由跳转
/// - 如果 `to` 为 None，则渲染为普通按钮（目前作为占位，将来可以绑定事件）
#[component]
fn SidebarIcon(icon: &'static str, label: &'static str, to: Option<Route>) -> Element {
    // 统一的图标按钮样式
    let base_style = "width:40px; height:40px; border-radius:14px; display:flex; align-items:center; justify-content:center;\
                      font-size:20px; cursor:pointer; border:none; outline:none; background:transparent; color:inherit;";

    match to {
        Some(route) => rsx! {
            Link {
                to: route,
                style: "{base_style}",
                // 简单的 title 当 tooltip
                title: "{label}",
                span { "{icon}" }
            }
        },
        None => rsx! {
            button {
                style: "{base_style} opacity:0.8;",
                title: "{label}",
                span { "{icon}" }
            }
        },
    }
}
