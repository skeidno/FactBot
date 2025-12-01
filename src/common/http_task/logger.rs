use chrono::Local;
use colored::*;
use std::sync::{Arc, Mutex};

/// 日志级别
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: String,
}

/// 日志记录器
/// 格式: [任务名称] [日志级别] [任务id] [时间] : 日志内容
/// 
/// 日志不会自动打印，而是存储在内存和数据库中，需要时才打印或获取
/// 颜色：INFO=白色, WARN=黄色, ERROR=红色, DEBUG=蓝色
pub struct Logger {
    task_name: String,
    task_id: String,        // 完整 UUID
    task_id_short: String,  // UUID 前8位（用于显示）
    logs: Arc<Mutex<Vec<LogEntry>>>,
}

impl Logger {
    /// 创建新的日志记录器
    /// 
    /// # 参数
    /// - `task_name`: 任务名称
    /// - `task_id`: 完整的任务 UUID
    pub fn new(task_name: String, task_id: String) -> Self {
        // 取 UUID 的前 8 位用于显示
        let task_id_short = task_id.chars().take(8).collect();
        
        Self {
            task_name,
            task_id: task_id.clone(),
            task_id_short,
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 格式化日志消息（带颜色）
    /// 格式: [任务名称] [日志级别] [任务id] [时间] : 日志内容
    fn format_message(&self, message: &str, timestamp: &str, level: &LogLevel) -> ColoredString {
        let level_str = match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };
        
        let formatted = format!(
            "[{}] [{}] [{}] [{}] : {}",
            self.task_name, level_str, self.task_id_short, timestamp, message
        );
        
        match level {
            LogLevel::Debug => formatted.blue(),
            LogLevel::Info => formatted.white(),
            LogLevel::Warn => formatted.yellow(),
            LogLevel::Error => formatted.red(),
        }
    }

    /// 添加日志条目（不打印，但保存到数据库）
    /// 
    /// 环境区分：
    /// - dev 模式：所有日志都保存到内存和数据库
    /// - 正式版：DEBUG 日志只入库，不保存到内存（UI 不展示）
    fn add_log(&self, level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // 正式版：DEBUG 日志不保存到内存（UI 不展示）
        #[cfg(not(feature = "dev"))]
        {
            if level != LogLevel::Debug {
                let entry = LogEntry {
                    level: level.clone(),
                    message: message.to_string(),
                    timestamp: timestamp.clone(),
                };
                
                if let Ok(mut logs) = self.logs.lock() {
                    logs.push(entry);
                }
            }
        }
        
        // dev 模式：所有日志都保存到内存
        #[cfg(feature = "dev")]
        {
            let entry = LogEntry {
                level: level.clone(),
                message: message.to_string(),
                timestamp: timestamp.clone(),
            };
            
            if let Ok(mut logs) = self.logs.lock() {
                logs.push(entry);
            }
        }
        
        // 所有日志都保存到数据库（包括 DEBUG）
        let task_name = self.task_name.clone();
        let task_id = self.task_id.clone();
        let level_str = match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };
        
        // 尝试保存到数据库，失败不影响程序运行
        let _ = crate::db::save_log(
            &task_name,
            level_str,
            &task_id,
            &timestamp,
            message,
        );
    }

    /// 调试日志 - 记录但不打印
    pub fn debug(&self, message: &str) {
        self.add_log(LogLevel::Debug, message);
    }

    /// 普通日志 - 记录但不打印
    pub fn info(&self, message: &str) {
        self.add_log(LogLevel::Info, message);
    }

    /// 警告日志 - 记录但不打印
    pub fn warn(&self, message: &str) {
        self.add_log(LogLevel::Warn, message);
    }

    /// 错误日志 - 记录但不打印
    pub fn error(&self, message: &str) {
        self.add_log(LogLevel::Error, message);
    }

    /// 获取所有日志
    pub fn get_logs(&self) -> Vec<LogEntry> {
        if let Ok(logs) = self.logs.lock() {
            logs.clone()
        } else {
            Vec::new()
        }
    }

    /// 获取指定级别的日志
    pub fn get_logs_by_level(&self, level: LogLevel) -> Vec<LogEntry> {
        if let Ok(logs) = self.logs.lock() {
            logs.iter()
                .filter(|entry| entry.level == level)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 打印所有日志到控制台（带颜色）
    /// 
    /// 环境区分：
    /// - dev 模式：打印所有日志（包括 DEBUG）
    /// - 正式版：不打印 DEBUG 日志
    pub fn print_all(&self) {
        if let Ok(logs) = self.logs.lock() {
            for entry in logs.iter() {
                // 正式版：跳过 DEBUG 日志
                #[cfg(not(feature = "dev"))]
                {
                    if entry.level == LogLevel::Debug {
                        continue;
                    }
                }
                
                let formatted = self.format_message(&entry.message, &entry.timestamp, &entry.level);
                match entry.level {
                    LogLevel::Error => eprintln!("{}", formatted),
                    _ => println!("{}", formatted),
                }
            }
        }
    }

    /// 打印指定级别的日志（带颜色）
    /// 
    /// 环境区分：
    /// - dev 模式：可以打印任何级别
    /// - 正式版：DEBUG 级别不打印
    pub fn print_level(&self, level: LogLevel) {
        // 正式版：禁止打印 DEBUG 日志
        #[cfg(not(feature = "dev"))]
        {
            if level == LogLevel::Debug {
                return;
            }
        }
        
        if let Ok(logs) = self.logs.lock() {
            for entry in logs.iter().filter(|e| e.level == level) {
                let formatted = self.format_message(&entry.message, &entry.timestamp, &entry.level);
                match entry.level {
                    LogLevel::Error => eprintln!("{}", formatted),
                    _ => println!("{}", formatted),
                }
            }
        }
    }

    /// 清空所有日志
    pub fn clear(&self) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.clear();
        }
    }

    /// 获取日志数量
    pub fn count(&self) -> usize {
        if let Ok(logs) = self.logs.lock() {
            logs.len()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = Logger::new(
            "测试任务".to_string(),
            "12345678-1234-1234-1234-123456789abc".to_string(),
        );

        assert_eq!(logger.task_name, "测试任务");
        assert_eq!(logger.task_id, "12345678-1234-1234-1234-123456789abc");
        assert_eq!(logger.task_id_short, "12345678");
    }

    #[test]
    fn test_logger_format() {
        let logger = Logger::new(
            "HTTP请求".to_string(),
            "abcd1234-5678-90ef-ghij-klmnopqrstuv".to_string(),
        );

        // 测试格式化（不实际打印）
        let timestamp = "2024-01-01 12:00:00";
        let message = logger.format_message("测试消息", timestamp, &LogLevel::Info);
        let message_str = message.to_string();
        assert!(message_str.contains("[HTTP请求]"));
        assert!(message_str.contains("[INFO]"));
        assert!(message_str.contains("[abcd1234]"));
        assert!(message_str.contains(": 测试消息"));
    }

    #[test]
    fn test_log_collection() {
        let logger = Logger::new(
            "完整测试".to_string(),
            "test1234-5678-90ab-cdef-ghijklmnopqr".to_string(),
        );

        // 添加不同级别的日志
        logger.info("信息日志");
        logger.debug("调试日志");
        logger.warn("警告日志");
        logger.error("错误日志");

        // 验证日志被收集
        assert_eq!(logger.count(), 4);

        // 验证可以获取所有日志
        let logs = logger.get_logs();
        assert_eq!(logs.len(), 4);

        // 验证可以按级别过滤
        let info_logs = logger.get_logs_by_level(LogLevel::Info);
        assert_eq!(info_logs.len(), 1);
        assert_eq!(info_logs[0].message, "信息日志");
    }

    #[test]
    fn test_log_clear() {
        let logger = Logger::new(
            "清理测试".to_string(),
            "clear123-5678-90ab-cdef-ghijklmnopqr".to_string(),
        );

        logger.info("测试1");
        logger.info("测试2");
        assert_eq!(logger.count(), 2);

        logger.clear();
        assert_eq!(logger.count(), 0);
    }
}
