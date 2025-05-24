# AI Review - 智能代码审查助手

一个基于 Rust + Tauri 构建的跨平台聊天窗口应用，支持命令行与UI实时交互。

## ✨ 主要功能

- 🖥️ **跨平台桌面应用** - 基于 Tauri 框架，支持 Windows、macOS、Linux
- 📡 **命令行与UI通信** - 通过 IPC (进程间通信) 实现实时消息传递
- 🔔 **系统级通知** - 收到新消息时显示系统通知
- 🪟 **智能弹窗** - 自动创建置顶的快速回复窗口
- ⏱️ **可配置超时** - 支持自定义等待回复的最大时间
- ⌨️ **快捷键支持** - Ctrl/Cmd+Enter 发送，Escape 取消
- 🎯 **实时倒计时** - 显示剩余回复时间
- 🚀 **自动CLI安装** - macOS应用安装后自动设置全局CLI命令

### 🎨 Vue 3 界面新特性

- ✨ **现代化设计** - 渐变背景、卡片式布局、流畅动画
- 📱 **响应式界面** - 适配不同窗口大小和设备
- 🚨 **智能提醒** - 剩余时间不足时红色闪烁警告
- 📝 **字符计数** - 实时显示输入长度，超长提醒
- 🎮 **完整快捷键** - 全键盘操作支持
- 🔄 **状态管理** - 清晰的等待、处理、紧急状态显示

## 🚀 快速开始

### 方式一：使用预编译的macOS应用（推荐）

#### 1. 下载并安装

1. 从 [Releases](https://github.com/imhuso/ai-review/releases) 页面下载最新的 `AI Review_x.x.x_x64.dmg` 文件
2. 双击 DMG 文件，将 `AI Review.app` 拖拽到 `Applications` 文件夹
3. 首次启动应用时，CLI命令会自动安装到系统

#### 2. 使用CLI命令

安装完成后，您可以在任何终端中使用 `ai-review-cli` 命令：

```bash
# 获取帮助信息
ai-review-cli help

# 发送消息（默认30秒超时）
ai-review-cli "请帮我审查这段代码"

# 自定义超时时间
ai-review-cli "这是一个测试消息" --timeout 60

# 获取初始化提示
ai-review-cli init

# 查看版本
ai-review-cli --version
```

#### 3. CLI命令详细说明

```bash
ai-review-cli [MESSAGE] [OPTIONS]

参数:
  MESSAGE                要发送给UI应用的消息 (默认: 'init')

选项:
  -t, --timeout <SECONDS>  超时时间（秒），默认30秒
  -h, --help              显示帮助信息
  -V, --version           显示版本信息

特殊命令:
  help                    显示详细帮助信息
  init                    获取AI Review初始化提示

示例:
  ai-review-cli                          # 获取默认帮助信息
  ai-review-cli help                     # 显示详细帮助
  ai-review-cli init                     # 获取初始化提示
  ai-review-cli "你好"                    # 发送自定义消息
  ai-review-cli "分析代码" -t 60           # 发送消息并设置60秒超时
```

### 方式二：从源码编译

#### 环境要求

- Rust 1.70+
- Node.js 18+ (用于前端构建)
- pnpm (推荐) 或 npm
- 系统依赖：
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `webkit2gtk-4.0-dev`, `libappindicator3-dev`
  - **Windows**: 无需额外依赖

#### 编译步骤

```bash
# 1. 克隆项目
git clone https://github.com/imhuso/ai-review.git
cd ai-review

# 2. 安装前端依赖
pnpm install

# 3. 构建前端资源
pnpm run build

# 4. 编译Tauri应用
cargo tauri build
```

#### 安装编译后的应用

```bash
# 运行安装脚本（macOS）
./install.sh
```

安装脚本会：
- 将应用安装到 `/Applications/AI Review.app`
- 创建全局CLI命令 `ai-review-cli`
- 验证安装是否成功

## 📖 使用说明

### 基本工作流程

1. **启动应用**: 
   - 从启动台或应用程序文件夹打开 `AI Review`
   - 或者在终端运行 `open "/Applications/AI Review.app"`

2. **发送消息**: 
   ```bash
   ai-review-cli "请帮我审查这段代码"
   ```

3. **接收和回复**:
   - 应用会显示系统通知
   - 弹出置顶的快速回复窗口
   - 在文本框中输入回复
   - 按 `Cmd + Enter` 发送或 `Escape` 取消

4. **查看结果**:
   - 命令行会显示您的回复
   - 窗口自动关闭

### UI界面操作

1. **接收消息**: 当命令行发送消息时，会：
   - 显示系统通知
   - 弹出置顶的快速回复窗口
   - 显示消息内容和剩余时间

2. **回复消息**:
   - 在文本框中输入回复
   - 点击"发送回复"或按 `Cmd + Enter`
   - 窗口会自动关闭

3. **取消操作**:
   - 点击"取消"按钮或按 `Escape` 键
   - 命令行会收到取消消息

### 快捷键

- `Cmd + Enter` (macOS) / `Ctrl + Enter` (Windows/Linux): 发送回复
- `Escape`: 取消当前请求

### 配置管理

应用支持自定义初始化提示词：

1. 在UI界面中点击设置按钮
2. 修改初始化提示词
3. 保存后，使用 `ai-review-cli init` 可获取自定义提示

## 🔧 故障排除

### 常见问题

#### 1. CLI命令未找到

```bash
# 检查CLI是否已安装
which ai-review-cli

# 如果未找到，手动创建符号链接
sudo ln -s "/Applications/AI Review.app/Contents/MacOS/ai-review-cli" /usr/local/bin/ai-review-cli

# 确保 /usr/local/bin 在PATH中
echo $PATH
```

#### 2. 应用无法启动

```bash
# 检查应用是否存在
ls -la "/Applications/AI Review.app"

# 查看应用日志
Console.app # 搜索 "AI Review"
```

#### 3. IPC连接失败

```bash
# 确保UI应用正在运行
ps aux | grep ai-review

# 重启应用
killall "AI Review"
open "/Applications/AI Review.app"
```

#### 4. 权限问题

```bash
# 给予应用执行权限
chmod +x "/Applications/AI Review.app/Contents/MacOS/ai-review-ui"
chmod +x "/Applications/AI Review.app/Contents/MacOS/ai-review-cli"
```

### 卸载

```bash
# 删除应用
rm -rf "/Applications/AI Review.app"

# 删除CLI命令
sudo rm -f /usr/local/bin/ai-review-cli

# 删除配置文件
rm -rf ~/Library/Application\ Support/com.imhuso.ai-review
```

## 🏗️ 项目结构

```
ai-review/
├── src/
│   ├── main.rs              # UI应用主程序
│   ├── cli.rs               # 命令行工具
│   └── ipc.rs               # 进程间通信模块
├── src-tauri/               # Tauri配置
├── dist/                    # 前端构建输出
├── scripts/                 # 安装脚本
│   └── postinstall.sh       # 安装后脚本
├── target/release/bundle/   # 编译输出
│   ├── macos/              # macOS应用包
│   └── dmg/                # DMG安装包
├── Cargo.toml              # Rust项目配置
├── tauri.conf.json         # Tauri应用配置
├── package.json            # 前端项目配置
├── install.sh              # 安装脚本
└── README.md               # 项目说明
```

## 🧪 测试

### 功能测试

```bash
# 1. 启动应用
open "/Applications/AI Review.app"

# 2. 测试基本功能
ai-review-cli "测试消息"

# 3. 测试超时功能
ai-review-cli "长时间测试" --timeout 10

# 4. 测试初始化
ai-review-cli init

# 5. 测试帮助
ai-review-cli help
```

### 开发测试

```bash
# 开发模式运行
cargo tauri dev

# 运行测试
cargo test

# 检查代码格式
cargo fmt --check

# 代码检查
cargo clippy
```

## 🔧 技术实现

### 核心技术栈

- **后端**: Rust + Tauri 2.0
- **前端**: Vue 3 + Vite + Ant Design Vue
- **通信**: Unix Domain Socket (IPC)
- **通知**: notify-rust + Tauri 通知系统
- **构建**: Tauri Bundle + DMG

### 关键特性

1. **IPC通信**: 使用 Unix Domain Socket 实现命令行与UI的双向通信
2. **异步处理**: 基于 Tokio 的异步运行时
3. **窗口管理**: 动态创建和管理弹窗窗口
4. **状态管理**: 使用 Tauri 的状态管理系统
5. **超时控制**: 支持自定义超时时间和实时倒计时
6. **自动安装**: 应用启动时自动检查并安装CLI命令

### 安全特性

- 应用签名（可选）
- 沙盒环境支持
- 权限最小化原则
- 安全的IPC通信

## 📝 开发说明

### 添加新功能

1. **后端功能**: 在 `src/main.rs` 中添加新的 Tauri 命令
2. **前端功能**: 在前端组件中添加新的界面元素
3. **IPC扩展**: 在 `src/ipc.rs` 中扩展消息类型
4. **CLI扩展**: 在 `src/cli.rs` 中添加新的命令行选项

### 调试模式

```bash
# 启用详细日志
RUST_LOG=debug cargo tauri dev

# 启用 Rust 回溯
RUST_BACKTRACE=1 cargo tauri dev

# 前端开发服务器
pnpm run dev
```

### 发布流程

```bash
# 1. 更新版本号
# 编辑 Cargo.toml, package.json, tauri.conf.json

# 2. 构建发布版本
cargo tauri build

# 3. 测试安装包
./install.sh

# 4. 创建发布
# 上传 target/release/bundle/dmg/*.dmg 到 GitHub Releases
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🔗 相关链接

- [项目仓库](https://github.com/imhuso/ai-review)
- [问题反馈](https://github.com/imhuso/ai-review/issues)
- [Tauri 官方文档](https://tauri.app/)
- [Rust 官方网站](https://www.rust-lang.org/)
- [Vue 3 文档](https://vuejs.org/)

---

**注意**: 
- 确保在使用命令行工具之前先启动UI应用，否则会出现连接错误
- 首次安装后可能需要重启终端以使CLI命令生效
- 如遇到权限问题，请检查系统安全设置并允许应用运行
