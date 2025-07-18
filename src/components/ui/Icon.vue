<template>
  <svg
    :class="iconClasses"
    :style="iconStyles"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <component :is="iconComponent" />
  </svg>
</template>

<script setup>
import { computed, defineAsyncComponent } from 'vue'

const props = defineProps({
  name: {
    type: String,
    required: true
  },
  size: {
    type: [String, Number],
    default: 20
  },
  color: {
    type: String,
    default: 'currentColor'
  }
})

const iconClasses = computed(() => [
  'icon',
  `icon--${props.name}`
])

const iconStyles = computed(() => ({
  width: typeof props.size === 'number' ? `${props.size}px` : props.size,
  height: typeof props.size === 'number' ? `${props.size}px` : props.size,
  color: props.color
}))

// 图标组件映射
const iconComponents = {
  // 导航图标
  home: () => import('./icons/HomeIcon.vue'),
  key: () => import('./icons/KeyIcon.vue'),
  logs: () => import('./icons/LogsIcon.vue'),
  settings: () => import('./icons/SettingsIcon.vue'),
  
  // 操作图标
  add: () => import('./icons/AddIcon.vue'),
  edit: () => import('./icons/EditIcon.vue'),
  delete: () => import('./icons/DeleteIcon.vue'),
  refresh: () => import('./icons/RefreshIcon.vue'),
  save: () => import('./icons/SaveIcon.vue'),
  
  // 状态图标
  check: () => import('./icons/CheckIcon.vue'),
  close: () => import('./icons/CloseIcon.vue'),
  warning: () => import('./icons/WarningIcon.vue'),
  info: () => import('./icons/InfoIcon.vue'),
  
  // 界面图标
  menu: () => import('./icons/MenuIcon.vue'),
  search: () => import('./icons/SearchIcon.vue'),
  filter: () => import('./icons/FilterIcon.vue'),
  upload: () => import('./icons/UploadIcon.vue'),
  download: () => import('./icons/DownloadIcon.vue'),
  
  // 其他图标
  eye: () => import('./icons/EyeIcon.vue'),
  'eye-off': () => import('./icons/EyeOffIcon.vue'),
  logout: () => import('./icons/LogoutIcon.vue'),
  user: () => import('./icons/UserIcon.vue'),
  
  // 箭头图标
  'arrow-left': () => import('./icons/ArrowLeftIcon.vue'),
  'arrow-right': () => import('./icons/ArrowRightIcon.vue'),
  'arrow-up': () => import('./icons/ArrowUpIcon.vue'),
  'arrow-down': () => import('./icons/ArrowDownIcon.vue'),
  
  // 加载和状态
  loading: () => import('./icons/LoadingIcon.vue'),
  success: () => import('./icons/SuccessIcon.vue'),
  error: () => import('./icons/ErrorIcon.vue'),
  
  // 新增图标
  'trending-up': () => import('./icons/TrendingUpIcon.vue'),
  activity: () => import('./icons/ActivityIcon.vue'),
  copy: () => import('./icons/CopyIcon.vue'),
  play: () => import('./icons/PlayIcon.vue'),
  pause: () => import('./icons/PauseIcon.vue')
}

const iconComponent = computed(() => {
  const component = iconComponents[props.name]
  if (component) {
    return defineAsyncComponent(component)
  }
  
  // 回退到默认图标
  console.warn(`Icon "${props.name}" not found, using default`)
  return defineAsyncComponent(() => import('./icons/DefaultIcon.vue'))
})
</script>

<style scoped>
.icon {
  display: inline-block;
  vertical-align: middle;
  flex-shrink: 0;
}

.icon--loading {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>