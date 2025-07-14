<template>
  <div class="home-page">
    <!-- Quick Actions -->
    <div class="quick-actions">
      <Button
        variant="primary"
        icon="add"
        @click="$router.push('/api-keys')"
        class="quick-action-btn"
      >
        添加 API 密钥
      </Button>
      <Button
        variant="outline"
        icon="logs"
        @click="$router.push('/logs')"
        class="quick-action-btn"
      >
        查看日志
      </Button>
      <Button
        variant="outline"
        icon="settings"
        @click="$router.push('/settings')"
        class="quick-action-btn"
      >
        系统设置
      </Button>
    </div>

    <!-- Statistics Cards -->
    <div class="stats-section">
      <h2 class="section-title">系统统计</h2>
      <div class="stats-grid">
        <StatCard
          title="API 密钥"
          :value="apiKeysStore.keys.length"
          icon="key"
          color="primary"
          :trend="{
            value: activeKeysCount,
            label: '活跃密钥',
            type: 'neutral'
          }"
        />
        
        <StatCard
          title="总请求数"
          :value="logsStore.stats?.totalRequests || 0"
          icon="logs"
          color="success"
          :loading="logsStore.loading"
        />
        
        <StatCard
          title="平均响应时间"
          :value="Math.round(logsStore.stats?.avgResponseTime || 0)"
          suffix="ms"
          icon="refresh"
          color="info"
          :loading="logsStore.loading"
        />
        
        <StatCard
          title="服务地址"
          value="127.0.0.1:5675"
          icon="check"
          color="success"
          :trend="{
            value: '运行中',
            label: '状态',
            type: 'neutral'
          }"
        />
      </div>
    </div>

    <!-- Service Information -->
    <div class="service-section">
      <h2 class="section-title">服务信息</h2>
      <div class="service-grid">
        <Card class="service-card" title="API 端点">
          <div class="endpoint-list">
            <div 
              v-for="endpoint in apiEndpoints" 
              :key="endpoint.path"
              class="endpoint-item"
            >
              <div class="endpoint-method" :class="endpoint.method.toLowerCase()">
                {{ endpoint.method }}
              </div>
              <div class="endpoint-path">{{ endpoint.path }}</div>
              <div class="endpoint-description">{{ endpoint.description }}</div>
            </div>
          </div>
        </Card>

        <Card class="service-card" title="推荐模型">
          <div class="model-list">
            <div 
              v-for="model in recommendedModels" 
              :key="model.name"
              class="model-item"
            >
              <div class="model-info">
                <h4 class="model-name">{{ model.name }}</h4>
                <p class="model-description">{{ model.description }}</p>
              </div>
              <div class="model-badge" :class="model.type">
                {{ model.label }}
              </div>
            </div>
          </div>
        </Card>

        <Card class="service-card" title="系统信息">
          <div class="system-info">
            <div class="info-item">
              <Icon name="info" size="16" />
              <span class="info-label">版本转换:</span>
              <span class="info-value">v1 → v1beta</span>
            </div>
            <div class="info-item">
              <Icon name="check" size="16" />
              <span class="info-label">流式支持:</span>
              <span class="info-value">Server-Sent Events</span>
            </div>
            <div class="info-item">
              <Icon name="key" size="16" />
              <span class="info-label">密钥轮换:</span>
              <span class="info-value">自动负载均衡</span>
            </div>
          </div>
        </Card>
      </div>
    </div>

    <!-- Recent Activity -->
    <div class="activity-section" v-if="recentLogs.length > 0">
      <h2 class="section-title">
        最近活动
        <Button
          variant="ghost"
          size="sm"
          icon="arrow-right"
          @click="$router.push('/logs')"
          class="view-all-btn"
        >
          查看全部
        </Button>
      </h2>
      <Card class="activity-card">
        <div class="activity-list">
          <div 
            v-for="log in recentLogs.slice(0, 5)" 
            :key="log.id"
            class="activity-item"
          >
            <div class="activity-icon" :class="getStatusClass(log.statusCode)">
              <Icon :name="getStatusIcon(log.statusCode)" size="16" />
            </div>
            <div class="activity-content">
              <div class="activity-main">
                <span class="activity-method">{{ log.method }}</span>
                <span class="activity-path">{{ log.path }}</span>
                <span class="activity-status" :class="getStatusClass(log.statusCode)">
                  {{ log.statusCode }}
                </span>
              </div>
              <div class="activity-meta">
                <span class="activity-time">{{ formatTime(log.createdAt) }}</span>
                <span class="activity-duration">{{ log.responseTimeMs }}ms</span>
                <span class="activity-key">{{ log.apiKeyName }}</span>
              </div>
            </div>
          </div>
        </div>
      </Card>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted, ref } from 'vue'
import { useApiKeysStore } from '../stores/apiKeys'
import { useLogsStore } from '../stores/logs'
import Button from '../components/ui/Button.vue'
import Card from '../components/ui/Card.vue'
import Icon from '../components/ui/Icon.vue'
import StatCard from '../components/ui/StatCard.vue'

const apiKeysStore = useApiKeysStore()
const logsStore = useLogsStore()

const activeKeysCount = computed(() => {
  return apiKeysStore.keys.filter(key => key.isActive).length
})

const recentLogs = computed(() => {
  return logsStore.logs.slice(0, 5)
})

const apiEndpoints = ref([
  {
    method: 'POST',
    path: '/v1/chat/completions',
    description: '聊天完成接口 (兼容 OpenAI)'
  },
  {
    method: 'POST',
    path: '/v1/completions',
    description: '文本完成接口'
  },
  {
    method: 'GET',
    path: '/v1/models',
    description: '获取可用模型列表'
  },
  {
    method: 'GET',
    path: '/health',
    description: '健康检查接口'
  }
])

const recommendedModels = ref([
  {
    name: 'gemini-1.5-pro',
    description: '最新的 Gemini Pro 模型，支持长文本和多模态',
    type: 'recommended',
    label: '推荐'
  },
  {
    name: 'gemini-1.5-flash',
    description: '快速响应的轻量级模型，适合日常对话',
    type: 'fast',
    label: '快速'
  },
  {
    name: 'gemini-pro',
    description: '经典的 Gemini Pro 模型，平衡性能与质量',
    type: 'stable',
    label: '稳定'
  }
])

const getStatusClass = (statusCode) => {
  if (statusCode >= 200 && statusCode < 300) return 'success'
  if (statusCode >= 400 && statusCode < 500) return 'warning'
  if (statusCode >= 500) return 'danger'
  return 'info'
}

const getStatusIcon = (statusCode) => {
  if (statusCode >= 200 && statusCode < 300) return 'check'
  if (statusCode >= 400 && statusCode < 500) return 'warning'
  if (statusCode >= 500) return 'error'
  return 'info'
}

const formatTime = (timestamp) => {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

onMounted(async () => {
  await apiKeysStore.fetchApiKeys()
  await logsStore.fetchStats()
  await logsStore.fetchLogs(5)
})
</script>

<style scoped>
.home-page {
  padding: var(--spacing-6);
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-8);
}

/* Quick Actions */
.quick-actions {
  display: flex;
  gap: var(--spacing-4);
  flex-wrap: wrap;
}

.quick-action-btn {
  flex: 1;
  min-width: 160px;
}

/* Section Titles */
.section-title {
  font-size: var(--text-2xl);
  font-weight: var(--font-bold);
  color: var(--color-text);
  margin: 0 0 var(--spacing-6) 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: var(--spacing-4);
}

.view-all-btn {
  flex-shrink: 0;
}

/* Statistics Section */
.stats-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: var(--spacing-4);
}

/* Service Section */
.service-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.service-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: var(--spacing-6);
}

.service-card {
  height: 100%;
}

/* API Endpoints */
.endpoint-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
}

.endpoint-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  padding: var(--spacing-3);
  background-color: var(--color-surface-secondary);
  border-radius: var(--radius-md);
  flex-wrap: wrap;
}

.endpoint-method {
  font-size: var(--text-xs);
  font-weight: var(--font-bold);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-sm);
  text-transform: uppercase;
  flex-shrink: 0;
}

.endpoint-method.post {
  background-color: var(--color-success);
  color: white;
}

.endpoint-method.get {
  background-color: var(--color-info);
  color: white;
}

.endpoint-path {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text);
  flex-shrink: 0;
}

.endpoint-description {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  flex: 1;
  min-width: 0;
}

/* Models */
.model-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
}

.model-item {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-3);
  padding: var(--spacing-4);
  background-color: var(--color-surface-secondary);
  border-radius: var(--radius-md);
}

.model-info {
  flex: 1;
  min-width: 0;
}

.model-name {
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0 0 var(--spacing-1) 0;
  font-family: var(--font-mono);
}

.model-description {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.4;
}

.model-badge {
  font-size: var(--text-xs);
  font-weight: var(--font-semibold);
  padding: var(--spacing-1) var(--spacing-3);
  border-radius: var(--radius-full);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  flex-shrink: 0;
}

.model-badge.recommended {
  background-color: var(--color-primary);
  color: white;
}

.model-badge.fast {
  background-color: var(--color-success);
  color: white;
}

.model-badge.stable {
  background-color: var(--color-secondary);
  color: white;
}

/* System Info */
.system-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
}

.info-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  padding: var(--spacing-3);
  background-color: var(--color-surface-secondary);
  border-radius: var(--radius-md);
}

.info-label {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text-secondary);
  min-width: 80px;
}

.info-value {
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  font-family: var(--font-mono);
}

/* Activity Section */
.activity-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.activity-card {
  overflow: hidden;
}

.activity-list {
  display: flex;
  flex-direction: column;
}

.activity-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-4);
  padding: var(--spacing-4);
  border-bottom: 1px solid var(--color-border);
  transition: all var(--transition-fast);
}

.activity-item:last-child {
  border-bottom: none;
}

.activity-item:hover {
  background-color: var(--color-surface-secondary);
}

.activity-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-full);
  flex-shrink: 0;
}

.activity-icon.success {
  background-color: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

.activity-icon.warning {
  background-color: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
}

.activity-icon.danger {
  background-color: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
}

.activity-icon.info {
  background-color: rgba(var(--color-info-rgb), 0.1);
  color: var(--color-info);
}

.activity-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1);
}

.activity-main {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  flex-wrap: wrap;
}

.activity-method {
  font-size: var(--text-xs);
  font-weight: var(--font-bold);
  padding: var(--spacing-1) var(--spacing-2);
  background-color: var(--color-info);
  color: white;
  border-radius: var(--radius-sm);
  text-transform: uppercase;
}

.activity-path {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--color-text);
  font-weight: var(--font-medium);
}

.activity-status {
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-sm);
}

.activity-status.success {
  background-color: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

.activity-status.warning {
  background-color: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
}

.activity-status.danger {
  background-color: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
}

.activity-meta {
  display: flex;
  align-items: center;
  gap: var(--spacing-4);
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  flex-wrap: wrap;
}

.activity-time {
  font-family: var(--font-mono);
}

.activity-duration {
  font-family: var(--font-mono);
  font-weight: var(--font-medium);
}

.activity-key {
  font-family: var(--font-mono);
  background-color: var(--color-surface-secondary);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-sm);
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .home-page {
    padding: var(--spacing-4);
    gap: var(--spacing-6);
  }
  
  .quick-actions {
    flex-direction: column;
  }
  
  .quick-action-btn {
    width: 100%;
    min-width: unset;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: var(--spacing-3);
  }
  
  .service-grid {
    grid-template-columns: 1fr;
    gap: var(--spacing-4);
  }
  
  .section-title {
    font-size: var(--text-xl);
    margin-bottom: var(--spacing-4);
  }
  
  .endpoint-item {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-2);
  }
  
  .model-item {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-3);
  }
  
  .model-badge {
    align-self: flex-start;
  }
  
  .activity-main {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-2);
  }
  
  .activity-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-2);
  }
}

/* Tablet optimizations */
@media (max-width: 768px) {
  .stats-grid {
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  }
  
  .service-grid {
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  }
}
</style>