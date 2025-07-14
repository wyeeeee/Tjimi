<template>
  <div class="layout">
    <nav class="navbar">
      <div class="nav-brand">
        <h2>Tjimi Proxy</h2>
      </div>
      
      <div class="nav-links">
        <router-link to="/" class="nav-link">
          首页
        </router-link>
        <router-link to="/api-keys" class="nav-link">
          API 密钥
        </router-link>
        <router-link to="/logs" class="nav-link">
          请求日志
        </router-link>
        <router-link to="/settings" class="nav-link">
          设置
        </router-link>
      </div>
      
      <div class="nav-user">
        <div v-if="authStore.isDefaultPassword" class="password-warning">
          <span class="warning-icon">⚠️</span>
          <span class="warning-text">默认密码</span>
        </div>
        <span class="system-status">系统管理员</span>
        <button @click="handleLogout" class="logout-btn">
          退出
        </button>
      </div>
    </nav>
    
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const handleLogout = () => {
  authStore.logout()
  router.push('/login')
}
</script>

<style scoped>
.layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
  background: #343a40;
  color: white;
  height: 60px;
}

.nav-brand h2 {
  margin: 0;
  color: white;
}

.nav-links {
  display: flex;
  gap: 20px;
}

.nav-link {
  color: #adb5bd;
  text-decoration: none;
  padding: 8px 16px;
  border-radius: 4px;
  transition: all 0.3s;
}

.nav-link:hover {
  color: white;
  background: rgba(255, 255, 255, 0.1);
}

.nav-link.router-link-active {
  color: white;
  background: rgba(255, 255, 255, 0.2);
}

.nav-user {
  display: flex;
  align-items: center;
  gap: 15px;
}

.password-warning {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: rgba(255, 193, 7, 0.2);
  border-radius: 4px;
  border: 1px solid rgba(255, 193, 7, 0.3);
}

.warning-icon {
  font-size: 14px;
}

.warning-text {
  color: #ffc107;
  font-size: 12px;
  font-weight: 500;
}

.system-status {
  color: #adb5bd;
  font-size: 14px;
}

.logout-btn {
  background: #dc3545;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  transition: background 0.3s;
}

.logout-btn:hover {
  background: #c82333;
}

.main-content {
  flex: 1;
  padding: 20px;
  background: #f8f9fa;
}

@media (max-width: 768px) {
  .navbar {
    flex-direction: column;
    height: auto;
    padding: 10px;
  }
  
  .nav-links {
    margin: 10px 0;
  }
  
  .nav-user {
    margin-top: 10px;
  }
}

@media (prefers-color-scheme: dark) {
  .navbar {
    background: #1a1a1a;
  }
  
  .main-content {
    background: #2c3e50;
  }
}
</style>