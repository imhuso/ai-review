use anyhow::Result;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use teloxide::prelude::*;
use tokio::sync::Mutex;

use super::core::{handle_callback_query, handle_text_message, TelegramCore, TelegramEvent};

/// Telegram集成管理器
pub struct TelegramIntegration {
    core: TelegramCore,
    app_handle: AppHandle,
    /// 当前选中的选项
    selected_options: Arc<Mutex<Vec<String>>>,
    /// 用户输入文本
    user_input: Arc<Mutex<String>>,
    /// 操作消息ID，用于过滤后续消息
    operation_message_id: Arc<Mutex<Option<i32>>>,
    /// 停止信号发送器
    stop_sender: Option<tokio::sync::oneshot::Sender<()>>,
}

impl TelegramIntegration {
    /// 创建新的Telegram集成实例
    pub fn new(bot_token: String, chat_id: String, app_handle: AppHandle) -> Result<Self> {
        let core = TelegramCore::new(bot_token, chat_id)?;

        Ok(Self {
            core,
            app_handle,
            selected_options: Arc::new(Mutex::new(Vec::new())),
            user_input: Arc::new(Mutex::new(String::new())),
            operation_message_id: Arc::new(Mutex::new(None)),
            stop_sender: None,
        })
    }

    /// 发送MCP请求消息到Telegram
    pub async fn send_mcp_request(
        &mut self,
        message: &str,
        predefined_options: Vec<String>,
        is_markdown: bool,
        continue_reply_enabled: bool,
    ) -> Result<()> {
        // 初始化选中选项状态
        {
            let mut selected = self.selected_options.lock().await;
            selected.clear();
        }

        // 发送选项消息
        self.core
            .send_options_message(message, &predefined_options, is_markdown)
            .await?;

        // 发送操作消息
        let op_msg_id = self
            .core
            .send_operation_message(continue_reply_enabled)
            .await?;

        // 保存操作消息ID
        {
            let mut op_id = self.operation_message_id.lock().await;
            *op_id = Some(op_msg_id);
        }

        // 启动消息监听
        self.start_message_listener().await?;

        Ok(())
    }

    /// 启动消息监听
    async fn start_message_listener(&mut self) -> Result<()> {
        let bot = self.core.bot.clone();
        let chat_id = self.core.chat_id;
        let app_handle = self.app_handle.clone();
        let selected_options = self.selected_options.clone();
        let user_input = self.user_input.clone();
        let operation_message_id = self.operation_message_id.clone();

        let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();
        self.stop_sender = Some(stop_tx);

        // 启动监听任务
        tokio::spawn(async move {
            let mut offset = 0i32;

            loop {
                tokio::select! {
                    _ = &mut stop_rx => {
                        break;
                    }
                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(1000)) => {
                        // 轮询获取更新
                        match bot.get_updates().offset(offset).await {
                            Ok(updates) => {
                                for update in updates {
                                    offset = update.id.0 as i32 + 1;

                                    // 处理不同类型的更新
                                    match update.kind {
                                        teloxide::types::UpdateKind::CallbackQuery(callback_query) => {
                                            // 使用核心模块的处理函数
                                            match handle_callback_query(
                                                &bot,
                                                &callback_query,
                                                chat_id,
                                            ).await {
                                                Ok(Some(option)) => {

                                                    // 切换选项状态
                                                    let selected = {
                                                        let mut selected_opts = selected_options.lock().await;
                                                        if selected_opts.contains(&option) {
                                                            selected_opts.retain(|x| x != &option);
                                                            false
                                                        } else {
                                                            selected_opts.push(option.clone());
                                                            true
                                                        }
                                                    };

                                                    // 发送更新后的事件到前端
                                                    let event = TelegramEvent::OptionToggled {
                                                        option: option.clone(),
                                                        selected,
                                                    };

                                                    if let Err(e) = app_handle.emit("telegram-event", &event) {
                                                        eprintln!("Telegram事件发送失败: {}", e);
                                                    }
                                                }
                                                Ok(None) => {
                                                    // CallbackQuery 被过滤或忽略
                                                }
                                                Err(e) => {
                                                    eprintln!("CallbackQuery处理失败: {}", e);
                                                }
                                            }
                                        }
                                        teloxide::types::UpdateKind::Message(message) => {
                                            // 获取操作消息ID
                                            let op_msg_id = {
                                                let op_id = operation_message_id.lock().await;
                                                *op_id
                                            };

                                            // 使用核心模块的处理函数
                                            match handle_text_message(
                                                &message,
                                                chat_id,
                                                op_msg_id,
                                            ).await {
                                                Ok(Some(event)) => {
                                                    // 如果是文本更新，保存到用户输入
                                                    if let TelegramEvent::TextUpdated { text } = &event {
                                                        let mut input = user_input.lock().await;
                                                        *input = text.clone();
                                                    }

                                                    // 发送事件到前端
                                                    if let Err(e) = app_handle.emit("telegram-event", &event) {
                                                        eprintln!("Telegram文本事件发送失败: {}", e);
                                                    }
                                                }
                                                Ok(None) => {
                                                    // 文本消息被过滤或忽略
                                                }
                                                Err(e) => {
                                                    eprintln!("文本消息处理失败: {}", e);
                                                }
                                            }
                                        }
                                        _ => {
                                            // 忽略其他类型的更新
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Telegram获取更新失败: {}", e);
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// 获取当前选中的选项
    pub async fn get_selected_options(&self) -> Vec<String> {
        let selected = self.selected_options.lock().await;
        selected.clone()
    }

    /// 获取用户输入的文本
    pub async fn get_user_input(&self) -> String {
        let input = self.user_input.lock().await;
        input.clone()
    }

    /// 停止Telegram集成
    pub async fn stop(&mut self) {
        if let Some(sender) = self.stop_sender.take() {
            let _ = sender.send(());
        }
    }
}
