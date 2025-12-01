use dioxus::prelude::*;
use crate::db::{self, LogEntry};

#[component]
pub fn Logs() -> Element {
    // 状态管理
    let mut logs = use_signal(Vec::<LogEntry>::new);
    let mut task_names = use_signal(Vec::<String>::new);
    let mut task_uuids = use_signal(Vec::<String>::new);
    
    // 筛选条件
    let mut selected_task_name = use_signal(String::new);
    let mut selected_task_uuid = use_signal(String::new);
    let mut selected_log_level = use_signal(String::new);
    let mut keyword = use_signal(String::new);
    let mut sort_order = use_signal(|| "desc".to_string()); // desc 或 asc
    
    // 分页
    let mut page_size = use_signal(|| 50usize);
    let mut current_page = use_signal(|| 1usize);

    // 初始化加载任务名称列表
    use_effect(move || {
        if let Ok(all_logs) = db::get_all_logs() {
            let mut names: Vec<String> = all_logs
                .iter()
                .map(|log| log.task_name.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            names.sort();
            task_names.set(names);
        }
    });

    // 当任务名称改变时，更新任务 UUID 列表
    let mut update_task_uuids = move || {
        let task_name = selected_task_name();
        if task_name.is_empty() {
            task_uuids.set(Vec::new());
            return;
        }
        
        if let Ok(all_logs) = db::get_all_logs() {
            let mut uuids: Vec<String> = all_logs
                .iter()
                .filter(|log| log.task_name == task_name)
                .map(|log| log.task_uuid.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            uuids.sort();
            task_uuids.set(uuids);
        }
    };

    // 加载日志数据
    let mut load_logs = move || {
        let task_name = selected_task_name();
        if task_name.is_empty() {
            logs.set(Vec::new());
            return;
        }

        if let Ok(mut all_logs) = db::get_all_logs() {
            // 正式版：过滤掉 DEBUG 日志
            #[cfg(not(feature = "dev"))]
            {
                all_logs.retain(|log| log.log_level != "DEBUG");
            }

            // 按任务名称筛选
            all_logs.retain(|log| log.task_name == task_name);

            // 按任务 UUID 筛选
            let task_uuid = selected_task_uuid();
            if !task_uuid.is_empty() {
                all_logs.retain(|log| log.task_uuid.contains(&task_uuid));
            }

            // 按日志级别筛选
            let log_level = selected_log_level();
            if !log_level.is_empty() {
                all_logs.retain(|log| log.log_level == log_level);
            }

            // 按关键词筛选
            let kw = keyword();
            if !kw.is_empty() {
                all_logs.retain(|log| log.message.contains(&kw));
            }

            // 排序
            if sort_order() == "asc" {
                all_logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            } else {
                all_logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            }

            logs.set(all_logs);
            current_page.set(1);
        }
    };

    // 分页计算
    let total_items = logs().len();
    let total_pages = if total_items == 0 { 1 } else { total_items.div_ceil(page_size()) };
    let start_index = (current_page() - 1) * page_size();
    let end_index = (start_index + page_size()).min(total_items);
    let current_logs: Vec<LogEntry> = logs().into_iter().skip(start_index).take(page_size()).collect();

    rsx! {
        document::Style {
            r#"
            .logs-scroll-container {{
                scrollbar-width: thin;
                scrollbar-color: rgba(99, 102, 241, 0.3) transparent;
            }}
            .logs-scroll-container::-webkit-scrollbar {{
                width: 8px;
            }}
            .logs-scroll-container::-webkit-scrollbar-track {{
                background: transparent;
                border-radius: 10px;
            }}
            .logs-scroll-container::-webkit-scrollbar-thumb {{
                background: rgba(99, 102, 241, 0.3);
                border-radius: 10px;
                transition: background 0.2s ease;
            }}
            .logs-scroll-container::-webkit-scrollbar-thumb:hover {{
                background: rgba(99, 102, 241, 0.5);
            }}
            "#
        }
        
        script {
            r#"
            // 复制日志内容到剪贴板
            function copyLogText(text) {{
                console.log('尝试复制:', text.substring(0, 50) + '...');
                if (navigator.clipboard && navigator.clipboard.writeText) {{
                    navigator.clipboard.writeText(text)
                        .then(() => {{
                            console.log('复制成功');
                        }})
                        .catch(err => {{
                            console.error('复制失败:', err);
                            // 降级方案：使用 execCommand
                            fallbackCopy(text);
                        }});
                }} else {{
                    // 降级方案
                    fallbackCopy(text);
                }}
            }}
            
            function fallbackCopy(text) {{
                const textarea = document.createElement('textarea');
                textarea.value = text;
                textarea.style.position = 'fixed';
                textarea.style.opacity = '0';
                document.body.appendChild(textarea);
                textarea.select();
                try {{
                    document.execCommand('copy');
                    console.log('使用降级方案复制成功');
                }} catch (err) {{
                    console.error('降级方案也失败:', err);
                }}
                document.body.removeChild(textarea);
            }}
            
            // 监听所有复制按钮点击
            document.addEventListener('click', function(e) {{
                if (e.target.hasAttribute('data-copy-text')) {{
                    const text = e.target.getAttribute('data-copy-text');
                    copyLogText(text);
                }}
            }});
            "#
        }

        div {
            class: "logs-scroll-container",
            style: "height:100%; overflow-y:auto; overflow-x:hidden; padding:24px 16px 24px 0;",
            
            div {
                style: "display:flex; flex-direction:column; gap:24px; max-width:1600px; margin:0 auto;",

                // 页面标题
                section {
                    style: "background:linear-gradient(135deg, #ede9fe 0%, #ddd6fe 100%); border-radius:20px; padding:28px 32px; border:1px solid #a78bfa; box-shadow:0 4px 20px rgba(139, 92, 246, 0.1);",
                    h1 {
                        style: "font-size:26px; font-weight:700; margin:0 0 10px 0; color:#5b21b6; letter-spacing:-0.02em;",
                        "📋 日志查看器"
                    }
                    p {
                        style: "color:#7c3aed; font-size:15px; margin:0; line-height:1.6;",
                        "查看和筛选任务执行日志，支持按任务名称、UUID、级别和关键词过滤"
                    }
                }

                // 筛选区域
                section {
                    style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
                    
                    h2 {
                        style: "font-size:18px; font-weight:700; margin:0 0 20px 0; color:#111827;",
                        "🔍 筛选条件"
                    }

                    div {
                        style: "display:grid; grid-template-columns:repeat(auto-fit, minmax(250px, 1fr)); gap:16px; margin-bottom:20px;",
                        
                        // 任务名称（必填）
                        div {
                            label {
                                style: "display:block; margin-bottom:8px; font-size:14px; font-weight:600; color:#374151;",
                                "任务名称 ",
                                span { style: "color:#ef4444;", "*" }
                            }
                            select {
                                value: selected_task_name(),
                                onchange: move |evt| {
                                    selected_task_name.set(evt.value());
                                    selected_task_uuid.set(String::new());
                                    update_task_uuids();
                                },
                                style: "width:100%; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:white; color:#111827; cursor:pointer; -webkit-appearance:none; appearance:none;",
                                option { value: "", "-- 请选择任务名称 --" }
                                for name in task_names() {
                                    option { value: "{name}", "{name}" }
                                }
                            }
                        }

                        // 任务 UUID（选填）
                        div {
                            label {
                                style: "display:block; margin-bottom:8px; font-size:14px; font-weight:600; color:#374151;",
                                "任务 UUID"
                            }
                            select {
                                value: selected_task_uuid(),
                                onchange: move |evt| selected_task_uuid.set(evt.value()),
                                disabled: selected_task_name().is_empty(),
                                style: if selected_task_name().is_empty() {
                                    "width:100%; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:#f3f4f6; color:#9ca3af; cursor:not-allowed; -webkit-appearance:none; appearance:none;"
                                } else {
                                    "width:100%; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:white; color:#111827; cursor:pointer; -webkit-appearance:none; appearance:none;"
                                },
                                option { value: "", "-- 全部 UUID --" }
                                for uuid in task_uuids() {
                                    option { value: "{uuid}", "{uuid}" }
                                }
                            }
                        }

                        // 日志级别（选填）
                        div {
                            label {
                                style: "display:block; margin-bottom:8px; font-size:14px; font-weight:600; color:#374151;",
                                "日志级别"
                            }
                            select {
                                value: selected_log_level(),
                                onchange: move |evt| selected_log_level.set(evt.value()),
                                style: "width:100%; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px; background:white; color:#111827; cursor:pointer; -webkit-appearance:none; appearance:none;",
                                option { value: "", "-- 全部级别 --" }
                                // dev 模式：显示 DEBUG 选项
                                // 正式版：不显示 DEBUG 选项
                                if cfg!(feature = "dev") {
                                    option { value: "DEBUG", "🔵 DEBUG" }
                                }
                                option { value: "INFO", "⚪ INFO" }
                                option { value: "WARN", "🟡 WARN" }
                                option { value: "ERROR", "🔴 ERROR" }
                            }
                        }

                        // 关键词搜索（选填）
                        div {
                            label {
                                style: "display:block; margin-bottom:8px; font-size:14px; font-weight:600; color:#374151;",
                                "关键词搜索"
                            }
                            input {
                                value: keyword(),
                                oninput: move |evt| keyword.set(evt.value()),
                                style: "width:100%; padding:10px 14px; border-radius:10px; border:1px solid #d1d5db; font-size:14px;",
                                placeholder: "搜索日志内容..."
                            }
                        }
                    }

                    div {
                        style: "display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:12px;",
                        
                        // 时间排序
                        div {
                            style: "display:flex; align-items:center; gap:12px;",
                            span {
                                style: "font-size:14px; font-weight:600; color:#374151;",
                                "时间排序："
                            }
                            button {
                                style: if sort_order() == "desc" {
                                    "padding:8px 16px; border-radius:8px; border:none; background:#4f46e5; color:white; font-weight:600; cursor:pointer;"
                                } else {
                                    "padding:8px 16px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#6b7280; font-weight:600; cursor:pointer;"
                                },
                                onclick: move |_| sort_order.set("desc".to_string()),
                                "⬇ 最新在前"
                            }
                            button {
                                style: if sort_order() == "asc" {
                                    "padding:8px 16px; border-radius:8px; border:none; background:#4f46e5; color:white; font-weight:600; cursor:pointer;"
                                } else {
                                    "padding:8px 16px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#6b7280; font-weight:600; cursor:pointer;"
                                },
                                onclick: move |_| sort_order.set("asc".to_string()),
                                "⬆ 最旧在前"
                            }
                        }

                        // 查询按钮
                        button {
                            style: "padding:10px 28px; border-radius:10px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; box-shadow:0 4px 12px rgba(79, 70, 229, 0.3);",
                            onclick: move |_| load_logs(),
                            "🔍 查询日志"
                        }
                    }
                }

                // 日志列表
                if !logs().is_empty() {
                    section {
                        style: "background:white; border-radius:20px; padding:28px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08);",
                        
                        div {
                            style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:16px;",
                            div {
                                style: "font-size:14px; color:#6b7280;",
                                "共 {total_items} 条日志，第 {start_index + 1}-{end_index} 条"
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
                                    option { value: "200", "200 条" }
                                }
                            }
                        }

                        div {
                            style: "background:#f9fafb; border-radius:12px; overflow:hidden; border:1px solid #e5e7eb;",
                            table {
                                style: "width:100%; border-collapse:collapse;",
                                thead {
                                    tr {
                                        style: "background:#f3f4f6;",
                                        th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb; width:140px;", "任务 ID" }
                                        th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb; width:100px;", "级别" }
                                        th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb; width:160px;", "时间" }
                                        th { style: "padding:12px 16px; text-align:left; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb;", "日志内容" }
                                        th { style: "padding:12px 16px; text-align:center; font-size:13px; font-weight:600; color:#374151; border-bottom:2px solid #e5e7eb; width:100px;", "操作" }
                                    }
                                }
                                tbody {
                                    for log in current_logs {
                                        LogTableRow { log }
                                    }
                                }
                            }
                        }

                        if total_pages > 1 {
                            div {
                                style: "display:flex; justify-content:center; align-items:center; gap:8px; margin-top:20px;",
                                
                                button {
                                    disabled: current_page() == 1,
                                    style: "padding:8px 16px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#6b7280; font-weight:600; cursor:pointer; disabled:opacity-50;",
                                    onclick: move |_| {
                                        if current_page() > 1 {
                                            current_page.set(current_page() - 1);
                                        }
                                    },
                                    "← 上一页"
                                }

                                span {
                                    style: "padding:8px 16px; font-size:14px; color:#374151; font-weight:600;",
                                    "{current_page()} / {total_pages}"
                                }

                                button {
                                    disabled: current_page() >= total_pages,
                                    style: "padding:8px 16px; border-radius:8px; border:1px solid #d1d5db; background:white; color:#6b7280; font-weight:600; cursor:pointer; disabled:opacity-50;",
                                    onclick: move |_| {
                                        if current_page() < total_pages {
                                            current_page.set(current_page() + 1);
                                        }
                                    },
                                    "下一页 →"
                                }
                            }
                        }
                    }
                } else if !selected_task_name().is_empty() {
                    section {
                        style: "background:white; border-radius:20px; padding:60px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08); text-align:center;",
                        div {
                            style: "font-size:48px; margin-bottom:16px;",
                            "📭"
                        }
                        div {
                            style: "font-size:18px; font-weight:600; color:#374151; margin-bottom:8px;",
                            "暂无日志数据"
                        }
                        div {
                            style: "font-size:14px; color:#6b7280;",
                            "当前筛选条件下没有找到日志记录"
                        }
                    }
                } else {
                    section {
                        style: "background:white; border-radius:20px; padding:60px 32px; border:1px solid #e5e7eb; box-shadow:0 8px 30px rgba(15,23,42,0.08); text-align:center;",
                        div {
                            style: "font-size:48px; margin-bottom:16px;",
                            "👆"
                        }
                        div {
                            style: "font-size:18px; font-weight:600; color:#374151; margin-bottom:8px;",
                            "请选择任务名称"
                        }
                        div {
                            style: "font-size:14px; color:#6b7280;",
                            "任务名称是必填项，请先选择一个任务名称开始查询"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LogTableRow(log: LogEntry) -> Element {
    let (level_color, level_bg, level_icon) = match log.log_level.as_str() {
        "DEBUG" => ("#3b82f6", "#dbeafe", "🔵"),
        "INFO" => ("#6b7280", "#f3f4f6", "⚪"),
        "WARN" => ("#f59e0b", "#fef3c7", "🟡"),
        "ERROR" => ("#ef4444", "#fee2e2", "🔴"),
        _ => ("#6b7280", "#f3f4f6", "⚪"),
    };

    let uuid_short = log.task_uuid.chars().take(8).collect::<String>();
    
    // 截断日志内容，超过 100 个字符显示省略号
    let max_length = 100;
    let is_long = log.message.len() > max_length;
    let display_message = if is_long {
        format!("{}...", log.message.chars().take(max_length).collect::<String>())
    } else {
        log.message.clone()
    };

    let mut show_tooltip = use_signal(|| false);
    let mut copy_feedback = use_signal(|| false);
    
    // 生成唯一 ID
    let button_id = format!("copy-btn-{}", log.id.unwrap_or(0));

    rsx! {
        tr {
            style: "border-bottom:1px solid #f3f4f6; transition:background 0.15s ease;",
            
            // 第一列：任务 ID
            td {
                style: "padding:12px 16px; font-size:12px; color:#6b7280; font-family:monospace;",
                title: "{log.task_uuid}",
                "{uuid_short}..."
            }
            
            // 第二列：级别
            td {
                style: "padding:12px 16px;",
                span {
                    style: "display:inline-block; padding:4px 12px; border-radius:6px; font-size:12px; font-weight:600; color:{level_color}; background:{level_bg};",
                    "{level_icon} {log.log_level}"
                }
            }
            
            // 第三列：时间
            td {
                style: "padding:12px 16px; font-size:13px; color:#6b7280; white-space:nowrap;",
                "{log.timestamp}"
            }
            
            // 第四列：日志内容（带气泡）
            td {
                style: "padding:12px 16px; font-size:13px; color:#111827; line-height:1.5; position:relative;",
                
                div {
                    style: if is_long { "cursor:pointer;" } else { "" },
                    onmouseenter: move |_| if is_long { show_tooltip.set(true); },
                    onmouseleave: move |_| show_tooltip.set(false),
                    
                    "{display_message}"
                    
                    // 气泡提示（显示完整内容）
                    if show_tooltip() && is_long {
                        div {
                            style: "position:absolute; left:0; top:100%; margin-top:8px; background:linear-gradient(135deg,#1e293b,#0f172a); color:#f1f5f9; padding:16px; border-radius:12px; font-size:13px; line-height:1.6; box-shadow:0 10px 40px rgba(0,0,0,0.5); border:1px solid rgba(148,163,184,0.2); z-index:1000; max-width:500px; word-break:break-word; white-space:pre-wrap;",
                            onclick: move |evt| evt.stop_propagation(),
                            
                            // 完整日志内容
                            div {
                                style: "margin-bottom:12px; max-height:300px; overflow-y:auto; padding-right:8px;",
                                "{log.message}"
                            }
                            
                            // 小三角箭头
                            div {
                                style: "position:absolute; bottom:100%; left:20px; width:0; height:0; border-left:8px solid transparent; border-right:8px solid transparent; border-bottom:8px solid #1e293b;",
                            }
                        }
                    }
                }
            }
            
            // 第五列：复制按钮
            td {
                style: "padding:12px 16px; text-align:center;",
                button {
                    id: "{button_id}",
                    class: "copy-log-btn",
                    "data-copy-text": "{log.message}",
                    style: "padding:6px 12px; border-radius:6px; border:none; background:linear-gradient(120deg,#4f46e5,#7c3aed); color:white; font-weight:600; cursor:pointer; font-size:12px; transition:all 0.2s ease; white-space:nowrap;",
                    onclick: move |_| {
                        copy_feedback.set(true);
                        
                        // 2秒后隐藏反馈
                        spawn(async move {
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            copy_feedback.set(false);
                        });
                    },
                    if copy_feedback() {
                        "✓ 已复制"
                    } else {
                        "📋 复制"
                    }
                }
            }
        }
    }
}
