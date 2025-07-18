<template>
  <div class="mobile-api-key-card" :class="{ 'inactive': !apiKey.isActive }">
    <div class="card-header">
      <div class="key-info">
        <div class="key-name">{{ apiKey.name || '密钥' }}</div>
        <div class="key-status" :class="statusClass">
          <div class="status-dot"></div>
          <span>{{ apiKey.isActive ? '正常' : '禁用' }}</span>
        </div>
      </div>
      <div class="key-actions">
        <button
          @click="toggleEnabled"
          :disabled="loading"
          class="action-btn"
          :class="{ 'active': apiKey.isActive }"
        >
          <Icon :name="apiKey.isActive ? 'pause' : 'play'" size="16" />
        </button>
        <button
          @click="$emit('delete', apiKey)"
          class="action-btn danger"
        >
          <Icon name="delete" size="16" />
        </button>
      </div>
    </div>
    
    <div class="key-display">
      <div class="key-value" @click="toggleKeyVisibility">
        <code>{{ showFullKey ? apiKey.keyValue : maskApiKey(apiKey.keyValue) }}</code>
        <Icon :name="showFullKey ? 'eye-off' : 'eye'" size="16" />
      </div>
      <button
        v-if="showFullKey"
        @click="copyKey"
        class="copy-btn"
      >
        <Icon name="copy" size="16" />
      </button>
    </div>
    
    <div class="key-stats">
      <div class="stat-item">
        <span class="stat-value">{{ formatNumber(apiKey.usageCount || 0) }}</span>
        <span class="stat-label">请求</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{{ formatDate(apiKey.lastUsed) }}</span>
        <span class="stat-label">最后使用</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useLoading } from '@/composables/useLoading'
import { maskApiKey, copyToClipboard, formatDate, formatNumber } from '@/utils/helpers'
import Icon from '@/components/ui/Icon.vue'

const props = defineProps({
  apiKey: {
    type: Object,
    required: true
  }
})

const emit = defineEmits(['delete', 'toggle-enabled'])

const { loading, execute } = useLoading()
const showFullKey = ref(false)

const statusClass = computed(() => ({
  'status-active': props.apiKey.isActive,
  'status-inactive': !props.apiKey.isActive
}))

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
.mobile-api-key-card {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  padding: 1rem;
  margin-bottom: 0.75rem;
  transition: opacity 0.2s;
}

.mobile-api-key-card.inactive {
  opacity: 0.6;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 0.75rem;
}

.key-info {
  flex: 1;
}

.key-name {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: 0.25rem;
}

.key-status {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
  font-weight: 500;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-text-secondary);
}

.status-active {
  color: var(--color-success);
}

.status-active .status-dot {
  background: var(--color-success);
}

.status-inactive {
  color: var(--color-warning);
}

.status-inactive .status-dot {
  background: var(--color-warning);
}

.key-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  background: var(--color-surface-secondary);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background: var(--color-surface-hover);
  color: var(--color-text);
}

.action-btn.active {
  background: var(--color-success);
  color: white;
  border-color: var(--color-success);
}

.action-btn.danger {
  border-color: var(--color-danger);
  color: var(--color-danger);
}

.action-btn.danger:hover {
  background: var(--color-danger);
  color: white;
}

.key-display {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.75rem;
}

.key-value {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  background: var(--color-surface-secondary);
  border-radius: 0.375rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.key-value:hover {
  background: var(--color-surface-hover);
}

.key-value code {
  flex: 1;
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--color-text);
  word-break: break-all;
}

.copy-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  background: var(--color-surface-secondary);
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.copy-btn:hover {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.key-stats {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--color-border);
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.stat-value {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text);
  line-height: 1.2;
}

.stat-label {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  line-height: 1.2;
}

@media (max-width: 480px) {
  .mobile-api-key-card {
    padding: 0.75rem;
  }
  
  .key-stats {
    gap: 0.75rem;
  }
  
  .stat-value {
    font-size: 0.875rem;
  }
}
</style>