<template>
  <div class="mobile-settings">
    <!-- 密码设置 -->
    <div class="settings-section">
      <div class="section-header">
        <div class="section-title">
          <Icon name="key" size="20" />
          <h3>密码设置</h3>
        </div>
        <p class="section-desc">修改登录密码保护系统安全</p>
      </div>
      
      <form @submit.prevent="handlePasswordChange" class="form-compact">
        <div class="form-group">
          <label>当前密码</label>
          <input
            v-model="passwordForm.currentPassword"
            type="password"
            placeholder="请输入当前密码"
            class="form-input"
            required
          />
        </div>

        <div class="form-group">
          <label>新密码</label>
          <input
            v-model="passwordForm.newPassword"
            type="password"
            placeholder="请输入新密码"
            class="form-input"
            minlength="6"
            required
          />
          <span class="form-hint">密码长度至少6位</span>
        </div>

        <div class="form-group">
          <label>确认新密码</label>
          <input
            v-model="passwordForm.confirmPassword"
            type="password"
            placeholder="请再次输入新密码"
            class="form-input"
            :class="{ 'error': passwordError }"
            required
          />
          <span v-if="passwordError" class="form-error">{{ passwordError }}</span>
        </div>

        <div class="form-actions">
          <button 
            type="submit" 
            :disabled="loading || !isPasswordFormValid"
            class="btn-primary"
          >
            {{ loading ? '修改中...' : '修改密码' }}
          </button>
        </div>
        
        <div v-if="error" class="message error">{{ error }}</div>
        <div v-if="successMessage" class="message success">{{ successMessage }}</div>
      </form>
    </div>

    <!-- API 访问控制 -->
    <div class="settings-section">
      <div class="section-header">
        <div class="section-title">
          <Icon name="settings" size="20" />
          <h3>API 访问控制</h3>
        </div>
        <p class="section-desc">API访问控制是强制性的，所有API请求都必须提供有效的验证密钥</p>
      </div>

      <div class="auth-status">
        <div class="status-indicator active">
          <div class="status-dot"></div>
          <span>验证已启用</span>
        </div>
        <div class="default-key-info">
          <small>默认密钥: <code>123456</code></small>
        </div>
      </div>

      <form @submit.prevent="handleCustomKeySubmit" class="form-compact">
        <div class="form-group">
          <label>自定义验证密钥</label>
          <input
            v-model="customKeyForm.key"
            type="password"
            placeholder="输入您的自定义验证密钥"
            class="form-input"
            :disabled="customKeyLoading"
          />
          <span class="form-hint">此密钥用于验证API请求</span>
        </div>

        <div class="form-actions">
          <button 
            type="submit" 
            :disabled="customKeyLoading || !customKeyForm.key.trim()"
            class="btn-primary"
          >
            {{ customKeyLoading ? '设置中...' : (hasCustomKey ? '更新' : '设置') }}
          </button>
          <button 
            type="button" 
            @click="handleClearCustomKey"
            :disabled="customKeyLoading"
            class="btn-danger"
          >
            重置为默认
          </button>
        </div>
        
        <div v-if="customKeyError" class="message error">{{ customKeyError }}</div>
        <div v-if="customKeySuccess" class="message success">{{ customKeySuccess }}</div>
      </form>
    </div>

    <!-- 错误重试设置 -->
    <div class="settings-section">
      <div class="section-header">
        <div class="section-title">
          <Icon name="refresh" size="20" />
          <h3>错误重试设置</h3>
        </div>
        <p class="section-desc">配置当API请求失败时的重试次数和行为</p>
      </div>

      <form @submit.prevent="handleRetrySubmit" class="form-compact">
        <div class="form-group">
          <label>重试次数</label>
          <input
            v-model.number="retryForm.count"
            type="number"
            min="1"
            placeholder="输入重试次数"
            class="form-input"
            :disabled="retryLoading"
            required
          />
          <span class="form-hint">设置API请求失败时的重试次数 (最少1次，默认3次，可设置为无限次)</span>
        </div>

        <div class="form-actions">
          <button 
            type="submit" 
            :disabled="retryLoading || retryForm.count < 1"
            class="btn-primary"
          >
            {{ retryLoading ? '保存中...' : '保存设置' }}
          </button>
        </div>
        
        <div v-if="retryError" class="message error">{{ retryError }}</div>
        <div v-if="retrySuccess" class="message success">{{ retrySuccess }}</div>
      </form>

      <div class="retry-info">
        <h4>重试机制说明</h4>
        <ul>
          <li>当API密钥返回错误时，系统会自动切换到下一个可用密钥进行重试</li>
          <li>支持流式和非流式请求的重试</li>
          <li>重试间隔采用指数退避策略，避免过于频繁的请求</li>
          <li>如果API密钥返回401或403错误，该密钥会被标记为失效</li>
          <li>重试次数达到限制后，请求会返回最后一次的错误信息</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import Icon from '@/components/ui/Icon.vue'

const props = defineProps({
  passwordForm: Object,
  customKeyForm: Object,
  retryForm: Object,
  loading: Boolean,
  customKeyLoading: Boolean,
  retryLoading: Boolean,
  error: String,
  successMessage: String,
  customKeyError: String,
  customKeySuccess: String,
  retryError: String,
  retrySuccess: String,
  hasCustomKey: Boolean
})

const emit = defineEmits([
  'password-change',
  'custom-key-submit', 
  'clear-custom-key',
  'retry-submit',
  'logout'
])

const passwordError = computed(() => {
  if (!props.passwordForm.newPassword || !props.passwordForm.confirmPassword) return null
  if (props.passwordForm.newPassword !== props.passwordForm.confirmPassword) {
    return '两次输入的密码不一致'
  }
  return null
})

const isPasswordFormValid = computed(() => {
  return props.passwordForm.currentPassword &&
         props.passwordForm.newPassword &&
         props.passwordForm.confirmPassword &&
         props.passwordForm.newPassword === props.passwordForm.confirmPassword &&
         props.passwordForm.newPassword.length >= 6
})

const handlePasswordChange = () => {
  emit('password-change')
}

const handleCustomKeySubmit = () => {
  emit('custom-key-submit')
}

const handleClearCustomKey = () => {
  emit('clear-custom-key')
}

const handleRetrySubmit = () => {
  emit('retry-submit')
}

const handleLogout = () => {
  emit('logout')
}
</script>

<style scoped>
.mobile-settings {
  padding: 1rem;
  max-width: 100%;
}

.settings-section {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  padding: 1rem;
  margin-bottom: 1rem;
}

.section-header {
  margin-bottom: 1rem;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.section-title h3 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
}

.section-desc {
  margin: 0;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  line-height: 1.4;
}

.form-compact {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.form-group label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
}

.form-input {
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  background: var(--color-surface-secondary);
  color: var(--color-text);
  font-size: 1rem;
  transition: border-color 0.2s;
}

.form-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.form-input.error {
  border-color: var(--color-danger);
}

.form-hint {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.form-error {
  font-size: 0.75rem;
  color: var(--color-danger);
}

.form-actions {
  display: flex;
  gap: 0.75rem;
  margin-top: 0.5rem;
}

.btn-primary {
  flex: 1;
  padding: 0.75rem;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 0.375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-danger {
  padding: 0.75rem 1rem;
  background: var(--color-danger);
  color: white;
  border: none;
  border-radius: 0.375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn-danger:hover:not(:disabled) {
  background: var(--color-danger-hover);
}

.btn-danger:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-danger.full-width {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.message {
  padding: 0.75rem;
  border-radius: 0.375rem;
  font-size: 0.875rem;
  margin-top: 0.5rem;
}

.message.error {
  background: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
  border: 1px solid rgba(var(--color-danger-rgb), 0.2);
}

.message.success {
  background: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
  border: 1px solid rgba(var(--color-success-rgb), 0.2);
}

.auth-status {
  padding: 0.75rem;
  background: var(--color-surface-secondary);
  border-radius: 0.375rem;
  margin-bottom: 1rem;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.status-indicator.active {
  color: var(--color-success);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-text-secondary);
}

.status-indicator.active .status-dot {
  background: var(--color-success);
}

.default-key-info {
  margin-top: 0.5rem;
  padding: 0.5rem;
  background: var(--color-surface);
  border-radius: 0.25rem;
  border: 1px solid var(--color-border);
}

.default-key-info small {
  color: var(--color-text-secondary);
  font-size: 0.75rem;
}

.default-key-info code {
  background: var(--color-primary);
  color: white;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-family: monospace;
  font-size: 0.75rem;
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: var(--color-surface-secondary);
  border-radius: 0.375rem;
}

.info-label {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.info-value {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
}

.info-value.status-active {
  color: var(--color-success);
}

.danger-actions {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.retry-info {
  padding: 0.75rem;
  background: var(--color-surface-secondary);
  border-radius: 0.375rem;
  border-left: 4px solid var(--color-primary);
  margin-top: 1rem;
}

.retry-info h4 {
  margin: 0 0 0.5rem 0;
  font-size: 0.875rem;
  color: var(--color-text);
}

.retry-info ul {
  margin: 0;
  padding-left: 1rem;
  color: var(--color-text-secondary);
  font-size: 0.75rem;
}

.retry-info li {
  margin-bottom: 0.375rem;
}

@media (max-width: 480px) {
  .mobile-settings {
    padding: 0.75rem;
  }
  
  .settings-section {
    padding: 0.75rem;
  }
  
  .form-actions {
    flex-direction: column;
  }
}
</style>