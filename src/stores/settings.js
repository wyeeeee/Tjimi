import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useSettingsStore = defineStore('settings', () => {
  const retryCount = ref(3)
  const proxySettings = ref({
    enabled: false,
    proxy_type: 'http',
    host: null,
    port: null,
    username: null,
    password: null
  })
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

  const getProxySettings = async () => {
    loading.value = true
    error.value = null
    
    try {
      const settings = await invoke('get_proxy_settings')
      proxySettings.value = settings
      return settings
    } catch (err) {
      error.value = err
      console.error('获取代理设置失败:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  const setProxySettings = async (settings) => {
    loading.value = true
    error.value = null
    
    try {
      await invoke('set_proxy_settings', { settings })
      proxySettings.value = settings
      return true
    } catch (err) {
      error.value = err
      console.error('设置代理失败:', err)
      throw err
    } finally {
      loading.value = false
    }
  }

  return {
    retryCount,
    proxySettings,
    loading,
    error,
    getRetryCount,
    setRetryCount,
    getProxySettings,
    setProxySettings
  }
})