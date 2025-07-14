<script setup>
import { onMounted } from 'vue'
import { useAuthStore } from './stores/auth'
import Layout from './components/Layout.vue'

const authStore = useAuthStore()

onMounted(() => {
  authStore.loadSession()
})
</script>

<template>
  <div id="app">
    <Layout v-if="authStore.isAuthenticated" />
    <router-view v-else />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background-color: #f8f9fa;
  color: #333;
}

#app {
  min-height: 100vh;
}

@media (prefers-color-scheme: dark) {
  body {
    background-color: #1a1a1a;
    color: #e0e0e0;
  }
}
</style>
