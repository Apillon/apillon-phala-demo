<template>
  <div v-if="nft" class="relative nft">
    <div class="absolute bottom-0 left-0 right-0 px-2 py-1 bg-black/30 text-xs">
      <span>ID: {{ nft.id }}</span> <br />
      <span>{{ nft.name }}</span>
    </div>
    <img :src="nft.image" class="nft_img" :alt="nft.name" />
  </div>
</template>

<script lang="ts" setup>
defineProps({
  nft: { type: Object as PropType<Nft>, default: null },
});
</script>
