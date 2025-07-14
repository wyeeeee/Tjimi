import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { useAuthStore } from './auth'

export const useLogsStore = defineStore('logs', {
  state: () => ({
    logs: [],
    stats: null,
    loading: false,
    error: null
  }),

  actions: {
    async fetchLogs(limit = 100) {
      const authStore = useAuthStore()
      if (!authStore.isAuthenticated) {
        console.warn('User not authenticated, skipping logs fetch')
        return
      }

      this.loading = true
      this.error = null

      try {
        console.log('Fetching logs with limit:', limit)
        const result = await invoke('get_request_logs', {
          limit
        })

        console.log('Logs fetch result:', result)

        if (result.success) {
          this.logs = result.data || []
          console.log('Loaded logs:', this.logs.length, 'items')
        } else {
          this.error = result.error
          console.error('Logs fetch error:', result.error)
        }
      } catch (error) {
        this.error = error.message
        console.error('Logs fetch exception:', error)
      } finally {
        this.loading = false
      }
    },

    async fetchStats() {
      const authStore = useAuthStore()
      if (!authStore.isAuthenticated) return

      this.loading = true
      this.error = null

      try {
        const result = await invoke('get_usage_stats')

        if (result.success) {
          this.stats = result.data
        } else {
          this.error = result.error
        }
      } catch (error) {
        this.error = error.message
      } finally {
        this.loading = false
      }
    }
  }
})