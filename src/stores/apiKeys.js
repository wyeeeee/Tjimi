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

    async createApiKey(apiKeyData) {
      this.loading = true
      this.error = null

      try {
        const result = await invoke('create_api_key', {
          request: {
            name: apiKeyData.name || `密钥 ${Date.now()}`,
            keyValue: apiKeyData.key_value
          }
        })

        if (result.success) {
          this.keys.push(result.data)
          // Refresh the list to ensure consistency
          await this.fetchApiKeys()
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

    async updateApiKey(apiKeyData) {
      this.loading = true
      this.error = null

      try {
        const result = await invoke('update_api_key', {
          keyId: apiKeyData.id,
          request: {
            name: apiKeyData.name,
            isActive: apiKeyData.isActive
          }
        })

        if (result.success) {
          const index = this.keys.findIndex(k => k.id === apiKeyData.id)
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
          keyId: keyId
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