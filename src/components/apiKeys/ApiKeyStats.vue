<template>
  <div class="api-key-stats">
    <div class="stats-grid">
      <StatCard
        title="总密钥数"
        :value="stats.total"
        icon="key"
        color="primary"
      />
      <StatCard
        title="活跃密钥"
        :value="stats.active"
        icon="check"
        color="success"
      />
      <StatCard
        title="今日请求"
        :value="formatNumber(stats.todayRequests)"
        icon="trending-up"
        color="info"
      />
      <StatCard
        title="总请求数"
        :value="formatNumber(stats.totalRequests)"
        icon="activity"
        color="warning"
      />
    </div>
    
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { formatNumber } from '@/utils/helpers'
import StatCard from '@/components/ui/StatCard.vue'
import Card from '@/components/ui/Card.vue'

const props = defineProps({
  apiKeys: {
    type: Array,
    default: () => []
  }
})

const stats = computed(() => {
  const total = props.apiKeys.length
  const active = props.apiKeys.filter(key => key.isActive).length
  const todayRequests = props.apiKeys.reduce((sum, key) => sum + (key.usageCount || 0), 0)
  const totalRequests = props.apiKeys.reduce((sum, key) => sum + (key.usageCount || 0), 0)
  
  return {
    total,
    active,
    todayRequests,
    totalRequests
  }
})
</script>

<style scoped>
.api-key-stats {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}


@media (max-width: 768px) {
  .stats-grid {
    grid-template-columns: 1fr 1fr;
  }
}

@media (max-width: 480px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }
}
</style>