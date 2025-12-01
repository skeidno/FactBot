use dioxus::prelude::*;

use views::{Airline, Blog, Home, Config as ConfigView, Notice, Help, Logs};
use components::Sidebar;

pub mod common;
mod components;
mod views;
mod db;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Sidebar)]
        #[route("/")]
        Home {},
        #[route("/airlines")]
        Airline {},
        #[route("/config")]
        ConfigView {},
        #[route("/logs")]
        Logs {},
        #[route("/notice")]
        Notice {},
        #[route("/help")]
        Help {},
        #[route("/blog/:id")]
        Blog { id: i32 },
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
    use dioxus_desktop::tao::window::Icon;
    use std::fs;

    // 清除 WebView 缓存，确保每次启动都是干净状态
    let webview_cache = std::env::temp_dir().join("factbot_webview");
    if webview_cache.exists() {
        let _ = fs::remove_dir_all(&webview_cache);
    }

    // 启动 Web API 服务器（后台线程）
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = fact_bot::api::start_server().await {
                eprintln!("Web API 服务器启动失败: {}", e);
            }
        });
    });

    // 加载图标
    let icon_bytes = include_bytes!("../assets/favicon.ico");
    let icon = Icon::from_rgba(load_icon_rgba(icon_bytes), 256, 256)
        .expect("Failed to load icon");

    let window = WindowBuilder::new()
        .with_title("FactBot 面板")
        .with_window_icon(Some(icon))
        .with_decorations(false) // 关闭系统边框和菜单栏
        .with_transparent(false) // 允许自定义背景覆盖整个窗口
        .with_inner_size(LogicalSize::new(1260.0, 900.0))
        .with_min_inner_size(LogicalSize::new(1120.0, 780.0));

    let cfg = Config::new()
        .with_window(window)
        .with_custom_head(HEAD_STYLE.to_string())
        .with_background_color((2, 6, 23, 255)) // 统一 WebView 背景色，与 UI 深色主题一致
        .with_data_directory(std::env::temp_dir().join("factbot_webview")); // 使用临时目录，避免历史记录持久化

    // contexts 使用默认空向量即可
    desktop_launch(App, Vec::new(), vec![Box::new(cfg)]);
}

#[cfg(feature = "desktop")]
fn load_icon_rgba(icon_bytes: &[u8]) -> Vec<u8> {
    use image::ImageReader;
    use std::io::Cursor;
    
    let img = ImageReader::new(Cursor::new(icon_bytes))
        .with_guessed_format()
        .expect("Failed to guess icon format")
        .decode()
        .expect("Failed to decode icon");
    
    img.to_rgba8().into_raw()
}

/// 非桌面平台：保持原有启动方式
#[cfg(not(feature = "desktop"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 初始化数据库（首次启动会自动标记）
    // 这会同时初始化配置数据库和日志数据库
    let _ = db::initialize_databases();
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
