<template>
  <div class="app-layout">
    <!-- Mobile Header -->
    <header class="mobile-header" v-if="isMobile">
      <div class="mobile-header-content">
        <button
          class="mobile-menu-toggle"
          @click="toggleMobileMenu"
          :aria-expanded="isMobileMenuOpen"
          aria-label="Toggle navigation menu"
        >
          <Icon :name="isMobileMenuOpen ? 'close' : 'menu'" size="24" />
        </button>
        
        <div class="mobile-header-title">
          <h1>{{ currentPageTitle }}</h1>
        </div>
        
        <div class="mobile-header-actions">
          <slot name="mobile-actions" />
        </div>
      </div>
    </header>

    <!-- Sidebar/Navigation -->
    <aside :class="sidebarClasses">
      <div class="sidebar-content">
        <!-- Brand/Logo -->
        <div class="sidebar-brand">
          <div class="brand-logo">
            <Icon name="key" size="32" color="var(--color-primary)" />
          </div>
          <div class="brand-text">
            <h2 class="brand-title">Tjimi Proxy</h2>
            <p class="brand-subtitle">API 轮询服务</p>
          </div>
        </div>

        <!-- Navigation -->
        <nav class="sidebar-nav" role="navigation">
          <ul class="nav-list">
            <li v-for="item in navigationItems" :key="item.path" class="nav-item">
              <router-link
                :to="item.path"
                class="nav-link"
                :class="{ 'nav-link--active': $route.path === item.path }"
                @click="closeMobileMenu"
              >
                <Icon :name="item.icon" size="20" class="nav-icon" />
                <span class="nav-text">{{ item.label }}</span>
                <div v-if="item.badge" class="nav-badge">{{ item.badge }}</div>
              </router-link>
            </li>
          </ul>
        </nav>

        <!-- Actions -->
        <div class="sidebar-footer">
          <div class="sidebar-actions">
            <Button
              variant="ghost"
              size="sm"
              icon="logout"
              @click="handleLogout"
              class="logout-btn"
            >
              退出登录
            </Button>
          </div>
        </div>
      </div>
    </aside>

    <!-- Mobile Menu Overlay -->
    <div
      v-if="isMobile && isMobileMenuOpen"
      class="mobile-overlay"
      @click="closeMobileMenu"
    />

    <!-- Main Content -->
    <main class="main-content">
      <!-- Desktop Header -->
      <header class="desktop-header" v-if="!isMobile">
        <div class="desktop-header-content">
          <div class="page-header">
            <h1 class="page-title">{{ currentPageTitle }}</h1>
            <p v-if="currentPageDescription" class="page-description">
              {{ currentPageDescription }}
            </p>
          </div>
          
          <div class="header-actions">
            <slot name="header-actions" />
          </div>
        </div>
      </header>

      <!-- Page Content -->
      <div class="page-content">
        <router-view v-slot="{ Component, route }">
          <transition name="page" mode="out-in">
            <component :is="Component" :key="route.path" />
          </transition>
        </router-view>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../../stores/auth'
import Button from '../ui/Button.vue'
import Icon from '../ui/Icon.vue'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const isMobile = ref(false)
const isMobileMenuOpen = ref(false)

// Navigation items
const navigationItems = computed(() => [
  {
    path: '/api-keys',
    label: '控制面板',
    icon: 'key',
    badge: null
  },
  {
    path: '/logs',
    label: '请求日志',
    icon: 'logs',
    badge: null
  },
  {
    path: '/settings',
    label: '设置',
    icon: 'settings',
    badge: authStore.isDefaultPassword ? '!' : null
  }
])

// Page metadata
const pageMetadata = {
  '/api-keys': {
    title: '控制面板',
    description: '管理您的 Gemini API 密钥'
  },
  '/logs': {
    title: '请求日志',
    description: '查看 API 请求历史记录'
  },
  '/settings': {
    title: '系统设置',
    description: '配置系统参数和安全设置'
  }
}

const currentPageTitle = computed(() => {
  return pageMetadata[route.path]?.title || '未知页面'
})

const currentPageDescription = computed(() => {
  return pageMetadata[route.path]?.description
})

const sidebarClasses = computed(() => [
  'sidebar',
  {
    'sidebar--mobile': isMobile.value,
    'sidebar--mobile-open': isMobile.value && isMobileMenuOpen.value
  }
])

// Responsive handling
const handleResize = () => {
  const newIsMobile = window.innerWidth < 768
  if (newIsMobile !== isMobile.value) {
    isMobile.value = newIsMobile
    if (!newIsMobile) {
      isMobileMenuOpen.value = false
    }
  }
}

const toggleMobileMenu = () => {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}

const closeMobileMenu = () => {
  isMobileMenuOpen.value = false
}

const handleLogout = async () => {
  authStore.logout()
  await router.push('/login')
}

onMounted(() => {
  handleResize()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<style scoped>
.app-layout {
  display: flex;
  min-height: 100vh;
  background-color: var(--color-background);
}

/* Mobile Header */
.mobile-header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 64px;
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  z-index: var(--z-fixed);
}

.mobile-header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 100%;
  padding: 0 var(--spacing-4);
}

.mobile-menu-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  border-radius: var(--radius-lg);
  color: var(--color-text);
  transition: background-color var(--transition-fast);
}

.mobile-menu-toggle:hover {
  background-color: var(--color-surface-hover);
}

.mobile-header-title h1 {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  margin: 0;
}

.mobile-header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  min-width: 44px;
  justify-content: flex-end;
}

/* Sidebar */
.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  width: 280px;
  height: 100vh;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  z-index: var(--z-fixed);
  transition: transform var(--transition-normal);
}

.sidebar--mobile {
  transform: translateX(-100%);
}

.sidebar--mobile-open {
  transform: translateX(0);
}

.sidebar-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: var(--spacing-4) 0;
}

/* Brand */
.sidebar-brand {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  padding: 0 var(--spacing-4);
  margin-bottom: var(--spacing-6);
}

.brand-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  background: linear-gradient(135deg, var(--color-primary), var(--color-secondary));
  border-radius: var(--radius-xl);
  flex-shrink: 0;
}

.brand-title {
  font-size: var(--text-xl);
  font-weight: var(--font-bold);
  margin: 0;
  color: var(--color-text);
}

.brand-subtitle {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
}

/* Navigation */
.sidebar-nav {
  flex: 1;
  overflow-y: auto;
}

.nav-list {
  list-style: none;
  padding: 0 var(--spacing-4);
  margin: 0;
}

.nav-item {
  margin-bottom: var(--spacing-1);
}

.nav-link {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  padding: var(--spacing-3) var(--spacing-4);
  border-radius: var(--radius-lg);
  color: var(--color-text-secondary);
  text-decoration: none;
  transition: all var(--transition-fast);
  position: relative;
}

.nav-link:hover {
  background-color: var(--color-surface-hover);
  color: var(--color-text);
}

.nav-link--active {
  background-color: var(--color-primary);
  color: var(--color-white);
}

.nav-link--active:hover {
  background-color: var(--color-primary-hover);
}

.nav-icon {
  flex-shrink: 0;
}

.nav-text {
  flex: 1;
  font-weight: var(--font-medium);
}

.nav-badge {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 20px;
  height: 20px;
  background-color: var(--color-danger);
  color: var(--color-white);
  font-size: var(--text-xs);
  font-weight: var(--font-bold);
  border-radius: var(--radius-full);
  padding: 0 var(--spacing-1);
}

.nav-link--active .nav-badge {
  background-color: rgba(255, 255, 255, 0.2);
}

/* Sidebar Footer */
.sidebar-footer {
  padding: var(--spacing-3) var(--spacing-4);
  border-top: 1px solid var(--color-border);
  margin-top: var(--spacing-4);
}

.sidebar-actions {
  display: flex;
  justify-content: center;
}

.logout-btn {
  width: 100%;
  justify-content: center;
}

/* Mobile Overlay */
.mobile-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: calc(var(--z-fixed) - 1);
}

/* Main Content */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  margin-left: 280px;
  transition: margin-left var(--transition-normal);
}

@media (max-width: 767px) {
  .main-content {
    margin-left: 0;
    padding-top: 64px;
  }
}

/* Desktop Header */
.desktop-header {
  background-color: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  padding: var(--spacing-6) var(--spacing-8);
}

.desktop-header-content {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--spacing-6);
}

.page-title {
  font-size: var(--text-3xl);
  font-weight: var(--font-bold);
  margin: 0 0 var(--spacing-1) 0;
  color: var(--color-text);
}

.page-description {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  flex-shrink: 0;
}

/* Page Content */
.page-content {
  flex: 1;
  padding: var(--spacing-8);
  overflow-y: auto;
}

@media (max-width: 767px) {
  .page-content {
    padding: var(--spacing-4);
  }
}

/* Page Transitions */
.page-enter-active,
.page-leave-active {
  transition: all var(--transition-normal);
}

.page-enter-from {
  opacity: 0;
  transform: translateY(var(--spacing-4));
}

.page-leave-to {
  opacity: 0;
  transform: translateY(calc(var(--spacing-4) * -1));
}

/* Responsive breakpoints */
@media (max-width: 1024px) {
  .desktop-header {
    padding: var(--spacing-4) var(--spacing-6);
  }
  
  .page-content {
    padding: var(--spacing-6);
  }
}

@media (max-width: 640px) {
  .desktop-header {
    padding: var(--spacing-4);
  }
  
  .page-content {
    padding: var(--spacing-4);
  }
  
  .page-title {
    font-size: var(--text-2xl);
  }
  
  .desktop-header-content {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-4);
  }
  
  .header-actions {
    align-self: flex-end;
  }
}
</style>