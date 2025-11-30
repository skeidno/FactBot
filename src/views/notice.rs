use dioxus::prelude::*;

#[component]
pub fn Notice() -> Element {
    rsx! {
        div {
            style: "height:100%; overflow-y:auto; padding:24px 16px;",
            
            div {
                style: "max-width:1200px; margin:0 auto; display:flex; flex-direction:column; gap:24px;",

                section {
                    style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:20px; padding:28px 32px; border:1px solid #fbbf24;",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#78350f;",
                        "📢 系统公告"
                    }
                    p {
                        style: "color:#92400e; font-size:15px; margin:0;",
                        "查看最新的系统更新和重要通知"
                    }
                }

                NoticeCard {
                    title: "欢迎使用 FactBot 控制台",
                    content: "FactBot 是一个强大的数据运维助手，提供航司报价查询、配置管理等功能。请先在配置管理页面设置代理分组和其他配置信息。",
                    date: "2024-01-15",
                    is_important: true,
                }

                NoticeCard {
                    title: "新增批量导入功能",
                    content: "代理配置现已支持批量导入功能，支持 ip:port:username:password 格式，每行一个代理，大大提升配置效率。",
                    date: "2024-01-10",
                    is_important: false,
                }

                NoticeCard {
                    title: "支持 16 家航司查询",
                    content: "目前已支持美国航空、白俄罗斯航空、飞狮航空、越捷航空、维珍航空、韩亚航空、巴拿马航空、乌拉尔航空、西伯利亚航空、伊拉克航空、俄罗斯国际航空、北风航空、皇雀航空、马来西亚国际航空、宿务航空等 16 家航司。",
                    date: "2024-01-05",
                    is_important: false,
                }
            }
        }
    }
}

#[component]
fn NoticeCard(title: &'static str, content: &'static str, date: &'static str, is_important: bool) -> Element {
    let card_style = if is_important {
        "background:linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%); border:2px solid #fca5a5;"
    } else {
        "background:white; border:1px solid #e5e7eb;"
    };

    rsx! {
        article {
            style: "{card_style} border-radius:18px; padding:24px 28px; box-shadow:0 6px 20px rgba(15,23,42,0.08);",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:flex-start; margin-bottom:16px;",
                div {
                    if is_important {
                        span {
                            style: "display:inline-block; padding:4px 12px; border-radius:999px; background:#dc2626; color:white; font-size:12px; font-weight:600; margin-bottom:12px;",
                            "🔥 重要"
                        }
                    } else {
                        span {
                            style: "display:inline-block; padding:4px 12px; border-radius:999px; background:#3b82f6; color:white; font-size:12px; font-weight:600; margin-bottom:12px;",
                            "📌 通知"
                        }
                    }
                    h2 {
                        style: "font-size:20px; font-weight:700; margin:0; color:#111827;",
                        "{title}"
                    }
                }
                time {
                    style: "font-size:13px; color:#6b7280; font-weight:500;",
                    "{date}"
                }
            }

            p {
                style: "margin:0; color:#374151; font-size:15px; line-height:1.7;",
                "{content}"
            }
        }
    }
}
