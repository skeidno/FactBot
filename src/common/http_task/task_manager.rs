use crate::common::{AppError, Result};
use crate::common::http_task::{
    CookieStore, IPManager, Interceptor, Logger, ProxyConfig, TaskConfig,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct TaskManager {
    task_name: String,
    task_id: String,
    config: TaskConfig,
    interceptor: Interceptor,
    ip_manager: IPManager,
    cookie_store: Option<CookieStore>,
    logger: Logger,
}

impl TaskManager {
    /// 创建新的任务管理器
    /// 
    /// # 参数
    /// - `task_name`: 任务名称，用于日志标识
    /// - `config`: 任务配置
    pub fn new(task_name: String, config: TaskConfig) -> Result<TaskManager> {
        let task_id = Uuid::new_v4().to_string();
        
        let interceptor = Interceptor::new(
            config.intercept_domains.clone(),
            config.intercept_keywords.clone(),
        );
        
        let ip_manager = IPManager::new(config.ip_mode.clone())?;
        
        let cookie_store = if config.enable_cookie_management {
            Some(CookieStore::new())
        } else {
            None
        };
        
        let logger = Logger::new(task_name.clone(), task_id.clone());
        
        logger.info(&format!("任务管理器初始化完成，ID: {}", task_id));
        
        Ok(TaskManager {
            task_name,
            task_id,
            config,
            interceptor,
            ip_manager,
            cookie_store,
            logger,
        })
    }
    
    /// 获取任务名称
    pub fn task_name(&self) -> &str {
        &self.task_name
    }
    
    pub fn task_id(&self) -> &str {
        &self.task_id
    }
    
    pub fn get_ip(&self) -> Option<&ProxyConfig> {
        self.ip_manager.get_current()
    }
    
    pub fn switch_next(&mut self) -> Result<()> {
        self.ip_manager.switch_next()
    }
    
    /// 获取日志记录器的引用
    pub fn logger(&self) -> &Logger {
        &self.logger
    }
    
    /// 构建 HTTP 客户端（带 TLS 指纹和浏览器模拟）
    /// 
    /// 这是一个占位符实现，实际使用时需要集成 wreq::Client
    /// 
    /// # 示例实现（需要 wreq crate）
    /// 
    /// ```ignore
    /// use wreq::{Client, Proxy, Emulation};
    /// use std::time::Duration;
    /// 
    /// pub fn build_client(&self) -> Result<Client> {
    ///     let timeout = self.config.timeout;
    ///     let mut builder = Client::builder();
    ///     
    ///     // 配置代理
    ///     if let Some(proxy) = self.get_ip() {
    ///         builder = builder.proxy(Proxy::all(proxy.http())?);
    ///     }
    ///     
    ///     // 配置基本选项
    ///     builder = builder
    ///         .timeout(timeout)
    ///         .cookie_store(self.config.enable_cookie_management)
    ///         .emulation(self.config.emulation.into());
    ///     
    ///     // 配置重定向
    ///     builder = builder.redirect(if self.config.allow_redirect {
    ///         wreq::redirect::Policy::default()
    ///     } else {
    ///         wreq::redirect::Policy::none()
    ///     });
    ///     
    ///     // 配置证书验证
    ///     #[cfg(any(env = "product", env = "beta"))]
    ///     {
    ///         if !self.config.verify_cert {
    ///             builder = builder.cert_verification(false);
    ///         }
    ///     }
    ///     
    ///     #[cfg(not(any(env = "product", env = "beta")))]
    ///     {
    ///         builder = builder.cert_verification(false);
    ///     }
    ///     
    ///     let client = builder.build()?;
    ///     Ok(client)
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn build_client_info(&self) -> String {
        format!(
            "Client Config: emulation={}, timeout={}s, proxy={}, cookies={}, redirect={}, verify_cert={}",
            self.config.emulation.as_str(),
            self.config.timeout.as_secs(),
            if self.get_ip().is_some() { "enabled" } else { "disabled" },
            if self.config.enable_cookie_management { "enabled" } else { "disabled" },
            if self.config.allow_redirect { "enabled" } else { "disabled" },
            if self.config.verify_cert { "enabled" } else { "disabled" }
        )
    }
    
    /// 记录调试日志
    pub fn debug(&self, message: &str) {
        self.logger.debug(message);
    }
    
    /// 记录信息日志
    pub fn info(&self, message: &str) {
        self.logger.info(message);
    }
    
    /// 记录警告日志
    pub fn warn(&self, message: &str) {
        self.logger.warn(message);
    }
    
    /// 记录错误日志
    pub fn error(&self, message: &str) {
        self.logger.error(message);
    }
    
    fn check_interceptor(&self, url: &str) -> Result<()> {
        if self.interceptor.should_block(url) {
            self.logger.warn(&format!("Request blocked by interceptor: {}", url));
            return Err(AppError::Blocked(url.to_string()));
        }
        
        if !self.interceptor.is_empty() {
            self.logger.debug(&format!("Request passed interceptor check: {}", url));
        }
        
        Ok(())
    }
    
    fn log_request_details(&self, url: &str, method: &str) {
        if let Some(proxy) = self.get_ip() {
            self.logger.debug(&format!(
                "Request: {} {} | Proxy: {}:{}",
                method, url, proxy.host, proxy.port
            ));
        } else {
            self.logger.debug(&format!("Request: {} {} | No proxy", method, url));
        }
    }
    
    // 注意：这里使用占位符实现，实际应该使用 wreq::Client
    // 您需要根据实际的 wreq::Client API 来实现这些方法
    pub async fn get(&mut self, url: &str) -> Result<MockResponse> {
        self.execute_request("GET", url, None).await
    }
    
    pub async fn post_json(
        &mut self,
        url: &str,
        body: serde_json::Value,
    ) -> Result<MockResponse> {
        self.execute_request("POST", url, Some(body.to_string())).await
    }
    
    pub async fn post_form(
        &mut self,
        url: &str,
        form: HashMap<String, String>,
    ) -> Result<MockResponse> {
        let body = form
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        self.execute_request("POST", url, Some(body)).await
    }
    
    pub async fn fetch_text(&mut self, url: &str) -> Result<String> {
        let response = self.get(url).await?;
        Ok(response.body)
    }
    
    pub async fn fetch_json(&mut self, url: &str) -> Result<serde_json::Value> {
        let text = self.fetch_text(url).await?;
        Ok(serde_json::from_str(&text)?)
    }
    
    pub async fn fetch_bytes(&mut self, url: &str) -> Result<Vec<u8>> {
        let response = self.get(url).await?;
        Ok(response.body.into_bytes())
    }
    
    async fn execute_request(
        &mut self,
        method: &str,
        url: &str,
        body: Option<String>,
    ) -> Result<MockResponse> {
        // 步骤 1: 拦截器检查
        self.check_interceptor(url)?;
        
        // 步骤 2: 获取代理
        let _proxy = self.get_ip();
        
        // 步骤 3: 配置指纹
        // 注意：实际的TLS和浏览器指纹配置需要在HTTP客户端实现中完成
        
        // 步骤 4: 插入 Cookie
        let cookie_header = self.cookie_store.as_ref().map(|store| store.get_header());
        
        // 步骤 5: 记录请求详情
        self.log_request_details(url, method);
        if let Some(ref b) = body {
            self.logger.debug(&format!("Request body: {}", b));
        }
        if let Some(ref cookies) = cookie_header {
            if !cookies.is_empty() {
                self.logger.debug(&format!("Cookies: {}", cookies));
            }
        }
        
        // 步骤 6: 发起请求（模拟实现）
        // 注意：实际项目中需要使用真实的HTTP客户端
        let response = MockResponse {
            status: 200,
            body: "Mock response".to_string(),
            headers: HashMap::new(),
        };
        
        // 步骤 7: 更新 Cookie
        if let Some(ref mut store) = self.cookie_store {
            if let Some(set_cookie) = response.headers.get("set-cookie") {
                store.update_from_set_cookie(set_cookie);
            }
        }
        
        // 步骤 8: 记录日志
        self.logger.info(&format!("请求完成: {} {}", method, url));
        
        Ok(response)
    }
}

// 占位符响应结构，实际应该使用 wreq::Response
#[derive(Debug)]
pub struct MockResponse {
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager_uuid_uniqueness() {
        let config1 = TaskConfig::default();
        let config2 = TaskConfig::default();
        
        let tm1 = TaskManager::new("任务1".to_string(), config1).unwrap();
        let tm2 = TaskManager::new("任务2".to_string(), config2).unwrap();
        
        assert_ne!(tm1.task_id(), tm2.task_id());
        assert_eq!(tm1.task_name(), "任务1");
        assert_eq!(tm2.task_name(), "任务2");
    }

    #[test]
    fn test_task_manager_config_applied() {
        let config = TaskConfig {
            intercept_domains: vec!["blocked.com".to_string()],
            enable_cookie_management: true,
            ..Default::default()
        };
        
        let tm = TaskManager::new("测试任务".to_string(), config).unwrap();
        
        assert!(tm.cookie_store.is_some());
        assert!(tm.check_interceptor("https://blocked.com/path").is_err());
        assert_eq!(tm.task_name(), "测试任务");
    }
}
