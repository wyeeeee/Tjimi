<template>
  <div class="logs">
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
</template>

<script setup>
import { onMounted } from 'vue'
import { useLogsStore } from '../stores/logs'

const logsStore = useLogsStore()

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
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.refresh-btn {
  background: #007bff;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 14px;
}

.refresh-btn:hover {
  background: #0056b3;
}

.loading, .error {
  text-align: center;
  padding: 20px;
  margin: 20px 0;
}

.error {
  color: #dc3545;
  background: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 5px;
}

.logs-container {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  overflow: hidden;
}

.logs-table {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
  background: white;
}

thead {
  background: #f1f3f4;
}

th, td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid #e9ecef;
}

th {
  font-weight: 600;
  color: #495057;
}

.method-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: bold;
}

.method-badge.get {
  background: #d4edda;
  color: #155724;
}

.method-badge.post {
  background: #cce5ff;
  color: #004085;
}

.method-badge.put {
  background: #fff3cd;
  color: #856404;
}

.method-badge.delete {
  background: #f8d7da;
  color: #721c24;
}

.path {
  font-family: monospace;
  font-size: 13px;
  color: #6c757d;
}

.status-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: bold;
}

.status-badge.success {
  background: #d4edda;
  color: #155724;
}

.status-badge.client-error {
  background: #fff3cd;
  color: #856404;
}

.status-badge.server-error {
  background: #f8d7da;
  color: #721c24;
}

.status-badge.other {
  background: #e2e3e5;
  color: #383d41;
}

.no-data-row {
  border: none;
}

.no-data {
  padding: 40px 20px;
  text-align: center;
  border: none !important;
}

.no-data-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.no-data-icon {
  font-size: 48px;
  opacity: 0.5;
}

.no-data-text {
  font-size: 18px;
  font-weight: 600;
  color: #6c757d;
}

.no-data-hint {
  font-size: 14px;
  color: #9ca3af;
}

@media (max-width: 768px) {
  .logs-table {
    font-size: 12px;
  }
  
  th, td {
    padding: 8px;
  }
}

@media (prefers-color-scheme: dark) {
  .logs-container {
    background: #2c3e50;
    border-color: #34495e;
  }
  
  table {
    background: #2c3e50;
  }
  
  thead {
    background: #34495e;
  }
  
  th {
    color: #ecf0f1;
  }
  
  td {
    color: #bdc3c7;
    border-bottom-color: #34495e;
  }
  
  .path {
    color: #95a5a6;
  }
  
  .no-logs {
    color: #95a5a6;
  }
}
</style>