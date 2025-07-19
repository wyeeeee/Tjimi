<template>
  <div class="settings">
    <!-- 移动端使用紧凑布局 -->
    <MobileSettings
      v-if="isMobile"
      :password-form="passwordForm"
      :custom-key-form="customKeyForm"
      :retry-form="retryForm"
      :loading="authStore.loading"
      :custom-key-loading="customKeyLoading"
      :retry-loading="retryLoading"
      :error="authStore.error"
      :success-message="successMessage"
      :custom-key-error="customKeyError"
      :custom-key-success="customKeySuccess"
      :retry-error="retryError"
      :retry-success="retrySuccess"
      :has-custom-key="hasCustomKey"
      @password-change="handlePasswordChange"
      @custom-key-submit="handleCustomKeySubmit"
      @clear-custom-key="handleClearCustomKey"
      @retry-submit="handleRetrySettingsSubmit"
      @logout="handleLogout"
    />

    <!-- 桌面端使用原有布局 -->
    <div v-else class="desktop-settings">
      <div class="settings-section">
        <div class="section-header">
          <h2>🔒 密码设置</h2>
          <p class="section-description">修改您的登录密码以保护系统安全</p>
        </div>

        <form @submit.prevent="handlePasswordChange" class="password-form">
          <div class="form-group">
            <label for="currentPassword">当前密码</label>
            <input
              id="currentPassword"
              v-model="passwordForm.currentPassword"
              type="password"
              required
              placeholder="请输入当前密码"
              class="form-input"
            />
          </div>

          <div class="form-group">
            <label for="newPassword">新密码</label>
            <input
              id="newPassword"
              v-model="passwordForm.newPassword"
              type="password"
              required
              placeholder="请输入新密码"
              class="form-input"
              minlength="6"
            />
            <small class="form-hint">密码长度至少6位</small>
          </div>

          <div class="form-group">
            <label for="confirmPassword">确认新密码</label>
            <input
              id="confirmPassword"
              v-model="passwordForm.confirmPassword"
              type="password"
              required
              placeholder="请再次输入新密码"
              class="form-input"
              :class="{ 'error': passwordForm.newPassword && passwordForm.confirmPassword && passwordForm.newPassword !== passwordForm.confirmPassword }"
            />
            <small v-if="passwordForm.newPassword && passwordForm.confirmPassword && passwordForm.newPassword !== passwordForm.confirmPassword" class="form-error">
              两次输入的密码不一致
            </small>
          </div>

          <div class="form-actions">
            <button 
              type="submit" 
              :disabled="authStore.loading || !isPasswordFormValid"
              class="btn-primary"
            >
              {{ authStore.loading ? '修改中...' : '修改密码' }}
            </button>

            <button 
              type="button" 
              @click="resetForm"
              class="btn-secondary"
            >
              重置
            </button>
          </div>

          <div v-if="authStore.error" class="error-message">
            {{ authStore.error }}
          </div>

          <div v-if="successMessage" class="success-message">
            {{ successMessage }}
          </div>
        </form>
      </div>

      <div class="settings-section">
        <div class="section-header">
          <h2>🔐 API 访问控制</h2>
          <p class="section-description">API访问控制是强制性的，所有API请求都必须提供有效的验证密钥</p>
        </div>

        <div class="custom-auth-section">
          <div class="auth-status">
            <div class="status-indicator active">
              <div class="status-dot"></div>
              <span>验证已启用</span>
            </div>
            <div class="default-key-info">
              <small>默认密钥: <code>123456</code></small>
            </div>
          </div>

          <form @submit.prevent="handleCustomKeySubmit" class="custom-key-form">
            <div class="form-group">
              <label for="customKey">自定义验证秘钥</label>
              <input
                id="customKey"
                v-model="customKeyForm.key"
                type="password"
                placeholder="输入您的自定义验证秘钥"
                class="form-input"
                :disabled="customKeyLoading"
              />
              <small class="form-hint">
                此秘钥用于验证API请求，请保管好您的秘钥
              </small>
            </div>

            <div class="form-actions">
              <button 
                type="submit" 
                :disabled="customKeyLoading || !customKeyForm.key.trim()"
                class="btn-primary"
              >
                {{ customKeyLoading ? '设置中...' : (hasCustomKey ? '更新秘钥' : '设置秘钥') }}
              </button>

              <button 
                type="button" 
                @click="handleClearCustomKey"
                :disabled="customKeyLoading"
                class="btn-danger"
              >
                {{ customKeyLoading ? '重置中...' : '重置为默认' }}
              </button>
            </div>

            <div v-if="customKeyError" class="error-message">
              {{ customKeyError }}
            </div>

            <div v-if="customKeySuccess" class="success-message">
              {{ customKeySuccess }}
            </div>
          </form>

          <div class="usage-info">
            <h4>使用说明</h4>
            <ul>
              <li>所有API请求都必须在Header中包含: <code>Authorization: Bearer your-custom-key</code></li>
              <li>默认密钥为 <code>123456</code>，建议修改为复杂的密钥以确保安全性</li>
              <li>可以随时更新密钥，或重置为默认密钥</li>
              <li>API访问控制现在是强制性的，无法完全关闭</li>
            </ul>
          </div>
        </div>
      </div>

      <div class="settings-section">
        <div class="section-header">
          <h2>🔄 错误重试设置</h2>
          <p class="section-description">配置当API请求失败时的重试次数和行为</p>
        </div>

        <form @submit.prevent="handleRetrySettingsSubmit" class="retry-form">
          <div class="form-group">
            <label for="retryCount">重试次数</label>
            <input
              id="retryCount"
              v-model.number="retryForm.count"
              type="number"
              min="1"
              required
              placeholder="输入重试次数"
              class="form-input"
              :disabled="retryLoading"
            />
            <small class="form-hint">
              设置API请求失败时的重试次数 (最少1次，默认3次，可设置为无限次)
            </small>
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

          <div v-if="retryError" class="error-message">
            {{ retryError }}
          </div>

          <div v-if="retrySuccess" class="success-message">
            {{ retrySuccess }}
          </div>
        </form>

        <div class="usage-info">
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
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useSettingsStore } from '../stores/settings'
import { useResponsive } from '@/composables/useResponsive'
import { invoke } from '@tauri-apps/api/core'
import MobileSettings from '@/components/mobile/MobileSettings.vue'

const router = useRouter()
const authStore = useAuthStore()
const settingsStore = useSettingsStore()
const { isMobile } = useResponsive()

const passwordForm = ref({
  currentPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const successMessage = ref('')

// 自定义验证秘钥相关
const customKeyForm = ref({
  key: ''
})
const customKeyLoading = ref(false)
const customKeyError = ref('')
const customKeySuccess = ref('')
const hasCustomKey = ref(false)

// 重试设置相关
const retryForm = ref({
  count: 3
})
const retryLoading = ref(false)
const retryError = ref('')
const retrySuccess = ref('')

const isPasswordFormValid = computed(() => {
  return passwordForm.value.currentPassword &&
         passwordForm.value.newPassword &&
         passwordForm.value.confirmPassword &&
         passwordForm.value.newPassword === passwordForm.value.confirmPassword &&
         passwordForm.value.newPassword.length >= 6
})

const resetForm = () => {
  passwordForm.value = {
    currentPassword: '',
    newPassword: '',
    confirmPassword: ''
  }
  authStore.error = null
  successMessage.value = ''
}

const handlePasswordChange = async () => {
  authStore.error = null
  successMessage.value = ''

  const success = await authStore.changePassword(
    passwordForm.value.currentPassword,
    passwordForm.value.newPassword
  )

  if (success) {
    successMessage.value = '密码修改成功！'
    resetForm()
    // 重新检查是否为默认密码
    await authStore.checkDefaultPassword()
  }
}

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
}

// 自定义验证秘钥相关函数
const checkCustomKey = async () => {
  try {
    hasCustomKey.value = await invoke('has_custom_auth_key')
  } catch (error) {
    console.error('检查自定义秘钥失败:', error)
  }
}

const handleCustomKeySubmit = async () => {
  if (!customKeyForm.value.key.trim()) {
    customKeyError.value = '请输入自定义验证秘钥'
    return
  }

  customKeyLoading.value = true
  customKeyError.value = ''
  customKeySuccess.value = ''

  try {
    await invoke('set_custom_auth_key', { key: customKeyForm.value.key })
    customKeySuccess.value = '自定义验证秘钥设置成功'
    customKeyForm.value.key = ''
    await checkCustomKey()
  } catch (error) {
    customKeyError.value = '设置失败: ' + error
  } finally {
    customKeyLoading.value = false
  }
}

const handleClearCustomKey = async () => {
  if (!confirm('确定要清除自定义验证秘钥吗？清除后API访问将不再受到保护。')) {
    return
  }

  customKeyLoading.value = true
  customKeyError.value = ''
  customKeySuccess.value = ''

  try {
    await invoke('clear_custom_auth_key')
    customKeySuccess.value = '自定义验证秘钥已清除'
    customKeyForm.value.key = ''
    await checkCustomKey()
  } catch (error) {
    customKeyError.value = '清除失败: ' + error
  } finally {
    customKeyLoading.value = false
  }
}

// 重试设置相关函数
const loadRetrySettings = async () => {
  try {
    await settingsStore.getRetryCount()
    retryForm.value.count = settingsStore.retryCount
  } catch (error) {
    console.error('加载重试设置失败:', error)
  }
}

const handleRetrySettingsSubmit = async () => {
  retryLoading.value = true
  retryError.value = ''
  retrySuccess.value = ''

  try {
    await settingsStore.setRetryCount(retryForm.value.count)
    retrySuccess.value = '重试设置保存成功'
  } catch (error) {
    retryError.value = '保存失败: ' + error
  } finally {
    retryLoading.value = false
  }
}

onMounted(() => {
  checkCustomKey()
  loadRetrySettings()
})
</script>

<style scoped>
.settings {
  padding: 2rem;
  max-width: 900px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.desktop-settings {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.settings-section {
  background: var(--color-surface);
  border-radius: 0.5rem;
  padding: 2rem;
  border: 1px solid var(--color-border);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.section-header {
  margin-bottom: 1.5rem;
}

.section-header h2 {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: 0.5rem;
}

.section-description {
  color: var(--color-text-secondary);
  font-size: 1rem;
  line-height: 1.5;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--color-text);
}

.form-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  background: var(--color-input-bg);
  color: var(--color-text);
  font-size: 1rem;
  transition: border-color 0.2s;
}

.form-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(var(--color-primary-rgb), 0.1);
}

.form-input.error {
  border-color: var(--color-danger);
}

.form-hint {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.form-error {
  color: var(--color-danger);
  font-size: 0.875rem;
  margin-top: 0.25rem;
}

.form-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1.5rem;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
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

.btn-secondary {
  background: var(--color-surface-secondary);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  padding: 0.75rem 1.5rem;
  border-radius: 0.375rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn-secondary:hover:not(:disabled) {
  background: var(--color-surface-hover);
}

.btn-danger {
  background: var(--color-danger);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
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

.error-message {
  background: rgba(var(--color-danger-rgb), 0.1);
  color: var(--color-danger);
  padding: 0.75rem;
  border-radius: 0.375rem;
  margin-top: 1rem;
  border: 1px solid rgba(var(--color-danger-rgb), 0.2);
}

.success-message {
  background: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
  padding: 0.75rem;
  border-radius: 0.375rem;
  margin-top: 1rem;
  border: 1px solid rgba(var(--color-success-rgb), 0.2);
}

.custom-auth-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.auth-status {
  padding: 1rem;
  background: var(--color-surface-secondary);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
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
  font-size: 0.875rem;
}

.default-key-info code {
  background: var(--color-primary);
  color: white;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-family: monospace;
  font-size: 0.875rem;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.info-item {
  padding: 1rem;
  background: var(--color-surface-secondary);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
}

.info-label {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  margin-bottom: 0.5rem;
}

.info-value {
  font-weight: 500;
  color: var(--color-text);
}

.status-active {
  color: var(--color-success);
}

.usage-info {
  padding: 1rem;
  background: var(--color-surface-secondary);
  border-radius: 0.5rem;
  border-left: 4px solid var(--color-primary);
}

.usage-info h4 {
  margin-bottom: 0.5rem;
  color: var(--color-text);
}

.usage-info ul {
  margin: 0;
  padding-left: 1.5rem;
  color: var(--color-text-secondary);
}

.usage-info li {
  margin-bottom: 0.5rem;
}

.usage-info code {
  background: var(--color-surface);
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--color-primary);
}

.danger-zone {
  padding: 1.5rem;
  background: rgba(var(--color-danger-rgb), 0.05);
  border: 2px solid rgba(var(--color-danger-rgb), 0.1);
  border-radius: 0.5rem;
}

.danger-zone-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.danger-zone-header h3 {
  color: var(--color-danger);
  margin: 0;
}

.danger-zone-description {
  color: var(--color-text-secondary);
  margin-bottom: 1rem;
}

@media (max-width: 768px) {
  .settings {
    padding: 1rem;
  }
  
  .settings-section {
    padding: 1rem;
  }
  
  .form-actions {
    flex-direction: column;
  }
  
  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>