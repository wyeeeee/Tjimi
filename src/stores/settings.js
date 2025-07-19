import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useSettingsStore = defineStore('settings', () => {
  const retryCount = ref(3)
  const loading = ref(false)
  const error = ref(null)

  const getRetryCount = async () => {
    loading.value = true
    error.value = null
    
    try {
      const count = await invoke('get_retry_count')
      retryCount.value = count
      return count
    } catch (err) {
      error.value = err
      console.error('获取重试次数失败:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  const setRetryCount = async (count) => {
    loading.value = true
    error.value = null
    
    try {
      await invoke('set_retry_count', { retryCount: count })
      retryCount.value = count
      return true
    } catch (err) {
      error.value = err
      console.error('设置重试次数失败:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  return {
    retryCount,
    loading,
    error,
    getRetryCount,
    setRetryCount
  }
})