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

#app {
  min-height: 100vh;
  width: 100%;
  margin: 0;
  padding: 0;
}

/* Ensure full viewport coverage for login page */
body, html {
  margin: 0;
  padding: 0;
  min-height: 100vh;
  width: 100%;
  overflow-x: hidden;
}
</style>
