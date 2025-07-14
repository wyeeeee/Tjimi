<template>
  <div class="login-page">
    <div class="login-container">
      <!-- Background Elements -->
      <div class="login-background">
        <div class="bg-gradient"></div>
        <div class="bg-pattern"></div>
      </div>

      <!-- Login Form -->
      <Card class="login-card" variant="elevated">
        <template #header>
          <div class="login-header">
            <div class="brand-section">
              <div class="brand-icon">
                <Icon name="key" size="48" color="var(--color-primary)" />
              </div>
              <div class="brand-info">
                <h1 class="brand-title">Tjimi Proxy</h1>
                <p class="brand-subtitle">API 管理系统</p>
              </div>
            </div>
            <h2 class="login-title">系统登录</h2>
            <p class="login-description">请输入密码以访问管理面板</p>
          </div>
        </template>

        <form @submit.prevent="handleSubmit" class="login-form">
          <Input
            v-model="password"
            type="password"
            label="密码"
            placeholder="默认密码: admin123"
            prefix-icon="key"
            :error="authStore.error"
            :loading="authStore.loading"
            required
            autofocus
            size="lg"
          />

          <Button
            type="submit"
            variant="primary"
            size="lg"
            :loading="authStore.loading"
            full-width
            class="login-button"
          >
            {{ authStore.loading ? '登录中...' : '登录系统' }}
          </Button>
        </form>

        <template #footer>
          <div class="login-footer">
            <div class="default-password-hint">
              <Icon name="info" size="16" />
              <span>首次登录请使用默认密码：<code>admin123</code></span>
            </div>
            
            <div class="login-features">
              <div class="feature-item">
                <Icon name="check" size="16" />
                <span>API 密钥管理</span>
              </div>
              <div class="feature-item">
                <Icon name="check" size="16" />
                <span>请求日志监控</span>
              </div>
              <div class="feature-item">
                <Icon name="check" size="16" />
                <span>安全认证</span>
              </div>
            </div>
          </div>
        </template>
      </Card>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import Card from '../components/ui/Card.vue'
import Input from '../components/ui/Input.vue'
import Button from '../components/ui/Button.vue'
import Icon from '../components/ui/Icon.vue'

const router = useRouter()
const authStore = useAuthStore()

const password = ref('')

const handleSubmit = async () => {
  const success = await authStore.login(password.value)
  
  if (success) {
    await router.push('/')
  }
}
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-4);
  position: relative;
  overflow: hidden;
}

.login-container {
  position: relative;
  z-index: 2;
  width: 100%;
  max-width: 480px;
}

/* Background */
.login-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: -1;
}

.bg-gradient {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    var(--color-primary) 0%, 
    var(--color-secondary) 50%, 
    var(--color-primary) 100%);
  opacity: 0.9;
}

.bg-pattern {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    radial-gradient(circle at 25% 25%, rgba(255, 255, 255, 0.1) 2px, transparent 2px),
    radial-gradient(circle at 75% 75%, rgba(255, 255, 255, 0.1) 2px, transparent 2px);
  background-size: 50px 50px;
  background-position: 0 0, 25px 25px;
  opacity: 0.3;
  animation: float 20s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0) rotate(0deg); }
  50% { transform: translateY(-20px) rotate(1deg); }
}

/* Login Card */
.login-card {
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 
    0 25px 50px -12px rgba(0, 0, 0, 0.25),
    0 0 0 1px rgba(255, 255, 255, 0.1);
  animation: slideInUp 0.6s ease-out;
}

@keyframes slideInUp {
  from {
    opacity: 0;
    transform: translateY(2rem);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Header */
.login-header {
  text-align: center;
}

.brand-section {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-4);
  margin-bottom: var(--spacing-6);
}

.brand-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 80px;
  height: 80px;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.2) 0%, 
    rgba(255, 255, 255, 0.1) 100%);
  border-radius: var(--radius-2xl);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.05); }
}

.brand-info {
  text-align: left;
}

.brand-title {
  font-size: var(--text-3xl);
  font-weight: var(--font-bold);
  margin: 0;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.brand-subtitle {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  opacity: 0.8;
}

.login-title {
  font-size: var(--text-2xl);
  font-weight: var(--font-semibold);
  margin: 0 0 var(--spacing-2) 0;
  color: var(--color-text);
}

.login-description {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  opacity: 0.8;
}

/* Form */
.login-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
  margin-top: var(--spacing-8);
}

.login-button {
  margin-top: var(--spacing-2);
}

/* Footer */
.login-footer {
  text-align: center;
  margin-top: var(--spacing-6);
}

.default-password-hint {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-2);
  padding: var(--spacing-3);
  background-color: rgba(var(--color-info-rgb), 0.1);
  border: 1px solid rgba(var(--color-info-rgb), 0.2);
  border-radius: var(--radius-lg);
  font-size: var(--text-sm);
  color: var(--color-info);
  margin-bottom: var(--spacing-6);
}

.default-password-hint code {
  background-color: rgba(var(--color-info-rgb), 0.2);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: 0.875em;
}

.login-features {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-3);
}

.feature-item {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-2);
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  opacity: 0.8;
}

.feature-item :deep(.icon) {
  color: var(--color-success);
}

/* Mobile Responsive */
@media (max-width: 640px) {
  .login-page {
    padding: var(--spacing-4);
  }
  
  .brand-section {
    flex-direction: column;
    gap: var(--spacing-3);
  }
  
  .brand-icon {
    width: 64px;
    height: 64px;
  }
  
  .brand-info {
    text-align: center;
  }
  
  .brand-title {
    font-size: var(--text-2xl);
  }
  
  .login-title {
    font-size: var(--text-xl);
  }
  
  .login-features {
    gap: var(--spacing-2);
  }
  
  .feature-item {
    font-size: var(--text-xs);
  }
}

/* Landscape mobile */
@media (max-height: 640px) and (orientation: landscape) {
  .login-page {
    padding: var(--spacing-2);
  }
  
  .brand-section {
    flex-direction: row;
    gap: var(--spacing-3);
    margin-bottom: var(--spacing-4);
  }
  
  .brand-icon {
    width: 48px;
    height: 48px;
  }
  
  .login-form {
    gap: var(--spacing-4);
    margin-top: var(--spacing-4);
  }
  
  .login-footer {
    margin-top: var(--spacing-4);
  }
  
  .login-features {
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: center;
    gap: var(--spacing-4);
  }
}

/* Dark mode enhancements */
@media (prefers-color-scheme: dark) {
  .bg-gradient {
    background: linear-gradient(135deg, 
      #1a1a2e 0%, 
      #16213e 50%, 
      #0f3460 100%);
  }
  
  .login-card {
    background-color: rgba(0, 0, 0, 0.3);
    border-color: rgba(255, 255, 255, 0.1);
  }
  
  .brand-icon {
    background: linear-gradient(135deg, 
      rgba(255, 255, 255, 0.1) 0%, 
      rgba(255, 255, 255, 0.05) 100%);
    border-color: rgba(255, 255, 255, 0.1);
  }
}

/* Reduce motion */
@media (prefers-reduced-motion: reduce) {
  .login-card,
  .brand-icon,
  .bg-pattern {
    animation: none;
  }
}
</style>