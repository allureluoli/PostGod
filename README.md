🚀 PostGod - CTF HTTP 请求神器

一个基于 Rust + egui 构建的图形化 HTTP 请求工具，专为 CTF Web 题设计，支持自定义请求头、GET/POST 快速调试。
## ✨ 功能特性

- ✅ 支持 GET / POST 请求
- ✅ 自定义请求头（支持批量输入）
- ✅ 一键填充浏览器 Header
- ✅ 实时 URL 预览
- ✅ POST Body 编辑
- ✅ 响应内容可视化展示
- ✅ 中文字体支持（内置字体）
- ✅ 请求计数统计
- ✅ 错误提示清晰

---

## 🖥️ 界面说明

**左侧：请求构造区**
- URL + Param 拼接
- Headers 输入
- POST Body

**右侧：响应显示区**

---

## 📦 项目结构


📦 项目结构
PostGod/
├── src/
│   ├── main.rs          # 主程序（UI + 逻辑）
│   ├── request_send.rs  # 请求发送模块
│   └── chinese.rs       # 中文字体相关
├── assets/
│   └── AaManHuaJia-2.ttf  # 中文字体
├── Cargo.toml
└── README.md

代码核心基于 egui UI 框架构建：

🚀 快速开始
1️⃣ 克隆项目
git clone https://github.com/allureluoli/PostGod.git
cd PostGod
2️⃣ 运行项目
cargo run
⚙️ 使用方法
🔹 GET 请求
输入 URL
输入参数（如：?id=1）
填写 Headers（可选）
点击 EXECUTE GET
🔹 POST 请求
输入 URL
填写 POST Body
填写 Headers
点击 EXECUTE POST
🔹 Header 格式
Key: Value

示例：

User-Agent: Mozilla/5.0
Cookie: PHPSESSID=xxxx
🔹 一键浏览器头

点击：

填充浏览器头

自动生成常用请求头，适合绕 WAF / 模拟真实用户。

⚠️ 常见问题
❌ 中文不显示

已内置字体，如果仍异常，请检查：

assets 字体文件是否存在
路径是否正确
❌ 请求失败

检查：

URL 是否正确
代理是否开启
Header 格式是否正确（必须 Key: Value）
🧠 设计思路
使用 egui 构建跨平台 GUI
使用 reqwest 发送 HTTP 请求
自定义 Header 解析逻辑：
Key: Value

支持多行解析与错误提示

🔥 适用场景
CTF Web 做题
渗透测试调试请求
API 调试（轻量 GUI 版）
替代 curl / Postman 的简化工具
🛠️ 后续优化方向
 Cookie 管理
 请求历史记录
 多标签页
 自动编码（URL/Base64）
 Burp Repeater 风格增强
