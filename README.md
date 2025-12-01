# FactBot - 数据运维助手

<div align="center">

![FactBot Logo](assets/favicon.svg)

**强大的数据运维助手，提供航司报价查询、配置管理和日志查看等功能**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.7-blue.svg)](https://dioxuslabs.com/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[功能特性](#功能特性) • [快速开始](#快速开始) • [文档](#文档) • [开发](#开发)

</div>

## 功能特性

### 🎯 核心功能

- **航司报价查询** - 支持 16 家主流航司实时报价
- **配置管理** - 统一管理代理、OTP 邮箱、支付卡片和购票人信息
- **日志查看** - 强大的日志查询和管理系统
- **浏览器指纹** - 支持 75+ 种浏览器指纹模拟

### 📋 日志系统

- ✅ 多条件筛选（任务名称、UUID、级别、关键词）
- ✅ 时间排序（最新/最旧在前）
- ✅ 分页显示（20/50/100/200 条）
- ✅ 日志截断 + 气泡提示
- ✅ 一键复制完整日志
- ✅ 环境区分（dev / 正式版）

### ⚙️ 配置管理

- ✅ 分组管理
- ✅ 批量导入
- ✅ 数据持久化
- ✅ 密码保护

### 🌐 HTTP 任务

- ✅ TLS 指纹模拟
- ✅ 浏览器特征模拟
- ✅ 代理支持（固定/池）
- ✅ Cookie 管理
- ✅ 请求拦截

## 快速开始

### 用户

1. **下载应用**
   ```bash
   # 从 Releases 页面下载对应平台的安装包
   ```

2. **启动应用**
   - 双击运行程序
   - 首次启动自动初始化数据库

3. **查看文档**
   - [用户使用手册](USER_GUIDE.md)

### 开发者

1. **环境准备**
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # 安装 Dioxus CLI
   curl -sSL http://dioxus.dev/install.sh | sh
   ```

2. **克隆项目**
   ```bash
   git clone <repository>
   cd FactBot
   ```

3. **运行开发环境**
   ```bash
   # 使用 dx（推荐）
   dx serve
   
   # 或使用 cargo
   cargo run
   ```

4. **查看文档**
   - [开发者文档](DEVELOPER.md)

## 文档

- 📖 [用户使用手册](USER_GUIDE.md) - 完整的用户指南
- 🔧 [开发者文档](DEVELOPER.md) - 技术文档和 API 说明
- 💾 [数据库文档](docs/DATABASE.md) - 数据库结构和 API
- 🎭 [浏览器指纹](docs/Emulation浏览器指纹配置.md) - Emulation 配置说明

## 项目结构

```
FactBot/
├── src/
│   ├── main.rs              # 应用入口
│   ├── db.rs                # 数据库操作
│   ├── common/              # 公共模块
│   │   └── http_task/       # HTTP 任务系统
│   ├── components/          # UI 组件
│   └── views/               # 页面视图
├── examples/                # 示例代码
├── docs/                    # 文档
├── assets/                  # 静态资源
├── DEVELOPER.md             # 开发者文档
├── USER_GUIDE.md            # 用户手册
└── README.md                # 本文件
```

## 开发

### 运行示例

```bash
# 数据库演示
cargo run --example log_database_demo

# 生成测试日志
cargo run --example generate_test_logs

# 浏览器指纹演示
cargo run --example emulation_demo
```

### 构建发布

```bash
# dev 模式（显示 DEBUG 日志）
cargo build --release

# 正式版（隐藏 DEBUG 日志）
cargo build --release --no-default-features --features desktop
```



## 技术栈

- **前端框架**: [Dioxus 0.7](https://dioxuslabs.com/)
- **桌面应用**: dioxus-desktop
- **数据库**: SQLite (rusqlite)
- **异步运行时**: Tokio
- **序列化**: Serde
- **日志**: 自定义 Logger

## 环境区分

### dev 模式（开发）

```bash
cargo run
```

- ✅ 显示所有日志（包括 DEBUG）
- ✅ 完整的调试信息

### 正式版（生产）

```bash
cargo run --no-default-features --features desktop
```

- ❌ 隐藏 DEBUG 日志（但仍入库）
- ✅ 界面简洁，性能更好

## 数据存储

### 配置数据库

- **位置**: 用户数据目录
- **文件**: `config.db`
- **内容**: 应用配置、用户设置
- **安全**: 密码保护

### 日志数据库

- **位置**: 软件运行目录
- **文件**: `logs.db`
- **内容**: 任务执行日志
- **安全**: 无密码（方便查看）

## 贡献

欢迎贡献代码！请查看 [开发者文档](DEVELOPER.md) 了解详情。

## 许可证

[根据项目实际情况填写]

## 作者

- **zilong** - liuzilong326@163.com

## 致谢

- [Dioxus](https://dioxuslabs.com/) - 优秀的 Rust UI 框架
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite 绑定
- 所有贡献者

---

<div align="center">

**[⬆ 回到顶部](#factbot---数据运维助手)**

Made with ❤️ by zilong

</div>
