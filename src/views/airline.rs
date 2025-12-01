use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::load_config;

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

const AIRLINE_OPTIONS: &[AirlineOption] = &[
    AirlineOption {
        code: "AA",
        name: "美国航空（现金）",
        url: "https://www.americanairlines.cn/intl/cn/index.jsp?locale=zh_CN",
        request_preview: r#"{
    "TRIP_TYPE": "O",
    "B_LOCATION_1": "TYO",
    "E_LOCATION_1": "JFK",
    "TRAVELLER_TYPE_1": "ADT"
}"#,
    },
    AirlineOption {
        code: "B2",
        name: "白俄罗斯航空",
        url: "https://en.belavia.by/",
        request_preview: r#"{
    "currency": "BYN",
    "segments": [{"departure": {"iata": "MSQ"}, "arrival": {"iata": "NOJ"}}],
    "passengers": [{"passengerType": "ADT", "count": 1}]
}"#,
    },
    AirlineOption {
        code: "FS",
        name: "飞狮航空",
        url: "https://www.flyarystan.com/",
        request_preview: r#"{
    "tripType": "ONE_WAY",
    "depPort": "ALA",
    "arrPort": "SCO",
    "adult": "1",
    "currency": "KZT"
}"#,
    },
    AirlineOption {
        code: "VJ",
        name: "越捷航空",
        url: "https://www.vietjetair.com/zh-CN/",
        request_preview: r#"{
    "currency": "AUD",
    "departurePlace": "HAN",
    "arrival": "BMV",
    "oneway": 1,
    "adultCount": 1
}"#,
    },
    AirlineOption {
        code: "VA",
        name: "维珍航空",
        url: "https://www.virginaustralia.com/au/en/",
        request_preview: r#"{
    "cabinClass": "First",
    "awardBooking": true,
    "from": {"code": "ORD"},
    "to": {"code": "HND"}
}"#,
    },
    AirlineOption {
        code: "OZ",
        name: "韩亚航空（里程票）",
        url: "https://flyasiana.com/C/US/EN/index",
        request_preview: r#"{
    "bizType": "RED",
    "tripType": "OW",
    "departureAirport": "LAX",
    "arrivalAirport": "ICN",
    "cabinClassList": ["B"]
}"#,
    },
    AirlineOption {
        code: "CM",
        name: "巴拿马航空（里程票）",
        url: "https://www.copaair.com/en-gs/",
        request_preview: r#"{
    "adults": 1,
    "departureAirport1": "LAX",
    "arrivalAirport1": "ICN",
    "isRoundTrip": false
}"#,
    },
    AirlineOption {
        code: "CM_CASH",
        name: "巴拿马航空（现金票）",
        url: "https://www.copaair.com/en-gs/",
        request_preview: r#"{
    "departureAirport1": "ASU",
    "arrivalAirport1": "PTY",
    "adults": "1",
    "isRoundTrip": "false"
}"#,
    },
    AirlineOption {
        code: "U6",
        name: "乌拉尔航空",
        url: "https://www.uralairlines.ru/en/",
        request_preview: r#"{
    "orig": "BJS",
    "dest": "SVX",
    "mode": "Mono",
    "adt": "1"
}"#,
    },
    AirlineOption {
        code: "S7",
        name: "西伯利亚航空",
        url: "https://www.s7.ru/",
        request_preview: r#"{
    "currency": "RUB",
    "origin": "MOW",
    "destination": "EVN",
    "tripType": "ONE_WAY"
}"#,
    },
    AirlineOption {
        code: "IA",
        name: "伊拉克航空",
        url: "http://www.iraqiairways.com.iq/",
        request_preview: r#"{
    "originLocationCode": "EBL",
    "destinationLocationCode": "BGW",
    "passengerTypeCode": "ADT"
}"#,
    },
    AirlineOption {
        code: "SU",
        name: "俄罗斯国际航空",
        url: "https://www.aeroflot.ru/",
        request_preview: r#"{
    "origin": "ABA",
    "destination": "MOW",
    "cabin": "economy",
    "adults": 1
}"#,
    },
    AirlineOption {
        code: "N4",
        name: "北风航空",
        url: "https://nordwindairlines.ru/ru",
        request_preview: r#"{
    "departure": {"iata": "KZN"},
    "arrival": {"iata": "MOW"},
    "passengerType": "ADT"
}"#,
    },
    AirlineOption {
        code: "DD",
        name: "皇雀航空",
        url: "https://www.nokair.com/",
        request_preview: r#"{
    "fromAirport": "DMK",
    "toAirport": "CNX",
    "currency": "THB",
    "passengers": [{"code": "ADT", "count": 1}]
}"#,
    },
    AirlineOption {
        code: "MH",
        name: "马来西亚国际航空",
        url: "https://www.malaysiaairlines.com/cn/zh_CN/home.html",
        request_preview: r#"{
    "originLocationCode": "PKX",
    "destinationLocationCode": "JED",
    "passengerTypeCode": "ADT"
}"#,
    },
    AirlineOption {
        code: "5J",
        name: "宿务航空",
        url: "https://www.cebupacificair.com/en-PH/",
        request_preview: r#"{
    "origin": "ICN",
    "destination": "KUL",
    "adultCount": 1,
    "currency": "KRW"
}"#,
    },
];

struct AirlineOption {
    code: &'static str,
    name: &'static str,
    #[allow(dead_code)]
    url: &'static str,
    request_preview: &'static str,
}

#[component]
pub fn Airline() -> Element {
    // 加载代理分组
    let proxy_groups = use_signal(|| {
        load_config("proxy_groups")
            .ok()
            .flatten()
            .and_then(|json| serde_json::from_str::<Vec<ProxyGroup>>(&json).ok())
            .unwrap_or_default()
    });

    let mut selected_group_index = use_signal(|| 0usize);
    let mut token = use_signal(|| "".to_string());
    let mut selected_code = use_signal(|| AIRLINE_OPTIONS[0].code.to_string());
    let test_message = use_signal(|| "尚未测试连接".to_string());
    let mut query_result = use_signal(|| "".to_string());
    let mut is_querying = use_signal(|| false);

    let current_airline = AIRLINE_OPTIONS
        .iter()
        .find(|option| option.code == selected_code())
        .unwrap_or(&AIRLINE_OPTIONS[0]);

    // 获取当前选中的代理分组（使用 memo 避免重复计算）
    let current_group = use_memo(move || {
        proxy_groups().get(selected_group_index()).cloned()
    });

    // 动态生成完整的请求数据用于预览（使用分组中第一个代理作为示例）
    let request_preview = use_memo(move || {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let pool = ((timestamp % 9000) + 1000) as i32;
        
        let (proxy_ip, proxy_port, proxy_user, proxy_pass) = if let Some(group) = proxy_groups().get(selected_group_index()) {
            if let Some(proxy) = group.proxies.first() {
                (proxy.ip.clone(), proxy.port.clone(), proxy.username.clone(), proxy.password.clone())
            } else {
                ("".to_string(), "".to_string(), "".to_string(), "".to_string())
            }
        } else {
            ("".to_string(), "".to_string(), "".to_string(), "".to_string())
        };
        
        // 转义 searchParams 为字符串
        let search_params_str = current_airline.request_preview
            .replace('\n', "")
            .replace("  ", "")
            .replace("\"", "\\\"");
        
        format!(
            r#"{{
  "air_type": "{}",
  "analysis": true,
  "token": "{}",
  "data": {{
    "pool": {},
    "proxy_config": {{
      "proxyUser": "{}",
      "proxyPass": "{}",
      "proxyHost": "{}",
      "proxyPort": "{}"
    }},
    "searchParams": "{}"
  }}
}}"#,
            selected_code(),
            token(),
            pool,
            proxy_user,
            proxy_pass,
            proxy_ip,
            proxy_port,
            search_params_str
        )
    });

    rsx! {
        // 自定义滚动条样式
        document::Style {
            r#"
            .airline-scroll-container {{
                scrollbar-width: thin;
                scrollbar-color: rgba(99, 102, 241, 0.3) transparent;
            }}
            .airline-scroll-container::-webkit-scrollbar {{
                width: 8px;
            }}
            .airline-scroll-container::-webkit-scrollbar-track {{
                background: transparent;
                border-radius: 10px;
            }}
            .airline-scroll-container::-webkit-scrollbar-thumb {{
                background: rgba(99, 102, 241, 0.3);
                border-radius: 10px;
                transition: background 0.2s ease;
            }}
            .airline-scroll-container::-webkit-scrollbar-thumb:hover {{
                background: rgba(99, 102, 241, 0.5);
            }}
            "#
        }
        
        div {
            class: "airline-scroll-container",
            style: "height:100%; overflow-y:auto; overflow-x:hidden; padding:24px 16px 24px 0;",
            
            div {
                style: "display:flex; flex-direction:column; gap:28px; max-width:1400px; margin:0 auto;",

                // 标题区
                section {
                    style: "background:linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%); border-radius:20px; padding:28px 32px; border:1px solid #c7d2fe; box-shadow:0 4px 20px rgba(79, 70, 229, 0.1); position:relative;",
                    
                    // 右上角端口号和健康状态显示
                    BackendStatus {}
                    
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#312e81; letter-spacing:-0.02em;",
                        "航司报价查询控制台"
                    }
                    p {
                        style: "color:#4338ca; font-size:15px; margin:0; line-height:1.6;",
                        "管理代理配置、Token 与航司二字码，快速测试各航司 API 报价参数。"
                    }
                }

                // 配置表单区 - 统一卡片
                section {
                    style: "background:white; border-radius:18px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 6px 24px rgba(15,23,42,0.06);",
                    
                    h2 {
                        style: "margin:0 0 8px 0; font-size:20px; font-weight:700; color:#111827; letter-spacing:-0.01em;",
                        "配置参数"
                    }
                    p {
                        style: "margin:0 0 24px 0; font-size:14px; color:#6b7280; line-height:1.5;",
                        "选择代理分组和配置认证凭证"
                    }

                    // 代理分组和 Token 配置（一行）
                    div {
                        style: "display:grid; grid-template-columns:1fr 1fr; gap:16px; margin-bottom:20px;",
                        
                        // 代理分组选择
                        div {
                            label {
                                style: "display:block; font-size:13px; font-weight:600; color:#374151; margin-bottom:8px;",
                                "代理分组"
                            }
                            if proxy_groups().is_empty() {
                                div {
                                    style: "padding:11px 14px; border-radius:10px; background:#fef3c7; border:1px solid #fbbf24; color:#92400e; font-size:13px;",
                                    "⚠️ 暂无代理"
                                }
                            } else {
                                select {
                                    style: "width:100%; padding:11px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:white;",
                                    value: selected_group_index().to_string(),
                                    onchange: move |evt| {
                                        if let Ok(index) = evt.value().parse::<usize>() {
                                            selected_group_index.set(index);
                                        }
                                    },
                                    for (index, group) in proxy_groups().iter().enumerate() {
                                        option {
                                            value: "{index}",
                                            "{group.name} ({group.proxies.len()} 个)"
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Token 配置
                        FormField {
                            label: "认证 Token",
                            value: token(),
                            placeholder: "请输入访问 Token",
                            onchange: move |value| token.set(value),
                        }
                    }

                    // 操作按钮行
                    div {
                        style: "display:flex; gap:12px; align-items:center; padding-top:8px; border-top:1px solid #f3f4f6;",
                        button {
                            style: BUTTON_SECONDARY_STYLE,
                            "校验 Token"
                        }
                        span {
                            style: "font-size:13px; color:#6b7280; margin-left:8px;",
                            "{test_message}"
                        }
                    }
                }

                // 航司选择与请求参数展示
                section {
                    style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
                    
                    header {
                        style: "display:flex; flex-wrap:wrap; align-items:center; gap:16px; margin-bottom:24px;",
                        div {
                            style: "flex:1; min-width:200px;",
                            h2 {
                                style: "font-size:22px; font-weight:700; margin:0 0 6px 0; color:#111827; letter-spacing:-0.02em;",
                                "航司报价参数"
                            }
                            p {
                                style: "margin:0; color:#6b7280; font-size:14px; line-height:1.5;",
                                "选择航司二字码，检查参数模版立即切换。"
                            }
                        }
                        select {
                            style: "padding:11px 16px; border-radius:12px; border:1px solid #d1d5db; font-size:15px; font-weight:500; background:white; cursor:pointer; transition:all 0.2s ease;",
                            value: selected_code(),
                            onchange: move |event| {
                                selected_code.set(event.value());
                            },
                            for option in AIRLINE_OPTIONS {
                                option {
                                    value: "{option.code}",
                                    "{option.code} · {option.name}"
                                }
                            }
                        }
                        button {
                            style: BUTTON_PRIMARY_STYLE,
                            disabled: is_querying(),
                            onclick: move |_| {
                                is_querying.set(true);
                                query_result.set("正在发送查询请求...".to_string());
                                
                                // 从当前分组中随机选择一个代理
                                let (proxy_ip, proxy_port, proxy_user, proxy_pass) = if let Some(group) = proxy_groups().get(selected_group_index()) {
                                    if !group.proxies.is_empty() {
                                        use std::time::{SystemTime, UNIX_EPOCH};
                                        let timestamp = SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis();
                                        let random_index = (timestamp as usize) % group.proxies.len();
                                        let proxy = &group.proxies[random_index];
                                        (proxy.ip.clone(), proxy.port.clone(), proxy.username.clone(), proxy.password.clone())
                                    } else {
                                        ("".to_string(), "".to_string(), "".to_string(), "".to_string())
                                    }
                                } else {
                                    ("".to_string(), "".to_string(), "".to_string(), "".to_string())
                                };
                                
                                // 生成随机 pool
                                use std::time::{SystemTime, UNIX_EPOCH};
                                let timestamp = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis();
                                let pool = ((timestamp % 9000) + 1000) as i32;
                                
                                // 转义 searchParams 为字符串
                                let search_params_str = current_airline.request_preview
                                    .replace('\n', "")
                                    .replace("  ", "")
                                    .replace("\"", "\\\"");
                                
                                // 构建请求数据
                                let request_data = format!(
                                    r#"{{
  "air_type": "{}",
  "analysis": true,
  "token": "{}",
  "data": {{
    "pool": {},
    "proxy_config": {{
      "proxyUser": "{}",
      "proxyPass": "{}",
      "proxyHost": "{}",
      "proxyPort": "{}"
    }},
    "searchParams": "{}"
  }}
}}"#,
                                    selected_code(),
                                    token(),
                                    pool,
                                    proxy_user,
                                    proxy_pass,
                                    proxy_ip,
                                    proxy_port,
                                    search_params_str
                                );
                                
                                // TODO: 这里可以添加实际的 API 调用
                                query_result.set(format!("✓ 查询请求已发送\n\n随机选择的代理：{}:{}\n\n完整请求数据：\n{}", proxy_ip, proxy_port, request_data));
                                is_querying.set(false);
                            },
                            if is_querying() { "查询中..." } else { "立即查询" }
                        }
                    }

                    div {
                        style: "display:grid; grid-template-columns:repeat(auto-fit,minmax(220px,1fr)); gap:18px;",
                        InfoTile {
                            label: "当前航司",
                            value: format!("{} · {}", current_airline.code, current_airline.name),
                        }
                        InfoTile {
                            label: "Token",
                            value: if token().is_empty() { "未配置".to_string() } else { "已配置".to_string() },
                        }
                        InfoTile {
                            label: "代理分组",
                            value: if let Some(group) = current_group() {
                                format!("{} ({} 个)", group.name, group.proxies.len())
                            } else {
                                "未选择".to_string()
                            },
                        }
                    }

                    div {
                        style: "margin-top:24px;",
                        label {
                            style: "display:block; font-size:14px; font-weight:600; color:#4b5563; margin-bottom:10px;",
                            "完整请求数据（实时更新）"
                        }
                        pre {
                            style: "background:linear-gradient(135deg, #0f172a 0%, #1e293b 100%); color:#e0e7ff; border-radius:14px; padding:18px; overflow:auto; font-size:12px; line-height:1.6; max-height:400px; border:1px solid rgba(148, 163, 184, 0.1); box-shadow:inset 0 2px 8px rgba(0,0,0,0.3); white-space:pre-wrap; word-wrap:break-word;",
                            "{request_preview()}"
                        }
                    }
                }

                // 查询结果展示区
                if !query_result().is_empty() {
                    section {
                        style: "background:linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%); border-radius:20px; padding:28px 32px; border:1px solid #86efac; box-shadow:0 4px 20px rgba(34, 197, 94, 0.1);",
                        h2 {
                            style: "margin:0 0 8px 0; font-size:20px; font-weight:700; color:#166534; letter-spacing:-0.01em;",
                            "查询结果"
                        }
                        p {
                            style: "margin:0 0 16px 0; font-size:14px; color:#15803d; line-height:1.5;",
                            "以下是生成的完整请求数据，可直接用于 API 调用"
                        }
                        pre {
                            style: "background:linear-gradient(135deg, #0f172a 0%, #1e293b 100%); color:#a5f3fc; border-radius:14px; padding:18px; overflow:auto; font-size:12px; line-height:1.6; max-height:400px; border:1px solid rgba(148, 163, 184, 0.1); box-shadow:inset 0 2px 8px rgba(0,0,0,0.3); white-space:pre-wrap; word-wrap:break-word;",
                            "{query_result()}"
                        }
                    }
                }
            }
        }
    }
}

const BUTTON_PRIMARY_STYLE: &str = "padding:11px 20px; border-radius:12px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3); transition:all 0.2s ease;";
const BUTTON_SECONDARY_STYLE: &str = "border-radius:12px; border:1px solid #c7d2fe; background:#eef2ff; padding:11px 20px; color:#4338ca; font-weight:600; cursor:pointer; transition:all 0.2s ease;";

#[component]
fn FormCard(title: &'static str, subtitle: &'static str, content: Element) -> Element {
    rsx! {
        section {
            style: "background:white; border-radius:18px; padding:24px; border:1px solid #e5e7eb; box-shadow:0 6px 24px rgba(15,23,42,0.06); transition:box-shadow 0.3s ease;",
            h3 {
                style: "margin:0 0 6px 0; font-size:19px; font-weight:700; color:#111827; letter-spacing:-0.01em;",
                "{title}"
            }
            p {
                style: "margin:0 0 18px 0; font-size:13px; color:#6b7280; line-height:1.5;",
                "{subtitle}"
            }
            {content}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct FormFieldProps {
    label: &'static str,
    value: String,
    placeholder: &'static str,
    onchange: EventHandler<String>,
    #[props(default = "text")]
    input_type: &'static str,
}

#[component]
fn FormField(props: FormFieldProps) -> Element {
    rsx! {
        label {
            style: "display:flex; flex-direction:column; gap:6px; font-size:13px; font-weight:600; color:#374151; margin-bottom:12px;",
            span { "{props.label}" }
            input {
                r#type: props.input_type,
                value: props.value.clone(),
                placeholder: props.placeholder,
                oninput: move |event| props.onchange.call(event.value()),
                style: "padding:11px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; transition:border-color 0.2s ease, box-shadow 0.2s ease; background:white;",
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct InfoTileProps {
    label: &'static str,
    value: String,
}

#[component]
fn InfoTile(props: InfoTileProps) -> Element {
    rsx! {
        div {
            style: "background:linear-gradient(135deg, #f9fafb 0%, #f3f4f6 100%); border-radius:14px; padding:14px 18px; border:1px solid #e5e7eb;",
            span {
                style: "display:block; font-size:12px; color:#6b7280; text-transform:uppercase; letter-spacing:0.08em;",
                "{props.label}"
            }
            strong {
                style: "display:block; margin-top:6px; font-size:16px; color:#111827;",
                "{props.value}"
            }
        }
    }
}


/// 后台健康状态组件
#[component]
fn BackendStatus() -> Element {
    let backend_port = use_signal(|| "8080".to_string());
    let mut is_healthy = use_signal(|| false);
    let mut is_checking = use_signal(|| true);
    let mut last_status = use_signal(|| None::<bool>);
    let mut first_check = use_signal(|| true);
    
    use_effect(move || {
        spawn(async move {
            use chrono::Local;
            use uuid::Uuid;
            
            let task_uuid = Uuid::new_v4().to_string();
            
            if first_check() {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let _ = crate::db::save_log(
                    "Backend Health Check",
                    "INFO",
                    &task_uuid,
                    &timestamp,
                    "开始检查 Web 后台健康状态"
                );
                first_check.set(false);
            }
            
            loop {
                is_checking.set(true);
                
                let port = backend_port();
                let health_url = format!("http://localhost:{}/health", port);
                
                let current_healthy = match reqwest::get(&health_url).await {
                    Ok(response) => response.status().is_success(),
                    Err(_) => false,
                };
                
                is_healthy.set(current_healthy);
                is_checking.set(false);
                
                let status_changed = match last_status() {
                    None => true,
                    Some(last) => last != current_healthy,
                };
                
                if status_changed {
                    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let (status_text, log_level) = if current_healthy {
                        ("运行中", "SUCCESS")
                    } else {
                        ("离线", "ERROR")
                    };
                    let message = format!("Web 后台状态: {}", status_text);
                    
                    let _ = crate::db::save_log(
                        "Backend Health Check",
                        log_level,
                        &task_uuid,
                        &timestamp,
                        &message
                    );
                    
                    last_status.set(Some(current_healthy));
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    });
    
    let (status_color, status_text, status_bg) = if is_checking() {
        ("#94a3b8", "检查中", "rgba(148, 163, 184, 0.1)")
    } else if is_healthy() {
        ("#10b981", "运行中", "rgba(16, 185, 129, 0.1)")
    } else {
        ("#ef4444", "离线", "rgba(239, 68, 68, 0.1)")
    };
    
    rsx! {
        div {
            style: "position:absolute; top:28px; right:32px; background:{status_bg}; border:1px solid {status_color}; border-radius:12px; padding:8px 16px; display:flex; align-items:center; gap:12px;",
            
            div {
                style: "display:flex; align-items:center; gap:6px;",
                div {
                    style: "width:10px; height:10px; border-radius:50%; background:{status_color}; box-shadow:0 0 8px {status_color};",
                }
                span {
                    style: "font-size:13px; font-weight:600; color:{status_color};",
                    "{status_text}"
                }
            }
            
            div {
                style: "width:1px; height:20px; background:rgba(148, 163, 184, 0.3);",
            }
            
            div {
                style: "display:flex; align-items:center; gap:6px;",
                span {
                    style: "font-size:13px; font-weight:600; color:#4338ca;",
                    "端口:"
                }
                span {
                    style: "font-size:15px; font-weight:700; color:#4f46e5; font-family:monospace;",
                    "{backend_port}"
                }
            }
        }
    }
}
