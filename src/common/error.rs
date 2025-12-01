use thiserror::Error;

/// 全局错误类型，包含所有模块的错误
#[derive(Debug, Error)]
pub enum AppError {
    // HTTP Task 相关错误
    #[error("Request blocked by interceptor: {0}")]
    Blocked(String),

    #[error("No proxy available in IP pool")]
    NoProxyAvailable,

    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Cookie parse error: {0}")]
    CookieParseError(String),

    // 数据库相关错误
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Config not found: {0}")]
    ConfigNotFound(String),

    // 日志相关错误
    #[error("Log initialization failed: {0}")]
    LogInitError(String),

    #[error("Log write failed: {0}")]
    LogWriteError(String),

    #[error("Invalid log level: {0}")]
    InvalidLogLevel(String),

    // 通用错误
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// 全局 Result 类型别名
/// 使用方式：Result<T> 而不是 Result<T, AppError>
pub type Result<T> = std::result::Result<T, AppError>;
