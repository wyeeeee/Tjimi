<template>
  <div class="settings">
    <div class="settings-header">
      <h1>系统设置</h1>
      <p class="settings-subtitle">管理您的系统配置和安全设置</p>
    </div>

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
        <p class="section-description">设置自定义验证秘钥以保护API访问</p>
      </div>

      <div class="custom-auth-section">
        <div class="auth-status">
          <div class="status-indicator" :class="{ 'active': hasCustomKey }">
            <div class="status-dot"></div>
            <span>{{ hasCustomKey ? '已启用自定义验证' : '未设置自定义验证' }}</span>
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
              v-if="hasCustomKey"
              type="button" 
              @click="handleClearCustomKey"
              :disabled="customKeyLoading"
              class="btn-danger"
            >
              {{ customKeyLoading ? '清除中...' : '清除秘钥' }}
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
            <li>设置后，所有API请求都需要在Header中包含: <code>Authorization: Bearer your-custom-key</code></li>
            <li>建议使用复杂的秘钥以确保安全性</li>
            <li>可以随时更新或清除秘钥</li>
          </ul>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <div class="section-header">
        <h2>ℹ️ 系统信息</h2>
        <p class="section-description">当前系统的基本信息</p>
      </div>

      <div class="info-grid">
        <div class="info-item">
          <div class="info-label">系统类型</div>
          <div class="info-value">单用户管理系统</div>
        </div>
        <div class="info-item">
          <div class="info-label">API 服务器</div>
          <div class="info-value">http://127.0.0.1:5675</div>
        </div>
        <div class="info-item">
          <div class="info-label">支持的 API</div>
          <div class="info-value">Gemini API v1beta</div>
        </div>
        <div class="info-item">
          <div class="info-label">会话状态</div>
          <div class="info-value status-active">已认证</div>
        </div>
      </div>
    </div>

    <!-- Session Management Section -->
    <Card class="settings-section">
      <div class="section-header">
        <div class="section-title">
          <Icon name="logout" size="24" />
          <h2>会话管理</h2>
        </div>
        <p class="section-description">管理您的登录会话和安全设置</p>
      </div>

      <div class="danger-zone">
        <div class="danger-zone-header">
          <Icon name="warning" size="20" />
          <h3>危险操作</h3>
        </div>
        <p class="danger-zone-description">
          退出登录后您需要重新输入密码才能访问系统。
        </p>
        <Button
          variant="danger"
          icon="logout"
          @click="handleLogout"
          size="lg"
        >
          退出登录
        </Button>
      </div>
    </Card>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const authStore = useAuthStore()

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

onMounted(() => {
  checkCustomKey()
})
</script>

<style scoped>
.settings-page {
  padding: var(--spacing-6);
  max-width: 900px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-8);
}

/* Page Header */
.page-header {
  text-align: center;
  padding: var(--spacing-8) 0;
}

.header-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4);
}

.page-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  font-size: var(--text-4xl);
  font-weight: var(--font-bold);
  color: var(--color-text);
  margin: 0;
}

.page-description {
  font-size: var(--text-lg);
  color: var(--color-text-secondary);
  margin: 0;
  text-align: center;
  max-width: 600px;
  line-height: 1.5;
}

/* Settings Section */
.settings-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.section-header {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-3);
}

.section-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
}

.section-title h2 {
  font-size: var(--text-2xl);
  font-weight: var(--font-bold);
  color: var(--color-text);
  margin: 0;
}

.section-description {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* Alert Card */
.alert-card {
  border: none;
}

.alert-content {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-4);
}

.alert-icon {
  flex-shrink: 0;
  margin-top: var(--spacing-1);
}

.alert-text {
  flex: 1;
  min-width: 0;
}

.alert-title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  margin: 0 0 var(--spacing-2) 0;
}

.alert-message {
  font-size: var(--text-base);
  margin: 0;
  line-height: 1.5;
  opacity: 0.9;
}

/* Password Form */
.password-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
  max-width: 600px;
}

.form-grid {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-5);
}

.form-group {
  width: 100%;
}

.form-actions {
  display: flex;
  gap: var(--spacing-4);
  padding-top: var(--spacing-2);
}

/* Message Cards */
.message-card {
  border: none;
}

.message-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  font-size: var(--text-base);
  font-weight: var(--font-medium);
}

/* Info Grid */
.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--spacing-5);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-3);
  padding: var(--spacing-5);
  background-color: var(--color-surface-secondary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
  transition: all var(--transition-normal);
}

.info-item:hover {
  background-color: var(--color-surface-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.info-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
}

.info-label {
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.info-value {
  font-size: var(--text-base);
  font-weight: var(--font-medium);
  color: var(--color-text);
  line-height: 1.4;
}

.info-value--mono {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  background-color: var(--color-surface);
  padding: var(--spacing-2) var(--spacing-3);
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border);
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-1);
  padding: var(--spacing-1) var(--spacing-3);
  border-radius: var(--radius-full);
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.status-badge--active {
  background-color: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

/* Danger Zone */
.danger-zone {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
  padding: var(--spacing-6);
  background-color: rgba(var(--color-danger-rgb), 0.05);
  border: 2px solid rgba(var(--color-danger-rgb), 0.1);
  border-radius: var(--radius-lg);
}

.danger-zone-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
}

.danger-zone-header h3 {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-danger);
  margin: 0;
}

.danger-zone-description {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .settings-page {
    padding: var(--spacing-4);
    gap: var(--spacing-6);
  }
  
  .page-header {
    padding: var(--spacing-6) 0;
  }
  
  .page-title {
    font-size: var(--text-3xl);
    flex-direction: column;
    text-align: center;
  }
  
  .page-description {
    font-size: var(--text-base);
  }
  
  .section-title {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-2);
  }
  
  .section-title h2 {
    font-size: var(--text-xl);
  }
  
  .form-actions {
    flex-direction: column;
  }
  
  .info-grid {
    grid-template-columns: 1fr;
    gap: var(--spacing-4);
  }
  
  .info-item {
    padding: var(--spacing-4);
  }
  
  .danger-zone {
    padding: var(--spacing-4);
  }
}

/* Tablet optimizations */
@media (max-width: 768px) {
  .info-grid {
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  }
  
  .password-form {
    max-width: none;
  }
}

/* Large screen optimizations */
@media (min-width: 1024px) {
  .settings-page {
    max-width: 1000px;
  }
  
  .info-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

/* Dark mode enhancements */
@media (prefers-color-scheme: dark) {
  .info-item:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }
  
  .danger-zone {
    background-color: rgba(var(--color-danger-rgb), 0.1);
    border-color: rgba(var(--color-danger-rgb), 0.2);
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .info-item,
  .danger-zone {
    border-width: 2px;
  }
  
  .status-badge {
    border: 1px solid currentColor;
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .info-item {
    transition: none;
  }
  
  .info-item:hover {
    transform: none;
  }
}

/* Focus management */
.info-item:focus-within {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

/* Custom Auth Section */
.custom-auth-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.auth-status {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  padding: var(--spacing-4);
  background-color: var(--color-surface-secondary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text-secondary);
}

.status-indicator.active {
  color: var(--color-success);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: var(--color-text-secondary);
}

.status-indicator.active .status-dot {
  background-color: var(--color-success);
  box-shadow: 0 0 0 2px rgba(var(--color-success-rgb), 0.2);
}

.custom-key-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
  max-width: 600px;
}

.usage-info {
  padding: var(--spacing-4);
  background-color: var(--color-surface-secondary);
  border-radius: var(--radius-lg);
  border-left: 4px solid var(--color-primary);
}

.usage-info h4 {
  margin: 0 0 var(--spacing-3) 0;
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  color: var(--color-text);
}

.usage-info ul {
  margin: 0;
  padding-left: var(--spacing-4);
  color: var(--color-text-secondary);
  font-size: var(--text-sm);
  line-height: 1.6;
}

.usage-info li {
  margin-bottom: var(--spacing-2);
}

.usage-info code {
  background-color: var(--color-surface);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--color-primary);
  border: 1px solid var(--color-border);
}

.btn-danger {
  background-color: var(--color-danger);
  color: var(--color-white);
  border: none;
  padding: var(--spacing-3) var(--spacing-4);
  border-radius: var(--radius-md);
  font-weight: var(--font-medium);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-danger:hover:not(:disabled) {
  background-color: var(--color-danger-hover);
}

.btn-danger:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>