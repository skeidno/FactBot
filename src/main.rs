use dioxus::prelude::*;

use views::{Airline, Blog, Home};
use components::Sidebar;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[layout(Sidebar)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
        #[route("/airlines")]
        Airline {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// 桌面端：使用 dioxus_desktop::launch::launch + Config 做无边框窗口
#[cfg(feature = "desktop")]
fn main() {
    use dioxus_desktop::{Config, WindowBuilder};
    use dioxus_desktop::launch::launch as desktop_launch;

    let cfg = Config::new().with_window(
        WindowBuilder::new()
            .with_title("FactBot 面板")
            .with_decorations(false), // 关闭系统边框和菜单栏
    );

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
    }
}
