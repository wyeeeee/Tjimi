<template>
  <div class="login-container">
    <div class="login-card">
      <div class="login-header">
        <h2>系统登录</h2>
        <p class="login-subtitle">Gemini API 管理系统</p>
      </div>
      
      <form @submit.prevent="handleSubmit">
        <div class="form-group">
          <label for="password">请输入密码</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            placeholder="默认密码: admin123"
            class="password-input"
          />
        </div>
        
        <button type="submit" :disabled="authStore.loading" class="login-btn">
          {{ authStore.loading ? '登录中...' : '登录' }}
        </button>
        
        <div v-if="authStore.error" class="error">
          {{ authStore.error }}
        </div>
      </form>
      
      <div v-if="authStore.isDefaultPassword && authStore.isAuthenticated" class="default-password-warning">
        <div class="warning-icon">⚠️</div>
        <div class="warning-text">
          <p>您正在使用默认密码</p>
          <p>建议尽快在设置中修改密码</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const password = ref('')

const handleSubmit = async () => {
  const success = await authStore.login(password.value)
  
  if (success) {
    router.push('/')
  }
}
</script>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  background: white;
  padding: 50px 40px;
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  width: 100%;
  max-width: 420px;
  backdrop-filter: blur(10px);
}

.login-header {
  text-align: center;
  margin-bottom: 40px;
}

.login-header h2 {
  color: #2c3e50;
  font-size: 28px;
  font-weight: 600;
  margin-bottom: 8px;
}

.login-subtitle {
  color: #7f8c8d;
  font-size: 14px;
  margin: 0;
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

.password-input {
  width: 100%;
  padding: 16px;
  border: 2px solid #e1e8ed;
  border-radius: 8px;
  font-size: 16px;
  transition: all 0.3s ease;
  background: #f8f9fa;
}

.password-input:focus {
  outline: none;
  border-color: #667eea;
  background: white;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.login-btn {
  width: 100%;
  padding: 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  margin-bottom: 16px;
}

.login-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
}

.login-btn:disabled {
  background: #bdc3c7;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.error {
  color: #e74c3c;
  margin-top: 16px;
  text-align: center;
  font-size: 14px;
  padding: 12px;
  background: #fdf2f2;
  border-radius: 6px;
  border-left: 4px solid #e74c3c;
}

.default-password-warning {
  margin-top: 20px;
  padding: 16px;
  background: #fff3cd;
  border: 1px solid #ffeaa7;
  border-radius: 8px;
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.warning-icon {
  font-size: 20px;
  flex-shrink: 0;
}

.warning-text {
  flex: 1;
}

.warning-text p {
  margin: 0;
  font-size: 13px;
  color: #856404;
  line-height: 1.4;
}

.warning-text p:first-child {
  font-weight: 600;
  margin-bottom: 4px;
}

@media (prefers-color-scheme: dark) {
  .login-card {
    background: #2c3e50;
    color: #ecf0f1;
  }
  
  .login-header h2 {
    color: #ecf0f1;
  }
  
  .login-subtitle {
    color: #95a5a6;
  }
  
  .form-group label {
    color: #bdc3c7;
  }
  
  .password-input {
    background: #34495e;
    border-color: #4a5f7a;
    color: #ecf0f1;
  }
  
  .password-input:focus {
    border-color: #667eea;
    background: #34495e;
  }
  
  .error {
    background: #2c1810;
    color: #f1c40f;
    border-left-color: #f39c12;
  }
  
  .default-password-warning {
    background: #2c2416;
    border-color: #f39c12;
  }
  
  .warning-text p {
    color: #f39c12;
  }
}

@media (max-width: 480px) {
  .login-container {
    padding: 16px;
  }
  
  .login-card {
    padding: 32px 24px;
  }
  
  .login-header h2 {
    font-size: 24px;
  }
}
</style>