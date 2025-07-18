import { ref } from 'vue'

export function useLoading() {
  const loading = ref(false)
  const error = ref(null)

  const execute = async (fn) => {
    loading.value = true
    error.value = null
    
    try {
      const result = await fn()
      return result
    } catch (err) {
      error.value = err.message || '操作失败'
      throw err
    } finally {
      loading.value = false
    }
  }

  const reset = () => {
    loading.value = false
    error.value = null
  }

  return {
    loading,
    error,
    execute,
    reset
  }
}