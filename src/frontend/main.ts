import {
  create,
  NAlert,
  NButton,
  NCard,
  NCheckbox,
  NCollapseTransition,
  NConfigProvider,
  NDialogProvider,
  NGrid,
  NGridItem,
  NIcon,
  NImage,
  NImageGroup,
  NInput,
  NInputGroup,
  NInputNumber,
  NMessageProvider,
  NNotificationProvider,
  NSkeleton,
  NSpace,
  NSpin,
  NSwitch,
  NTab,
  NTabPane,
  NTabs,
  NTag,
  NTooltip,
} from 'naive-ui'
import { createApp } from 'vue'
import App from './App.vue'
import 'virtual:uno.css'
import '../assets/css/style.css'

const naive = create({
  components: [
    NAlert,
    NButton,
    NCard,
    NCheckbox,
    NCollapseTransition,
    NConfigProvider,
    NDialogProvider,
    NMessageProvider,
    NNotificationProvider,
    NSpace,
    NSpin,
    NSwitch,
    NTab,
    NTabPane,
    NTabs,
    NInput,
    NInputGroup,
    NInputNumber,
    NTooltip,
    NIcon,
    NImage,
    NImageGroup,
    NGrid,
    NGridItem,
    NTag,
    NSkeleton,
  ],
})

const app = createApp(App)
app.use(naive)
app.mount('#app')
