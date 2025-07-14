<template>
  <Card class="stat-card" :class="`stat-card--${color}`" hoverable>
    <div class="stat-content">
      <div class="stat-header">
        <div class="stat-icon" :class="`stat-icon--${color}`">
          <Icon v-if="!loading" :name="icon" size="24" />
          <Icon v-else name="loading" size="24" />
        </div>
        <div class="stat-info">
          <h3 class="stat-title">{{ title }}</h3>
          <div class="stat-value-container">
            <span v-if="!loading" class="stat-value">
              {{ formattedValue }}
              <span v-if="suffix" class="stat-suffix">{{ suffix }}</span>
            </span>
            <div v-else class="stat-loading">
              <div class="loading-bar"></div>
            </div>
          </div>
        </div>
      </div>
      
      <div v-if="trend" class="stat-trend">
        <div class="trend-content" :class="`trend--${trend.type}`">
          <Icon v-if="trend.type === 'up'" name="arrow-up" size="14" />
          <Icon v-else-if="trend.type === 'down'" name="arrow-down" size="14" />
          <Icon v-else name="info" size="14" />
          <span class="trend-value">{{ trend.value }}</span>
          <span class="trend-label">{{ trend.label }}</span>
        </div>
      </div>
    </div>
  </Card>
</template>

<script setup>
import { computed } from 'vue'
import Card from './Card.vue'
import Icon from './Icon.vue'

const props = defineProps({
  title: {
    type: String,
    required: true
  },
  value: {
    type: [String, Number],
    required: true
  },
  suffix: {
    type: String,
    default: null
  },
  icon: {
    type: String,
    required: true
  },
  color: {
    type: String,
    default: 'primary',
    validator: (value) => ['primary', 'secondary', 'success', 'warning', 'danger', 'info'].includes(value)
  },
  loading: {
    type: Boolean,
    default: false
  },
  trend: {
    type: Object,
    default: null,
    validator: (value) => {
      if (!value) return true
      return value.hasOwnProperty('value') && 
             value.hasOwnProperty('label') && 
             value.hasOwnProperty('type') &&
             ['up', 'down', 'neutral'].includes(value.type)
    }
  }
})

const formattedValue = computed(() => {
  if (typeof props.value === 'string') {
    return props.value
  }
  
  if (typeof props.value === 'number') {
    // Format large numbers
    if (props.value >= 1000000) {
      return (props.value / 1000000).toFixed(1) + 'M'
    } else if (props.value >= 1000) {
      return (props.value / 1000).toFixed(1) + 'K'
    }
    return props.value.toLocaleString()
  }
  
  return props.value
})
</script>

<style scoped>
.stat-card {
  border: none;
  transition: all var(--transition-normal);
  position: relative;
  overflow: hidden;
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, var(--stat-color), var(--stat-color-light));
  opacity: 0.8;
}

.stat-card--primary {
  --stat-color: var(--color-primary);
  --stat-color-light: var(--color-primary);
  --stat-color-bg: rgba(var(--color-primary-rgb), 0.1);
}

.stat-card--secondary {
  --stat-color: var(--color-secondary);
  --stat-color-light: var(--color-secondary);
  --stat-color-bg: rgba(102, 126, 234, 0.1);
}

.stat-card--success {
  --stat-color: var(--color-success);
  --stat-color-light: var(--color-success);
  --stat-color-bg: rgba(var(--color-success-rgb), 0.1);
}

.stat-card--warning {
  --stat-color: var(--color-warning);
  --stat-color-light: var(--color-warning);
  --stat-color-bg: rgba(var(--color-warning-rgb), 0.1);
}

.stat-card--danger {
  --stat-color: var(--color-danger);
  --stat-color-light: var(--color-danger);
  --stat-color-bg: rgba(var(--color-danger-rgb), 0.1);
}

.stat-card--info {
  --stat-color: var(--color-info);
  --stat-color-light: var(--color-info);
  --stat-color-bg: rgba(var(--color-info-rgb), 0.1);
}

.stat-content {
  padding: var(--spacing-4);
}

.stat-header {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-3);
  margin-bottom: var(--spacing-3);
}

.stat-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: var(--radius-lg);
  background-color: var(--stat-color-bg);
  color: var(--stat-color);
  flex-shrink: 0;
  transition: all var(--transition-normal);
}

.stat-card:hover .stat-icon {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(var(--stat-color), 0.3);
}

.stat-info {
  flex: 1;
  min-width: 0;
}

.stat-title {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text-secondary);
  margin: 0 0 var(--spacing-2) 0;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value-container {
  position: relative;
  height: auto;
  display: flex;
  align-items: center;
  min-height: 2rem;
}

.stat-value {
  font-size: var(--text-2xl);
  font-weight: var(--font-bold);
  color: var(--color-text);
  line-height: 1.2;
  display: flex;
  align-items: baseline;
  gap: var(--spacing-1);
}

.stat-suffix {
  font-size: var(--text-lg);
  font-weight: var(--font-medium);
  color: var(--color-text-secondary);
}

.stat-loading {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
}

.loading-bar {
  width: 60%;
  height: 8px;
  background-color: var(--color-surface-secondary);
  border-radius: var(--radius-full);
  position: relative;
  overflow: hidden;
}

.loading-bar::after {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, 
    transparent, 
    var(--stat-color), 
    transparent);
  animation: loading-slide 1.5s ease-in-out infinite;
}

@keyframes loading-slide {
  0% { left: -100%; }
  100% { left: 100%; }
}

.stat-trend {
  padding-top: var(--spacing-2);
  border-top: 1px solid var(--color-border);
}

.trend-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  font-size: var(--text-sm);
  flex-wrap: wrap;
  min-width: 0;
}

.trend--up {
  color: var(--color-success);
}

.trend--down {
  color: var(--color-danger);
}

.trend--neutral {
  color: var(--color-text-secondary);
}

.trend-value {
  font-weight: var(--font-semibold);
  flex-shrink: 0;
  min-width: 0;
  word-break: break-all;
}

.trend-label {
  opacity: 0.8;
  flex-shrink: 0;
  white-space: nowrap;
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .stat-content {
    padding: var(--spacing-3);
  }
  
  .stat-header {
    gap: var(--spacing-2);
    margin-bottom: var(--spacing-2);
  }
  
  .stat-icon {
    width: 32px;
    height: 32px;
  }
  
  .stat-value {
    font-size: var(--text-lg);
  }
  
  .trend-content {
    font-size: var(--text-xs);
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-1);
  }
  
  .trend-value,
  .trend-label {
    white-space: normal;
    word-break: break-word;
  }
}

/* Dark mode enhancements */
@media (prefers-color-scheme: dark) {
  .stat-card::before {
    opacity: 0.6;
  }
  
  .stat-icon {
    background-color: rgba(var(--stat-color), 0.15);
  }
  
  .loading-bar {
    background-color: var(--color-surface-hover);
  }
}
</style>