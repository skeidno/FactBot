use dioxus::prelude::*;

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
    let mut proxy_ip = use_signal(|| "127.0.0.1".to_string());
    let mut proxy_port = use_signal(|| "7897".to_string());
    let mut proxy_username = use_signal(|| "".to_string());
    let mut proxy_password = use_signal(|| "".to_string());
    let mut token = use_signal(|| "".to_string());
    let mut selected_code = use_signal(|| AIRLINE_OPTIONS[0].code.to_string());
    let mut test_message = use_signal(|| "尚未测试连接".to_string());
    let mut query_result = use_signal(|| "".to_string());
    let mut is_querying = use_signal(|| false);

    let current_airline = AIRLINE_OPTIONS
        .iter()
        .find(|option| option.code == selected_code())
        .unwrap_or(&AIRLINE_OPTIONS[0]);

    // 动态生成完整的请求数据用于预览
    let request_preview = use_memo(move || {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let pool = ((timestamp % 9000) + 1000) as i32;
        
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
        "searchParams": {}
    }}
}}"#,
            selected_code(),
            token(),
            pool,
            proxy_username(),
            proxy_password(),
            proxy_ip(),
            proxy_port(),
            current_airline.request_preview
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
                    style: "background:linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%); border-radius:20px; padding:28px 32px; border:1px solid #c7d2fe; box-shadow:0 4px 20px rgba(79, 70, 229, 0.1);",
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
                        "配置代理信息、认证凭证以及测试连接状态"
                    }

                    // 第一行：代理 IP 和端口
                    div {
                        style: "display:grid; grid-template-columns:2fr 1fr; gap:16px; margin-bottom:16px;",
                        FormField {
                            label: "代理 IP 地址",
                            value: proxy_ip(),
                            placeholder: "例：127.0.0.1",
                            onchange: move |value| proxy_ip.set(value),
                        }
                        FormField {
                            label: "代理端口",
                            value: proxy_port(),
                            placeholder: "例：7897",
                            onchange: move |value| proxy_port.set(value),
                        }
                    }

                    // 第二行：用户名和密码
                    div {
                        style: "display:grid; grid-template-columns:1fr 1fr; gap:16px; margin-bottom:16px;",
                        FormField {
                            label: "用户名",
                            value: proxy_username(),
                            placeholder: "可选",
                            onchange: move |value| proxy_username.set(value),
                        }
                        FormField {
                            label: "密码",
                            value: proxy_password(),
                            placeholder: "可选",
                            onchange: move |value| proxy_password.set(value),
                            input_type: "password",
                        }
                    }

                    // 第三行：Token
                    div {
                        style: "margin-bottom:20px;",
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
                            style: BUTTON_PRIMARY_STYLE,
                            onclick: move |_| {
                                test_message.set(format!(
                                    "正在测试代理 {}:{}...", proxy_ip(), proxy_port()
                                ));
                            },
                            "测试代理连接"
                        }
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
                                
                                // 复制当前的请求数据用于查询
                                let request_data = request_preview();
                                
                                // TODO: 这里可以添加实际的 API 调用
                                // 目前只是模拟显示结果
                                query_result.set(format!("✓ 查询请求已发送\n\n使用的请求数据：\n{}", request_data));
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
                            value: token(),
                        }
                        InfoTile {
                            label: "代理地址",
                            value: format!("{}:{}", proxy_ip(), proxy_port()),
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

