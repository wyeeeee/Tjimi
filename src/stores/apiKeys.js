import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export const useApiKeysStore = defineStore('apiKeys', {
  state: () => ({
    keys: [],
    loading: false,
    error: null
  }),

  actions: {
    async fetchApiKeys() {
      this.loading = true
      this.error = null

      try {
        const result = await invoke('get_all_api_keys')

        if (result.success) {
          this.keys = result.data || []
        } else {
          this.error = result.error
        }
      } catch (error) {
        this.error = error.message
      } finally {
        this.loading = false
      }
    },

    async createApiKey(name, keyValue) {
      this.loading = true
      this.error = null

      try {
        console.log('Creating API key:', { name, keyValue: keyValue.substring(0, 10) + '...' })
        const result = await invoke('create_api_key', {
          request: { name, keyValue }
        })

        console.log('API key creation result:', result)

        if (result.success) {
          this.keys.push(result.data)
          // Refresh the list to ensure consistency
          await this.fetchApiKeys()
          return true
        } else {
          console.error('API key creation failed:', result.error)
          this.error = result.error
          return false
        }
      } catch (error) {
        console.error('API key creation error:', error)
        this.error = error.message
        return false
      } finally {
        this.loading = false
      }
    },

    async updateApiKey(keyId, updates) {
      this.loading = true
      this.error = null

      try {
        const result = await invoke('update_api_key', {
          keyId,
          request: updates
        })

        if (result.success) {
          const index = this.keys.findIndex(k => k.id === keyId)
          if (index !== -1) {
            this.keys[index] = result.data
          }
          return true
        } else {
          this.error = result.error
          return false
        }
      } catch (error) {
        this.error = error.message
        return false
      } finally {
        this.loading = false
      }
    },

    async deleteApiKey(keyId) {
      this.loading = true
      this.error = null

      try {
        const result = await invoke('delete_api_key', {
          keyId
        })

        if (result.success && result.data) {
          this.keys = this.keys.filter(k => k.id !== keyId)
          return true
        } else {
          this.error = result.error || 'Failed to delete API key'
          return false
        }
      } catch (error) {
        this.error = error.message
        return false
      } finally {
        this.loading = false
      }
    }
  }
})