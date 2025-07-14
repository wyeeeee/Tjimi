<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="show" class="modal-overlay" @click="handleOverlayClick">
        <div 
          class="modal-container"
          :class="modalClasses"
          role="dialog"
          aria-modal="true"
          :aria-labelledby="title ? 'modal-title' : undefined"
          @click.stop
        >
          <div class="modal-header" v-if="title || $slots.header">
            <slot name="header">
              <h2 id="modal-title" class="modal-title">{{ title }}</h2>
            </slot>
            <Button
              v-if="closable"
              variant="ghost"
              size="sm"
              icon="close"
              @click="close"
              class="modal-close"
              aria-label="关闭对话框"
            />
          </div>
          
          <div class="modal-body">
            <slot />
          </div>
          
          <div class="modal-footer" v-if="$slots.footer">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { computed, watch, nextTick } from 'vue'
import Button from './Button.vue'

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  },
  title: {
    type: String,
    default: null
  },
  size: {
    type: String,
    default: 'md',
    validator: (value) => ['sm', 'md', 'lg', 'xl', 'full'].includes(value)
  },
  closable: {
    type: Boolean,
    default: true
  },
  closeOnOverlay: {
    type: Boolean,
    default: true
  }
})

const emit = defineEmits(['close', 'update:show'])

const modalClasses = computed(() => [
  'modal',
  `modal--${props.size}`
])

const close = () => {
  emit('close')
  emit('update:show', false)
}

const handleOverlayClick = () => {
  if (props.closeOnOverlay) {
    close()
  }
}

// Handle escape key
let escapeHandler = null

watch(() => props.show, (show, oldShow) => {
  if (show && !oldShow) {
    // Modal is opening
    nextTick(() => {
      escapeHandler = (event) => {
        if (event.key === 'Escape' && props.closable) {
          close()
        }
      }
      document.addEventListener('keydown', escapeHandler)
    })
  } else if (!show && oldShow) {
    // Modal is closing
    if (escapeHandler) {
      document.removeEventListener('keydown', escapeHandler)
      escapeHandler = null
    }
  }
})

// Cleanup on unmount
import { onUnmounted } from 'vue'
onUnmounted(() => {
  if (escapeHandler) {
    document.removeEventListener('keydown', escapeHandler)
  }
})
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  padding: var(--spacing-4);
}

.modal-container {
  background-color: var(--color-surface);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-2xl);
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  position: relative;
}

.modal--sm {
  width: 100%;
  max-width: 400px;
}

.modal--md {
  width: 100%;
  max-width: 500px;
}

.modal--lg {
  width: 100%;
  max-width: 700px;
}

.modal--xl {
  width: 100%;
  max-width: 900px;
}

.modal--full {
  width: 100%;
  height: 100%;
  max-width: none;
  max-height: none;
  border-radius: 0;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-6);
  border-bottom: 1px solid var(--color-border);
  gap: var(--spacing-4);
}

.modal-title {
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
  flex: 1;
  min-width: 0;
}

.modal-close {
  flex-shrink: 0;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-6);
}

.modal-footer {
  padding: var(--spacing-6);
  border-top: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-3);
  flex-wrap: wrap;
}

/* Transition animations */
.modal-enter-active,
.modal-leave-active {
  transition: all var(--transition-normal);
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: scale(0.9) translateY(-20px);
}

.modal-enter-to .modal-container,
.modal-leave-from .modal-container {
  transform: scale(1) translateY(0);
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .modal-overlay {
    padding: var(--spacing-2);
  }
  
  .modal-container {
    max-height: 95vh;
  }
  
  .modal--sm,
  .modal--md,
  .modal--lg,
  .modal--xl {
    width: 100%;
    max-width: none;
  }
  
  .modal-header {
    padding: var(--spacing-4);
  }
  
  .modal-body {
    padding: var(--spacing-4);
  }
  
  .modal-footer {
    padding: var(--spacing-4);
    flex-direction: column-reverse;
    gap: var(--spacing-2);
  }
  
  .modal-footer :deep(.button) {
    width: 100%;
  }
  
  .modal-title {
    font-size: var(--text-lg);
  }
}

/* Dark mode enhancements */
@media (prefers-color-scheme: dark) {
  .modal-overlay {
    background-color: rgba(0, 0, 0, 0.8);
  }
}

/* Focus management */
.modal-container:focus {
  outline: none;
}

/* Accessibility improvements */
@media (prefers-reduced-motion: reduce) {
  .modal-enter-active,
  .modal-leave-active {
    transition-duration: 0.2s;
  }
  
  .modal-enter-from .modal-container,
  .modal-leave-to .modal-container {
    transform: scale(1) translateY(0);
  }
}
</style>