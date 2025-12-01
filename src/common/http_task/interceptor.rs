use url::Url;

#[derive(Debug, Clone)]
pub struct Interceptor {
    domains: Vec<String>,
    url_keywords: Vec<String>,
}

impl Interceptor {
    pub fn new(domains: Vec<String>, url_keywords: Vec<String>) -> Self {
        Self {
            domains,
            url_keywords,
        }
    }

    pub fn should_block(&self, url: &str) -> bool {
        // 检查域名匹配
        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                for blocked_domain in &self.domains {
                    if domain.contains(blocked_domain) || blocked_domain.contains(domain) {
                        return true;
                    }
                }
            }
        }

        // 检查 URL 关键词匹配
        for keyword in &self.url_keywords {
            if url.contains(keyword) {
                return true;
            }
        }

        false
    }

    pub fn is_empty(&self) -> bool {
        self.domains.is_empty() && self.url_keywords.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_blocking() {
        let interceptor = Interceptor::new(
            vec!["example.com".to_string()],
            vec![],
        );

        assert!(interceptor.should_block("https://example.com/path"));
        assert!(interceptor.should_block("https://sub.example.com/path"));
        assert!(!interceptor.should_block("https://other.com/path"));
    }

    #[test]
    fn test_keyword_blocking() {
        let interceptor = Interceptor::new(
            vec![],
            vec!["blocked".to_string()],
        );

        assert!(interceptor.should_block("https://example.com/blocked/path"));
        assert!(!interceptor.should_block("https://example.com/allowed/path"));
    }

    #[test]
    fn test_empty_interceptor() {
        let interceptor = Interceptor::new(vec![], vec![]);

        assert!(interceptor.is_empty());
        assert!(!interceptor.should_block("https://example.com/path"));
    }
}
