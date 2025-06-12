# AI Review

[English](./README_EN.md) | 简体中文

AI Review 是一个基于 MCP (Model Context Protocol) 的智能代码审查工具，提供弹窗交互和记忆管理功能。

## ✨ 特性

- 🎯 **MCP 标准兼容**: 完全符合 MCP 2024-11-05 协议规范
- 🎨 **美观的弹窗界面**: 使用 Vue 3 + Tailwind CSS 构建的现代化UI
- 🔒 **安全的 Markdown 渲染**: 支持代码高亮和安全的内容渲染
- 🖥️ **原生应用体验**: 基于 Tauri 的跨平台桌面应用
- ⚡ **高效通信**: 使用进程间通信，响应迅速
- 🛠️ **灵活交互**: 支持预定义选项、多选和自由文本输入
- 🧠 **全局记忆管理**: 智能存储和管理开发规范、用户偏好和最佳实践
- 📷 **图片支持**: 支持图片上传和Base64处理

## 📸 功能演示

### 智能弹窗界面

![AI Review 弹窗演示](./screenshots/demo.png)

*AI Review 的现代化弹窗界面，支持 Markdown 渲染、代码高亮、预定义选项和自由文本输入*

## 🚀 快速开始

### macOS 安装方式

```bash
# 1. 安装前端依赖
pnpm install

# 2. 构建项目
cargo build --release

# 3. 全局安装（推荐，需要管理员权限）
./install.sh

# 4. 验证安装
which ai-review-mcp
```

### Windows 安装方式（未测试）

```powershell
# 1. 安装前端依赖
pnpm install

# 2. 构建项目
cargo build --release

# 3. 手动复制可执行文件到系统PATH
# 将 target\release\ai-review-mcp.exe 复制到 C:\Windows\System32\ 或其他PATH目录

# 4. 验证安装
where ai-review-mcp
```

### 本地开发

```bash
# 1. 安装依赖
pnpm install

# 2. 开发模式运行前端
pnpm dev

# 3. 构建并运行 MCP 服务器
cargo build --release
./target/release/ai-review-mcp  # macOS/Linux
# 或
.\target\release\ai-review-mcp.exe  # Windows
```

## 📋 工作流程

### 系统架构

```
AI IDE/Claude Desktop
        ↓ (MCP 调用)
AI Review MCP 服务器
        ↓ (启动弹窗)
Tauri 应用 (Vue + Tailwind CSS)
        ↓ (用户交互)
响应返回给 MCP 服务器
```

### 通信方式

- **MCP 协议**: JSON-RPC 2.0 over stdio
- **弹窗启动**: 直接调用 Tauri 应用
- **用户界面**: 原生 Tauri 应用 (Vue + Tailwind CSS)
- **响应传递**: 进程间通信，实时响应

## 🛠️ MCP 工具

### 1. ai_review_chat - 智能交互工具

弹窗交互工具，支持多种输入方式和Markdown渲染。

**主要参数**：
- `message` (必需): 显示给用户的消息内容
- `predefined_options` (可选): 预定义选项列表，支持多选
- `is_markdown` (可选): 是否启用Markdown格式渲染

**功能特性**：
- ✅ 预定义选项多选 + 自由文本输入
- ✅ 图片上传和Base64处理
- ✅ Markdown渲染和代码高亮
- ✅ 现代化暗黑主题UI

### 2. memory_manager - 全局记忆管理工具

智能记忆管理系统，按项目存储开发规范和偏好设置。

**主要参数**：
- `action` (必需): 操作类型 - `add` 或 `get_project_info`
- `project_path` (必需): 项目路径（git根目录）
- `content` (add时必需): 记忆内容
- `category` (可选): 分类 - `rule`/`preference`/`pattern`/`context`

**功能特性**：
- ✅ 按项目隔离存储记忆
- ✅ 智能分类管理（规范/偏好/模式/上下文）
- ✅ 自动识别"请记住："并添加记忆

## 🔧 配置

### Claude Desktop 配置

#### 全局安装后（推荐）

在 `~/.config/claude-desktop/claude_desktop_config.json` 中添加：

```json
{
  "mcpServers": {
    "ai-review": {
      "command": "ai-review-mcp",
      "args": []
    }
  }
}
```

#### 本地运行

```json
{
  "mcpServers": {
    "ai-review": {
      "command": "/path/to/ai-review/target/release/ai-review-mcp",
      "args": []
    }
  }
}
```

### Cursor 配置

在 Cursor 的 MCP 设置中添加：

```json
{
  "name": "ai-review",
  "command": "ai-review-mcp"  // macOS/Linux
  // Windows: "command": "ai-review-mcp.exe"
}
```

### Windows 配置说明（未测试）

Windows 用户需要：
1. 确保 `ai-review-mcp.exe` 在系统 PATH 中
2. 在配置文件中使用 `.exe` 扩展名
3. 可能需要使用完整路径：`"C:\\path\\to\\ai-review-mcp.exe"`

## 📁 项目结构

```
ai-review/
├── src/
│   ├── main.rs              # Tauri 主应用
│   ├── mcp_server.rs        # MCP 服务器实现
│   ├── memory.rs            # 记忆管理模块
│   ├── components/
│   │   └── McpPopup.vue     # MCP 弹窗组件
│   ├── utils/
│   │   └── message.js       # 消息处理工具
│   ├── App.vue              # 主应用组件
│   ├── main.js              # 前端入口
│   └── style.css            # 样式文件
├── scripts/
│   └── postinstall.sh       # 安装后脚本
├── icons/
│   └── icon.png             # 应用图标
├── install.sh               # 全局安装脚本
├── Cargo.toml               # Rust 依赖
├── package.json             # 前端依赖
├── tauri.conf.json          # Tauri 配置
└── README.md                # 项目文档
```

## 🧪 测试

### 功能测试

```bash
# 启动 MCP 服务器进行测试
./target/release/ai-review-mcp

# 在另一个终端测试 MCP 通信
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | ./target/release/ai-review-mcp
```

### 弹窗测试

通过 Claude Desktop 或其他 MCP 客户端调用 `ai_review_chat` 工具进行测试。

## 🔍 故障排除

### 常见问题

1. **弹窗无法启动**
   - 检查应用是否正确安装到 `/Applications/AI Review.app`
   - 确认命令行工具链接是否正确：`which ai-review-mcp`
   - 重新运行安装脚本：`./install.sh`

2. **MCP 服务器无响应**
   - 检查可执行文件是否存在：`./target/release/ai-review-mcp`
   - 运行 `cargo build --release` 重新构建
   - 查看错误日志：`RUST_LOG=debug ai-review-mcp`

3. **记忆管理问题**
   - 确保在 git 仓库根目录中使用
   - 检查 `~/.ai-review/` 目录权限

### 调试模式

```bash
# 启用详细日志
RUST_LOG=debug ai-review-mcp

# 查看记忆文件
ls -la ~/.ai-review/
```

## 🛠️ 开发

### 添加新功能

1. **修改 MCP 服务器**: 编辑 `src/mcp_server.rs`
2. **更新弹窗界面**: 编辑 `src/components/McpPopup.vue`
3. **测试更改**: 运行 `cargo build --release` 重新编译

### 构建发布版本

```bash
# 构建优化版本
cargo build --release

# 构建完整应用包
./install.sh --build-only
```

## 🤖 AI 生成说明

**重要提示**: 本项目的所有代码均由 AI 生成，非人工手动编写。包括但不限于：
- Rust 后端代码 (MCP 服务器、记忆管理)
- Vue 前端代码 (弹窗界面、组件)
- 配置文件 (Tauri、Vite、ESLint 等)
- 文档内容 (README、注释)

这是一个展示 AI 代码生成能力的项目，代码质量和最佳实践均由 AI 自主完成。

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 支持

如有问题，请创建 GitHub Issue 或联系维护者。
