<template>
  <div class="api-keys">
    <div class="header">
      <h1>API 密钥管理</h1>
      <button @click="showAddDialog = true" class="add-btn">
        添加新密钥
      </button>
    </div>

    <div v-if="apiKeysStore.loading" class="loading">
      加载中...
    </div>

    <div v-if="apiKeysStore.error" class="error">
      {{ apiKeysStore.error }}
    </div>

    <div class="keys-grid">
      <div 
        v-for="key in apiKeysStore.keys" 
        :key="key.id"
        class="key-card"
        :class="{ inactive: !key.isActive }"
      >
        <div class="key-header">
          <h3>{{ getKeyDisplayName(key.keyValue) }}</h3>
          <div class="key-status">
            <span :class="{ active: key.isActive, inactive: !key.isActive }">
              {{ key.isActive ? '活跃' : '禁用' }}
            </span>
          </div>
        </div>

        <div class="key-info">
          <p><strong>密钥:</strong> {{ maskKey(key.keyValue) }}</p>
          <p><strong>使用次数:</strong> {{ key.usageCount }}</p>
          <p><strong>最后使用:</strong> {{ formatDate(key.lastUsed) }}</p>
          <p><strong>创建时间:</strong> {{ formatDate(key.createdAt) }}</p>
        </div>

        <div class="key-actions">
          <button 
            @click="toggleKeyStatus(key)"
            :class="{ 'enable-btn': !key.isActive, 'disable-btn': key.isActive }"
          >
            {{ key.isActive ? '禁用' : '启用' }}
          </button>
          <button @click="deleteKey(key)" class="delete-btn">
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- Add Keys Dialog -->
    <div v-if="showAddDialog" class="dialog-overlay">
      <div class="dialog">
        <h3>批量添加 API 密钥</h3>
        
        <form @submit.prevent="submitKeys">
          <div class="form-group">
            <label for="keyInput">API 密钥</label>
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
            ></textarea>
            <div class="key-validation">
              <div v-if="validKeys.length > 0" class="valid-keys">
                <p>✅ 发现 {{ validKeys.length }} 个有效密钥:</p>
                <ul>
                  <li v-for="(key, index) in validKeys" :key="index">
                    {{ maskKey(key) }}
                  </li>
                </ul>
              </div>
              <div v-if="invalidKeys.length > 0" class="invalid-keys">
                <p>❌ {{ invalidKeys.length }} 个无效密钥:</p>
                <ul>
                  <li v-for="(key, index) in invalidKeys" :key="index">
                    {{ key }}
                  </li>
                </ul>
              </div>
              <div v-if="duplicateKeys.length > 0" class="duplicate-keys">
                <p>⚠️ {{ duplicateKeys.length }} 个重复密钥 (已存在):</p>
                <ul>
                  <li v-for="(key, index) in duplicateKeys" :key="index">
                    {{ maskKey(key) }}
                  </li>
                </ul>
              </div>
            </div>
          </div>
          
          <div class="dialog-actions">
            <button type="button" @click="closeDialog">
              取消
            </button>
            <button 
              type="submit" 
              :disabled="apiKeysStore.loading || validKeys.length === 0"
            >
              {{ apiKeysStore.loading ? '添加中...' : `添加 ${validKeys.length} 个密钥` }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, computed } from 'vue'
import { useApiKeysStore } from '../stores/apiKeys'

const apiKeysStore = useApiKeysStore()

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
  await apiKeysStore.updateApiKey(key.id, {
    isActive: !key.isActive
  })
}

const deleteKey = async (key) => {
  const displayName = getKeyDisplayName(key.keyValue)
  if (confirm(`确定要删除密钥 "${displayName}" 吗？`)) {
    await apiKeysStore.deleteApiKey(key.id)
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
.api-keys {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.add-btn {
  background: #28a745;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
  font-size: 14px;
}

.add-btn:hover {
  background: #218838;
}

.loading, .error {
  text-align: center;
  padding: 20px;
  margin: 20px 0;
}

.error {
  color: #dc3545;
  background: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 5px;
}

.keys-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
}

.key-card {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 8px;
  padding: 20px;
  transition: box-shadow 0.3s;
}

.key-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.key-card.inactive {
  opacity: 0.7;
  background: #f1f3f4;
}

.key-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.key-header h3 {
  margin: 0;
  color: #495057;
}

.key-status span {
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: bold;
}

.key-status .active {
  background: #d4edda;
  color: #155724;
}

.key-status .inactive {
  background: #f8d7da;
  color: #721c24;
}

.key-info {
  margin-bottom: 15px;
}

.key-info p {
  margin: 5px 0;
  font-size: 14px;
  color: #6c757d;
}

.key-actions {
  display: flex;
  gap: 10px;
}

.key-actions button {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  transition: background 0.3s;
}

.enable-btn {
  background: #28a745;
  color: white;
}

.enable-btn:hover {
  background: #218838;
}

.disable-btn {
  background: #ffc107;
  color: #212529;
}

.disable-btn:hover {
  background: #e0a800;
}

.edit-btn {
  background: #007bff;
  color: white;
}

.edit-btn:hover {
  background: #0056b3;
}

.delete-btn {
  background: #dc3545;
  color: white;
}

.delete-btn:hover {
  background: #c82333;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.dialog {
  background: white;
  padding: 30px;
  border-radius: 10px;
  width: 400px;
  max-width: 90vw;
}

.dialog h3 {
  margin-top: 0;
  margin-bottom: 20px;
  color: #333;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  color: #555;
  font-weight: 500;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 5px;
  font-size: 14px;
  font-family: 'Courier New', monospace;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #007bff;
}

.form-group textarea {
  resize: vertical;
  min-height: 120px;
}

.key-validation {
  margin-top: 15px;
  padding: 10px;
  border-radius: 5px;
  background: #f8f9fa;
}

.valid-keys {
  margin-bottom: 10px;
}

.valid-keys p {
  color: #28a745;
  font-weight: bold;
  margin-bottom: 5px;
}

.invalid-keys {
  margin-bottom: 10px;
}

.invalid-keys p {
  color: #dc3545;
  font-weight: bold;
  margin-bottom: 5px;
}

.duplicate-keys p {
  color: #ffc107;
  font-weight: bold;
  margin-bottom: 5px;
}

.key-validation ul {
  margin: 0;
  padding-left: 20px;
  max-height: 120px;
  overflow-y: auto;
}

.key-validation li {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  margin: 2px 0;
}

.dialog-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  margin-top: 20px;
}

.dialog-actions button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.dialog-actions button[type="button"] {
  background: #6c757d;
  color: white;
}

.dialog-actions button[type="button"]:hover {
  background: #545b62;
}

.dialog-actions button[type="submit"] {
  background: #007bff;
  color: white;
}

.dialog-actions button[type="submit"]:hover:not(:disabled) {
  background: #0056b3;
}

.dialog-actions button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

@media (prefers-color-scheme: dark) {
  .key-card {
    background: #2c3e50;
    border-color: #34495e;
    color: #ecf0f1;
  }
  
  .key-card.inactive {
    background: #1a252f;
  }
  
  .key-header h3 {
    color: #ecf0f1;
  }
  
  .key-info p {
    color: #bdc3c7;
  }
  
  .dialog {
    background: #2c3e50;
    color: #ecf0f1;
  }
  
  .dialog h3 {
    color: #ecf0f1;
  }
  
  .form-group label {
    color: #bdc3c7;
  }
  
  .form-group input,
  .form-group textarea {
    background: #34495e;
    border-color: #4a5f7a;
    color: #ecf0f1;
  }
  
  .form-group input:focus,
  .form-group textarea:focus {
    border-color: #3498db;
  }
  
  .key-validation {
    background: #34495e;
  }
  
  .valid-keys p {
    color: #2ecc71;
  }
  
  .invalid-keys p {
    color: #e74c3c;
  }
  
  .duplicate-keys p {
    color: #f39c12;
  }
}
</style>