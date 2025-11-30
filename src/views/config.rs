use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::{save_config, load_config};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ProxyConfig {
    id: usize,
    ip: String,
    port: String,
    username: String,
    password: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ProxyGroup {
    id: usize,
    name: String,
    proxies: Vec<ProxyConfig>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct CardConfig {
    id: usize,
    card_type: String,
    card_number: String,
    cvv: String,
    expiry: String,
    holder_name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct CardGroup {
    id: usize,
    name: String,
    cards: Vec<CardConfig>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PassengerInfo {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    passport: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct PassengerGroup {
    id: usize,
    name: String,
    passengers: Vec<PassengerInfo>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct OtpConfig {
    id: usize,
    name: String,
    email: String,
    api_key: String,
    service_provider: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct OtpGroup {
    id: usize,
    name: String,
    configs: Vec<OtpConfig>,
}

#[component]
pub fn Config() -> Element {
    // 从数据库加载数据
    let proxy_groups = use_signal(|| {
        load_config("proxy_groups")
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<ProxyGroup>>(&json).ok())
            .unwrap_or_else(|| vec![
                ProxyGroup {
                    id: 1,
                    name: "默认分组".to_string(),
                    proxies: vec![],
                }
            ])
    });
    
    let card_groups = use_signal(|| {
        load_config("card_groups")
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<CardGroup>>(&json).ok())
            .unwrap_or_else(|| vec![
                CardGroup {
                    id: 1,
                    name: "默认分组".to_string(),
                    cards: vec![],
                }
            ])
    });
    
    let passenger_groups = use_signal(|| {
        load_config("passenger_groups")
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<PassengerGroup>>(&json).ok())
            .unwrap_or_else(|| vec![
                PassengerGroup {
                    id: 1,
                    name: "默认分组".to_string(),
                    passengers: vec![],
                }
            ])
    });
    
    let otp_groups = use_signal(|| {
        load_config("otp_groups")
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<OtpGroup>>(&json).ok())
            .unwrap_or_else(|| vec![
                OtpGroup {
                    id: 1,
                    name: "默认分组".to_string(),
                    configs: vec![],
                }
            ])
    });

    let active_tab = use_signal(|| "proxy".to_string());

    rsx! {
        document::Style {
            r#"
            .config-scroll-container {{
                scrollbar-width: thin;
                scrollbar-color: rgba(99, 102, 241, 0.3) transparent;
            }}
            .config-scroll-container::-webkit-scrollbar {{
                width: 8px;
            }}
            .config-scroll-container::-webkit-scrollbar-track {{
                background: transparent;
                border-radius: 10px;
            }}
            .config-scroll-container::-webkit-scrollbar-thumb {{
                background: rgba(99, 102, 241, 0.3);
                border-radius: 10px;
                transition: background 0.2s ease;
            }}
            .config-scroll-container::-webkit-scrollbar-thumb:hover {{
                background: rgba(99, 102, 241, 0.5);
            }}
            "#
        }

        div {
            class: "config-scroll-container",
            style: "height:100%; overflow-y:auto; overflow-x:hidden; padding:24px 16px 24px 0;",
            
            div {
                style: "display:flex; flex-direction:column; gap:28px; max-width:1400px; margin:0 auto;",

                section {
                    style: "background:linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%); border-radius:20px; padding:28px 32px; border:1px solid #7dd3fc; box-shadow:0 4px 20px rgba(14, 165, 233, 0.1);",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#0c4a6e; letter-spacing:-0.02em;",
                        "配置管理中心"
                    }
                    p {
                        style: "color:#0369a1; font-size:15px; margin:0; line-height:1.6;",
                        "管理代理配置、OTP邮箱服务、支付卡片和购票人信息（支持分组批量保存）"
                    }
                }

                div {
                    style: "background:white; border-radius:18px; padding:8px; border:1px solid #e5e7eb; box-shadow:0 2px 8px rgba(15,23,42,0.04); display:flex; gap:8px; flex-wrap:wrap;",
                    
                    TabButton { label: "代理配置", value: "proxy", active_tab, icon: "🌐" }
                    TabButton { label: "OTP邮箱", value: "otp", active_tab, icon: "📧" }
                    TabButton { label: "支付卡片", value: "card", active_tab, icon: "💳" }
                    TabButton { label: "购票人信息", value: "passenger", active_tab, icon: "👤" }
                }

                match active_tab().as_str() {
                    "proxy" => rsx! {
                        ProxyGroupSection { proxy_groups }
                    },
                    "otp" => rsx! {
                        OtpGroupSection { otp_groups }
                    },
                    "card" => rsx! {
                        CardGroupSection { card_groups }
                    },
                    "passenger" => rsx! {
                        PassengerGroupSection { passenger_groups }
                    },
                    _ => rsx! { div {} }
                }
            }
        }
    }
}

#[component]
fn TabButton(label: &'static str, value: &'static str, active_tab: Signal<String>, icon: &'static str) -> Element {
    let is_active = active_tab() == value;
    let style = if is_active {
        "padding:12px 24px; border-radius:12px; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; border:none; transition:all 0.2s ease; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3);"
    } else {
        "padding:12px 24px; border-radius:12px; background:transparent; color:#6b7280; font-weight:500; cursor:pointer; border:none; transition:all 0.2s ease;"
    };

    rsx! {
        button {
            style: "{style}",
            onclick: move |_| active_tab.set(value.to_string()),
            "{icon} {label}"
        }
    }
}

#[component]
fn ProxyGroupSection(proxy_groups: Signal<Vec<ProxyGroup>>) -> Element {
    let mut selected_group = use_signal(|| 0usize);

    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:24px;",
                div {
                    h2 {
                        style: "font-size:22px; font-weight:700; margin:0 0 6px 0; color:#111827; letter-spacing:-0.02em;",
                        "代理配置分组"
                    }
                    p {
                        style: "margin:0; color:#6b7280; font-size:14px;",
                        "每个分组可以保存多个代理配置"
                    }
                }
                button {
                    style: "padding:11px 20px; border-radius:12px; border:none; background:linear-gradient(120deg,#10b981,#059669); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(16, 185, 129, 0.3);",
                    onclick: move |_| {
                        let mut groups = proxy_groups();
                        let new_id = groups.iter().map(|g| g.id).max().unwrap_or(0) + 1;
                        groups.push(ProxyGroup {
                            id: new_id,
                            name: format!("分组 {}", new_id),
                            proxies: vec![],
                        });
                        proxy_groups.set(groups);
                    },
                    "+ 新建分组"
                }
            }

            div {
                style: "display:flex; gap:12px; margin-bottom:24px; flex-wrap:wrap;",
                for (index, group) in proxy_groups().iter().enumerate() {
                    {
                        let is_selected = selected_group() == index;
                        let btn_style = if is_selected {
                            "padding:10px 18px; border-radius:10px; background:#4f46e5; color:white; font-weight:600; cursor:pointer; border:1px solid #4f46e5;"
                        } else {
                            "padding:10px 18px; border-radius:10px; background:white; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #d1d5db;"
                        };
                        rsx! {
                            button {
                                key: "{group.id}",
                                style: "{btn_style}",
                                onclick: move |_| selected_group.set(index),
                                "{group.name} ({group.proxies.len()})"
                            }
                        }
                    }
                }
            }

            if let Some(group) = proxy_groups().get(selected_group()) {
                ProxyGroupEditor {
                    group: group.clone(),
                    group_index: selected_group(),
                    proxy_groups
                }
            }
        }
    }
}

#[component]
fn ProxyGroupEditor(group: ProxyGroup, group_index: usize, proxy_groups: Signal<Vec<ProxyGroup>>) -> Element {
    let mut show_batch_modal = use_signal(|| false);
    let batch_input = use_signal(|| "".to_string());
    let mut page_size = use_signal(|| 20usize);
    let mut current_page = use_signal(|| 1usize);
    
    let total_items = group.proxies.len();
    let total_pages = (total_items + page_size() - 1) / page_size();
    let start_index = (current_page() - 1) * page_size();
    let end_index = (start_index + page_size()).min(total_items);

    rsx! {
        div {
            style: "background:linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%); border-radius:16px; padding:24px; border:1px solid #86efac;",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:20px;",
                input {
                    value: group.name.clone(),
                    oninput: move |evt| {
                        let mut groups = proxy_groups();
                        groups[group_index].name = evt.value();
                        proxy_groups.set(groups);
                    },
                    style: "flex:1; max-width:300px; padding:10px 14px; border-radius:10px; border:1px solid #86efac; font-size:16px; font-weight:600; background:white;",
                    placeholder: "分组名称"
                }
                div {
                    style: "display:flex; gap:12px;",
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:none; background:linear-gradient(120deg,#8b5cf6,#6366f1); color:white; font-weight:600; cursor:pointer;",
                        onclick: move |_| show_batch_modal.set(true),
                        "📋 批量导入"
                    }
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            let mut groups = proxy_groups();
                            groups.remove(group_index);
                            proxy_groups.set(groups);
                        },
                        "删除分组"
                    }
                }
            }

            if !group.proxies.is_empty() {
                div {
                    style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:12px; padding:0 4px;",
                    div {
                        style: "font-size:13px; color:#6b7280;",
                        "共 {total_items} 条记录，第 {start_index + 1}-{end_index} 条"
                    }
                    div {
                        style: "display:flex; align-items:center; gap:12px;",
                        span {
                            style: "font-size:13px; color:#6b7280;",
                            "每页显示："
                        }
                        select {
                            value: page_size().to_string(),
                            onchange: move |evt| {
                                if let Ok(size) = evt.value().parse::<usize>() {
                                    page_size.set(size);
                                    current_page.set(1);
                                }
                            },
                            style: "padding:6px 10px; border-radius:6px; border:1px solid #d1d5db; font-size:13px; background:white; cursor:pointer;",
                            option { value: "20", "20 条" }
                            option { value: "50", "50 条" }
                            option { value: "100", "100 条" }
                        }
                    }
                }
                
                div {
                    style: "background:white; border-radius:12px; overflow:hidden; border:1px solid #e5e7eb;",
                    table {
                        style: "width:100%; border-collapse:collapse;",
                        thead {
                            tr {
                                style: "background:#f9fafb;",
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "#" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "IP 地址" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "端口" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "用户名" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "密码" }
                                th { style: "padding:12px 16px; text-align:center; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "操作" }
                            }
                        }
                        tbody {
                            for (proxy_index, proxy) in group.proxies.iter().enumerate().skip(start_index).take(page_size()) {
                                ProxyTableRow {
                                    proxy: proxy.clone(),
                                    proxy_index,
                                    group_index,
                                    proxy_groups
                                }
                            }
                        }
                    }
                }
                
                if total_pages > 1 {
                    Pagination { current_page, total_pages }
                }
            } else {
                div {
                    style: "text-align:center; padding:40px; color:#6b7280; font-size:14px; background:white; border-radius:12px; border:1px dashed #d1d5db;",
                    "暂无代理配置，点击"批量导入"开始配置"
                }
            }

            div {
                style: "margin-top:20px; padding-top:20px; border-top:1px solid #86efac; display:flex; justify-content:flex-end;",
                button {
                    style: "padding:12px 28px; border-radius:12px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3);",
                    onclick: move |_| {
                        let groups = proxy_groups();
                        if let Ok(json) = serde_json::to_string(&groups) {
                            let _ = save_config("proxy_groups", &json);
                        }
                    },
                    "💾 保存分组配置"
                }
            }
        }

        if show_batch_modal() {
            BatchImportModal {
                show_modal: show_batch_modal,
                batch_input,
                group_index,
                proxy_groups
            }
        }
    }
}

#[component]
fn ProxyTableRow(proxy: ProxyConfig, proxy_index: usize, group_index: usize, proxy_groups: Signal<Vec<ProxyGroup>>) -> Element {
    rsx! {
        tr {
            style: "border-bottom:1px solid #f3f4f6; transition:background 0.15s ease;",
            td {
                style: "padding:12px 16px; font-size:14px; color:#6b7280;",
                "{proxy.id}"
            }
            td {
                style: "padding:12px 16px;",
                input {
                    value: proxy.ip.clone(),
                    oninput: move |evt| {
                        let mut groups = proxy_groups();
                        groups[group_index].proxies[proxy_index].ip = evt.value();
                        proxy_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "127.0.0.1"
                }
            }
            td {
                style: "padding:12px 16px;",
                input {
                    value: proxy.port.clone(),
                    oninput: move |evt| {
                        let mut groups = proxy_groups();
                        groups[group_index].proxies[proxy_index].port = evt.value();
                        proxy_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "7897"
                }
            }
            td {
                style: "padding:12px 16px;",
                input {
                    value: proxy.username.clone(),
                    oninput: move |evt| {
                        let mut groups = proxy_groups();
                        groups[group_index].proxies[proxy_index].username = evt.value();
                        proxy_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "可选"
                }
            }
            td {
                style: "padding:12px 16px;",
                input {
                    r#type: "password",
                    value: proxy.password.clone(),
                    oninput: move |evt| {
                        let mut groups = proxy_groups();
                        groups[group_index].proxies[proxy_index].password = evt.value();
                        proxy_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "可选"
                }
            }
            td {
                style: "padding:12px 16px; text-align:center;",
                button {
                    style: "padding:6px 12px; border-radius:6px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:500; cursor:pointer; font-size:12px;",
                    onclick: move |_| {
                        let mut groups = proxy_groups();
                        groups[group_index].proxies.remove(proxy_index);
                        proxy_groups.set(groups);
                    },
                    "删除"
                }
            }
        }
    }
}

#[component]
fn BatchImportModal(show_modal: Signal<bool>, batch_input: Signal<String>, group_index: usize, proxy_groups: Signal<Vec<ProxyGroup>>) -> Element {
    rsx! {
        div {
            style: "position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(0,0,0,0.5); display:flex; align-items:center; justify-content:center; z-index:9999; backdrop-filter:blur(4px);",
            onclick: move |_| show_modal.set(false),
            
            div {
                style: "background:white; border-radius:20px; padding:32px; max-width:700px; width:90%; box-shadow:0 25px 50px rgba(0,0,0,0.3);",
                onclick: move |evt| evt.stop_propagation(),
                
                h3 {
                    style: "margin:0 0 8px 0; font-size:22px; font-weight:700; color:#111827;",
                    "批量导入代理"
                }
                p {
                    style: "margin:0 0 20px 0; color:#6b7280; font-size:14px; line-height:1.6;",
                    "支持格式：ip:port:username:password 或 ip:port（每行一个）"
                }
                
                textarea {
                    value: batch_input(),
                    oninput: move |evt| batch_input.set(evt.value()),
                    style: "width:100%; height:300px; padding:14px; border-radius:12px; border:1px solid #d1d5db; font-size:13px; font-family:monospace; resize:vertical; line-height:1.6;",
                    placeholder: "127.0.0.1:7897:user1:pass1\n192.168.1.1:8080:user2:pass2\n10.0.0.1:3128"
                }
                
                div {
                    style: "margin-top:24px; display:flex; justify-content:flex-end; gap:12px;",
                    button {
                        style: "padding:10px 20px; border-radius:10px; border:1px solid #d1d5db; background:white; color:#6b7280; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            show_modal.set(false);
                            batch_input.set("".to_string());
                        },
                        "取消"
                    }
                    button {
                        style: "padding:10px 20px; border-radius:10px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3);",
                        onclick: move |_| {
                            let input = batch_input();
                            let mut groups = proxy_groups();
                            let mut next_id = groups[group_index].proxies.iter().map(|p| p.id).max().unwrap_or(0) + 1;
                            
                            for line in input.lines() {
                                let line = line.trim();
                                if line.is_empty() {
                                    continue;
                                }
                                
                                let parts: Vec<&str> = line.split(':').collect();
                                if parts.len() >= 2 {
                                    let ip = parts[0].to_string();
                                    let port = parts[1].to_string();
                                    let username = if parts.len() > 2 { parts[2].to_string() } else { "".to_string() };
                                    let password = if parts.len() > 3 { parts[3].to_string() } else { "".to_string() };
                                    
                                    groups[group_index].proxies.push(ProxyConfig {
                                        id: next_id,
                                        ip,
                                        port,
                                        username,
                                        password,
                                    });
                                    next_id += 1;
                                }
                            }
                            
                            proxy_groups.set(groups);
                            show_modal.set(false);
                            batch_input.set("".to_string());
                        },
                        "导入"
                    }
                }
            }
        }
    }
}

#[component]
fn OtpGroupSection(otp_groups: Signal<Vec<OtpGroup>>) -> Element {
    let mut selected_group = use_signal(|| 0usize);

    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:24px;",
                div {
                    h2 {
                        style: "font-size:22px; font-weight:700; margin:0 0 6px 0; color:#111827; letter-spacing:-0.02em;",
                        "OTP 邮箱服务分组"
                    }
                    p {
                        style: "margin:0; color:#6b7280; font-size:14px;",
                        "每个分组可以保存多个 OTP 邮箱配置"
                    }
                }
                button {
                    style: "padding:11px 20px; border-radius:12px; border:none; background:linear-gradient(120deg,#10b981,#059669); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(16, 185, 129, 0.3);",
                    onclick: move |_| {
                        let mut groups = otp_groups();
                        let new_id = groups.iter().map(|g| g.id).max().unwrap_or(0) + 1;
                        groups.push(OtpGroup {
                            id: new_id,
                            name: format!("分组 {}", new_id),
                            configs: vec![],
                        });
                        otp_groups.set(groups);
                    },
                    "+ 新建分组"
                }
            }

            div {
                style: "display:flex; gap:12px; margin-bottom:24px; flex-wrap:wrap;",
                for (index, group) in otp_groups().iter().enumerate() {
                    {
                        let is_selected = selected_group() == index;
                        let btn_style = if is_selected {
                            "padding:10px 18px; border-radius:10px; background:#4f46e5; color:white; font-weight:600; cursor:pointer; border:1px solid #4f46e5;"
                        } else {
                            "padding:10px 18px; border-radius:10px; background:white; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #d1d5db;"
                        };
                        rsx! {
                            button {
                                key: "{group.id}",
                                style: "{btn_style}",
                                onclick: move |_| selected_group.set(index),
                                "{group.name} ({group.configs.len()})"
                            }
                        }
                    }
                }
            }

            if let Some(group) = otp_groups().get(selected_group()) {
                OtpGroupEditor {
                    group: group.clone(),
                    group_index: selected_group(),
                    otp_groups
                }
            }
        }
    }
}

#[component]
fn OtpGroupEditor(group: OtpGroup, group_index: usize, otp_groups: Signal<Vec<OtpGroup>>) -> Element {
    let mut page_size = use_signal(|| 20usize);
    let mut current_page = use_signal(|| 1usize);
    
    let total_items = group.configs.len();
    let total_pages = (total_items + page_size() - 1) / page_size();
    let start_index = (current_page() - 1) * page_size();
    let end_index = (start_index + page_size()).min(total_items);

    rsx! {
        div {
            style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:16px; padding:24px; border:1px solid #fbbf24;",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:20px;",
                input {
                    value: group.name.clone(),
                    oninput: move |evt| {
                        let mut groups = otp_groups();
                        groups[group_index].name = evt.value();
                        otp_groups.set(groups);
                    },
                    style: "flex:1; max-width:300px; padding:10px 14px; border-radius:10px; border:1px solid #fbbf24; font-size:16px; font-weight:600; background:white;",
                    placeholder: "分组名称"
                }
                div {
                    style: "display:flex; gap:12px;",
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:none; background:linear-gradient(120deg,#10b981,#059669); color:white; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            let mut groups = otp_groups();
                            let new_id = groups[group_index].configs.iter().map(|c| c.id).max().unwrap_or(0) + 1;
                            groups[group_index].configs.push(OtpConfig {
                                id: new_id,
                                name: format!("配置 {}", new_id),
                                email: "".to_string(),
                                api_key: "".to_string(),
                                service_provider: "".to_string(),
                            });
                            otp_groups.set(groups);
                        },
                        "+ 添加配置"
                    }
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            let mut groups = otp_groups();
                            groups.remove(group_index);
                            otp_groups.set(groups);
                        },
                        "删除分组"
                    }
                }
            }

            if !group.configs.is_empty() {
                div {
                    style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:12px; padding:0 4px;",
                    div {
                        style: "font-size:13px; color:#92400e;",
                        "共 {total_items} 条记录，第 {start_index + 1}-{end_index} 条"
                    }
                    div {
                        style: "display:flex; align-items:center; gap:12px;",
                        span {
                            style: "font-size:13px; color:#92400e;",
                            "每页显示："
                        }
                        select {
                            value: page_size().to_string(),
                            onchange: move |evt| {
                                if let Ok(size) = evt.value().parse::<usize>() {
                                    page_size.set(size);
                                    current_page.set(1);
                                }
                            },
                            style: "padding:6px 10px; border-radius:6px; border:1px solid #fbbf24; font-size:13px; background:white; cursor:pointer;",
                            option { value: "20", "20 条" }
                            option { value: "50", "50 条" }
                            option { value: "100", "100 条" }
                        }
                    }
                }
                
                div {
                    style: "background:white; border-radius:12px; overflow:hidden; border:1px solid #e5e7eb;",
                    table {
                        style: "width:100%; border-collapse:collapse;",
                        thead {
                            tr {
                                style: "background:#f9fafb;",
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "#" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "配置名称" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "邮箱地址" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "API Key" }
                                th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "服务商" }
                                th { style: "padding:12px 16px; text-align:center; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "操作" }
                            }
                        }
                        tbody {
                            for (config_index, config) in group.configs.iter().enumerate().skip(start_index).take(page_size()) {
                                OtpTableRow {
                                    config: config.clone(),
                                    config_index,
                                    group_index,
                                    otp_groups
                                }
                            }
                        }
                    }
                }
                
                if total_pages > 1 {
                    Pagination { current_page, total_pages }
                }
            } else {
                div {
                    style: "text-align:center; padding:40px; color:#92400e; font-size:14px; background:white; border-radius:12px; border:1px dashed #fbbf24;",
                    "暂无 OTP 配置，点击"添加配置"开始配置"
                }
            }

            div {
                style: "margin-top:20px; padding-top:20px; border-top:1px solid #fbbf24; display:flex; justify-content:flex-end;",
                button {
                    style: "padding:12px 28px; border-radius:12px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3);",
                    onclick: move |_| {
                        let groups = otp_groups();
                        if let Ok(json) = serde_json::to_string(&groups) {
                            let _ = save_config("otp_groups", &json);
                        }
                    },
                    "💾 保存分组配置"
                }
            }
        }
    }
}

#[component]
fn OtpTableRow(config: OtpConfig, config_index: usize, group_index: usize, otp_groups: Signal<Vec<OtpGroup>>) -> Element {
    rsx! {
        tr {
            style: "border-bottom:1px solid #f3f4f6;",
            td {
                style: "padding:12px 16px; font-size:14px; color:#6b7280;",
                "{config.id}"
            }
            td {
                style: "padding:12px 16px;",
                input {
                    value: config.name.clone(),
                    oninput: move |evt| {
                        let mut groups = otp_groups();
                        groups[group_index].configs[config_index].name = evt.value();
                        otp_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "配置名称"
                }
            }
            td {
                style: "padding:12px 16px;",
                input {
                    value: config.email.clone(),
                    oninput: move |evt| {
                        let mut groups = otp_groups();
                        groups[group_index].configs[config_index].email = evt.value();
                        otp_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "example@gmail.com"
                }
            }
            td {
                style: "padding:12px 16px;",
                input {
                    r#type: "password",
                    value: config.api_key.clone(),
                    oninput: move |evt| {
                        let mut groups = otp_groups();
                        groups[group_index].configs[config_index].api_key = evt.value();
                        otp_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "API Key"
                }
            }
            td {
                style: "padding:12px 16px;",
                input {
                    value: config.service_provider.clone(),
                    oninput: move |evt| {
                        let mut groups = otp_groups();
                        groups[group_index].configs[config_index].service_provider = evt.value();
                        otp_groups.set(groups);
                    },
                    style: "width:100%; padding:8px 10px; border-radius:6px; border:1px solid #e5e7eb; font-size:13px;",
                    placeholder: "Gmail"
                }
            }
            td {
                style: "padding:12px 16px; text-align:center;",
                button {
                    style: "padding:6px 12px; border-radius:6px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:500; cursor:pointer; font-size:12px;",
                    onclick: move |_| {
                        let mut groups = otp_groups();
                        groups[group_index].configs.remove(config_index);
                        otp_groups.set(groups);
                    },
                    "删除"
                }
            }
        }
    }
}

#[component]
fn CardGroupSection(card_groups: Signal<Vec<CardGroup>>) -> Element {
    let mut selected_group = use_signal(|| 0usize);

    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:24px;",
                div {
                    h2 {
                        style: "font-size:22px; font-weight:700; margin:0 0 6px 0; color:#111827; letter-spacing:-0.02em;",
                        "支付卡片分组"
                    }
                    p {
                        style: "margin:0; color:#6b7280; font-size:14px;",
                        "每个分组可以保存多张卡片配置"
                    }
                }
                button {
                    style: "padding:11px 20px; border-radius:12px; border:none; background:linear-gradient(120deg,#10b981,#059669); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(16, 185, 129, 0.3);",
                    onclick: move |_| {
                        let mut groups = card_groups();
                        let new_id = groups.iter().map(|g| g.id).max().unwrap_or(0) + 1;
                        groups.push(CardGroup {
                            id: new_id,
                            name: format!("分组 {}", new_id),
                            cards: vec![],
                        });
                        card_groups.set(groups);
                    },
                    "+ 新建分组"
                }
            }

            div {
                style: "display:flex; gap:12px; margin-bottom:24px; flex-wrap:wrap;",
                for (index, group) in card_groups().iter().enumerate() {
                    {
                        let is_selected = selected_group() == index;
                        let btn_style = if is_selected {
                            "padding:10px 18px; border-radius:10px; background:#4f46e5; color:white; font-weight:600; cursor:pointer; border:1px solid #4f46e5;"
                        } else {
                            "padding:10px 18px; border-radius:10px; background:white; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #d1d5db;"
                        };
                        rsx! {
                            button {
                                key: "{group.id}",
                                style: "{btn_style}",
                                onclick: move |_| selected_group.set(index),
                                "{group.name} ({group.cards.len()})"
                            }
                        }
                    }
                }
            }

            if let Some(group) = card_groups().get(selected_group()) {
                CardGroupEditor {
                    group: group.clone(),
                    group_index: selected_group(),
                    card_groups
                }
            }
        }
    }
}

#[component]
fn CardGroupEditor(group: CardGroup, group_index: usize, card_groups: Signal<Vec<CardGroup>>) -> Element {
    let mut page_size = use_signal(|| 20usize);
    let mut current_page = use_signal(|| 1usize);
    
    let total_items = group.cards.len();
    let total_pages = (total_items + page_size() - 1) / page_size();
    let start_index = (current_page() - 1) * page_size();
    let end_index = (start_index + page_size()).min(total_items);

    rsx! {
        div {
            style: "background:linear-gradient(135deg, #fef3c7 0%, #fde68a 100%); border-radius:16px; padding:24px; border:1px solid #fbbf24;",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:20px;",
                input {
                    value: group.name.clone(),
                    oninput: move |evt| {
                        let mut groups = card_groups();
                        groups[group_index].name = evt.value();
                        card_groups.set(groups);
                    },
                    style: "flex:1; max-width:300px; padding:10px 14px; border-radius:10px; border:1px solid #fbbf24; font-size:16px; font-weight:600; background:white;",
                    placeholder: "分组名称"
                }
                div {
                    style: "display:flex; gap:12px;",
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:none; background:linear-gradient(120deg,#10b981,#059669); color:white; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            let mut groups = card_groups();
                            let new_id = groups[group_index].cards.iter().map(|c| c.id).max().unwrap_or(0) + 1;
                            groups[group_index].cards.push(CardConfig {
                                id: new_id,
                                card_type: "credit".to_string(),
                                card_number: "".to_string(),
                                cvv: "".to_string(),
                                expiry: "".to_string(),
                                holder_name: "".to_string(),
                            });
                            card_groups.set(groups);
                        },
                        "+ 添加卡片"
                    }
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            let mut groups = card_groups();
                            groups.remove(group_index);
                            card_groups.set(groups);
                        },
                        "删除分组"
                    }
                }
            }

            if !group.cards.is_empty() {
                div {
                    style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:12px; padding:0 4px;",
                    div {
                        style: "font-size:13px; color:#92400e;",
                        "共 {total_items} 条记录，第 {start_index + 1}-{end_index} 条"
                    }
                    div {
                        style: "display:flex; align-items:center; gap:12px;",
                        span {
                            style: "font-size:13px; color:#92400e;",
                            "每页显示："
                        }
                        select {
                            value: page_size().to_string(),
                            onchange: move |evt| {
                                if let Ok(size) = evt.value().parse::<usize>() {
                                    page_size.set(size);
                                    current_page.set(1);
                                }
                            },
                            style: "padding:6px 10px; border-radius:6px; border:1px solid #fbbf24; font-size:13px; background:white; cursor:pointer;",
                            option { value: "20", "20 条" }
                            option { value: "50", "50 条" }
                            option { value: "100", "100 条" }
                        }
                    }
                }
            }
            
            div {
                style: "display:flex; flex-direction:column; gap:16px;",
                for (card_index, card) in group.cards.iter().enumerate().skip(start_index).take(page_size()) {
                    CardConfigItem {
                        card: card.clone(),
                        card_index,
                        group_index,
                        card_groups
                    }
                }
                if group.cards.is_empty() {
                    div {
                        style: "text-align:center; padding:40px; color:#92400e; font-size:14px;",
                        "暂无卡片配置，点击"添加卡片"开始配置"
                    }
                }
            }
            
            if total_pages > 1 {
                Pagination { current_page, total_pages }
            }

            div {
                style: "margin-top:20px; padding-top:20px; border-top:1px solid #fbbf24; display:flex; justify-content:flex-end;",
                button {
                    style: "padding:12px 28px; border-radius:12px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3);",
                    onclick: move |_| {
                        let groups = card_groups();
                        if let Ok(json) = serde_json::to_string(&groups) {
                            let _ = save_config("card_groups", &json);
                        }
                    },
                    "💾 保存分组配置"
                }
            }
        }
    }
}

#[component]
fn CardConfigItem(card: CardConfig, card_index: usize, group_index: usize, card_groups: Signal<Vec<CardGroup>>) -> Element {
    rsx! {
        div {
            style: "background:white; border-radius:14px; padding:18px; border:1px solid #d1d5db;",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:14px;",
                h4 {
                    style: "margin:0; font-size:15px; font-weight:600; color:#78350f;",
                    "卡片 #{card.id}"
                }
                button {
                    style: "padding:6px 14px; border-radius:8px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:500; cursor:pointer; font-size:13px;",
                    onclick: move |_| {
                        let mut groups = card_groups();
                        groups[group_index].cards.remove(card_index);
                        card_groups.set(groups);
                    },
                    "删除"
                }
            }

            div {
                style: "margin-bottom:12px;",
                label {
                    style: "display:block; font-size:13px; font-weight:600; color:#78350f; margin-bottom:8px;",
                    "卡片类型"
                }
                select {
                    style: "width:100%; padding:11px 14px; border-radius:10px; border:1px solid #d97706; font-size:14px; background:white;",
                    value: card.card_type.clone(),
                    onchange: move |evt| {
                        let mut groups = card_groups();
                        groups[group_index].cards[card_index].card_type = evt.value();
                        card_groups.set(groups);
                    },
                    option { value: "credit", "信用卡" }
                    option { value: "gift", "礼品卡" }
                }
            }

            div {
                style: "display:grid; grid-template-columns:2fr 1fr 1fr; gap:12px; margin-bottom:12px;",
                ConfigInput {
                    label: "卡号",
                    value: card.card_number.clone(),
                    placeholder: "1234 5678 9012 3456",
                    onchange: move |val| {
                        let mut groups = card_groups();
                        groups[group_index].cards[card_index].card_number = val;
                        card_groups.set(groups);
                    }
                }
                ConfigInput {
                    label: "CVV",
                    value: card.cvv.clone(),
                    placeholder: "123",
                    input_type: "password",
                    onchange: move |val| {
                        let mut groups = card_groups();
                        groups[group_index].cards[card_index].cvv = val;
                        card_groups.set(groups);
                    }
                }
                ConfigInput {
                    label: "有效期",
                    value: card.expiry.clone(),
                    placeholder: "MM/YY",
                    onchange: move |val| {
                        let mut groups = card_groups();
                        groups[group_index].cards[card_index].expiry = val;
                        card_groups.set(groups);
                    }
                }
            }

            ConfigInput {
                label: "持卡人姓名",
                value: card.holder_name.clone(),
                placeholder: "ZHANG SAN",
                onchange: move |val| {
                    let mut groups = card_groups();
                    groups[group_index].cards[card_index].holder_name = val;
                    card_groups.set(groups);
                }
            }
        }
    }
}

#[component]
fn PassengerGroupSection(passenger_groups: Signal<Vec<PassengerGroup>>) -> Element {
    let mut selected_group = use_signal(|| 0usize);

    rsx! {
        section {
            style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:24px;",
                div {
                    h2 {
                        style: "font-size:22px; font-weight:700; margin:0 0 6px 0; color:#111827; letter-spacing:-0.02em;",
                        "购票人信息分组"
                    }
                    p {
                        style: "margin:0; color:#6b7280; font-size:14px;",
                        "每个分组可以保存多个购票人信息"
                    }
                }
                button {
                    style: "padding:11px 20px; border-radius:12px; border:none; background:linear-gradient(120deg,#10b981,#059669); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(16, 185, 129, 0.3);",
                    onclick: move |_| {
                        let mut groups = passenger_groups();
                        let new_id = groups.iter().map(|g| g.id).max().unwrap_or(0) + 1;
                        groups.push(PassengerGroup {
                            id: new_id,
                            name: format!("分组 {}", new_id),
                            passengers: vec![],
                        });
                        passenger_groups.set(groups);
                    },
                    "+ 新建分组"
                }
            }

            div {
                style: "display:flex; gap:12px; margin-bottom:24px; flex-wrap:wrap;",
                for (index, group) in passenger_groups().iter().enumerate() {
                    {
                        let is_selected = selected_group() == index;
                        let btn_style = if is_selected {
                            "padding:10px 18px; border-radius:10px; background:#4f46e5; color:white; font-weight:600; cursor:pointer; border:1px solid #4f46e5;"
                        } else {
                            "padding:10px 18px; border-radius:10px; background:white; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #d1d5db;"
                        };
                        rsx! {
                            button {
                                key: "{group.id}",
                                style: "{btn_style}",
                                onclick: move |_| selected_group.set(index),
                                "{group.name} ({group.passengers.len()})"
                            }
                        }
                    }
                }
            }

            if let Some(group) = passenger_groups().get(selected_group()) {
                PassengerGroupEditor {
                    group: group.clone(),
                    group_index: selected_group(),
                    passenger_groups
                }
            }
        }
    }
}

#[component]
fn PassengerGroupEditor(group: PassengerGroup, group_index: usize, passenger_groups: Signal<Vec<PassengerGroup>>) -> Element {
    let mut page_size = use_signal(|| 20usize);
    let mut current_page = use_signal(|| 1usize);
    
    let total_items = group.passengers.len();
    let total_pages = (total_items + page_size() - 1) / page_size();
    let start_index = (current_page() - 1) * page_size();
    let end_index = (start_index + page_size()).min(total_items);

    rsx! {
        div {
            style: "background:linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%); border-radius:16px; padding:24px; border:1px solid #60a5fa;",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:20px;",
                input {
                    value: group.name.clone(),
                    oninput: move |evt| {
                        let mut groups = passenger_groups();
                        groups[group_index].name = evt.value();
                        passenger_groups.set(groups);
                    },
                    style: "flex:1; max-width:300px; padding:10px 14px; border-radius:10px; border:1px solid #60a5fa; font-size:16px; font-weight:600; background:white;",
                    placeholder: "分组名称"
                }
                div {
                    style: "display:flex; gap:12px;",
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:none; background:linear-gradient(120deg,#10b981,#059669); color:white; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            let mut groups = passenger_groups();
                            let new_id = groups[group_index].passengers.iter().map(|p| p.id).max().unwrap_or(0) + 1;
                            groups[group_index].passengers.push(PassengerInfo {
                                id: new_id,
                                first_name: "".to_string(),
                                last_name: "".to_string(),
                                email: "".to_string(),
                                phone: "".to_string(),
                                passport: "".to_string(),
                            });
                            passenger_groups.set(groups);
                        },
                        "+ 添加购票人"
                    }
                    button {
                        style: "padding:10px 18px; border-radius:10px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:600; cursor:pointer;",
                        onclick: move |_| {
                            let mut groups = passenger_groups();
                            groups.remove(group_index);
                            passenger_groups.set(groups);
                        },
                        "删除分组"
                    }
                }
            }

            if !group.passengers.is_empty() {
                div {
                    style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:12px; padding:0 4px;",
                    div {
                        style: "font-size:13px; color:#1e40af;",
                        "共 {total_items} 条记录，第 {start_index + 1}-{end_index} 条"
                    }
                    div {
                        style: "display:flex; align-items:center; gap:12px;",
                        span {
                            style: "font-size:13px; color:#1e40af;",
                            "每页显示："
                        }
                        select {
                            value: page_size().to_string(),
                            onchange: move |evt| {
                                if let Ok(size) = evt.value().parse::<usize>() {
                                    page_size.set(size);
                                    current_page.set(1);
                                }
                            },
                            style: "padding:6px 10px; border-radius:6px; border:1px solid #60a5fa; font-size:13px; background:white; cursor:pointer;",
                            option { value: "20", "20 条" }
                            option { value: "50", "50 条" }
                            option { value: "100", "100 条" }
                        }
                    }
                }
            }
            
            div {
                style: "display:flex; flex-direction:column; gap:16px;",
                for (passenger_index, passenger) in group.passengers.iter().enumerate().skip(start_index).take(page_size()) {
                    PassengerConfigItem {
                        passenger: passenger.clone(),
                        passenger_index,
                        group_index,
                        passenger_groups
                    }
                }
                if group.passengers.is_empty() {
                    div {
                        style: "text-align:center; padding:40px; color:#1e3a8a; font-size:14px;",
                        "暂无购票人信息，点击"添加购票人"开始配置"
                    }
                }
            }

            if total_pages > 1 {
                Pagination { current_page, total_pages }
            }

            div {
                style: "margin-top:20px; padding-top:20px; border-top:1px solid #60a5fa; display:flex; justify-content:flex-end;",
                button {
                    style: "padding:12px 28px; border-radius:12px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3);",
                    onclick: move |_| {
                        let groups = passenger_groups();
                        if let Ok(json) = serde_json::to_string(&groups) {
                            let _ = save_config("passenger_groups", &json);
                        }
                    },
                    "💾 保存分组配置"
                }
            }
        }
    }
}

#[component]
fn PassengerConfigItem(passenger: PassengerInfo, passenger_index: usize, group_index: usize, passenger_groups: Signal<Vec<PassengerGroup>>) -> Element {
    rsx! {
        div {
            style: "background:white; border-radius:14px; padding:18px; border:1px solid #d1d5db;",
            
            div {
                style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:14px;",
                h4 {
                    style: "margin:0; font-size:15px; font-weight:600; color:#1e3a8a;",
                    "购票人 #{passenger.id}"
                }
                button {
                    style: "padding:6px 14px; border-radius:8px; border:1px solid #fca5a5; background:#fee2e2; color:#dc2626; font-weight:500; cursor:pointer; font-size:13px;",
                    onclick: move |_| {
                        let mut groups = passenger_groups();
                        groups[group_index].passengers.remove(passenger_index);
                        passenger_groups.set(groups);
                    },
                    "删除"
                }
            }

            div {
                style: "display:grid; grid-template-columns:1fr 1fr; gap:12px; margin-bottom:12px;",
                ConfigInput {
                    label: "名（First Name）",
                    value: passenger.first_name.clone(),
                    placeholder: "SAN",
                    onchange: move |val| {
                        let mut groups = passenger_groups();
                        groups[group_index].passengers[passenger_index].first_name = val;
                        passenger_groups.set(groups);
                    }
                }
                ConfigInput {
                    label: "姓（Last Name）",
                    value: passenger.last_name.clone(),
                    placeholder: "ZHANG",
                    onchange: move |val| {
                        let mut groups = passenger_groups();
                        groups[group_index].passengers[passenger_index].last_name = val;
                        passenger_groups.set(groups);
                    }
                }
            }

            div {
                style: "display:grid; grid-template-columns:1fr 1fr; gap:12px; margin-bottom:12px;",
                ConfigInput {
                    label: "邮箱",
                    value: passenger.email.clone(),
                    placeholder: "example@email.com",
                    onchange: move |val| {
                        let mut groups = passenger_groups();
                        groups[group_index].passengers[passenger_index].email = val;
                        passenger_groups.set(groups);
                    }
                }
                ConfigInput {
                    label: "电话",
                    value: passenger.phone.clone(),
                    placeholder: "+86 138 0000 0000",
                    onchange: move |val| {
                        let mut groups = passenger_groups();
                        groups[group_index].passengers[passenger_index].phone = val;
                        passenger_groups.set(groups);
                    }
                }
            }

            ConfigInput {
                label: "护照号码",
                value: passenger.passport.clone(),
                placeholder: "E12345678",
                onchange: move |val| {
                    let mut groups = passenger_groups();
                    groups[group_index].passengers[passenger_index].passport = val;
                    passenger_groups.set(groups);
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ConfigInputProps {
    label: &'static str,
    value: String,
    placeholder: &'static str,
    #[props(default = "text")]
    input_type: &'static str,
    onchange: EventHandler<String>,
}

#[component]
fn ConfigInput(props: ConfigInputProps) -> Element {
    rsx! {
        label {
            style: "display:flex; flex-direction:column; gap:6px;",
            span {
                style: "font-size:13px; font-weight:600; color:#374151;",
                "{props.label}"
            }
            input {
                r#type: props.input_type,
                value: props.value,
                placeholder: props.placeholder,
                oninput: move |evt| props.onchange.call(evt.value()),
                style: "padding:11px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; transition:border-color 0.2s ease, box-shadow 0.2s ease; background:white;",
            }
        }
    }
}

#[component]
fn Pagination(current_page: Signal<usize>, total_pages: usize) -> Element {
    rsx! {
        div {
            style: "display:flex; justify-content:center; align-items:center; gap:8px; margin-top:16px;",
            button {
                style: "padding:8px 14px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#6b7280; font-weight:500; cursor:pointer; font-size:13px;",
                disabled: current_page() == 1,
                onclick: move |_| {
                    if current_page() > 1 {
                        current_page.set(current_page() - 1);
                    }
                },
                "上一页"
            }
            for page in 1..=total_pages {
                {
                    let is_current = page == current_page();
                    let btn_style = if is_current {
                        "padding:8px 12px; border-radius:8px; background:#4f46e5; color:white; font-weight:600; cursor:pointer; border:none; font-size:13px;"
                    } else {
                        "padding:8px 12px; border-radius:8px; background:white; color:#6b7280; font-weight:500; cursor:pointer; border:1px solid #d1d5db; font-size:13px;"
                    };
                    rsx! {
                        button {
                            key: "{page}",
                            style: "{btn_style}",
                            onclick: move |_| current_page.set(page),
                            "{page}"
                        }
                    }
                }
            }
            button {
                style: "padding:8px 14px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#6b7280; font-weight:500; cursor:pointer; font-size:13px;",
                disabled: current_page() == total_pages,
                onclick: move |_| {
                    if current_page() < total_pages {
                        current_page.set(current_page() + 1);
                    }
                },
                "下一页"
            }
        }
    }
}
