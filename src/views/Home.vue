<template>
  <div class="home">
    <h1>Gemini API 代理管理面板</h1>
    
    <div class="stats-grid">
      <div class="stat-card">
        <h3>API 密钥数量</h3>
        <p class="stat-value">{{ apiKeysStore.keys.length }}</p>
      </div>
      
      <div class="stat-card">
        <h3>活跃密钥</h3>
        <p class="stat-value">{{ activeKeysCount }}</p>
      </div>
      
      <div class="stat-card" v-if="logsStore.stats">
        <h3>总请求数</h3>
        <p class="stat-value">{{ logsStore.stats.totalRequests }}</p>
      </div>
      
      <div class="stat-card" v-if="logsStore.stats">
        <h3>平均响应时间</h3>
        <p class="stat-value">{{ Math.round(logsStore.stats.avgResponseTime) }}ms</p>
      </div>
    </div>

    <div class="server-info">
      <h2>代理服务器信息</h2>
      <div class="info-card">
        <p><strong>服务器地址:</strong> http://127.0.0.1:5675</p>
        <p><strong>状态:</strong> <span class="status-active">运行中</span></p>
        <p><strong>支持的端点:</strong></p>
        <ul>
          <li>GET /v1/models - 获取模型列表</li>
          <li>GET /v1/models/{model} - 获取模型详情</li>
          <li>POST /v1/models/{model}/generateContent - 生成内容</li>
          <li>POST /v1/models/{model}/streamGenerateContent - 流式生成内容</li>
          <li>GET /health - 健康检查</li>
        </ul>
        <p><strong>推荐模型:</strong> gemini-1.5-flash, gemini-pro</p>
        <p><strong>注意:</strong> 请求会自动转换为 Gemini API v1beta 格式</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useApiKeysStore } from '../stores/apiKeys'
import { useLogsStore } from '../stores/logs'

const apiKeysStore = useApiKeysStore()
const logsStore = useLogsStore()

const activeKeysCount = computed(() => {
  return apiKeysStore.keys.filter(key => key.isActive).length
})

onMounted(async () => {
  await apiKeysStore.fetchApiKeys()
  await logsStore.fetchStats()
})
</script>

<style scoped>
.home {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin: 20px 0;
}

.stat-card {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 8px;
  text-align: center;
  border: 1px solid #e9ecef;
}

.stat-card h3 {
  margin: 0 0 10px 0;
  color: #495057;
  font-size: 14px;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #007bff;
  margin: 0;
}

.server-info {
  margin-top: 40px;
}

.info-card {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 8px;
  border: 1px solid #e9ecef;
}

.info-card p {
  margin: 10px 0;
}

.info-card ul {
  margin: 10px 0;
  padding-left: 20px;
}

.info-card li {
  margin: 5px 0;
  font-family: monospace;
  background: #e9ecef;
  padding: 2px 5px;
  border-radius: 3px;
  display: inline-block;
}

.status-active {
  color: #28a745;
  font-weight: bold;
}

@media (prefers-color-scheme: dark) {
  .stat-card, .info-card {
    background: #2c3e50;
    border-color: #34495e;
  }
  
  .stat-card h3 {
    color: #bdc3c7;
  }
  
  .info-card li {
    background: #34495e;
    color: #ecf0f1;
  }
}
</style>