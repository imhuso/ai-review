<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { useMessage } from 'naive-ui'
import { computed, ref, watch } from 'vue'
import { useKeyboard } from '../../composables/useKeyboard'

interface Props {
  request: McpRequest | null
  loading?: boolean
  submitting?: boolean
}

interface Emits {
  update: [data: {
    userInput: string
    selectedOptions: string[]
    draggedImages: string[]
  }]
  imageAdd: [image: string]
  imageRemove: [index: number]
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  submitting: false,
})

const emit = defineEmits<Emits>()

// 响应式数据
const userInput = ref('')
const selectedOptions = ref<string[]>([])
const uploadedImages = ref<string[]>([])
const textareaRef = ref<HTMLTextAreaElement | null>(null)

// 使用键盘快捷键 composable
const { pasteShortcut } = useKeyboard()

const message = useMessage()

// 计算属性
const hasOptions = computed(() => (props.request?.predefined_options?.length ?? 0) > 0)
const canSubmit = computed(() => {
  const hasOptionsSelected = selectedOptions.value.length > 0
  const hasInputText = userInput.value.trim().length > 0
  const hasImages = uploadedImages.value.length > 0

  if (hasOptions.value) {
    return hasOptionsSelected || hasInputText || hasImages
  }
  return hasInputText || hasImages
})

// 工具栏状态文本
const statusText = computed(() => {
  // 检查是否有任何输入内容
  const hasInput = selectedOptions.value.length > 0
    || uploadedImages.value.length > 0
    || userInput.value.trim().length > 0

  // 如果有任何输入内容，返回空字符串让 PopupActions 显示快捷键
  if (hasInput) {
    return ''
  }

  return '等待输入...'
})

// 发送更新事件
function emitUpdate() {
  emit('update', {
    userInput: userInput.value,
    selectedOptions: selectedOptions.value,
    draggedImages: uploadedImages.value,
  })
}

// 处理选项变化
function handleOptionChange(option: string, checked: boolean) {
  if (checked) {
    selectedOptions.value.push(option)
  }
  else {
    const idx = selectedOptions.value.indexOf(option)
    if (idx > -1)
      selectedOptions.value.splice(idx, 1)
  }
  emitUpdate()
}

// 处理选项切换（整行点击）
function handleOptionToggle(option: string) {
  const idx = selectedOptions.value.indexOf(option)
  if (idx > -1) {
    selectedOptions.value.splice(idx, 1)
  }
  else {
    selectedOptions.value.push(option)
  }
  emitUpdate()
}

// 移除了所有拖拽和上传组件相关的代码

function handleImagePaste(event: ClipboardEvent) {
  const items = event.clipboardData?.items
  let hasImage = false

  if (items) {
    for (const item of items) {
      if (item.type.includes('image')) {
        hasImage = true
        const file = item.getAsFile()
        if (file) {
          handleImageFiles([file])
        }
      }
    }
  }

  if (hasImage) {
    event.preventDefault()
  }
}

async function handleImageFiles(files: FileList | File[]): Promise<void> {
  console.log('=== 处理图片文件 ===')
  console.log('文件数量:', files.length)

  for (const file of files) {
    console.log('处理文件:', file.name, '类型:', file.type, '大小:', file.size)

    if (file.type.startsWith('image/')) {
      try {
        console.log('开始转换为 Base64...')
        const base64 = await fileToBase64(file)
        console.log('Base64转换成功，长度:', base64.length)

        // 检查是否已存在相同图片，避免重复添加
        if (!uploadedImages.value.includes(base64)) {
          uploadedImages.value.push(base64)
          console.log('图片已添加到数组，当前数量:', uploadedImages.value.length)
          message.success(`图片 ${file.name} 已添加`)
          emitUpdate()
        }
        else {
          console.log('图片已存在，跳过:', file.name)
          message.warning(`图片 ${file.name} 已存在`)
        }
      }
      catch (error) {
        console.error('图片处理失败:', error)
        message.error(`图片 ${file.name} 处理失败`)
        throw error
      }
    }
    else {
      console.log('跳过非图片文件:', file.type)
    }
  }

  console.log('=== 图片文件处理完成 ===')
}

function fileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result as string)
    reader.onerror = reject
    reader.readAsDataURL(file)
  })
}

function removeImage(index: number) {
  uploadedImages.value.splice(index, 1)
  emit('imageRemove', index)
  emitUpdate()
}

// 移除自定义图片预览功能，改用 Naive UI 的内置预览

// 监听用户输入变化
watch(userInput, () => {
  emitUpdate()
})

// 移除了所有拖拽相关的代码

// 重置数据
function reset() {
  userInput.value = ''
  selectedOptions.value = []
  uploadedImages.value = []
  emitUpdate()
}

// 更新数据（用于外部同步）
function updateData(data: { userInput?: string, selectedOptions?: string[], draggedImages?: string[] }) {
  if (data.userInput !== undefined) {
    userInput.value = data.userInput
  }
  if (data.selectedOptions !== undefined) {
    selectedOptions.value = data.selectedOptions
  }
  if (data.draggedImages !== undefined) {
    uploadedImages.value = data.draggedImages
  }

  emitUpdate()
}

// 移除了文件选择和测试图片功能

// 暴露方法给父组件
defineExpose({
  reset,
  canSubmit,
  statusText,
  updateData,
})
</script>

<template>
  <div class="space-y-3">
    <!-- 预定义选项 -->
    <div v-if="!loading && hasOptions" class="space-y-3">
      <h4 class="text-sm font-medium text-white">
        请选择选项
      </h4>
      <n-space vertical size="small">
        <div
          v-for="(option, index) in request!.predefined_options"
          :key="`option-${index}`"
          class="rounded-lg p-3 border border-gray-600 bg-gray-100 cursor-pointer hover:opacity-80 transition-opacity"
          @click="handleOptionToggle(option)"
        >
          <n-checkbox
            :value="option"
            :checked="selectedOptions.includes(option)"
            :disabled="submitting"
            size="medium"
            @update:checked="(checked: boolean) => handleOptionChange(option, checked)"
            @click.stop
          >
            {{ option }}
          </n-checkbox>
        </div>
      </n-space>
    </div>

    <!-- 图片预览区域 -->
    <div v-if="!loading && uploadedImages.length > 0" class="space-y-3">
      <h4 class="text-sm font-medium text-white">
        已添加的图片 ({{ uploadedImages.length }})
      </h4>

      <!-- 使用 Naive UI 的图片组件，支持预览和放大 -->
      <n-image-group>
        <div class="flex flex-wrap gap-3">
          <div
            v-for="(image, index) in uploadedImages"
            :key="`image-${index}`"
            class="relative"
          >
            <!-- 使用 n-image 组件，启用预览功能 -->
            <n-image
              :src="image"
              width="100"
              height="100"
              object-fit="cover"
              class="rounded-lg border-2 border-gray-300 hover:border-primary-400 transition-all duration-200 cursor-pointer"
            />

            <!-- 删除按钮 -->
            <n-button
              class="absolute -top-2 -right-2 z-10"
              size="tiny"
              type="error"
              circle
              @click="removeImage(index)"
            >
              <template #icon>
                <div class="i-carbon-close w-3 h-3" />
              </template>
            </n-button>

            <!-- 序号 -->
            <div class="absolute bottom-1 left-1 w-5 h-5 bg-primary-500 text-white text-xs rounded-full flex items-center justify-center font-bold shadow-sm z-5">
              {{ index + 1 }}
            </div>
          </div>
        </div>
      </n-image-group>
    </div>

    <!-- 文本输入区域 -->
    <div v-if="!loading" class="space-y-3">
      <h4 class="text-sm font-medium text-white">
        {{ hasOptions ? '补充说明 (可选)' : '请输入您的回复' }}
      </h4>

      <!-- 图片提示区域 -->
      <div v-if="uploadedImages.length === 0" class="text-center">
        <div class="text-xs text-gray-400">
          💡 提示：可以在输入框中粘贴图片 ({{ pasteShortcut }})
        </div>
      </div>

      <!-- 文本输入框 -->
      <n-input
        ref="textareaRef"
        v-model:value="userInput"
        type="textarea"
        size="small"
        :placeholder="hasOptions ? `您可以在这里添加补充说明... (支持粘贴图片 ${pasteShortcut})` : `请输入您的回复... (支持粘贴图片 ${pasteShortcut})`"
        :disabled="submitting"
        :autosize="{ minRows: 3, maxRows: 6 }"
        @paste="handleImagePaste"
      />
    </div>
  </div>
</template>
