<template>
  <div class="logs">
    <!-- 移动端布局 -->
    <MobileLogs
      v-if="isMobile"
      :logs="logsStore.logs"
      :loading="logsStore.loading"
      :error="logsStore.error"
      @refresh="refreshLogs"
    />

    <!-- 桌面端布局 -->
    <div v-else class="desktop-logs">
      <div class="header">
        <h1>请求日志</h1>
        <button @click="refreshLogs" class="refresh-btn">
          刷新
        </button>
      </div>

      <div v-if="logsStore.loading" class="loading">
        加载中...
      </div>

      <div v-if="logsStore.error" class="error">
        {{ logsStore.error }}
      </div>

      <div class="logs-container">
        <div class="logs-table">
          <table>
            <thead>
              <tr>
                <th>时间</th>
                <th>方法</th>
                <th>路径</th>
                <th>状态码</th>
                <th>响应时间</th>
                <th>API 密钥</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="log in logsStore.logs" :key="log.id">
                <td>{{ formatDate(log.createdAt) }}</td>
                <td>
                  <span class="method-badge" :class="log.method.toLowerCase()">
                    {{ log.method }}
                  </span>
                </td>
                <td class="path">{{ log.path }}</td>
                <td>
                  <span class="status-badge" :class="getStatusClass(log.statusCode)">
                    {{ log.statusCode }}
                  </span>
                </td>
                <td>{{ log.responseTimeMs }}ms</td>
                <td>{{ log.apiKeyName }}</td>
              </tr>
              <tr v-if="logsStore.logs.length === 0 && !logsStore.loading" class="no-data-row">
                <td colspan="6" class="no-data">
                  <div class="no-data-content">
                    <div class="no-data-icon">📝</div>
                    <div class="no-data-text">暂无请求日志</div>
                    <div class="no-data-hint">发送一些API请求后，日志将显示在这里</div>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { useLogsStore } from '../stores/logs'
import { useResponsive } from '@/composables/useResponsive'
import MobileLogs from '@/components/mobile/MobileLogs.vue'

const logsStore = useLogsStore()
const { isMobile } = useResponsive()

const formatDate = (dateString) => {
  return new Date(dateString).toLocaleString('zh-CN')
}

const getStatusClass = (statusCode) => {
  if (statusCode >= 200 && statusCode < 300) return 'success'
  if (statusCode >= 400 && statusCode < 500) return 'client-error'
  if (statusCode >= 500) return 'server-error'
  return 'other'
}

const refreshLogs = () => {
  logsStore.fetchLogs()
}

onMounted(() => {
  logsStore.fetchLogs()
})
</script>

<style scoped>
.logs {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.desktop-logs {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
}

.refresh-btn {
  padding: 0.75rem 1.5rem;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 0.375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.refresh-btn:hover {
  background: var(--color-primary-hover);
}

.loading,
.error {
  text-align: center;
  padding: 2rem;
  color: var(--color-text-secondary);
}

.error {
  color: var(--color-danger);
}

.logs-container {
  background: var(--color-surface);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
  overflow: hidden;
}

.logs-table {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 1rem;
  text-align: left;
  border-bottom: 1px solid var(--color-border);
}

th {
  background: var(--color-surface-secondary);
  font-weight: 600;
  color: var(--color-text);
  font-size: 0.875rem;
}

td {
  color: var(--color-text);
  font-size: 0.875rem;
}

.path {
  font-family: monospace;
  word-break: break-all;
  max-width: 200px;
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
  min-width: 56px;
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
  min-width: 64px;
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

.no-data {
  text-align: center;
  padding: 2rem;
}

.no-data-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.no-data-icon {
  font-size: 2rem;
}

.no-data-text {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
}

.no-data-hint {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

@media (max-width: 768px) {
  .desktop-logs {
    padding: 1rem;
  }
  
  .header {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }
  
  th,
  td {
    padding: 0.75rem 0.5rem;
  }
  
  .path {
    max-width: 120px;
  }
  
  .method-badge,
  .status-badge {
    min-width: 48px;
    padding: 0.25rem 0.375rem;
  }
}

/* Method Badge */
.method-badge {
  display: inline-flex;
  align-items: center;
  padding: var(--spacing-1) var(--spacing-3);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--font-bold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  min-width: 56px;
  justify-content: center;
}

.method-badge--get {
  background-color: rgba(var(--color-info-rgb), 0.1);
  color: var(--color-info);
}

.method-badge--post {
  background-color: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

.method-badge--put {
  background-color: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
}

.method-badge--patch {
  background-color: rgba(var(--color-secondary-rgb), 0.1);
  color: var(--color-secondary);
}

.method-badge--delete {
  background-color: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
}

/* Path Text */
.path-text {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  word-break: break-all;
}

/* Status Badge */
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-1);
  padding: var(--spacing-1) var(--spacing-3);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--font-semibold);
  min-width: 64px;
  justify-content: center;
}

.status-badge--success {
  background-color: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

.status-badge--warning {
  background-color: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
}

.status-badge--danger {
  background-color: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
}

.status-badge--info {
  background-color: rgba(var(--color-info-rgb), 0.1);
  color: var(--color-info);
}

/* Response Time */
.response-time {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
}

.response-time--fast {
  color: var(--color-success);
}

.response-time--normal {
  color: var(--color-text);
}

.response-time--slow {
  color: var(--color-warning);
}

.response-time--very-slow {
  color: var(--color-danger);
}

/* API Key Name */
.api-key-name {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  background-color: var(--color-surface-secondary);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-sm);
}

/* Time Cell */
.time-cell {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1);
}

.time-main {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text);
}

.time-date {
  font-size: var(--text-xs);
  color: var(--color-text-tertiary);
}

/* Filters Panel */
.filters-panel {
  margin-top: var(--spacing-4);
  border: 2px solid var(--color-border);
}

.filters-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-4);
  padding: var(--spacing-4);
  border-bottom: 1px solid var(--color-border);
}

.filters-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
}

.filters-content {
  padding: var(--spacing-6);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-3);
}

.filter-label {
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--color-text-secondary);
}

.filter-options {
  display: flex;
  gap: var(--spacing-2);
  flex-wrap: wrap;
}

.filter-option {
  min-width: 80px;
}

/* Path Column */
:deep(.path-column) {
  max-width: 200px;
  word-break: break-all;
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .logs-page {
    padding: var(--spacing-4);
    gap: var(--spacing-4);
  }
  
  .time-cell {
    align-items: flex-start;
  }
  
  .filters-content {
    padding: var(--spacing-4);
    gap: var(--spacing-4);
  }
  
  .filter-options {
    gap: var(--spacing-1);
  }
  
  .filter-option {
    min-width: 60px;
    font-size: var(--text-xs);
  }
  
  :deep(.path-column) {
    max-width: 120px;
  }
}

/* Tablet optimizations */
@media (max-width: 768px) {
  .method-badge,
  .status-badge {
    min-width: 48px;
    padding: var(--spacing-1) var(--spacing-2);
  }
  
  :deep(.path-column) {
    max-width: 160px;
  }
}

/* Dark mode enhancements */
@media (prefers-color-scheme: dark) {
  .filters-panel {
    border-color: rgba(255, 255, 255, 0.1);
  }
  
  .filters-header {
    border-bottom-color: rgba(255, 255, 255, 0.1);
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .method-badge,
  .status-badge {
    border: 1px solid currentColor;
  }
}
</style>