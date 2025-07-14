<template>
  <div class="api-keys-page">
    <div class="page-header">
      <div class="header-content">
        <h1 class="page-title">API 密钥管理</h1>
        <p class="page-description">管理 Gemini API 密钥，支持批量添加和轮换负载均衡</p>
      </div>
      <Button
        variant="primary"
        icon="add"
        @click="showAddDialog = true"
        size="lg"
        class="add-button"
      >
        添加新密钥
      </Button>
    </div>

    <!-- Loading State -->
    <div v-if="apiKeysStore.loading && apiKeysStore.keys.length === 0" class="loading-state">
      <Card class="loading-card">
        <div class="loading-content">
          <Icon name="loading" size="32" />
          <h3>加载中...</h3>
          <p>正在获取 API 密钥列表</p>
        </div>
      </Card>
    </div>

    <!-- Error State -->
    <Card v-if="apiKeysStore.error" variant="danger" class="error-card">
      <div class="error-content">
        <Icon name="error" size="24" />
        <div class="error-text">
          <h3>加载失败</h3>
          <p>{{ apiKeysStore.error }}</p>
        </div>
        <Button
          variant="outline"
          size="sm"
          @click="apiKeysStore.fetchApiKeys()"
          icon="refresh"
        >
          重试
        </Button>
      </div>
    </Card>

    <!-- Keys Grid -->
    <div v-if="!apiKeysStore.loading || apiKeysStore.keys.length > 0" class="keys-section">
      <div v-if="apiKeysStore.keys.length === 0" class="empty-state">
        <Card class="empty-card">
          <div class="empty-content">
            <Icon name="key" size="48" />
            <h3>暂无 API 密钥</h3>
            <p>点击上方按钮添加第一个 Gemini API 密钥</p>
            <Button
              variant="primary"
              icon="add"
              @click="showAddDialog = true"
            >
              立即添加
            </Button>
          </div>
        </Card>
      </div>

      <div v-else class="keys-grid">
        <Card 
          v-for="key in apiKeysStore.keys" 
          :key="key.id"
          class="key-card"
          :class="{ 'key-card--inactive': !key.isActive }"
          hoverable
        >
          <div class="key-header">
            <div class="key-title">
              <Icon name="key" size="20" />
              <h3>{{ getKeyDisplayName(key.keyValue) }}</h3>
            </div>
            <div class="key-status">
              <span 
                class="status-badge" 
                :class="key.isActive ? 'status-badge--active' : 'status-badge--inactive'"
              >
                <Icon :name="key.isActive ? 'check' : 'warning'" size="12" />
                {{ key.isActive ? '活跃' : '禁用' }}
              </span>
            </div>
          </div>

          <div class="key-info">
            <div class="info-item">
              <span class="info-label">密钥:</span>
              <span class="info-value info-value--mono">{{ maskKey(key.keyValue) }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">使用次数:</span>
              <span class="info-value">{{ key.usageCount.toLocaleString() }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">最后使用:</span>
              <span class="info-value">{{ formatDate(key.lastUsed) }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">创建时间:</span>
              <span class="info-value">{{ formatDate(key.createdAt) }}</span>
            </div>
          </div>

          <div class="key-actions">
            <Button
              :variant="key.isActive ? 'warning' : 'success'"
              size="sm"
              :icon="key.isActive ? 'warning' : 'check'"
              @click="toggleKeyStatus(key)"
              :loading="updatingKeys.has(key.id)"
            >
              {{ key.isActive ? '禁用' : '启用' }}
            </Button>
            <Button
              variant="danger"
              size="sm"
              icon="error"
              @click="deleteKey(key)"
              :loading="deletingKeys.has(key.id)"
            >
              删除
            </Button>
          </div>
        </Card>
      </div>
    </div>

    <!-- Add Keys Modal -->
    <Modal
      v-model:show="showAddDialog"
      title="批量添加 API 密钥"
      size="lg"
      @close="closeDialog"
    >
      <form @submit.prevent="submitKeys" class="add-form">
        <div class="form-section">
          <label for="keyInput" class="form-label">
            <Icon name="key" size="16" />
            API 密钥列表
          </label>
          <p class="form-description">
            请输入一个或多个 Gemini API 密钥，每行一个。支持批量验证和添加。
          </p>
          
          <textarea
            id="keyInput"
            v-model="keyInput"
            rows="8"
            placeholder="请输入一个或多个 Gemini API 密钥，每行一个

例如：
AIzaSyBnUlu8wdZIjWuAmgA4V3ZRi3YVbgOYukg
AIzaSyAbcDef123456789...
AIzaSyXyz987654321..."
            @input="validateKeys"
            class="key-input"
          ></textarea>
        </div>
        
        <!-- Validation Results -->
        <div v-if="keyInput.trim()" class="validation-section">
          <Card v-if="validKeys.length > 0" variant="success" class="validation-card">
            <div class="validation-header">
              <Icon name="check" size="20" />
              <h4>发现 {{ validKeys.length }} 个有效密钥</h4>
            </div>
            <div class="validation-list">
              <div 
                v-for="(key, index) in validKeys" 
                :key="index"
                class="validation-item"
              >
                <Icon name="key" size="14" />
                <span class="key-preview">{{ maskKey(key) }}</span>
              </div>
            </div>
          </Card>

          <Card v-if="invalidKeys.length > 0" variant="danger" class="validation-card">
            <div class="validation-header">
              <Icon name="error" size="20" />
              <h4>{{ invalidKeys.length }} 个无效密钥</h4>
            </div>
            <div class="validation-list">
              <div 
                v-for="(key, index) in invalidKeys" 
                :key="index"
                class="validation-item"
              >
                <Icon name="error" size="14" />
                <span class="invalid-key">{{ key }}</span>
              </div>
            </div>
          </Card>

          <Card v-if="duplicateKeys.length > 0" variant="warning" class="validation-card">
            <div class="validation-header">
              <Icon name="warning" size="20" />
              <h4>{{ duplicateKeys.length }} 个重复密钥 (已存在)</h4>
            </div>
            <div class="validation-list">
              <div 
                v-for="(key, index) in duplicateKeys" 
                :key="index"
                class="validation-item"
              >
                <Icon name="warning" size="14" />
                <span class="key-preview">{{ maskKey(key) }}</span>
              </div>
            </div>
          </Card>
        </div>
      </form>
      
      <template #footer>
        <Button
          variant="ghost"
          @click="closeDialog"
        >
          取消
        </Button>
        <Button
          variant="primary"
          type="submit"
          @click="submitKeys"
          :disabled="validKeys.length === 0"
          :loading="apiKeysStore.loading"
          icon="add"
        >
          {{ validKeys.length > 0 ? `添加 ${validKeys.length} 个密钥` : '请输入有效密钥' }}
        </Button>
      </template>
    </Modal>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, computed } from 'vue'
import { useApiKeysStore } from '../stores/apiKeys'
import Button from '../components/ui/Button.vue'
import Card from '../components/ui/Card.vue'
import Icon from '../components/ui/Icon.vue'
import Modal from '../components/ui/Modal.vue'

const apiKeysStore = useApiKeysStore()
const updatingKeys = ref(new Set())
const deletingKeys = ref(new Set())

const showAddDialog = ref(false)
const keyInput = ref('')
const validKeys = ref([])
const invalidKeys = ref([])
const duplicateKeys = ref([])

// Gemini API key format: AIza + base64 characters, 39 characters total
const isValidGeminiKey = (key) => {
  const trimmed = key.trim()
  return /^AIza[A-Za-z0-9_-]{35}$/.test(trimmed)
}

const getKeyDisplayName = (keyValue) => {
  return `API Key ${keyValue.substring(4, 10)}***`
}

const maskKey = (key) => {
  if (!key) return ''
  return key.substring(0, 8) + '...' + key.substring(key.length - 8)
}

const formatDate = (dateString) => {
  if (!dateString) return '从未'
  return new Date(dateString).toLocaleString('zh-CN')
}

const validateKeys = () => {
  const input = keyInput.value.trim()
  if (!input) {
    validKeys.value = []
    invalidKeys.value = []
    duplicateKeys.value = []
    return
  }

  const lines = input.split('\n').map(line => line.trim()).filter(line => line)
  const existingKeys = apiKeysStore.keys.map(k => k.keyValue)
  
  const valid = []
  const invalid = []
  const duplicates = []
  const seen = new Set()

  for (const line of lines) {
    if (seen.has(line)) {
      continue // Skip duplicates within input
    }
    seen.add(line)

    if (existingKeys.includes(line)) {
      duplicates.push(line)
    } else if (isValidGeminiKey(line)) {
      valid.push(line)
    } else if (line.length > 0) {
      invalid.push(line)
    }
  }

  validKeys.value = valid
  invalidKeys.value = invalid
  duplicateKeys.value = duplicates
}

const toggleKeyStatus = async (key) => {
  updatingKeys.value.add(key.id)
  try {
    await apiKeysStore.updateApiKey(key.id, {
      isActive: !key.isActive
    })
  } finally {
    updatingKeys.value.delete(key.id)
  }
}

const deleteKey = async (key) => {
  const displayName = getKeyDisplayName(key.keyValue)
  if (confirm(`确定要删除密钥 "${displayName}" 吗？\n\n此操作无法撤销，该密钥将立即停止使用。`)) {
    deletingKeys.value.add(key.id)
    try {
      await apiKeysStore.deleteApiKey(key.id)
    } finally {
      deletingKeys.value.delete(key.id)
    }
  }
}

const submitKeys = async () => {
  if (validKeys.value.length === 0) return

  let successCount = 0
  for (const keyValue of validKeys.value) {
    const displayName = getKeyDisplayName(keyValue)
    const success = await apiKeysStore.createApiKey(displayName, keyValue)
    if (success) successCount++
  }

  if (successCount > 0) {
    console.log(`成功添加 ${successCount} 个密钥`)
  }

  closeDialog()
}

const closeDialog = () => {
  showAddDialog.value = false
  keyInput.value = ''
  validKeys.value = []
  invalidKeys.value = []
  duplicateKeys.value = []
}

onMounted(() => {
  apiKeysStore.fetchApiKeys()
})
</script>

<style scoped>
.api-keys-page {
  padding: var(--spacing-6);
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: var(--spacing-8);
}

/* Page Header */
.page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--spacing-6);
  flex-wrap: wrap;
}

.header-content {
  flex: 1;
  min-width: 0;
}

.page-title {
  font-size: var(--text-3xl);
  font-weight: var(--font-bold);
  color: var(--color-text);
  margin: 0 0 var(--spacing-2) 0;
}

.page-description {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

.add-button {
  flex-shrink: 0;
}

/* Loading State */
.loading-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 300px;
}

.loading-card {
  max-width: 400px;
  width: 100%;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4);
  text-align: center;
  padding: var(--spacing-8);
}

.loading-content h3 {
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
}

.loading-content p {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
}

/* Error State */
.error-card {
  margin-bottom: var(--spacing-6);
}

.error-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-4);
}

.error-text {
  flex: 1;
  min-width: 0;
}

.error-text h3 {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  margin: 0 0 var(--spacing-1) 0;
}

.error-text p {
  font-size: var(--text-sm);
  margin: 0;
  opacity: 0.9;
}

/* Empty State */
.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
}

.empty-card {
  max-width: 500px;
  width: 100%;
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-6);
  text-align: center;
  padding: var(--spacing-8);
}

.empty-content h3 {
  font-size: var(--text-2xl);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
}

.empty-content p {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* Keys Grid */
.keys-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.keys-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
  gap: var(--spacing-6);
}

/* Key Card */
.key-card {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-5);
  transition: all var(--transition-normal);
  position: relative;
  overflow: hidden;
}

.key-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, var(--color-success), var(--color-success));
  transition: all var(--transition-normal);
}

.key-card--inactive::before {
  background: linear-gradient(90deg, var(--color-warning), var(--color-warning));
}

.key-card--inactive {
  opacity: 0.8;
}

.key-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-4);
}

.key-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  flex: 1;
  min-width: 0;
}

.key-title h3 {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
  font-family: var(--font-mono);
}

.key-status {
  flex-shrink: 0;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: var(--spacing-1);
  padding: var(--spacing-1) var(--spacing-3);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--font-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.status-badge--active {
  background-color: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
}

.status-badge--inactive {
  background-color: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
}

/* Key Info */
.key-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-3);
}

.info-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--spacing-3);
  flex-wrap: wrap;
}

.info-label {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text-secondary);
  min-width: 80px;
  flex-shrink: 0;
}

.info-value {
  font-size: var(--text-sm);
  color: var(--color-text);
  text-align: right;
  flex: 1;
  min-width: 0;
}

.info-value--mono {
  font-family: var(--font-mono);
  font-weight: var(--font-medium);
}

/* Key Actions */
.key-actions {
  display: flex;
  gap: var(--spacing-3);
  padding-top: var(--spacing-4);
  border-top: 1px solid var(--color-border);
}

.key-actions :deep(.button) {
  flex: 1;
}

/* Add Form */
.add-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-6);
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
}

.form-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
}

.form-description {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

.key-input {
  width: 100%;
  min-height: 200px;
  padding: var(--spacing-4);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-lg);
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  line-height: 1.5;
  color: var(--color-text);
  background-color: var(--color-surface);
  resize: vertical;
  transition: all var(--transition-fast);
}

.key-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(var(--color-primary-rgb), 0.1);
}

.key-input::placeholder {
  color: var(--color-text-tertiary);
  font-family: var(--font-mono);
}

/* Validation Section */
.validation-section {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
}

.validation-card {
  border: none;
}

.validation-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  margin-bottom: var(--spacing-4);
}

.validation-header h4 {
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
  margin: 0;
}

.validation-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
  max-height: 200px;
  overflow-y: auto;
}

.validation-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-2) var(--spacing-3);
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: var(--radius-md);
  font-family: var(--font-mono);
  font-size: var(--text-sm);
}

.key-preview {
  color: inherit;
  opacity: 0.9;
}

.invalid-key {
  color: inherit;
  opacity: 0.8;
  word-break: break-all;
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .api-keys-page {
    padding: var(--spacing-4);
    gap: var(--spacing-6);
  }
  
  .page-header {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-4);
  }
  
  .add-button {
    width: 100%;
  }
  
  .page-title {
    font-size: var(--text-2xl);
  }
  
  .keys-grid {
    grid-template-columns: 1fr;
    gap: var(--spacing-4);
  }
  
  .info-item {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-1);
  }
  
  .info-label {
    min-width: unset;
  }
  
  .info-value {
    text-align: left;
  }
  
  .key-actions {
    flex-direction: column;
  }
  
  .validation-list {
    max-height: 150px;
  }
  
  .validation-item {
    flex-wrap: wrap;
  }
}

/* Tablet optimizations */
@media (max-width: 768px) {
  .keys-grid {
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  }
}

/* Large screen optimizations */
@media (min-width: 1200px) {
  .keys-grid {
    grid-template-columns: repeat(auto-fill, minmax(420px, 1fr));
  }
}

/* Dark mode enhancements */
@media (prefers-color-scheme: dark) {
  .key-input {
    background-color: var(--color-surface-secondary);
  }
  
  .validation-item {
    background-color: rgba(255, 255, 255, 0.05);
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .key-card,
  .key-input {
    transition: none;
  }
}
</style>