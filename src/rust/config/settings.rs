use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_ui_config")]
    pub ui_config: UiConfig, // UI相关配置（主题、窗口、置顶等）
    #[serde(default = "default_audio_config")]
    pub audio_config: AudioConfig, // 音频相关配置
    #[serde(default = "default_reply_config")]
    pub reply_config: ReplyConfig, // 继续回复配置
    #[serde(default = "default_mcp_config")]
    pub mcp_config: McpConfig, // MCP工具配置
    #[serde(default = "default_telegram_config")]
    pub telegram_config: TelegramConfig, // Telegram Bot配置
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    // 主题设置
    #[serde(default = "default_theme")]
    pub theme: String, // "light", "dark", "system"

    // 窗口设置
    #[serde(default = "default_window_config")]
    pub window_config: WindowConfig,

    // 置顶设置
    #[serde(default = "default_always_on_top")]
    pub always_on_top: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    // 窗口约束设置
    #[serde(default = "default_auto_resize")]
    pub auto_resize: bool,
    #[serde(default = "default_max_width")]
    pub max_width: f64,
    #[serde(default = "default_max_height")]
    pub max_height: f64,
    #[serde(default = "default_min_width")]
    pub min_width: f64,
    #[serde(default = "default_min_height")]
    pub min_height: f64,

    // 当前模式
    #[serde(default = "default_window_fixed")]
    pub fixed: bool,

    // 固定模式的尺寸设置
    #[serde(default = "default_fixed_width")]
    pub fixed_width: f64,
    #[serde(default = "default_fixed_height")]
    pub fixed_height: f64,

    // 自由拉伸模式的尺寸设置
    #[serde(default = "default_free_width")]
    pub free_width: f64,
    #[serde(default = "default_free_height")]
    pub free_height: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    #[serde(default = "default_audio_notification_enabled")]
    pub notification_enabled: bool,
    #[serde(default = "default_audio_url")]
    pub custom_url: String, // 自定义音效URL
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplyConfig {
    #[serde(default = "default_enable_continue_reply")]
    pub enable_continue_reply: bool,
    #[serde(default = "default_auto_continue_threshold")]
    pub auto_continue_threshold: u32, // 字符数阈值
    #[serde(default = "default_continue_prompt")]
    pub continue_prompt: String, // 继续回复的提示词
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpConfig {
    #[serde(default = "default_mcp_tools")]
    pub tools: HashMap<String, bool>, // MCP工具启用状态
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramConfig {
    #[serde(default = "default_telegram_enabled")]
    pub enabled: bool, // 是否启用Telegram Bot
    #[serde(default = "default_telegram_bot_token")]
    pub bot_token: String, // Bot Token
    #[serde(default = "default_telegram_chat_id")]
    pub chat_id: String, // Chat ID
    #[serde(default = "default_telegram_hide_frontend_popup")]
    pub hide_frontend_popup: bool, // 是否隐藏前端弹窗，仅使用Telegram交互
}

#[derive(Debug)]
pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub response_channel: Mutex<Option<tokio::sync::oneshot::Sender<String>>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ui_config: default_ui_config(),
            audio_config: default_audio_config(),
            reply_config: default_reply_config(),
            mcp_config: default_mcp_config(),
            telegram_config: default_telegram_config(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Mutex::new(AppConfig::default()),
            response_channel: Mutex::new(None),
        }
    }
}

// 默认值函数
pub fn default_ui_config() -> UiConfig {
    UiConfig {
        theme: default_theme(),
        window_config: default_window_config(),
        always_on_top: default_always_on_top(),
    }
}

pub fn default_audio_config() -> AudioConfig {
    AudioConfig {
        notification_enabled: default_audio_notification_enabled(),
        custom_url: default_audio_url(),
    }
}

pub fn default_mcp_config() -> McpConfig {
    McpConfig {
        tools: default_mcp_tools(),
    }
}

pub fn default_telegram_config() -> TelegramConfig {
    TelegramConfig {
        enabled: default_telegram_enabled(),
        bot_token: default_telegram_bot_token(),
        chat_id: default_telegram_chat_id(),
        hide_frontend_popup: default_telegram_hide_frontend_popup(),
    }
}

pub fn default_always_on_top() -> bool {
    true // 默认启用置顶
}

pub fn default_audio_notification_enabled() -> bool {
    false // 默认关闭音频通知
}

pub fn default_theme() -> String {
    "dark".to_string() // 默认深色主题
}

pub fn default_audio_url() -> String {
    "".to_string() // 默认为空，使用内置音效
}

pub fn default_window_config() -> WindowConfig {
    WindowConfig {
        auto_resize: true,
        max_width: 1500.0,
        max_height: 1000.0,
        min_width: 600.0,
        min_height: 400.0,
        fixed: false,
        fixed_width: 600.0,
        fixed_height: 900.0,
        free_width: 600.0,
        free_height: 900.0,
    }
}

pub fn default_reply_config() -> ReplyConfig {
    ReplyConfig {
        enable_continue_reply: true,
        auto_continue_threshold: 1000,
        continue_prompt: "请按照最佳实践继续".to_string(),
    }
}

pub fn default_auto_resize() -> bool {
    true
}

pub fn default_max_width() -> f64 {
    1500.0
}

pub fn default_max_height() -> f64 {
    1000.0
}

pub fn default_min_width() -> f64 {
    600.0
}

pub fn default_min_height() -> f64 {
    400.0
}

pub fn default_enable_continue_reply() -> bool {
    true
}

pub fn default_auto_continue_threshold() -> u32 {
    1000
}

pub fn default_continue_prompt() -> String {
    "请按照最佳实践继续".to_string()
}

pub fn default_mcp_tools() -> HashMap<String, bool> {
    let mut tools = HashMap::new();
    tools.insert("zhi".to_string(), true); // 寸止工具默认启用
    tools.insert("ji".to_string(), true); // 记忆管理工具默认启用
    tools
}

pub fn default_window_width() -> f64 {
    600.0
}

pub fn default_window_height() -> f64 {
    900.0
}

pub fn default_window_fixed() -> bool {
    false
}

pub fn default_fixed_width() -> f64 {
    600.0
}

pub fn default_fixed_height() -> f64 {
    900.0
}

pub fn default_free_width() -> f64 {
    600.0
}

pub fn default_free_height() -> f64 {
    900.0
}

pub fn default_telegram_enabled() -> bool {
    false // 默认关闭Telegram Bot
}

pub fn default_telegram_bot_token() -> String {
    "".to_string() // 默认为空
}

pub fn default_telegram_chat_id() -> String {
    "".to_string() // 默认为空
}

pub fn default_telegram_hide_frontend_popup() -> bool {
    false // 默认不隐藏前端弹窗，保持现有行为
}

impl WindowConfig {
    // 获取当前模式的宽度
    pub fn current_width(&self) -> f64 {
        if self.fixed {
            self.fixed_width
        } else {
            self.free_width
        }
    }

    // 获取当前模式的高度
    pub fn current_height(&self) -> f64 {
        if self.fixed {
            self.fixed_height
        } else {
            self.free_height
        }
    }

    // 更新当前模式的尺寸
    pub fn update_current_size(&mut self, width: f64, height: f64) {
        if self.fixed {
            self.fixed_width = width;
            self.fixed_height = height;
        } else {
            self.free_width = width;
            self.free_height = height;
        }
    }
}
