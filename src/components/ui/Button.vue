<template>
  <button
    :class="buttonClasses"
    :disabled="disabled || loading"
    @click="handleClick"
    :type="type"
  >
    <div class="button-content">
      <svg
        v-if="loading"
        class="loading-spinner"
        viewBox="0 0 24 24"
        fill="none"
      >
        <circle
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-dasharray="31.416"
          stroke-dashoffset="31.416"
        >
          <animate
            attributeName="stroke-dasharray"
            dur="2s"
            values="0 31.416;15.708 15.708;0 31.416"
            repeatCount="indefinite"
          />
          <animate
            attributeName="stroke-dashoffset"
            dur="2s"
            values="0;-15.708;-31.416"
            repeatCount="indefinite"
          />
        </circle>
      </svg>
      
      <Icon v-if="!loading && icon" :name="icon" class="button-icon" />
      
      <span v-if="$slots.default" class="button-text">
        <slot />
      </span>
    </div>
  </button>
</template>

<script setup>
import { computed } from 'vue'
import Icon from './Icon.vue'

const props = defineProps({
  variant: {
    type: String,
    default: 'primary',
    validator: (value) => ['primary', 'secondary', 'outline', 'ghost', 'danger', 'success'].includes(value)
  },
  size: {
    type: String,
    default: 'md',
    validator: (value) => ['xs', 'sm', 'md', 'lg', 'xl'].includes(value)
  },
  disabled: {
    type: Boolean,
    default: false
  },
  loading: {
    type: Boolean,
    default: false
  },
  icon: {
    type: String,
    default: null
  },
  fullWidth: {
    type: Boolean,
    default: false
  },
  type: {
    type: String,
    default: 'button',
    validator: (value) => ['button', 'submit', 'reset'].includes(value)
  }
})

const emit = defineEmits(['click'])

const buttonClasses = computed(() => [
  'btn',
  `btn--${props.variant}`,
  `btn--${props.size}`,
  {
    'btn--full-width': props.fullWidth,
    'btn--loading': props.loading,
    'btn--disabled': props.disabled
  }
])

const handleClick = (event) => {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>

<style scoped>
.btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: 1px solid transparent;
  border-radius: 0.5rem;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  outline: none;
  text-decoration: none;
  white-space: nowrap;
  user-select: none;
}

.btn:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

/* Sizes */
.btn--xs {
  height: 2rem;
  padding: 0 0.75rem;
  font-size: 0.75rem;
  line-height: 1rem;
}

.btn--sm {
  height: 2.25rem;
  padding: 0 1rem;
  font-size: 0.875rem;
  line-height: 1.25rem;
}

.btn--md {
  height: 2.5rem;
  padding: 0 1.25rem;
  font-size: 0.875rem;
  line-height: 1.25rem;
}

.btn--lg {
  height: 2.75rem;
  padding: 0 1.5rem;
  font-size: 1rem;
  line-height: 1.5rem;
}

.btn--xl {
  height: 3rem;
  padding: 0 2rem;
  font-size: 1.125rem;
  line-height: 1.75rem;
}

/* Variants */
.btn--primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-color: transparent;
}

.btn--primary:hover:not(.btn--disabled):not(.btn--loading) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn--secondary {
  background-color: var(--color-surface);
  color: var(--color-text);
  border-color: var(--color-border);
}

.btn--secondary:hover:not(.btn--disabled):not(.btn--loading) {
  background-color: var(--color-surface-hover);
  border-color: var(--color-border-hover);
}

.btn--outline {
  background-color: transparent;
  color: var(--color-primary);
  border-color: var(--color-primary);
}

.btn--outline:hover:not(.btn--disabled):not(.btn--loading) {
  background-color: var(--color-primary);
  color: white;
}

.btn--ghost {
  background-color: transparent;
  color: var(--color-text);
  border-color: transparent;
}

.btn--ghost:hover:not(.btn--disabled):not(.btn--loading) {
  background-color: var(--color-surface-hover);
}

.btn--danger {
  background-color: var(--color-danger);
  color: white;
  border-color: transparent;
}

.btn--danger:hover:not(.btn--disabled):not(.btn--loading) {
  background-color: var(--color-danger-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.4);
}

.btn--success {
  background-color: var(--color-success);
  color: white;
  border-color: transparent;
}

.btn--success:hover:not(.btn--disabled):not(.btn--loading) {
  background-color: var(--color-success-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(34, 197, 94, 0.4);
}

/* States */
.btn--disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.btn--loading {
  cursor: not-allowed;
}

.btn--full-width {
  width: 100%;
}

/* Content */
.button-content {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.loading-spinner {
  width: 1rem;
  height: 1rem;
}

.button-icon {
  width: 1rem;
  height: 1rem;
}

.button-text {
  white-space: nowrap;
}

/* Responsive */
@media (max-width: 640px) {
  .btn {
    min-height: 44px; /* iOS touch target */
  }
  
  .btn--xs { min-height: 36px; }
  .btn--sm { min-height: 40px; }
  .btn--md { min-height: 44px; }
  .btn--lg { min-height: 48px; }
  .btn--xl { min-height: 52px; }
}
</style>