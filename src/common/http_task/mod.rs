mod config;
mod cookie_store;
mod interceptor;
mod ip_manager;
mod logger;
mod task_manager;
mod types;

pub use config::{Emulation, IPMode, ProxyConfig, TaskConfig};
pub use cookie_store::{Cookie, CookieStore};
pub use interceptor::Interceptor;
pub use ip_manager::IPManager;
pub use logger::{LogEntry, LogLevel as LoggerLevel, Logger};
pub use task_manager::TaskManager;
// 重新导出全局错误类型，保持向后兼容
pub use crate::common::{AppError, Result};
pub type TaskError = AppError;
