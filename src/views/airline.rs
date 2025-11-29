use dioxus::prelude::*;

const AIRLINE_OPTIONS: &[AirlineOption] = &[
    AirlineOption {
        code: "MU",
        name: "东方航空",
        request_preview: r#"{
    "carrier": "MU",
    "region": "CN",
    "currency": "CNY"
}"#,
    },
    AirlineOption {
        code: "CZ",
        name: "南方航空",
        request_preview: r#"{
    "carrier": "CZ",
    "region": "CN",
    "currency": "CNY"
}"#,
    },
    AirlineOption {
        code: "CA",
        name: "中国国航",
        request_preview: r#"{
    "carrier": "CA",
    "region": "CN",
    "currency": "USD"
}"#,
    },
    AirlineOption {
        code: "HU",
        name: "海南航空",
        request_preview: r#"{
    "carrier": "HU",
    "region": "APAC",
    "currency": "USD"
}"#,
    },
];

struct AirlineOption {
    code: &'static str,
    name: &'static str,
    request_preview: &'static str,
}

#[component]
pub fn Airline() -> Element {
    let mut proxy_ip = use_signal(|| "127.0.0.1".to_string());
    let mut proxy_port = use_signal(|| "7897".to_string());
    let mut proxy_username = use_signal(|| "代理用户名".to_string());
    let mut proxy_password = use_signal(|| "代理密码".to_string());
    let mut token = use_signal(|| "Test".to_string());
    let mut selected_code = use_signal(|| AIRLINE_OPTIONS[0].code.to_string());
    let mut test_message = use_signal(|| "尚未测试连接".to_string());

    let current_airline = AIRLINE_OPTIONS
        .iter()
        .find(|option| option.code == selected_code())
        .unwrap_or(&AIRLINE_OPTIONS[0]);

    rsx! {
        div {
            style: "display:flex; flex-direction:column; gap:24px;",

            // 标题区
            section {
                style: "background:#eef2ff; border-radius:16px; padding:24px; border:1px solid #c7d2fe;",
                h1 {
                    style: "font-size:24px; font-weight:600; margin-bottom:8px; color:#312e81;",
                    "航司报价查询控制台"
                }
                p {
                    style: "color:#4338ca; font-size:14px;",
                    "管理代理配置、Token 与航司二字码，快速测试各航司 API 报价参数。"
                }
            }

            // 表单区
            div {
                style: "display:grid; grid-template-columns:repeat(auto-fit,minmax(320px,1fr)); gap:20px;",

                // 代理设置
                FormCard {
                    title: "代理设置",
                    subtitle: "配置代理信息以穿透企业网络",
                    content: rsx! {
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
                        div {
                            style: "margin-top:16px; display:flex; gap:12px; align-items:center;",
                            button {
                                style: BUTTON_PRIMARY_STYLE,
                                onclick: move |_| {
                                    test_message.set(format!(
                                        "正在测试代理 {}:{}...", proxy_ip(), proxy_port()
                                    ));
                                },
                                "测试代理"
                            }
                            span {
                                style: "font-size:13px; color:#4c1d95;",
                                "{test_message}"
                            }
                        }
                    }
                }

                // Token 设置
                FormCard {
                    title: "认证 Token",
                    subtitle: "配置访问 API 所需凭证",
                    content: rsx! {
                        FormField {
                            label: "Token",
                            value: token(),
                            placeholder: "请输入访问 Token",
                            onchange: move |value| token.set(value),
                        }
                        button {
                            style: "margin-top:16px; width:100%;".to_string() + BUTTON_SECONDARY_STYLE,
                            "校验 Token（待接入）"
                        }
                    }
                }
            }

            // 航司选择与请求参数展示
            section {
                style: "background:white; border-radius:16px; padding:24px; border:1px solid #e5e7eb; box-shadow:0 10px 25px rgba(15,23,42,0.08);",
                header {
                    style: "display:flex; flex-wrap:wrap; align-items:center; gap:16px; margin-bottom:20px;",
                    div {
                        style: "flex:1;",
                        h2 {
                            style: "font-size:20px; font-weight:600; margin:0;",
                            "航司报价参数"
                        }
                        p {
                            style: "margin:4px 0 0; color:#6b7280; font-size:14px;",
                            "选择航司二字码，检查参数模版立即切换。"
                        }
                    }
                    select {
                        style: "padding:10px 14px; border-radius:12px; border:1px solid #d1d5db; font-size:15px; font-weight:500;",
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
                        "立即查询（待接入）"
                    }
                }

                div {
                    style: "display:grid; grid-template-columns:repeat(auto-fit,minmax(280px,1fr)); gap:16px;",

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
                    style: "margin-top:20px;",
                    label {
                        style: "display:block; font-size:13px; font-weight:600; color:#4b5563; margin-bottom:8px;",
                        "请求参数示例"
                    }
                    pre {
                        style: "background:#0f172a; color:#e0e7ff; border-radius:12px; padding:16px; overflow:auto; font-size:13px;",
                        "{current_airline.request_preview}"
                    }
                }
            }
        }
    }
}

const BUTTON_PRIMARY_STYLE: &str = "padding:10px 18px; border-radius:12px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer;";
const BUTTON_SECONDARY_STYLE: &str = "border-radius:12px; border:1px solid #c7d2fe; background:#eef2ff; padding:10px 18px; color:#4338ca; font-weight:600;";

#[component]
fn FormCard(title: &'static str, subtitle: &'static str, content: Element) -> Element {
    rsx! {
        section {
            style: "background:white; border-radius:16px; padding:20px; border:1px solid #e5e7eb; box-shadow:0 15px 30px rgba(15,23,42,0.05);",
            h3 {
                style: "margin:0; font-size:18px; font-weight:600;",
                "{title}"
            }
            p {
                style: "margin:4px 0 16px; font-size:13px; color:#6b7280;",
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
                style: "padding:10px 12px; border-radius:10px; border:1px solid #d1d5db; font-size:14px;",
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
            style: "background:#f3f4f6; border-radius:12px; padding:12px 16px;",
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

