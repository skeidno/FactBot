use dioxus::prelude::*;

use views::{Airline, Blog, Home, Config, Notice, Help};
use components::Sidebar;

mod components;
mod views;
mod db;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[layout(Sidebar)]
        #[route("/")]
        Home {},
        #[route("/config")]
        Config {},
        #[route("/blog/:id")]
        Blog { id: i32 },
        #[route("/airlines")]
        Airline {},
        #[route("/notice")]
        Notice {},
        #[route("/help")]
        Help {},
}

const FAVICON: Asset = asset!("/assets/favicon.svg");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const HEAD_STYLE: &str = r#"
    <style>
        html, body, #main {
            width: 100%;
            height: 100%;
            margin: 0;
            padding: 0;
            overflow: hidden;
            overscroll-behavior: contain;
        }
        html::-webkit-scrollbar,
        body::-webkit-scrollbar,
        #main::-webkit-scrollbar {
            width: 0;
            height: 0;
            display: none;
        }
    </style>
"#;

/// 桌面端：使用 dioxus_desktop::launch::launch + Config 做无边框窗口
#[cfg(feature = "desktop")]
fn main() {
    use dioxus_desktop::{Config, WindowBuilder, LogicalSize};
    use dioxus_desktop::launch::launch as desktop_launch;

    let window = WindowBuilder::new()
        .with_title("FactBot 面板")
        .with_decorations(false) // 关闭系统边框和菜单栏
        .with_transparent(false) // 允许自定义背景覆盖整个窗口
        .with_inner_size(LogicalSize::new(1260.0, 900.0))
        .with_min_inner_size(LogicalSize::new(1120.0, 780.0));

    let cfg = Config::new()
        .with_window(window)
        .with_custom_head(HEAD_STYLE.to_string())
        .with_background_color((2, 6, 23, 255)); // 统一 WebView 背景色，与 UI 深色主题一致

    // contexts 使用默认空向量即可
    desktop_launch(App, Vec::new(), vec![Box::new(cfg)]);
}

/// 非桌面平台：保持原有启动方式
#[cfg(not(feature = "desktop"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
        
        // 在 Router 之后检查首次启动
        FirstLaunchHandler {}
    }
}

#[component]
fn FirstLaunchHandler() -> Element {
    use_effect(move || {
        if db::is_first_launch() {
            let nav = navigator();
            nav.replace(Route::Home {});
        }
    });
    
    rsx! {}
}
