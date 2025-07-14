<template>
  <div :class="cardClasses">
    <header v-if="$slots.header || title" class="card-header">
      <slot name="header">
        <h3 v-if="title" class="card-title">{{ title }}</h3>
        <p v-if="subtitle" class="card-subtitle">{{ subtitle }}</p>
      </slot>
      <div v-if="$slots.actions" class="card-actions">
        <slot name="actions" />
      </div>
    </header>
    
    <div v-if="$slots.default" class="card-content">
      <slot />
    </div>
    
    <footer v-if="$slots.footer" class="card-footer">
      <slot name="footer" />
    </footer>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  title: {
    type: String,
    default: null
  },
  subtitle: {
    type: String,
    default: null
  },
  variant: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'outlined', 'elevated', 'glass'].includes(value)
  },
  padding: {
    type: String,
    default: 'md',
    validator: (value) => ['none', 'sm', 'md', 'lg'].includes(value)
  },
  hoverable: {
    type: Boolean,
    default: false
  },
  clickable: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['click'])

const cardClasses = computed(() => [
  'card',
  `card--${props.variant}`,
  `card--padding-${props.padding}`,
  {
    'card--hoverable': props.hoverable,
    'card--clickable': props.clickable
  }
])

const handleClick = (event) => {
  if (props.clickable) {
    emit('click', event)
  }
}
</script>

<style scoped>
.card {
  position: relative;
  display: flex;
  flex-direction: column;
  background-color: var(--color-surface);
  border-radius: 0.75rem;
  overflow: hidden;
  transition: all 0.2s ease-in-out;
}

/* Variants */
.card--default {
  border: 1px solid var(--color-border);
}

.card--outlined {
  border: 1px solid var(--color-border);
  background-color: transparent;
}

.card--elevated {
  border: 1px solid var(--color-border);
  box-shadow: 
    0 1px 3px 0 rgba(0, 0, 0, 0.1),
    0 1px 2px 0 rgba(0, 0, 0, 0.06);
}

.card--glass {
  background-color: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

/* Padding */
.card--padding-none {
  padding: 0;
}

.card--padding-sm .card-header,
.card--padding-sm .card-content,
.card--padding-sm .card-footer {
  padding: 0.75rem;
}

.card--padding-md .card-header,
.card--padding-md .card-content,
.card--padding-md .card-footer {
  padding: 1.5rem;
}

.card--padding-lg .card-header,
.card--padding-lg .card-content,
.card--padding-lg .card-footer {
  padding: 2rem;
}

/* States */
.card--hoverable:hover {
  transform: translateY(-2px);
  box-shadow: 
    0 4px 6px -1px rgba(0, 0, 0, 0.1),
    0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.card--clickable {
  cursor: pointer;
}

.card--clickable:hover {
  transform: translateY(-1px);
  box-shadow: 
    0 4px 6px -1px rgba(0, 0, 0, 0.1),
    0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.card--clickable:active {
  transform: translateY(0);
}

/* Header */
.card-header {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  border-bottom: 1px solid var(--color-border);
}

.card-header:has(.card-actions) {
  flex-direction: row;
  align-items: flex-start;
  justify-content: space-between;
}

.card-title {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
  line-height: 1.5;
}

.card-subtitle {
  margin: 0;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  line-height: 1.4;
}

.card-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-shrink: 0;
}

/* Content */
.card-content {
  flex: 1;
  min-height: 0;
}

/* Footer */
.card-footer {
  border-top: 1px solid var(--color-border);
  background-color: var(--color-surface-secondary);
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .card--glass {
    background-color: rgba(0, 0, 0, 0.2);
    border-color: rgba(255, 255, 255, 0.1);
  }
}

/* Responsive */
@media (max-width: 640px) {
  .card-header:has(.card-actions) {
    flex-direction: column;
    align-items: stretch;
  }
  
  .card-actions {
    align-self: flex-end;
  }
}
</style>