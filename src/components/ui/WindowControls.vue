<template>
  <div class="window-controls" v-if="isDesktop">
    <div class="window-controls-container">
      <!-- 拖拽区域 -->
      <div 
        class="drag-area"
        @mousedown="handleStartDrag"
        @dblclick="handleToggleMaximize"
      >
        <div class="window-title">
          <slot name="title">{{ title }}</slot>
        </div>
      </div>
      
      <!-- 控制按钮 -->
      <div class="control-buttons">
        <button
          @click="handleMinimize"
          class="control-btn minimize-btn"
          title="最小化"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M2 6h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
        
        <button
          @click="handleToggleMaximize"
          class="control-btn maximize-btn"
          :title="isMaximized ? '还原' : '最大化'"
        >
          <svg v-if="!isMaximized" width="12" height="12" viewBox="0 0 12 12">
            <rect x="2" y="2" width="8" height="8" rx="1" fill="none" stroke="currentColor" stroke-width="1.5"/>
          </svg>
          <svg v-else width="12" height="12" viewBox="0 0 12 12">
            <rect x="2" y="3" width="6" height="6" rx="1" fill="none" stroke="currentColor" stroke-width="1.5"/>
            <path d="M5 2h5a1 1 0 0 1 1 1v5" fill="none" stroke="currentColor" stroke-width="1.5"/>
          </svg>
        </button>
        
        <button
          @click="handleClose"
          class="control-btn close-btn"
          title="关闭"
        >
          <svg width="12" height="12" viewBox="0 0 12 12">
            <path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  title: {
    type: String,
    default: 'Tjimi Proxy'
  }
})

const isDesktop = ref(false)
const isMaximized = ref(false)

// 检查是否为桌面端
const checkDesktop = async () => {
  try {
    isDesktop.value = await invoke('is_desktop')
  } catch (error) {
    console.error('Failed to check desktop:', error)
    isDesktop.value = false
  }
}

// 检查窗口最大化状态
const checkMaximized = async () => {
  if (!isDesktop.value) return
  
  try {
    isMaximized.value = await invoke('is_window_maximized')
  } catch (error) {
    console.error('Failed to check maximized state:', error)
  }
}

// 窗口控制方法
const handleMinimize = async () => {
  try {
    await invoke('minimize_window')
  } catch (error) {
    console.error('Failed to minimize window:', error)
  }
}

const handleToggleMaximize = async () => {
  try {
    await invoke('toggle_maximize_window')
    // 延迟检查状态，等待窗口动画完成
    setTimeout(checkMaximized, 100)
  } catch (error) {
    console.error('Failed to toggle maximize:', error)
  }
}

const handleClose = async () => {
  try {
    await invoke('close_window')
  } catch (error) {
    console.error('Failed to close window:', error)
  }
}

const handleStartDrag = async (event) => {
  // 防止在按钮区域开始拖拽
  if (event.target.closest('.control-buttons')) return
  
  try {
    await invoke('start_drag')
  } catch (error) {
    console.error('Failed to start drag:', error)
  }
}

// 监听窗口状态变化
let resizeObserver = null

onMounted(async () => {
  await checkDesktop()
  await checkMaximized()
  
  // 监听窗口resize事件来检测最大化状态变化
  if (isDesktop.value) {
    const handleResize = () => {
      setTimeout(checkMaximized, 50)
    }
    
    window.addEventListener('resize', handleResize)
    
    // 清理函数
    onUnmounted(() => {
      window.removeEventListener('resize', handleResize)
    })
  }
})
</script>

<style scoped>
.window-controls {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 40px;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  z-index: 1000;
  user-select: none;
}

.window-controls-container {
  display: flex;
  height: 100%;
  align-items: center;
}

.drag-area {
  flex: 1;
  height: 100%;
  display: flex;
  align-items: center;
  padding: 0 16px;
  cursor: grab;
}

.drag-area:active {
  cursor: grabbing;
}

.window-title {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.control-buttons {
  display: flex;
  height: 100%;
  flex-shrink: 0;
}

.control-btn {
  width: 46px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--color-text);
  cursor: pointer;
  transition: background-color 0.2s;
}

.control-btn:hover {
  background: var(--color-surface-hover);
}

.control-btn:active {
  background: var(--color-surface-pressed);
}

.minimize-btn:hover {
  background: rgba(var(--color-warning-rgb), 0.1);
}

.maximize-btn:hover {
  background: rgba(var(--color-success-rgb), 0.1);
}

.close-btn:hover {
  background: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
}

.close-btn:active {
  background: var(--color-danger);
  color: white;
}

/* 移动端和安卓端隐藏 */
@media (max-width: 768px) {
  .window-controls {
    display: none;
  }
}

/* 安卓端强制隐藏 */
@media (orientation: portrait) and (max-width: 768px) {
  .window-controls {
    display: none !important;
  }
}

/* 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .window-controls {
    background: var(--color-surface);
    border-bottom-color: var(--color-border);
  }
}
</style>