<template>
  <div class="error-state">
    <Card variant="danger" class="error-card">
      <div class="error-content">
        <Icon name="error" size="32" />
        <div class="error-text">
          <h3>{{ title }}</h3>
          <p>{{ message }}</p>
        </div>
        <Button
          variant="outline"
          size="sm"
          @click="$emit('retry')"
          icon="refresh"
          v-if="showRetry"
        >
          重试
        </Button>
      </div>
    </Card>
  </div>
</template>

<script setup>
import Card from './Card.vue'
import Icon from './Icon.vue'
import Button from './Button.vue'

defineProps({
  title: {
    type: String,
    default: '加载失败'
  },
  message: {
    type: String,
    required: true
  },
  showRetry: {
    type: Boolean,
    default: true
  }
})

defineEmits(['retry'])
</script>

<style scoped>
.error-state {
  display: flex;
  justify-content: center;
  padding: 2rem 0;
}

.error-card {
  max-width: 500px;
  width: 100%;
}

.error-content {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
}

.error-text {
  flex: 1;
}

.error-text h3 {
  margin: 0 0 0.5rem 0;
  color: var(--color-text);
}

.error-text p {
  margin: 0;
  color: var(--color-text-secondary);
}

@media (max-width: 768px) {
  .error-content {
    flex-direction: column;
    text-align: center;
  }
}
</style>