<template>
  <div class="mobile-stats">
    <div class="stats-grid">
      <div class="stat-card" v-for="stat in stats" :key="stat.key">
        <div class="stat-icon">
          <Icon :name="stat.icon" size="20" />
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ stat.value }}</div>
          <div class="stat-label">{{ stat.label }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import Icon from '@/components/ui/Icon.vue'

const props = defineProps({
  apiKeys: {
    type: Array,
    default: () => []
  }
})

const stats = computed(() => {
  const total = props.apiKeys.length
  const active = props.apiKeys.filter(key => key.isActive).length
  const totalRequests = props.apiKeys.reduce((sum, key) => sum + (key.usageCount || 0), 0)
  
  return [
    {
      key: 'total',
      label: '总密钥',
      value: total,
      icon: 'key'
    },
    {
      key: 'active',
      label: '活跃',
      value: active,
      icon: 'check'
    },
    {
      key: 'requests',
      label: '请求数',
      value: totalRequests > 999 ? `${Math.floor(totalRequests / 1000)}K` : totalRequests,
      icon: 'trending-up'
    }
  ]
})
</script>

<style scoped>
.mobile-stats {
  padding: 0.75rem;
  background: var(--color-surface);
  border-radius: 0.5rem;
  margin-bottom: 1rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 0.75rem;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem;
  background: var(--color-surface-secondary);
  border-radius: 0.375rem;
  border: 1px solid var(--color-border);
}

.stat-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: var(--color-primary);
  color: white;
  border-radius: 0.375rem;
  flex-shrink: 0;
}

.stat-content {
  flex: 1;
  min-width: 0;
}

.stat-value {
  font-size: 1.25rem;
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
  .stats-grid {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }
  
  .stat-card {
    padding: 0.5rem;
  }
  
  .stat-value {
    font-size: 1.125rem;
  }
}
</style>