# Gemini API 代理管理系统

这是一个基于 Tauri + Vue 3 开发的 Gemini API 代理管理系统，提供 API 密钥管理、请求转发和负载均衡功能。

## 功能特性

### 🔐 用户认证系统
- 用户注册和登录
- 密码哈希存储 (bcrypt)
- 本地会话管理

### 🔑 API 密钥管理
- 添加、编辑、删除 Gemini API 密钥
- 批量管理多个 API 密钥
- 密钥启用/禁用切换
- 使用统计和监控

### 🌐 代理服务
- HTTP 代理服务器 (端口 5675)
- 自动密钥轮换和负载均衡
- 支持流式和非流式响应
- 失败密钥自动禁用

### 📊 请求日志
- 详细的请求日志记录
- 响应时间统计
- 状态码监控
- 使用情况分析

## 技术栈

### 前端
- **Vue 3** - 响应式前端框架
- **Vue Router** - 路由管理
- **Pinia** - 状态管理
- **Vite** - 构建工具

### 后端
- **Rust** - 高性能后端语言
- **Tauri** - 跨平台桌面应用框架
- **Axum** - 现代 HTTP 框架
- **SQLite** - 轻量级数据库
- **SQLx** - 异步数据库访问

## 快速开始

### 1. 环境准备
```bash
# 确保已安装 Node.js 和 Rust
node --version
rustc --version

# 安装 Tauri CLI
npm install --global @tauri-apps/cli
```

### 2. 安装依赖
```bash
npm install
```

### 3. 开发模式运行
```bash
npm run tauri dev
```

### 4. 构建应用
```bash
npm run tauri build
```

## 使用方法

### 启动应用
1. 运行 `npm run tauri dev` 启动开发服务器
2. 应用程序会自动打开桌面窗口
3. 后台会启动 HTTP 代理服务器 (http://127.0.0.1:5675)

### 用户管理
1. 首次使用需要注册账户
2. 注册后会自动登录到管理界面
3. 后续使用直接登录即可

### API 密钥管理
1. 在 "API 密钥" 页面添加你的 Gemini API 密钥
2. 可以为每个密钥设置友好的名称
3. 通过开关控制密钥的启用/禁用状态
4. 查看每个密钥的使用统计

### 使用代理服务
代理服务器会自动启动在 `http://127.0.0.1:5675`，支持以下端点：

```bash
# 获取模型列表
GET http://127.0.0.1:5675/v1/models

# 获取特定模型信息 (推荐使用 gemini-1.5-flash 或 gemini-pro)
GET http://127.0.0.1:5675/v1/models/gemini-1.5-flash

# 生成内容 (非流式)
POST http://127.0.0.1:5675/v1/models/gemini-1.5-flash/generateContent

# 生成内容 (流式)
POST http://127.0.0.1:5675/v1/models/gemini-1.5-flash/streamGenerateContent

# 健康检查
GET http://127.0.0.1:5675/health
```

**重要提醒**：
- 系统会自动将 `/v1/` 路径转换为 `/v1beta/` 以符合 Gemini API 规范
- API 密钥会自动作为查询参数添加，无需手动处理
- 流式请求会自动添加 `alt=sse` 参数

### 请求示例
```bash
# 使用 curl 测试 (注意正确的请求格式)
curl -X POST http://127.0.0.1:5675/v1/models/gemini-1.5-flash/generateContent \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [
      {
        "role": "user",
        "parts": [
          {
            "text": "Hello, how are you?"
          }
        ]
      }
    ]
  }'

# 流式请求示例
curl -X POST http://127.0.0.1:5675/v1/models/gemini-1.5-flash/streamGenerateContent \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [
      {
        "role": "user",
        "parts": [
          {
            "text": "Write a short story"
          }
        ]
      }
    ]
  }'

# 测试代理服务
python test_api.py
```

## 项目结构

```
src/
├── components/          # Vue 组件
├── views/              # 页面组件
├── stores/             # Pinia 状态管理
├── router/             # Vue Router 配置
└── main.js            # 前端入口

src-tauri/
├── src/
│   ├── models/         # 数据模型
│   ├── services/       # 业务逻辑
│   ├── database/       # 数据库操作
│   ├── server/         # HTTP 服务器
│   ├── commands/       # Tauri 命令
│   └── lib.rs         # 应用入口
└── Cargo.toml         # Rust 依赖配置
```

## 密钥轮换策略

系统使用智能的密钥轮换策略：
1. 优先使用使用次数最少的密钥
2. 在使用次数相同时，优先使用最久未使用的密钥
3. 当密钥返回 401/403 错误时，自动禁用该密钥
4. 支持手动启用/禁用密钥

## 数据存储

- 数据库文件存储在系统临时目录
- 包含用户信息、API 密钥和请求日志
- 密码使用 bcrypt 哈希存储
- 支持数据备份和恢复

## 安全特性

- 密码哈希存储，不保存明文密码
- API 密钥加密存储
- 本地数据库，数据不会上传到服务器
- 请求日志记录，便于审计

## 故障排除

### 应用启动失败
1. 检查端口 5675 是否被占用
2. 确认 Rust 和 Node.js 环境正常
3. 检查数据库权限

### 代理服务不工作
1. 确认至少有一个活跃的 API 密钥
2. 检查 Gemini API 密钥是否有效
3. 查看请求日志排查问题

### 前端显示异常
1. 清除浏览器缓存
2. 重新安装依赖: `npm install`
3. 检查控制台错误信息

## 开发说明

### 添加新功能
1. 在 `src-tauri/src/models/` 定义数据模型
2. 在 `src-tauri/src/services/` 实现业务逻辑
3. 在 `src-tauri/src/commands/` 添加 Tauri 命令
4. 在前端创建相应的 Vue 组件

### 数据库迁移
修改 `src-tauri/src/database/migrations.rs` 文件来更新数据库结构。

### 调试技巧
- 使用 `tracing::info!()` 添加日志
- 在开发模式下查看控制台输出
- 使用浏览器开发工具调试前端

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！