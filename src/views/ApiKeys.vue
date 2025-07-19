<template>
  <div class="api-keys-page">
    <!-- 移动端头部 -->
    <div v-if="isMobile" class="mobile-header">
      <div class="mobile-title">
        <h1>控制面板</h1>
        <p>管理 API 密钥</p>
      </div>
      <Button
        variant="primary"
        icon="add"
        @click="openAddDialog"
        size="sm"
        class="mobile-add-btn"
      >
        添加
      </Button>
    </div>

    <!-- 桌面端头部 -->
    <div v-else class="desktop-header">
      <div class="header-content">
        <h1 class="page-title">Tjimi Proxy 控制面板</h1>
        <p class="page-description">管理 Gemini API 密钥，支持批量添加和轮换负载均衡</p>
      </div>
      <Button
        variant="primary"
        icon="add"
        @click="openAddDialog"
        size="lg"
        class="add-button"
      >
        添加新密钥
      </Button>
    </div>

    <!-- 统计信息 -->
    <MobileStats v-if="isMobile" :api-keys="apiKeysStore.keys" :logs-stats="logsStore.stats" />
    <ApiKeyStats v-else :api-keys="apiKeysStore.keys" :logs-stats="logsStore.stats" />

    <!-- 加载状态 -->
    <LoadingState
      v-if="loading && apiKeysStore.keys.length === 0"
      message="正在获取 API 密钥列表"
    />

    <!-- 错误状态 -->
    <ErrorState
      v-else-if="error"
      :message="error"
      @retry="fetchApiKeys"
    />

    <!-- 密钥列表 -->
    <div v-else class="keys-section">
      <div v-if="!isMobile" class="section-header">
        <h2>密钥列表</h2>
        <div class="section-actions">
          <Button
            variant="outline"
            icon="refresh"
            @click="fetchApiKeys"
            :loading="loading"
          >
            刷新
          </Button>
        </div>
      </div>

      <div v-if="apiKeysStore.keys.length === 0" class="empty-state">
        <Card class="empty-card">
          <div class="empty-content">
            <Icon name="key" :size="isMobile ? 32 : 48" />
            <h3>暂无API密钥</h3>
            <p>开始添加您的第一个 Gemini API 密钥</p>
            <Button
              variant="primary"
              icon="add"
              @click="openAddDialog"
            >
              添加API密钥
            </Button>
          </div>
        </Card>
      </div>

      <!-- 移动端列表 -->
      <div v-if="isMobile" class="mobile-keys-list">
        <div class="list-header">
          <span class="list-title">API 密钥 ({{ apiKeysStore.keys.length }})</span>
          <Button
            variant="ghost"
            icon="refresh"
            @click="fetchApiKeys"
            :loading="loading"
            size="sm"
          />
        </div>
        <MobileApiKeyCard
          v-for="key in apiKeysStore.keys"
          :key="key.id"
          :api-key="key"
          :today-requests="getApiKeyTodayRequests(key.id)"
          @delete="openDeleteDialog"
          @toggle-enabled="toggleApiKey"
        />
      </div>

      <!-- 桌面端网格（保持原有布局） -->
      <div v-else class="keys-grid">
        <ApiKeyCard
          v-for="key in apiKeysStore.keys"
          :key="key.id"
          :api-key="key"
          :today-requests="getApiKeyTodayRequests(key.id)"
          @delete="openDeleteDialog"
          @toggle-enabled="toggleApiKey"
        />
      </div>
    </div>

    <!-- 分页控件 -->
    <Pagination
      v-if="!isMobile && apiKeysStore.keys.length > 0"
      :current-page="apiKeysStore.pagination.currentPage"
      :total-pages="apiKeysStore.pagination.totalPages"
      :total-count="apiKeysStore.pagination.totalCount"
      :per-page="apiKeysStore.pagination.perPage"
      @page-change="handlePageChange"
      @per-page-change="handlePerPageChange"
    />

    <!-- 添加对话框 -->
    <ApiKeyForm
      :show="formModal.isOpen.value"
      @close="formModal.close"
      @submit="handleFormSubmit"
    />

    <!-- 删除确认对话框 -->
    <ConfirmDialog
      :show="showDeleteDialog"
      title="确认删除"
      :message="`确定要删除密钥 '${deleteApiKey?.name || '未知'}' 吗？此操作无法撤销。`"
      confirm-text="删除"
      confirm-variant="danger"
      @confirm="handleDelete"
      @cancel="() => { showDeleteDialog = false; deleteApiKey = null }"
    />
  </div>
</template>

<script setup>
import { onMounted, computed, watch, ref } from 'vue'
import { useApiKeysStore } from '@/stores/apiKeys'
import { useLogsStore } from '@/stores/logs'
import { useLoading } from '@/composables/useLoading'
import { useModal } from '@/composables/useModal'
import { useResponsive } from '@/composables/useResponsive'
import ApiKeyStats from '@/components/apiKeys/ApiKeyStats.vue'
import ApiKeyCard from '@/components/apiKeys/ApiKeyCard.vue'
import ApiKeyForm from '@/components/apiKeys/ApiKeyForm.vue'
import MobileStats from '@/components/mobile/MobileStats.vue'
import MobileApiKeyCard from '@/components/mobile/MobileApiKeyCard.vue'
import LoadingState from '@/components/ui/LoadingState.vue'
import ErrorState from '@/components/ui/ErrorState.vue'
import ConfirmDialog from '@/components/ui/ConfirmDialog.vue'
import Button from '@/components/ui/Button.vue'
import Card from '@/components/ui/Card.vue'
import Icon from '@/components/ui/Icon.vue'
import Pagination from '@/components/ui/Pagination.vue'

const apiKeysStore = useApiKeysStore()
const logsStore = useLogsStore()
const { loading, error, execute } = useLoading()
const formModal = useModal()
const { isMobile, isTablet, isDesktop } = useResponsive()
// 临时用简单的响应式变量替代
const showDeleteDialog = ref(false)
const deleteApiKey = ref(null)

onMounted(() => {
  fetchApiKeys()
  logsStore.fetchStats()
})

const fetchApiKeys = async () => {
  if (isMobile.value) {
    // 移动端仍使用原有的无分页模式
    await execute(() => apiKeysStore.fetchApiKeys())
  } else {
    // 桌面端使用分页模式
    await execute(() => apiKeysStore.fetchApiKeysPaginated(1, 20))
  }
}

const handlePageChange = async (page) => {
  await execute(() => apiKeysStore.fetchApiKeysPaginated(page, apiKeysStore.pagination.perPage))
}

const handlePerPageChange = async (perPage) => {
  await execute(() => apiKeysStore.fetchApiKeysPaginated(1, perPage))
}

const openAddDialog = () => {
  formModal.open()
}

const openDeleteDialog = (apiKey) => {
  deleteApiKey.value = apiKey
  showDeleteDialog.value = true
}

const handleFormSubmit = async (keys) => {
  await execute(async () => {
    // 批量添加密钥
    for (const key of keys) {
      await apiKeysStore.createApiKey({
        key_value: key
      })
    }
    formModal.close()
  })
}

const handleDelete = async () => {
  await execute(async () => {
    await apiKeysStore.deleteApiKey(deleteApiKey.value.id)
    showDeleteDialog.value = false
    deleteApiKey.value = null
  })
}

const getApiKeyTodayRequests = (apiKeyId) => {
  return logsStore.stats?.apiKeyTodayRequests?.[apiKeyId] || 0
}

const toggleApiKey = async (apiKey) => {
  await execute(async () => {
    await apiKeysStore.updateApiKey({
      id: apiKey.id,
      name: apiKey.name,
      isActive: !apiKey.isActive
    })
  })
}
</script>

<style scoped>
.api-keys-page {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

/* 移动端头部 */
.mobile-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--color-surface);
  border-radius: 0.5rem;
  margin-bottom: 1rem;
}

.mobile-title h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
}

.mobile-title p {
  margin: 0;
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.mobile-add-btn {
  flex-shrink: 0;
}

/* 桌面端头部 */
.desktop-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

.header-content {
  flex: 1;
}

.page-title {
  margin: 0 0 0.5rem 0;
  font-size: 2rem;
  font-weight: 700;
  color: var(--color-text);
}

.page-description {
  margin: 0;
  color: var(--color-text-secondary);
  line-height: 1.5;
}

.keys-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.section-header h2 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--color-text);
}

.section-actions {
  display: flex;
  gap: 0.5rem;
}

/* 桌面端网格（保持原有布局） */
.keys-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: 1.5rem;
}

/* 移动端列表 */
.mobile-keys-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  background: var(--color-surface);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
}

.list-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text);
}

.empty-state {
  display: flex;
  justify-content: center;
  padding: 2rem 0;
}

.empty-card {
  max-width: 400px;
  width: 100%;
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  text-align: center;
  padding: 2rem;
}

.empty-content h3 {
  margin: 0;
  color: var(--color-text);
}

.empty-content p {
  margin: 0;
  color: var(--color-text-secondary);
}

/* 平板和小屏幕适配 */
@media (max-width: 1024px) {
  .keys-grid {
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  }
}

@media (max-width: 768px) {
  .api-keys-page {
    gap: 1rem;
  }
  
  .desktop-header {
    flex-direction: column;
    align-items: stretch;
  }
  
  .keys-grid {
    grid-template-columns: 1fr;
  }
  
  .section-header {
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }
  
  .empty-content {
    padding: 1.5rem;
  }
}

@media (max-width: 480px) {
  .mobile-header {
    padding: 0.75rem;
  }
  
  .mobile-title h1 {
    font-size: 1.25rem;
  }
  
  .mobile-title p {
    font-size: 0.8rem;
  }
}
</style>