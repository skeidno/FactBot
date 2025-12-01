use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ProxyConfig {
    /// 生成代理 URL（格式：http://host:port）
    pub fn http(&self) -> String {
        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            format!("http://{}:{}@{}:{}", username, password, self.host, self.port)
        } else {
            format!("http://{}:{}", self.host, self.port)
        }
    }
}

#[derive(Debug, Clone)]
pub enum IPMode {
    Fixed(ProxyConfig),
    Pool(Vec<ProxyConfig>),
    None,
}

/// 浏览器指纹模拟类型
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum Emulation {
    // Chrome 系列
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome105,
    Chrome106,
    Chrome107,
    Chrome108,
    Chrome109,
    Chrome110,
    Chrome114,
    Chrome116,
    Chrome117,
    Chrome118,
    Chrome119,
    Chrome120,
    Chrome123,
    Chrome124,
    Chrome126,
    Chrome127,
    Chrome128,
    Chrome129,
    Chrome130,
    Chrome131,
    Chrome132,
    Chrome133,
    Chrome134,
    Chrome135,
    Chrome136,
    #[default]
    Chrome137,
    
    // Safari 系列
    SafariIos17_2,
    SafariIos17_4_1,
    SafariIos16_5,
    Safari15_3,
    Safari15_5,
    Safari15_6_1,
    Safari16,
    Safari16_5,
    Safari17_0,
    Safari17_2_1,
    Safari17_4_1,
    Safari17_5,
    Safari18,
    SafariIPad18,
    Safari18_2,
    SafariIos18_1_1,
    Safari18_3,
    Safari18_3_1,
    Safari18_5,
    
    // OkHttp 系列
    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp4_12,
    OkHttp5,
    
    // Edge 系列
    Edge101,
    Edge122,
    Edge127,
    Edge131,
    Edge134,
    
    // Firefox 系列
    Firefox109,
    Firefox117,
    Firefox128,
    Firefox133,
    Firefox135,
    FirefoxPrivate135,
    FirefoxAndroid135,
    Firefox136,
    FirefoxPrivate136,
    Firefox139,
    
    // Opera 系列
    Opera116,
    Opera117,
    Opera118,
    Opera119,
}


impl Emulation {
    /// 转换为字符串标识符
    pub fn as_str(&self) -> &'static str {
        match self {
            // Chrome
            Emulation::Chrome100 => "chrome_100",
            Emulation::Chrome101 => "chrome_101",
            Emulation::Chrome104 => "chrome_104",
            Emulation::Chrome105 => "chrome_105",
            Emulation::Chrome106 => "chrome_106",
            Emulation::Chrome107 => "chrome_107",
            Emulation::Chrome108 => "chrome_108",
            Emulation::Chrome109 => "chrome_109",
            Emulation::Chrome110 => "chrome_110",
            Emulation::Chrome114 => "chrome_114",
            Emulation::Chrome116 => "chrome_116",
            Emulation::Chrome117 => "chrome_117",
            Emulation::Chrome118 => "chrome_118",
            Emulation::Chrome119 => "chrome_119",
            Emulation::Chrome120 => "chrome_120",
            Emulation::Chrome123 => "chrome_123",
            Emulation::Chrome124 => "chrome_124",
            Emulation::Chrome126 => "chrome_126",
            Emulation::Chrome127 => "chrome_127",
            Emulation::Chrome128 => "chrome_128",
            Emulation::Chrome129 => "chrome_129",
            Emulation::Chrome130 => "chrome_130",
            Emulation::Chrome131 => "chrome_131",
            Emulation::Chrome132 => "chrome_132",
            Emulation::Chrome133 => "chrome_133",
            Emulation::Chrome134 => "chrome_134",
            Emulation::Chrome135 => "chrome_135",
            Emulation::Chrome136 => "chrome_136",
            Emulation::Chrome137 => "chrome_137",
            
            // Safari
            Emulation::SafariIos17_2 => "safari_ios_17.2",
            Emulation::SafariIos17_4_1 => "safari_ios_17.4.1",
            Emulation::SafariIos16_5 => "safari_ios_16.5",
            Emulation::Safari15_3 => "safari_15.3",
            Emulation::Safari15_5 => "safari_15.5",
            Emulation::Safari15_6_1 => "safari_15.6.1",
            Emulation::Safari16 => "safari_16",
            Emulation::Safari16_5 => "safari_16.5",
            Emulation::Safari17_0 => "safari_17.0",
            Emulation::Safari17_2_1 => "safari_17.2.1",
            Emulation::Safari17_4_1 => "safari_17.4.1",
            Emulation::Safari17_5 => "safari_17.5",
            Emulation::Safari18 => "safari_18",
            Emulation::SafariIPad18 => "safari_ipad_18",
            Emulation::Safari18_2 => "safari_18.2",
            Emulation::SafariIos18_1_1 => "safari_ios_18.1.1",
            Emulation::Safari18_3 => "safari_18.3",
            Emulation::Safari18_3_1 => "safari_18.3.1",
            Emulation::Safari18_5 => "safari_18.5",
            
            // OkHttp
            Emulation::OkHttp3_9 => "okhttp_3.9",
            Emulation::OkHttp3_11 => "okhttp_3.11",
            Emulation::OkHttp3_13 => "okhttp_3.13",
            Emulation::OkHttp3_14 => "okhttp_3.14",
            Emulation::OkHttp4_9 => "okhttp_4.9",
            Emulation::OkHttp4_10 => "okhttp_4.10",
            Emulation::OkHttp4_12 => "okhttp_4.12",
            Emulation::OkHttp5 => "okhttp_5",
            
            // Edge
            Emulation::Edge101 => "edge_101",
            Emulation::Edge122 => "edge_122",
            Emulation::Edge127 => "edge_127",
            Emulation::Edge131 => "edge_131",
            Emulation::Edge134 => "edge_134",
            
            // Firefox
            Emulation::Firefox109 => "firefox_109",
            Emulation::Firefox117 => "firefox_117",
            Emulation::Firefox128 => "firefox_128",
            Emulation::Firefox133 => "firefox_133",
            Emulation::Firefox135 => "firefox_135",
            Emulation::FirefoxPrivate135 => "firefox_private_135",
            Emulation::FirefoxAndroid135 => "firefox_android_135",
            Emulation::Firefox136 => "firefox_136",
            Emulation::FirefoxPrivate136 => "firefox_private_136",
            Emulation::Firefox139 => "firefox_139",
            
            // Opera
            Emulation::Opera116 => "opera_116",
            Emulation::Opera117 => "opera_117",
            Emulation::Opera118 => "opera_118",
            Emulation::Opera119 => "opera_119",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskConfig {
    pub intercept_domains: Vec<String>,
    pub intercept_keywords: Vec<String>,
    pub ip_mode: IPMode,
    pub emulation: Emulation,
    pub timeout: Duration,
    pub enable_cookie_management: bool,
    pub allow_redirect: bool,
    pub verify_cert: bool,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            intercept_domains: Vec::new(),
            intercept_keywords: Vec::new(),
            ip_mode: IPMode::None,
            emulation: Emulation::default(),
            timeout: Duration::from_secs(30),
            enable_cookie_management: false,
            allow_redirect: true,
            verify_cert: true,
        }
    }
}
