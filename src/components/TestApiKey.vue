<template>
  <div class="test-api-key">
    <h3>测试 API 密钥添加</h3>
    
    <div class="test-form">
      <div class="form-group">
        <label>测试密钥:</label>
        <input 
          v-model="testKey" 
          placeholder="AIzaSyBnUlu8wdZIjWuAmgA4V3ZRi3YVbgOYukg"
          style="width: 400px; font-family: monospace;"
        />
      </div>
      
      <div class="form-group">
        <button @click="testAddKey" :disabled="loading">
          {{ loading ? '测试中...' : '测试添加密钥' }}
        </button>
      </div>
      
      <div v-if="result" class="result">
        <h4>测试结果:</h4>
        <pre>{{ result }}</pre>
      </div>
      
      <div v-if="error" class="error">
        <h4>错误:</h4>
        <pre>{{ error }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useApiKeysStore } from '../stores/apiKeys'
import { useAuthStore } from '../stores/auth'

const apiKeysStore = useApiKeysStore()
const authStore = useAuthStore()

const testKey = ref('AIzaSyBnUlu8wdZIjWuAmgA4V3ZRi3YVbgOYukg')
const loading = ref(false)
const result = ref(null)
const error = ref(null)

const testAddKey = async () => {
  loading.value = true
  result.value = null
  error.value = null
  
  try {
    console.log('Auth store user:', authStore.user)
    console.log('Test key:', testKey.value)
    
    const success = await apiKeysStore.createApiKey('Test Key', testKey.value)
    
    result.value = {
      success,
      keysCount: apiKeysStore.keys.length,
      error: apiKeysStore.error
    }
    
    if (success) {
      console.log('✅ API key added successfully')
    } else {
      console.log('❌ API key addition failed')
    }
    
  } catch (e) {
    error.value = e.message
    console.error('Test error:', e)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.test-api-key {
  border: 2px solid #007bff;
  border-radius: 8px;
  padding: 20px;
  margin: 20px 0;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

.form-group input {
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.form-group button {
  padding: 10px 20px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.form-group button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.result {
  background: #d4edda;
  border: 1px solid #c3e6cb;
  border-radius: 4px;
  padding: 10px;
  margin-top: 10px;
}

.error {
  background: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 4px;
  padding: 10px;
  margin-top: 10px;
}

pre {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  margin: 0;
}
</style>