/// Emulation 浏览器指纹模拟演示
/// 
/// 展示如何配置不同的浏览器指纹进行 HTTP 请求

use fact_bot::common::http_task::{Emulation, IPMode, ProxyConfig, TaskConfig, TaskManager};
use std::time::Duration;

fn main() {
    println!("=== Emulation 浏览器指纹模拟演示 ===\n");

    // 示例 1: 使用 Chrome 137（默认）
    println!("1. Chrome 137 配置:");
    let config1 = TaskConfig {
        emulation: Emulation::Chrome137,
        timeout: Duration::from_secs(30),
        enable_cookie_management: true,
        allow_redirect: true,
        verify_cert: true,
        ..Default::default()
    };
    
    let tm1 = TaskManager::new("Chrome137任务".to_string(), config1).unwrap();
    println!("   {}", tm1.build_client_info());
    println!("   Emulation: {}\n", Emulation::Chrome137.as_str());

    // 示例 2: 使用 Safari 18
    println!("2. Safari 18 配置:");
    let config2 = TaskConfig {
        emulation: Emulation::Safari18,
        timeout: Duration::from_secs(30),
        enable_cookie_management: true,
        ..Default::default()
    };
    
    let tm2 = TaskManager::new("Safari18任务".to_string(), config2).unwrap();
    println!("   {}", tm2.build_client_info());
    println!("   Emulation: {}\n", Emulation::Safari18.as_str());

    // 示例 3: 使用 Firefox 139
    println!("3. Firefox 139 配置:");
    let config3 = TaskConfig {
        emulation: Emulation::Firefox139,
        timeout: Duration::from_secs(30),
        enable_cookie_management: false,
        allow_redirect: false,
        ..Default::default()
    };
    
    let tm3 = TaskManager::new("Firefox139任务".to_string(), config3).unwrap();
    println!("   {}", tm3.build_client_info());
    println!("   Emulation: {}\n", Emulation::Firefox139.as_str());

    // 示例 4: 使用 Edge 134 + 代理
    println!("4. Edge 134 + 代理配置:");
    let proxy = ProxyConfig {
        host: "127.0.0.1".to_string(),
        port: 7890,
        username: Some("user".to_string()),
        password: Some("pass".to_string()),
    };
    
    let config4 = TaskConfig {
        emulation: Emulation::Edge134,
        ip_mode: IPMode::Fixed(proxy),
        timeout: Duration::from_secs(60),
        enable_cookie_management: true,
        verify_cert: false,
        ..Default::default()
    };
    
    let tm4 = TaskManager::new("Edge134任务".to_string(), config4).unwrap();
    println!("   {}", tm4.build_client_info());
    println!("   Emulation: {}", Emulation::Edge134.as_str());
    println!("   Proxy: {}\n", tm4.get_ip().unwrap().http());

    // 示例 5: 使用 OkHttp 5（移动端）
    println!("5. OkHttp 5 配置:");
    let config5 = TaskConfig {
        emulation: Emulation::OkHttp5,
        timeout: Duration::from_secs(30),
        ..Default::default()
    };
    
    let tm5 = TaskManager::new("OkHttp5任务".to_string(), config5).unwrap();
    println!("   {}", tm5.build_client_info());
    println!("   Emulation: {}\n", Emulation::OkHttp5.as_str());

    // 所有可用的 Emulation 类型
    println!("\n=== 所有可用的 Emulation 类型 ===\n");
    
    println!("Chrome 系列:");
    let chrome_versions = vec![
        Emulation::Chrome100, Emulation::Chrome101, Emulation::Chrome104,
        Emulation::Chrome105, Emulation::Chrome106, Emulation::Chrome107,
        Emulation::Chrome108, Emulation::Chrome109, Emulation::Chrome110,
        Emulation::Chrome114, Emulation::Chrome116, Emulation::Chrome117,
        Emulation::Chrome118, Emulation::Chrome119, Emulation::Chrome120,
        Emulation::Chrome123, Emulation::Chrome124, Emulation::Chrome126,
        Emulation::Chrome127, Emulation::Chrome128, Emulation::Chrome129,
        Emulation::Chrome130, Emulation::Chrome131, Emulation::Chrome132,
        Emulation::Chrome133, Emulation::Chrome134, Emulation::Chrome135,
        Emulation::Chrome136, Emulation::Chrome137,
    ];
    for (i, e) in chrome_versions.iter().enumerate() {
        print!("  {}", e.as_str());
        if (i + 1) % 4 == 0 {
            println!();
        }
    }
    println!("\n");

    println!("Safari 系列:");
    let safari_versions = vec![
        Emulation::Safari15_3, Emulation::Safari15_5, Emulation::Safari15_6_1,
        Emulation::Safari16, Emulation::Safari16_5, Emulation::Safari17_0,
        Emulation::Safari17_2_1, Emulation::Safari17_4_1, Emulation::Safari17_5,
        Emulation::Safari18, Emulation::Safari18_2, Emulation::Safari18_3,
        Emulation::Safari18_3_1, Emulation::Safari18_5, Emulation::SafariIPad18,
        Emulation::SafariIos16_5, Emulation::SafariIos17_2, Emulation::SafariIos17_4_1,
        Emulation::SafariIos18_1_1,
    ];
    for (i, e) in safari_versions.iter().enumerate() {
        print!("  {}", e.as_str());
        if (i + 1) % 3 == 0 {
            println!();
        }
    }
    println!("\n");

    println!("Firefox 系列:");
    let firefox_versions = vec![
        Emulation::Firefox109, Emulation::Firefox117, Emulation::Firefox128,
        Emulation::Firefox133, Emulation::Firefox135, Emulation::FirefoxPrivate135,
        Emulation::FirefoxAndroid135, Emulation::Firefox136, Emulation::FirefoxPrivate136,
        Emulation::Firefox139,
    ];
    for e in firefox_versions.iter() {
        println!("  {}", e.as_str());
    }
    println!();

    println!("Edge 系列:");
    let edge_versions = vec![
        Emulation::Edge101, Emulation::Edge122, Emulation::Edge127,
        Emulation::Edge131, Emulation::Edge134,
    ];
    for e in edge_versions.iter() {
        println!("  {}", e.as_str());
    }
    println!();

    println!("Opera 系列:");
    let opera_versions = vec![
        Emulation::Opera116, Emulation::Opera117, Emulation::Opera118,
        Emulation::Opera119,
    ];
    for e in opera_versions.iter() {
        println!("  {}", e.as_str());
    }
    println!();

    println!("OkHttp 系列（移动端）:");
    let okhttp_versions = vec![
        Emulation::OkHttp3_9, Emulation::OkHttp3_11, Emulation::OkHttp3_13,
        Emulation::OkHttp3_14, Emulation::OkHttp4_9, Emulation::OkHttp4_10,
        Emulation::OkHttp4_12, Emulation::OkHttp5,
    ];
    for e in okhttp_versions.iter() {
        println!("  {}", e.as_str());
    }

    println!("\n=== 演示完成 ===");
    println!("\n使用说明：");
    println!("1. 选择合适的 Emulation 类型模拟不同浏览器");
    println!("2. 配置代理、超时、Cookie 等选项");
    println!("3. 使用 TaskManager 执行 HTTP 请求");
    println!("4. 实际使用时需要集成 wreq::Client");
}
