<script setup>
import { onMounted } from 'vue'
import { useAuthStore } from './stores/auth'
import AppLayout from './components/layout/AppLayout.vue'

const authStore = useAuthStore()

onMounted(() => {
  authStore.loadSession()
})
</script>

<template>
  <div id="app">
    <AppLayout v-if="authStore.isAuthenticated" />
    <router-view v-else />
  </div>
</template>

<style>
@import './assets/styles/base.css';
</style>
