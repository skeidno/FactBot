use crate::common::{AppError, Result};
use crate::common::http_task::{IPMode, ProxyConfig};

pub struct IPManager {
    mode: IPMode,
    current_index: usize,
}

impl IPManager {
    pub fn new(mode: IPMode) -> Result<Self> {
        // 验证 IP 池模式不为空
        if let IPMode::Pool(ref proxies) = mode {
            if proxies.is_empty() {
                return Err(AppError::NoProxyAvailable);
            }
        }

        Ok(Self {
            mode,
            current_index: 0,
        })
    }

    pub fn get_current(&self) -> Option<&ProxyConfig> {
        match &self.mode {
            IPMode::Fixed(proxy) => Some(proxy),
            IPMode::Pool(proxies) => proxies.get(self.current_index),
            IPMode::None => None,
        }
    }

    pub fn switch_next(&mut self) -> Result<()> {
        match &self.mode {
            IPMode::Pool(proxies) => {
                if proxies.is_empty() {
                    return Err(AppError::NoProxyAvailable);
                }
                self.current_index = (self.current_index + 1) % proxies.len();
                Ok(())
            }
            IPMode::Fixed(_) => Ok(()), // 固定 IP 模式不切换
            IPMode::None => Ok(()),      // 无代理模式不切换
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_proxy(host: &str) -> ProxyConfig {
        ProxyConfig {
            host: host.to_string(),
            port: 8080,
            username: None,
            password: None,
        }
    }

    #[test]
    fn test_fixed_ip_mode() {
        let proxy = create_proxy("proxy1.com");
        let manager = IPManager::new(IPMode::Fixed(proxy.clone())).unwrap();

        assert_eq!(manager.get_current().unwrap().host, "proxy1.com");
    }

    #[test]
    fn test_pool_mode_rotation() {
        let proxies = vec![
            create_proxy("proxy1.com"),
            create_proxy("proxy2.com"),
            create_proxy("proxy3.com"),
        ];
        let mut manager = IPManager::new(IPMode::Pool(proxies)).unwrap();

        assert_eq!(manager.get_current().unwrap().host, "proxy1.com");
        manager.switch_next().unwrap();
        assert_eq!(manager.get_current().unwrap().host, "proxy2.com");
        manager.switch_next().unwrap();
        assert_eq!(manager.get_current().unwrap().host, "proxy3.com");
        manager.switch_next().unwrap();
        assert_eq!(manager.get_current().unwrap().host, "proxy1.com"); // 循环回第一个
    }

    #[test]
    fn test_none_mode() {
        let manager = IPManager::new(IPMode::None).unwrap();
        assert!(manager.get_current().is_none());
    }

    #[test]
    fn test_empty_pool_error() {
        let result = IPManager::new(IPMode::Pool(vec![]));
        assert!(result.is_err());
    }
}
