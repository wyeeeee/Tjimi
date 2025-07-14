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

      <div v-if="authStore.isDefaultPassword" class="default-password-alert">
        <div class="alert-icon">⚠️</div>
        <div class="alert-content">
          <h3>安全提醒</h3>
          <p>您当前使用的是默认密码，为了保护系统安全，请尽快修改密码。</p>
        </div>
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

    <div class="settings-section">
      <div class="section-header">
        <h2>🚪 会话管理</h2>
        <p class="section-description">管理您的登录会话</p>
      </div>

      <div class="session-actions">
        <button 
          @click="handleLogout"
          class="btn-danger"
        >
          退出登录
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const passwordForm = ref({
  currentPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const successMessage = ref('')

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
</script>

<style scoped>
.settings {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.settings-header {
  text-align: center;
  margin-bottom: 40px;
}

.settings-header h1 {
  color: #2c3e50;
  font-size: 32px;
  font-weight: 600;
  margin-bottom: 8px;
}

.settings-subtitle {
  color: #7f8c8d;
  font-size: 16px;
  margin: 0;
}

.settings-section {
  background: white;
  border-radius: 12px;
  padding: 32px;
  margin-bottom: 32px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
  border: 1px solid #e1e8ed;
}

.section-header {
  margin-bottom: 24px;
}

.section-header h2 {
  color: #2c3e50;
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 8px;
}

.section-description {
  color: #7f8c8d;
  font-size: 14px;
  margin: 0;
}

.default-password-alert {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px;
  background: #fff3cd;
  border: 1px solid #ffeaa7;
  border-radius: 8px;
  margin-bottom: 24px;
}

.alert-icon {
  font-size: 24px;
  flex-shrink: 0;
}

.alert-content h3 {
  color: #856404;
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 8px 0;
}

.alert-content p {
  color: #856404;
  font-size: 14px;
  margin: 0;
  line-height: 1.5;
}

.password-form {
  max-width: 500px;
}

.form-group {
  margin-bottom: 24px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  color: #2c3e50;
  font-weight: 500;
  font-size: 14px;
}

.form-input {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid #e1e8ed;
  border-radius: 8px;
  font-size: 16px;
  transition: all 0.3s ease;
  background: #f8f9fa;
}

.form-input:focus {
  outline: none;
  border-color: #667eea;
  background: white;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.form-input.error {
  border-color: #e74c3c;
}

.form-hint {
  display: block;
  margin-top: 4px;
  color: #7f8c8d;
  font-size: 12px;
}

.form-error {
  display: block;
  margin-top: 4px;
  color: #e74c3c;
  font-size: 12px;
}

.form-actions {
  display: flex;
  gap: 12px;
  margin-top: 32px;
}

.btn-primary {
  padding: 12px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(102, 126, 234, 0.3);
}

.btn-primary:disabled {
  background: #bdc3c7;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.btn-secondary {
  padding: 12px 24px;
  background: #f8f9fa;
  color: #495057;
  border: 2px solid #e1e8ed;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.btn-secondary:hover {
  background: #e9ecef;
  border-color: #ced4da;
}

.btn-danger {
  padding: 12px 24px;
  background: #e74c3c;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.btn-danger:hover {
  background: #c0392b;
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(231, 76, 60, 0.3);
}

.error-message {
  margin-top: 16px;
  padding: 12px;
  background: #fdf2f2;
  color: #e74c3c;
  border-radius: 6px;
  border-left: 4px solid #e74c3c;
  font-size: 14px;
}

.success-message {
  margin-top: 16px;
  padding: 12px;
  background: #d4edda;
  color: #155724;
  border-radius: 6px;
  border-left: 4px solid #28a745;
  font-size: 14px;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.info-item {
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e1e8ed;
}

.info-label {
  font-size: 12px;
  color: #7f8c8d;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.info-value {
  font-size: 14px;
  color: #2c3e50;
  font-weight: 500;
}

.status-active {
  color: #28a745 !important;
}

.session-actions {
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  .settings-header h1 {
    color: #ecf0f1;
  }
  
  .settings-subtitle {
    color: #95a5a6;
  }
  
  .settings-section {
    background: #2c3e50;
    border-color: #34495e;
  }
  
  .section-header h2 {
    color: #ecf0f1;
  }
  
  .section-description {
    color: #95a5a6;
  }
  
  .default-password-alert {
    background: #2c2416;
    border-color: #f39c12;
  }
  
  .alert-content h3,
  .alert-content p {
    color: #f39c12;
  }
  
  .form-group label {
    color: #bdc3c7;
  }
  
  .form-input {
    background: #34495e;
    border-color: #4a5f7a;
    color: #ecf0f1;
  }
  
  .form-input:focus {
    background: #34495e;
    border-color: #667eea;
  }
  
  .form-hint {
    color: #95a5a6;
  }
  
  .btn-secondary {
    background: #34495e;
    color: #ecf0f1;
    border-color: #4a5f7a;
  }
  
  .btn-secondary:hover {
    background: #4a5f7a;
    border-color: #5d6d7e;
  }
  
  .error-message {
    background: #2c1810;
    color: #f1c40f;
    border-left-color: #f39c12;
  }
  
  .success-message {
    background: #1e2f23;
    color: #27ae60;
    border-left-color: #27ae60;
  }
  
  .info-item {
    background: #34495e;
    border-color: #4a5f7a;
  }
  
  .info-label {
    color: #95a5a6;
  }
  
  .info-value {
    color: #ecf0f1;
  }
}

@media (max-width: 768px) {
  .settings {
    padding: 16px;
  }
  
  .settings-section {
    padding: 24px;
  }
  
  .form-actions {
    flex-direction: column;
  }
  
  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>