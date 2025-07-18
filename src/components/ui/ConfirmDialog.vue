<template>
  <Modal
    :show="show"
    :title="title"
    @close="$emit('cancel')"
    size="sm"
    :close-on-overlay="true"
  >
    <div class="confirm-dialog">
      <div class="confirm-content">
        <Icon :name="icon" :size="48" :color="iconColor" />
        <p class="confirm-message">{{ message }}</p>
      </div>
      
      <div class="confirm-actions">
        <Button
          variant="outline"
          @click="$emit('cancel')"
        >
          {{ cancelText }}
        </Button>
        <Button
          :variant="confirmVariant"
          @click="$emit('confirm')"
          :loading="loading"
        >
          {{ confirmText }}
        </Button>
      </div>
    </div>
  </Modal>
</template>

<script setup>
import { computed } from 'vue'
import Modal from './Modal.vue'
import Button from './Button.vue'
import Icon from './Icon.vue'

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  },
  title: {
    type: String,
    default: '确认操作'
  },
  message: {
    type: String,
    required: true
  },
  confirmText: {
    type: String,
    default: '确认'
  },
  cancelText: {
    type: String,
    default: '取消'
  },
  confirmVariant: {
    type: String,
    default: 'primary'
  },
  loading: {
    type: Boolean,
    default: false
  },
  icon: {
    type: String,
    default: 'warning'
  }
})

defineEmits(['confirm', 'cancel'])

const iconColor = computed(() => {
  const colors = {
    danger: 'var(--color-danger)',
    warning: 'var(--color-warning)',
    primary: 'var(--color-primary)',
    success: 'var(--color-success)',
    info: 'var(--color-info)'
  }
  return colors[props.confirmVariant] || colors.warning
})
</script>

<style scoped>
.confirm-dialog {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.confirm-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  text-align: center;
  padding: 1rem 0;
}

.confirm-message {
  margin: 0;
  color: var(--color-text);
  line-height: 1.5;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border);
}

@media (max-width: 768px) {
  .confirm-actions {
    flex-direction: column-reverse;
  }
}
</style>