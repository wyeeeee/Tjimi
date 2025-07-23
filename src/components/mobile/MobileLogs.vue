<template>
  <div class="mobile-logs">
    <!-- 头部 -->
    <div class="mobile-header">
      <div class="header-title">
        <Icon name="list" size="20" />
        <h1>请求日志</h1>
      </div>
      <button
        @click="$emit('refresh')"
        :disabled="loading"
        class="refresh-btn"
      >
        <Icon name="refresh" size="16" />
      </button>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="loading-state">
      <Icon name="loading" size="20" />
      <span>加载中...</span>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="error" class="error-state">
      <Icon name="error" size="20" />
      <span>{{ error }}</span>
    </div>

    <!-- 日志列表 -->
    <div v-else class="logs-list">
      <div v-if="logs.length === 0" class="empty-state">
        <Icon name="document" size="32" />
        <h3>暂无请求日志</h3>
        <p>发送一些API请求后，日志将显示在这里</p>
      </div>

      <div v-else class="log-cards">
        <div
          v-for="log in logs"
          :key="log.id"
          class="log-card"
        >
          <div class="log-header">
            <div class="log-method">
              <span class="method-badge" :class="log.method.toLowerCase()">
                {{ log.method }}
              </span>
              <span class="status-badge" :class="getStatusClass(log.statusCode)">
                {{ log.statusCode }}
              </span>
            </div>
            <div class="log-time">
              {{ formatTime(log.createdAt) }}
            </div>
          </div>

          <div class="log-content">
            <div class="log-path">
              <Icon name="link" size="14" />
              <span class="path-text">{{ log.path }}</span>
            </div>
            
            <div class="log-details">
              <div class="detail-item">
                <Icon name="clock" size="14" />
                <span>{{ log.responseTimeMs }}ms</span>
              </div>
              <div class="detail-item">
                <Icon name="key" size="14" />
                <span>{{ log.apiKeyName }}</span>
              </div>
            </div>
            
            <div class="log-actions" v-if="log.requestBody || log.responseBody">
              <button v-if="log.requestBody" @click="showRequestBody(log)" class="action-btn" title="查看请求内容">
                📤 请求
              </button>
              <button v-if="log.responseBody" @click="showResponseBody(log)" class="action-btn" title="查看返回内容">
                📥 返回
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 移动端分页 -->
    <MobilePagination
      v-if="logs.length > 0"
      :current-page="currentPage"
      :total-pages="totalPages"
      :total-count="totalCount"
      @page-change="$emit('page-change', $event)"
    />
    
    <!-- 内容查看弹窗 -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h3>{{ modalTitle }}</h3>
          <button @click="closeModal" class="close-btn">&times;</button>
        </div>
        <div class="modal-content">
          <pre>{{ modalContent }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import Icon from '@/components/ui/Icon.vue'
import MobilePagination from './MobilePagination.vue'

defineProps({
  logs: {
    type: Array,
    default: () => []
  },
  loading: Boolean,
  error: String,
  currentPage: {
    type: Number,
    default: 1
  },
  totalPages: {
    type: Number,
    default: 1
  },
  totalCount: {
    type: Number,
    default: 0
  }
})

defineEmits(['refresh', 'page-change'])

const showModal = ref(false)
const modalTitle = ref('')
const modalContent = ref('')

const showRequestBody = (log) => {
  modalTitle.value = '请求内容'
  modalContent.value = log.requestBody || ''
  showModal.value = true
}

const showResponseBody = (log) => {
  modalTitle.value = '返回内容'
  modalContent.value = log.responseBody || ''
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  modalTitle.value = ''
  modalContent.value = ''
}

const formatTime = (dateString) => {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMins / 60)
  const diffDays = Math.floor(diffHours / 24)

  if (diffMins < 1) return '刚刚'
  if (diffMins < 60) return `${diffMins}分钟前`
  if (diffHours < 24) return `${diffHours}小时前`
  if (diffDays < 7) return `${diffDays}天前`
  
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const getStatusClass = (statusCode) => {
  if (statusCode >= 200 && statusCode < 300) return 'success'
  if (statusCode >= 400 && statusCode < 500) return 'client-error'
  if (statusCode >= 500) return 'server-error'
  return 'other'
}
</script>

<style scoped>
.mobile-logs {
  padding: 1rem;
  max-width: 100%;
}

.mobile-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--color-surface);
  border-radius: 0.5rem;
  margin-bottom: 1rem;
  border: 1px solid var(--color-border);
}

.header-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.header-title h1 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
}

.refresh-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: var(--color-surface-secondary);
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.refresh-btn:hover:not(:disabled) {
  background: var(--color-surface-hover);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.loading-state,
.error-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 2rem;
  color: var(--color-text-secondary);
}

.error-state {
  color: var(--color-danger);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 2rem;
  text-align: center;
  color: var(--color-text-secondary);
}

.empty-state h3 {
  margin: 0;
  font-size: 1.125rem;
  color: var(--color-text);
}

.empty-state p {
  margin: 0;
  font-size: 0.875rem;
  line-height: 1.4;
}

.logs-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.log-card {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  padding: 1rem;
  transition: all 0.2s;
}

.log-card:hover {
  border-color: var(--color-primary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.log-method {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.method-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  min-width: 48px;
}

.method-badge.get {
  background: rgba(var(--color-info-rgb), 0.1);
  color: var(--color-info);
}

.method-badge.post {
  background: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

.method-badge.put {
  background: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
}

.method-badge.patch {
  background: rgba(var(--color-secondary-rgb), 0.1);
  color: var(--color-secondary);
}

.method-badge.delete {
  background: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
}

.status-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.75rem;
  font-weight: 600;
  min-width: 48px;
}

.status-badge.success {
  background: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

.status-badge.client-error {
  background: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
}

.status-badge.server-error {
  background: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
}

.status-badge.other {
  background: rgba(var(--color-text-rgb), 0.1);
  color: var(--color-text-secondary);
}

.log-time {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.log-content {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.log-path {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  background: var(--color-surface-secondary);
  border-radius: 0.375rem;
}

.path-text {
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--color-text);
  word-break: break-all;
  line-height: 1.4;
}

.log-details {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
}

.detail-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.log-actions {
  display: flex;
  gap: 0.75rem;
  margin-top: 0.75rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--color-border);
}

.action-btn {
  flex: 1;
  background: var(--color-surface-secondary);
  border: 1px solid var(--color-border);
  padding: 0.5rem 0.75rem;
  border-radius: 0.375rem;
  cursor: pointer;
  font-size: 0.875rem;
  color: var(--color-text);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
}

.action-btn:hover {
  background: var(--color-surface-hover);
  border-color: var(--color-primary);
  color: var(--color-primary);
}

@media (max-width: 480px) {
  .mobile-logs {
    padding: 0.75rem;
  }
  
  .mobile-header {
    padding: 0.75rem;
  }
  
  .header-title h1 {
    font-size: 1.125rem;
  }
  
  .log-card {
    padding: 0.75rem;
  }
  
  .log-details {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .method-badge,
  .status-badge {
    min-width: 40px;
    padding: 0.25rem 0.375rem;
  }
  
  .action-btn {
    font-size: 0.8125rem;
    padding: 0.375rem 0.5rem;
  }
}

/* 弹窗样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  padding: 1rem;
}

.modal {
  background: var(--color-surface);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
  max-width: 90vw;
  max-height: 90vh;
  width: 100%;
  max-width: 500px;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--color-border);
}

.modal-header h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--color-text-secondary);
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 0.25rem;
  transition: all 0.2s;
}

.close-btn:hover {
  background: var(--color-surface-hover);
  color: var(--color-text);
}

.modal-content {
  padding: 1rem;
  overflow: auto;
  flex: 1;
}

.modal-content pre {
  background: var(--color-surface-secondary);
  padding: 0.75rem;
  border-radius: 0.375rem;
  border: 1px solid var(--color-border);
  font-family: monospace;
  font-size: 0.8125rem;
  line-height: 1.4;
  color: var(--color-text);
  white-space: pre-wrap;
  word-wrap: break-word;
  margin: 0;
  max-height: 50vh;
  overflow: auto;
}
</style>