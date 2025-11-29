use crate::Route;
use dioxus::prelude::*;

/// 欢迎页：突出产品定位，并引导用户进入航司报价页面
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            style: "min-height:100%; display:flex; align-items:center; justify-content:center; padding:48px; background:radial-gradient(circle at top,#eef2ff,#fef2f2);",
            div {
                style: "max-width:720px; width:100%; background:white; border-radius:28px; padding:48px; box-shadow:0 25px 60px rgba(79,70,229,0.15); text-align:center;",
                span {
                    style: "display:inline-flex; padding:6px 14px; border-radius:999px; background:#eef2ff; color:#4f46e5; font-size:13px; font-weight:600;",
                    "欢迎来到 FactBot · 数据运维助手"
                }
                h1 {
                    style: "margin:24px 0 12px; font-size:36px; font-weight:700; color:#111827;",
                    "集中管理多家航司报价、代理与接口配置"
                }
                p {
                    style: "margin:0 auto 28px; max-width:540px; color:#6b7280; font-size:16px; line-height:1.7;",
                    "FactBot 提供统一的工作台，帮助你在一个界面完成代理配置、Token 维护以及各航司查询。通过左侧侧边栏即可随时切换到航司功能页。"
                }
                div {
                    style: "display:flex; flex-wrap:wrap; justify-content:center; gap:16px;",
                    Link {
                        to: Route::Airline {},
                        style: "padding:14px 34px; border-radius:18px; font-size:15px; font-weight:600; color:white; background:linear-gradient(120deg,#7c3aed,#4f46e5); text-decoration:none;",
                        "进入航司报价"
                    }
                    Link {
                        to: Route::Blog { id: 1 },
                        style: "padding:14px 28px; border-radius:18px; font-size:15px; font-weight:600; color:#4f46e5; background:#eef2ff; text-decoration:none;",
                        "了解最新动态"
                    }
                }
            }
        }
    }
}
