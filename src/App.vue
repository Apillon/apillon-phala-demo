<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import HelloWorld from "./components/HelloWorld.vue";

interface Window {
  ethereum: any
}

</script>

<template>
  <header>
  </header>

  <RouterView />
</template>

<style scoped>

</style>