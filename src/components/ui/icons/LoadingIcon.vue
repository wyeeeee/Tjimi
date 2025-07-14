<template>
  <g>
    <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
  </g>
</template>

<style scoped>
g {
  animation: spin 1s linear infinite;
  transform-origin: center;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>