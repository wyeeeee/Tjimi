<template>
  <div class="loading-state">
    <Card class="loading-card">
      <div class="loading-content">
        <Icon name="loading" size="32" />
        <h3>加载中...</h3>
        <p>{{ message }}</p>
      </div>
    </Card>
  </div>
</template>

<script setup>
import Card from './Card.vue'
import Icon from './Icon.vue'

defineProps({
  message: {
    type: String,
    default: '正在加载数据'
  }
})
</script>

<style scoped>
.loading-state {
  display: flex;
  justify-content: center;
  padding: 2rem 0;
}

.loading-card {
  max-width: 400px;
  width: 100%;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  text-align: center;
  padding: 2rem;
}

.loading-content h3 {
  margin: 0;
  color: var(--color-text);
}

.loading-content p {
  margin: 0;
  color: var(--color-text-secondary);
}
</style>