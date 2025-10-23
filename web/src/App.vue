<script setup lang="ts">
// Minimal App for Relaynaut — Phase 1
</script>

<template>
  <div id="relay-root">
    <h1>Relaynaut</h1>
    <p class="phase">Phase 1</p>
  </div>
</template>

<style scoped>
#relay-root {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
}

h1 {
  margin: 0;
  font-size: 3rem;
  color: inherit;
  /* keep from global style.css */
}

.phase {
  margin-top: 0.5rem;
  color: #888;
}
</style>
