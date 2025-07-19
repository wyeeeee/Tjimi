<template>
  <div class="logs">
    <!-- 移动端布局 -->
    <MobileLogs
      v-if="isMobile"
      :logs="logsStore.logs"
      :loading="logsStore.loading"
      :error="logsStore.error"
      :current-page="logsStore.pagination.currentPage"
      :total-pages="logsStore.pagination.totalPages"
      :total-count="logsStore.pagination.totalCount"
      @refresh="refreshLogs"
      @page-change="handlePageChange"
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
                <th class="logs-header">请求日志</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="log in logsStore.logs" :key="log.id" class="log-row">
                <td class="main-row">
                  <div class="log-main-info">
                    <div class="log-time">{{ formatDate(log.createdAt) }}</div>
                    <div class="log-method">
                      <span class="method-badge" :class="log.method.toLowerCase()">
                        {{ log.method }}
                      </span>
                    </div>
                    <div class="log-status">
                      <span class="status-badge" :class="getStatusClass(log.statusCode)">
                        {{ log.statusCode }}
                      </span>
                    </div>
                    <div class="log-response-time">{{ log.responseTimeMs }}ms</div>
                    <div class="log-api-key">{{ log.apiKeyName }}</div>
                  </div>
                  <div class="log-path">
                    <span class="path-text">{{ log.path }}</span>
                  </div>
                </td>
              </tr>
              <tr v-if="logsStore.logs.length === 0 && !logsStore.loading" class="no-data-row">
                <td class="no-data">
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
      
      <!-- 分页控件 -->
      <Pagination
        v-if="!isMobile && logsStore.logs.length > 0"
        :current-page="logsStore.pagination.currentPage"
        :total-pages="logsStore.pagination.totalPages"
        :total-count="logsStore.pagination.totalCount"
        :per-page="logsStore.pagination.perPage"
        @page-change="handlePageChange"
        @per-page-change="handlePerPageChange"
      />
    </div>
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { useLogsStore } from '../stores/logs'
import { useResponsive } from '@/composables/useResponsive'
import MobileLogs from '@/components/mobile/MobileLogs.vue'
import Pagination from '@/components/ui/Pagination.vue'

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
  if (isMobile.value) {
    // 移动端也使用分页模式，但每页更少
    logsStore.fetchLogsPaginated(1, 20)
  } else {
    // 桌面端使用分页模式
    logsStore.fetchLogsPaginated(1, 50)
  }
}

const handlePageChange = (page) => {
  logsStore.fetchLogsPaginated(page, logsStore.pagination.perPage)
}

const handlePerPageChange = (perPage) => {
  logsStore.fetchLogsPaginated(1, perPage)
}

onMounted(() => {
  refreshLogs()
})
</script>

<style scoped>
.logs {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.desktop-logs {
  padding: 1.5rem;
  max-width: 100%;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  width: 100%;
}

/* 大屏幕优化 */
@media (min-width: 1200px) {
  .desktop-logs {
    max-width: 1400px;
    padding: 2rem;
    gap: 2rem;
  }
  
  .log-main-info {
    padding: 1.25rem 1.5rem;
    gap: 2rem;
  }
  
  .log-time {
    width: 16%;
    min-width: 140px;
  }
  
  .log-method {
    width: 8%;
    min-width: 70px;
  }
  
  .log-status {
    width: 8%;
    min-width: 70px;
  }
  
  .log-response-time {
    width: 10%;
    min-width: 85px;
  }
  
  .log-path {
    padding: 1rem 1.5rem;
  }
}

/* 中等屏幕优化 */
@media (min-width: 769px) and (max-width: 1199px) {
  .desktop-logs {
    max-width: 100%;
    padding: 1.5rem 2rem;
  }
  
  .log-main-info {
    padding: 1rem 1.25rem;
    gap: 1.5rem;
  }
  
  .log-time {
    width: 20%;
    min-width: 120px;
  }
  
  .log-method {
    width: 12%;
    min-width: 60px;
  }
  
  .log-status {
    width: 12%;
    min-width: 60px;
  }
  
  .log-response-time {
    width: 14%;
    min-width: 75px;
  }
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

/* 新的日志行样式 */
.log-row {
  border-bottom: 1px solid var(--color-border);
}

.main-row {
  padding: 0;
}

.log-main-info {
  display: flex;
  align-items: center;
  padding: 1rem;
  gap: 1rem;
  width: 100%;
  justify-content: space-between;
}

.log-time {
  flex: 0 0 auto;
  width: 18%;
  min-width: 120px;
  font-size: 0.875rem;
  color: var(--color-text);
  font-weight: 500;
}

.log-method {
  flex: 0 0 auto;
  width: 10%;
  min-width: 60px;
}

.log-status {
  flex: 0 0 auto;
  width: 10%;
  min-width: 60px;
}

.log-response-time {
  flex: 0 0 auto;
  width: 12%;
  min-width: 75px;
  font-size: 0.875rem;
  color: var(--color-text);
  font-weight: 500;
  font-family: var(--font-mono, monospace);
}

.log-api-key {
  flex: 1;
  min-width: 100px;
  font-size: 0.875rem;
  color: var(--color-text);
  font-family: var(--font-mono, monospace);
  text-align: right;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.log-path {
  padding: 0.75rem 1rem;
  background: var(--color-surface-secondary);
  border-top: 1px solid var(--color-border);
  border-radius: 0 0 0.375rem 0.375rem;
}

.logs-header {
  font-weight: 600;
  color: var(--color-text);
  text-align: center;
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

/* 小屏幕优化 */
@media (max-width: 768px) {
  .desktop-logs {
    padding: 1rem;
    gap: 1rem;
  }
  
  .header {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
    margin-bottom: 1rem;
  }
  
  .header h1 {
    font-size: 1.25rem;
  }
  
  .log-main-info {
    flex-direction: column;
    align-items: stretch;
    gap: 0.875rem;
    padding: 1rem;
    justify-content: stretch;
  }
  
  .log-time,
  .log-method,
  .log-status, 
  .log-response-time,
  .log-api-key {
    width: 100%;
    min-width: auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.875rem;
    text-align: left;
  }
  
  .log-time::before { content: "时间: "; font-weight: 600; color: var(--color-text-secondary); }
  .log-response-time::before { content: "响应时间: "; font-weight: 600; color: var(--color-text-secondary); }
  .log-api-key::before { content: "API密钥: "; font-weight: 600; color: var(--color-text-secondary); }
  
  .log-method {
    justify-content: flex-start;
    gap: 0.75rem;
  }
  
  .log-method::before { content: "方法: "; font-weight: 600; color: var(--color-text-secondary); }
  
  .log-status {
    justify-content: flex-start; 
    gap: 0.75rem;
  }
  
  .log-status::before { content: "状态: "; font-weight: 600; color: var(--color-text-secondary); }
  
  .log-api-key {
    text-align: right;
    white-space: nowrap;
    overflow: visible;
    text-overflow: clip;
  }
  
  .method-badge,
  .status-badge {
    min-width: 52px;
    padding: 0.375rem 0.5rem;
    font-size: 0.75rem;
  }
  
  .log-path {
    padding: 0.75rem 1rem;
    margin-top: 0.25rem;
  }
  
  .path-text {
    font-size: 0.8125rem;
    line-height: 1.5;
  }
}

/* 超小屏幕优化 */
@media (max-width: 480px) {
  .desktop-logs {
    padding: 0.75rem;
  }
  
  .log-main-info {
    padding: 0.75rem;
    gap: 0.75rem;
  }
  
  .method-badge,
  .status-badge {
    min-width: 48px;
    padding: 0.25rem 0.375rem;
    font-size: 0.6875rem;
  }
  
  .log-path {
    padding: 0.625rem 0.75rem;
  }
  
  .path-text {
    font-size: 0.75rem;
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
  line-height: 1.4;
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
    max-width: 80px;
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
    max-width: 120px;
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