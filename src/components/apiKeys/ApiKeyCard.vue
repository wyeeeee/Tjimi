<template>
  <Card class="api-key-card" :class="{ 'api-key-card--inactive': !apiKey.isActive }">
    <div class="api-key-header">
      <div class="api-key-info">
        <div class="api-key-name">
          <h3>{{ apiKey.name || `密钥 #${apiKey.id}` }}</h3>
          <span class="api-key-status" :class="`status--${apiKey.isActive ? 'active' : 'inactive'}`">
            {{ apiKey.isActive ? '正常' : '已禁用' }}
          </span>
        </div>
        <div class="api-key-value">
          <code class="api-key-code">{{ showFullKey ? apiKey.keyValue : maskApiKey(apiKey.keyValue) }}</code>
          <Button
            variant="ghost"
            size="sm"
            icon="eye"
            @click="toggleKeyVisibility"
            :title="showFullKey ? '隐藏' : '显示'"
            class="toggle-btn"
          />
          <Button
            v-if="showFullKey"
            variant="ghost"
            size="sm"
            icon="copy"
            @click="copyKey"
            title="复制完整密钥"
            class="copy-btn"
          />
        </div>
      </div>
      
      <div class="api-key-actions">
        <Button
          variant="outline"
          size="sm"
          @click="toggleEnabled"
          :loading="loading"
          :icon="apiKey.isActive ? 'pause' : 'play'"
        >
          {{ apiKey.isActive ? '禁用' : '启用' }}
        </Button>
        <Button
          variant="danger"
          size="sm"
          @click="$emit('delete', apiKey)"
          icon="delete"
        >
          删除
        </Button>
      </div>
    </div>

    <div class="api-key-stats">
      <div class="stat-item">
        <span class="stat-label">今日请求</span>
        <span class="stat-value">{{ formatNumber(apiKey.usageCount || 0) }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">总请求</span>
        <span class="stat-value">{{ formatNumber(apiKey.usageCount || 0) }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">最后使用</span>
        <span class="stat-value">{{ formatDate(apiKey.lastUsed) }}</span>
      </div>
    </div>

  </Card>
</template>

<script setup>
import { ref } from 'vue'
import { useLoading } from '@/composables/useLoading'
import { maskApiKey, copyToClipboard, formatDate, formatNumber } from '@/utils/helpers'
import { API_KEY_STATUS_LABELS } from '@/utils/constants'
import Card from '@/components/ui/Card.vue'
import Button from '@/components/ui/Button.vue'

const props = defineProps({
  apiKey: {
    type: Object,
    required: true
  }
})

const emit = defineEmits(['delete', 'toggle-enabled'])

const { loading, execute } = useLoading()
const showFullKey = ref(false)


const toggleKeyVisibility = () => {
  showFullKey.value = !showFullKey.value
}

const copyKey = async () => {
  await copyToClipboard(props.apiKey.keyValue)
}

const toggleEnabled = async () => {
  await execute(() => {
    emit('toggle-enabled', props.apiKey)
  })
}
</script>

<style scoped>
.api-key-card {
  transition: all 0.3s ease;
}

.api-key-card--inactive {
  opacity: 0.6;
}

.api-key-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.api-key-info {
  flex: 1;
}

.api-key-name {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.api-key-name h3 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
}

.api-key-status {
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.75rem;
  font-weight: 500;
  text-transform: uppercase;
}

.status--active {
  background-color: var(--color-success-light);
  color: var(--color-success);
}

.status--inactive {
  background-color: var(--color-warning-light);
  color: var(--color-warning);
}

.status--error {
  background-color: var(--color-danger-light);
  color: var(--color-danger);
}

.api-key-value {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.api-key-code {
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  background-color: var(--color-background-secondary);
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  word-break: break-all;
  max-width: 280px;
}

.toggle-btn,
.copy-btn {
  flex-shrink: 0;
}

.api-key-actions {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.api-key-stats {
  display: flex;
  gap: 1rem;
  padding: 1rem 0;
  border-top: 1px solid var(--color-border);
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.stat-label {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
}

.stat-value {
  font-weight: 600;
  color: var(--color-text);
}


@media (max-width: 768px) {
  .api-key-header {
    flex-direction: column;
    gap: 1rem;
  }
  
  .api-key-actions {
    width: 100%;
    justify-content: flex-end;
  }
  
  .api-key-stats {
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>