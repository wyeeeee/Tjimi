import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    isAuthenticated: false,
    sessionToken: null,
    loading: false,
    error: null,
    isDefaultPassword: false
  }),

  actions: {
    async login(password) {
      this.loading = true
      this.error = null
      
      try {
        const result = await invoke('login', {
          request: { password }
        })
        
        if (result.success) {
          this.isAuthenticated = true
          this.sessionToken = result.data.sessionToken
          localStorage.setItem('sessionToken', result.data.sessionToken)
          
          // Check if still using default password
          await this.checkDefaultPassword()
          
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

    async changePassword(currentPassword, newPassword) {
      this.loading = true
      this.error = null
      
      try {
        const result = await invoke('change_password', {
          request: { 
            currentPassword, 
            newPassword 
          }
        })
        
        if (result.success) {
          this.isDefaultPassword = false
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

    async checkDefaultPassword() {
      try {
        const result = await invoke('check_default_password')
        if (result.success) {
          this.isDefaultPassword = result.data
        }
      } catch (error) {
        console.error('Failed to check default password:', error)
      }
    },

    async loadSession() {
      const token = localStorage.getItem('sessionToken')
      if (token) {
        this.sessionToken = token
        this.isAuthenticated = true
        await this.checkDefaultPassword()
      }
    },

    logout() {
      this.isAuthenticated = false
      this.sessionToken = null
      this.error = null
      this.isDefaultPassword = false
      localStorage.removeItem('sessionToken')
    }
  }
})