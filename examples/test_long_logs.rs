/// 测试长日志内容
/// 用于测试日志截断、气泡提示和复制功能

use fact_bot::common::http_task::Logger;
use uuid::Uuid;

fn main() {
    println!("=== 生成长日志测试数据 ===\n");

    // 初始化数据库
    let _ = fact_bot::db::initialize_databases();

    // 创建包含长日志的任务
    let task_id = Uuid::new_v4().to_string();
    let logger = Logger::new("长日志测试".to_string(), task_id.clone());

    // 短日志
    logger.info("这是一条正常长度的日志");

    // 中等长度日志
    logger.warn("这是一条稍微长一点的日志，包含更多的信息，用于测试日志显示的效果，看看是否会被截断");

    // 长日志 - JSON 格式
    logger.error(r#"API 请求失败: {"error": "Internal Server Error", "message": "数据库连接超时，无法完成查询操作", "details": {"host": "db.example.com", "port": 5432, "database": "production", "timeout": 30000, "retry_count": 3}, "timestamp": "2024-12-01T12:00:00Z", "request_id": "req-12345678-abcd-efgh-ijkl-mnopqrstuvwx"}"#);

    // 超长日志 - 堆栈跟踪
    logger.error(
        "系统异常: java.lang.NullPointerException: Cannot invoke method 'getData()' on null object reference\n\
        at com.example.service.DataService.processData(DataService.java:145)\n\
        at com.example.controller.ApiController.handleRequest(ApiController.java:89)\n\
        at com.example.filter.AuthFilter.doFilter(AuthFilter.java:67)\n\
        at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:119)\n\
        at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:336)\n\
        at org.springframework.security.web.authentication.AbstractAuthenticationProcessingFilter.doFilter(AbstractAuthenticationProcessingFilter.java:218)\n\
        Caused by: java.sql.SQLException: Connection timeout after 30000ms\n\
        at com.mysql.cj.jdbc.ConnectionImpl.connectOneTryOnly(ConnectionImpl.java:956)\n\
        at com.mysql.cj.jdbc.ConnectionImpl.createNewIO(ConnectionImpl.java:826)"
    );

    // 长日志 - SQL 查询
    logger.debug(
        "执行 SQL 查询: SELECT u.id, u.username, u.email, u.created_at, u.updated_at, p.profile_image, p.bio, p.location, \
        r.role_name, r.permissions FROM users u LEFT JOIN profiles p ON u.id = p.user_id LEFT JOIN user_roles ur ON u.id = ur.user_id \
        LEFT JOIN roles r ON ur.role_id = r.id WHERE u.status = 'active' AND u.deleted_at IS NULL AND u.created_at >= '2024-01-01' \
        ORDER BY u.created_at DESC LIMIT 100 OFFSET 0"
    );

    // 长日志 - HTTP 请求详情
    logger.info(
        "HTTP 请求详情: POST https://api.example.com/v1/users/create | Headers: {Content-Type: application/json, \
        Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ, \
        User-Agent: FactBot/1.0.0, Accept: application/json, Accept-Encoding: gzip, deflate} | Body: {\"username\": \"testuser\", \
        \"email\": \"test@example.com\", \"password\": \"********\", \"profile\": {\"firstName\": \"Test\", \"lastName\": \"User\", \
        \"age\": 25, \"location\": \"Beijing, China\"}} | Response: 201 Created | Duration: 1234ms"
    );

    // 长日志 - 配置信息
    logger.debug(
        "加载配置: {\"database\": {\"host\": \"localhost\", \"port\": 5432, \"name\": \"factbot\", \"user\": \"admin\", \
        \"password\": \"********\", \"pool_size\": 10, \"timeout\": 30000}, \"redis\": {\"host\": \"localhost\", \"port\": 6379, \
        \"db\": 0, \"password\": \"********\"}, \"server\": {\"host\": \"0.0.0.0\", \"port\": 8080, \"workers\": 4, \
        \"max_connections\": 1000}, \"logging\": {\"level\": \"debug\", \"format\": \"json\", \"output\": \"stdout\"}, \
        \"features\": {\"enable_cache\": true, \"enable_metrics\": true, \"enable_tracing\": true}}"
    );

    println!("✓ 已生成 7 条测试日志（包含不同长度）");
    println!("\n测试说明：");
    println!("1. 短日志：正常显示");
    println!("2. 中等长度：会被截断，显示 '...'");
    println!("3. 长日志：鼠标悬停显示完整内容");
    println!("4. 气泡中有复制按钮，点击可复制完整日志");
    println!("\n现在可以启动应用查看效果！");
}
