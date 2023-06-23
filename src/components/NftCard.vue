<template>
  <div :id="'nft_' + id" class="nft">
    <img :src="src" class="nft_img" :alt="name" />
    <div class="nft_content">
      <h3>{{ name || `#${id}` }}</h3>
      <p>{{ description }}</p>
    </div>
  </div>
</template>

  <script lang="ts" setup>
  defineProps({
    id: { type: Number, default: 0 },
    src: { type: String, default: null },
    name: { type: String, default: null },
    description: { type: String, default: null },
  });
  </script>
  