<template>
  <Modal
    :show="show"
    title="添加API密钥"
    @close="handleClose"
    size="lg"
  >
    <form @submit.prevent="handleSubmit" class="api-key-form">
      <div class="form-group">
        <label for="keys">API密钥</label>
        <textarea
          id="keys"
          v-model="keysText"
          placeholder="请输入API密钥，支持多个密钥（每行一个）&#10;密钥格式：AIza...&#10;&#10;示例：&#10;AIzaSyExample1234567890&#10;AIzaSyExample0987654321"
          :class="['keys-textarea', { 'error': error }]"
          rows="8"
          required
        />
        <div v-if="error" class="error-message">{{ error }}</div>
        <div class="form-hint">
          <p>支持批量添加，每行一个密钥。系统会自动验证AIza开头的密钥格式。</p>
          <a href="https://makersuite.google.com/app/apikey" target="_blank" rel="noopener noreferrer">
            获取API密钥 →
          </a>
        </div>
      </div>

      <div class="preview-section" v-if="validKeys.length > 0">
        <h4>检测到的密钥（{{ validKeys.length }}个）:</h4>
        <ul class="keys-preview">
          <li v-for="(key, index) in validKeys" :key="index" class="key-item">
            <span class="key-preview">{{ maskKey(key) }}</span>
            <span class="key-status">✓ 有效</span>
          </li>
        </ul>
      </div>

      <div class="form-actions">
        <Button
          type="button"
          variant="outline"
          @click="handleClose"
        >
          取消
        </Button>
        <Button
          type="submit"
          variant="primary"
          :loading="loading"
          :disabled="validKeys.length === 0"
        >
          添加 {{ validKeys.length }} 个密钥
        </Button>
      </div>
    </form>
  </Modal>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useLoading } from '@/composables/useLoading'
import { maskApiKey } from '@/utils/helpers'
import Modal from '@/components/ui/Modal.vue'
import Button from '@/components/ui/Button.vue'

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['close', 'submit'])

const { loading, execute } = useLoading()

const keysText = ref('')
const error = ref('')

// 验证API密钥格式
const isValidApiKey = (key) => {
  return key.startsWith('AIza') && key.length >= 20 && /^[A-Za-z0-9_-]+$/.test(key)
}

// 掩码显示密钥
const maskKey = (key) => {
  return maskApiKey(key)
}

// 提取有效密钥
const validKeys = computed(() => {
  if (!keysText.value.trim()) return []
  
  const lines = keysText.value.split('\n')
  const keys = []
  
  lines.forEach(line => {
    const trimmed = line.trim()
    if (trimmed && isValidApiKey(trimmed)) {
      keys.push(trimmed)
    }
  })
  
  return [...new Set(keys)] // 去重
})

// 监听输入变化，更新错误状态
watch(keysText, (newText) => {
  error.value = ''
  if (newText.trim()) {
    const lines = newText.split('\n')
    const invalidLines = lines.filter(line => {
      const trimmed = line.trim()
      return trimmed && !isValidApiKey(trimmed)
    })
    
    if (invalidLines.length > 0) {
      error.value = `检测到${invalidLines.length}个无效密钥格式，请确保密钥以AIza开头`
    }
  }
})

// 监听显示状态，重置表单
watch(() => props.show, (show) => {
  if (!show) {
    keysText.value = ''
    error.value = ''
  }
})

const handleSubmit = async () => {
  if (validKeys.value.length === 0) {
    error.value = '请输入至少一个有效的API密钥'
    return
  }

  await execute(async () => {
    emit('submit', validKeys.value)
  })
}

const handleClose = () => {
  emit('close')
}
</script>

<style scoped>
.api-key-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-weight: 500;
  color: var(--color-text);
}

.keys-textarea {
  width: 100%;
  min-height: 200px;
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  line-height: 1.4;
  background-color: var(--color-background);
  color: var(--color-text);
  resize: vertical;
}

.keys-textarea:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.keys-textarea.error {
  border-color: var(--color-danger);
}

.error-message {
  color: var(--color-danger);
  font-size: 0.875rem;
  margin-top: 0.25rem;
}

.form-hint {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.form-hint a {
  color: var(--color-primary);
  text-decoration: none;
}

.form-hint a:hover {
  text-decoration: underline;
}

.preview-section {
  margin-top: 1rem;
  padding: 1rem;
  background-color: var(--color-background-secondary);
  border-radius: 0.5rem;
}

.preview-section h4 {
  margin: 0 0 0.75rem 0;
  color: var(--color-text);
  font-size: 1rem;
}

.keys-preview {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.key-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background-color: var(--color-background);
  border-radius: 0.25rem;
  border: 1px solid var(--color-border);
}

.key-preview {
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  color: var(--color-text);
}

.key-status {
  color: var(--color-success);
  font-size: 0.8rem;
  font-weight: 500;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border);
}

@media (max-width: 768px) {
  .form-actions {
    flex-direction: column-reverse;
  }
}
</style>