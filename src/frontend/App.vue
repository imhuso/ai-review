<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import { computed, onMounted, ref } from 'vue'
import AppContent from './components/AppContent.vue'
import { useSettings } from './composables/useSettings'
import { useTheme } from './composables/useTheme'

// 响应式数据
const mcpRequest = ref(null)
const showMcpPopup = ref(false)
const isInitializing = ref(true)

const { currentTheme, naiveTheme, setTheme, loadTheme, setupSystemThemeListener } = useTheme()
const settings = useSettings()

// 创建配置对象
const appConfig = computed(() => ({
  theme: currentTheme.value,
  window: {
    alwaysOnTop: settings.alwaysOnTop.value,
    width: settings.windowWidth.value,
    height: settings.windowHeight.value,
    fixed: settings.fixedWindowSize.value,
  },
  audio: {
    enabled: settings.audioNotificationEnabled.value,
    url: settings.audioUrl.value,
  },
  reply: {
    enabled: settings.continueReplyEnabled.value,
    prompt: settings.continuePrompt.value,
  },
}))

// 创建设置操作对象
const settingsActions = {
  toggleAlwaysOnTop: settings.toggleAlwaysOnTop,
  toggleAudioNotification: settings.toggleAudioNotification,
  updateAudioUrl: settings.updateAudioUrl,
  testAudio: settings.testAudioSound,
  stopAudio: settings.stopAudioSound,
  updateWindowSize: settings.updateWindowSize,
  updateReplyConfig: settings.updateReplyConfig,
}

// 统一的MCP响应处理
async function handleMcpResponse(response: any) {
  try {
    // 通过Tauri命令发送响应并退出应用
    await invoke('send_mcp_response', { response })
    await invoke('exit_app')
  }
  catch (error) {
    console.error('处理MCP响应失败:', error)
  }
}

// 统一的MCP取消处理
async function handleMcpCancel() {
  try {
    // 发送取消信息并退出应用
    await invoke('send_mcp_response', { response: 'CANCELLED' })
    await invoke('exit_app')
  }
  catch (error) {
    console.error('处理MCP取消失败:', error)
  }
}

// 显示MCP弹窗
async function showMcpDialog(request: any) {
  // 获取Telegram配置，检查是否需要隐藏前端弹窗
  let shouldShowFrontendPopup = true
  try {
    const telegramConfig = await invoke('get_telegram_config')
    // 如果Telegram启用且配置了隐藏前端弹窗，则不显示前端弹窗
    if (telegramConfig && (telegramConfig as any).enabled && (telegramConfig as any).hide_frontend_popup) {
      shouldShowFrontendPopup = false
      console.log('🔕 根据Telegram配置，隐藏前端弹窗')
    }
  }
  catch (error) {
    console.error('获取Telegram配置失败:', error)
    // 配置获取失败时，保持默认行为（显示弹窗）
  }

  // 根据配置决定是否显示前端弹窗
  if (shouldShowFrontendPopup) {
    // 设置请求数据和显示状态
    mcpRequest.value = request
    showMcpPopup.value = true
    console.log('✅ 显示前端弹窗')
  } else {
    console.log('🔕 跳过前端弹窗显示，仅使用Telegram交互')
  }

  // 播放音频通知（无论是否显示弹窗都播放）
  try {
    await invoke('play_notification_sound')
  }
  catch (error) {
    console.error('播放音频通知失败:', error)
  }

  // 启动Telegram同步（无论是否显示弹窗都启动）
  try {
    if (request?.message) {
      await invoke('start_telegram_sync', {
        message: request.message,
        predefinedOptions: request.predefined_options || [],
        isMarkdown: request.is_markdown || false
      })
      console.log('✅ Telegram同步启动成功')
    }
  }
  catch (error) {
    console.error('启动Telegram同步失败:', error)
  }
}

// 检查MCP模式
async function checkMcpMode() {
  try {
    const args = await invoke('get_cli_args')

    if (args && (args as any).mcp_request) {
      // 读取MCP请求文件
      const content = await invoke('read_mcp_request', { filePath: (args as any).mcp_request })

      if (content) {
        showMcpDialog(content)
      }
      return true // 表示是MCP模式
    }
  }
  catch (error) {
    console.error('检查MCP模式失败:', error)
  }
  return false // 表示不是MCP模式
}

// 设置MCP事件监听器
async function setupMcpEventListener() {
  try {
    await listen('mcp-request', (event) => {
      showMcpDialog(event.payload)
    })
  }
  catch (error) {
    console.error('设置MCP事件监听器失败:', error)
  }
}

// 注意：Telegram事件监听已移到McpPopup组件中，避免冲突

// 处理消息实例就绪
function handleMessageReady(message: any) {
  settings.setMessageInstance(message)
}

// 处理音频测试错误
function handleTestAudioError(error: any) {
  console.error('音频测试错误:', error)
  // 这里可以显示错误提示给用户
}

// 初始化
onMounted(async () => {
  // 首先加载主题设置
  await loadTheme()

  // 检查是否为MCP模式
  const isMcp = await checkMcpMode()

  // 无论是否为MCP模式，都加载窗口设置
  await settings.loadWindowSettings()
  await settings.loadWindowConfig()

  // 如果不是MCP模式，设置事件监听器
  if (!isMcp) {
    await setupMcpEventListener()
  }

  // 注意：Telegram事件监听已移到McpPopup组件中

  // 监听系统主题变化
  setupSystemThemeListener()

  // 结束初始化状态
  isInitializing.value = false
})
</script>

<template>
  <div class="min-h-screen bg-black">
    <n-config-provider :theme="naiveTheme">
      <n-message-provider>
        <n-notification-provider>
          <n-dialog-provider>
            <AppContent :mcp-request="mcpRequest" :show-mcp-popup="showMcpPopup" :app-config="appConfig"
              :is-initializing="isInitializing" @mcp-response="handleMcpResponse" @mcp-cancel="handleMcpCancel"
              @theme-change="setTheme" @toggle-always-on-top="settingsActions.toggleAlwaysOnTop"
              @toggle-audio-notification="settingsActions.toggleAudioNotification"
              @update-audio-url="settingsActions.updateAudioUrl" @test-audio="settingsActions.testAudio"
              @stop-audio="settingsActions.stopAudio" @test-audio-error="handleTestAudioError"
              @update-window-size="settingsActions.updateWindowSize"
              @update-reply-config="settingsActions.updateReplyConfig" @message-ready="handleMessageReady" />
          </n-dialog-provider>
        </n-notification-provider>
      </n-message-provider>
    </n-config-provider>
  </div>
</template>
