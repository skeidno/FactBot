use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<SystemTime>,
    pub secure: bool,
    pub http_only: bool,
}

impl Cookie {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            domain: None,
            path: None,
            expires: None,
            secure: false,
            http_only: false,
        }
    }

    pub fn parse(cookie_str: &str) -> Option<Self> {
        let parts: Vec<&str> = cookie_str.split(';').collect();
        if parts.is_empty() {
            return None;
        }

        let name_value: Vec<&str> = parts[0].trim().splitn(2, '=').collect();
        if name_value.len() != 2 {
            return None;
        }

        let mut cookie = Cookie::new(
            name_value[0].trim().to_string(),
            name_value[1].trim().to_string(),
        );

        for part in parts.iter().skip(1) {
            let kv: Vec<&str> = part.trim().splitn(2, '=').collect();
            if kv.is_empty() {
                continue;
            }

            let key = kv[0].trim().to_lowercase();
            match key.as_str() {
                "domain" if kv.len() == 2 => cookie.domain = Some(kv[1].trim().to_string()),
                "path" if kv.len() == 2 => cookie.path = Some(kv[1].trim().to_string()),
                "secure" => cookie.secure = true,
                "httponly" => cookie.http_only = true,
                _ => {}
            }
        }

        Some(cookie)
    }
}

pub struct CookieStore {
    cookies: HashMap<String, Cookie>,
}

impl Default for CookieStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CookieStore {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }

    pub fn remove(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    pub fn get_header(&self) -> String {
        self.cookies
            .values()
            .map(|c| format!("{}={}", c.name, c.value))
            .collect::<Vec<_>>()
            .join("; ")
    }

    pub fn update_from_set_cookie(&mut self, set_cookie_header: &str) {
        if let Some(cookie) = Cookie::parse(set_cookie_header) {
            self.add(cookie);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cookie_parse() {
        let cookie_str = "session=abc123; Domain=example.com; Path=/; Secure; HttpOnly";
        let cookie = Cookie::parse(cookie_str).unwrap();

        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert_eq!(cookie.path, Some("/".to_string()));
        assert!(cookie.secure);
        assert!(cookie.http_only);
    }

    #[test]
    fn test_cookie_store_add_and_get() {
        let mut store = CookieStore::new();
        store.add(Cookie::new("key1".to_string(), "value1".to_string()));
        store.add(Cookie::new("key2".to_string(), "value2".to_string()));

        let header = store.get_header();
        assert!(header.contains("key1=value1"));
        assert!(header.contains("key2=value2"));
    }

    #[test]
    fn test_cookie_store_update() {
        let mut store = CookieStore::new();
        store.add(Cookie::new("session".to_string(), "old_value".to_string()));
        store.add(Cookie::new("session".to_string(), "new_value".to_string()));

        let header = store.get_header();
        assert_eq!(header, "session=new_value");
    }

    #[test]
    fn test_cookie_store_remove() {
        let mut store = CookieStore::new();
        store.add(Cookie::new("key1".to_string(), "value1".to_string()));
        store.remove("key1");

        let header = store.get_header();
        assert_eq!(header, "");
    }
}
