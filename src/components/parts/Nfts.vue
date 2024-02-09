<template>
  <div v-for="(nft, key) in nfts" :key="key" class="relative">
    <input
      v-model="tokenId"
      type="radio"
      name="nft"
      class="absolute"
      :id="`nft_${nft.id}`"
      :value="nft.id"
      :checked="modelValue === nft.id"
      @change="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
    <label :for="`nft_${nft.id}`">
      <NftCard :nft="nft" :open="false" />
    </label>
  </div>
</template>

<script lang="ts" setup>
defineProps({
  nfts: { type: Array<Nft>, default: [] },
  modelValue: { type: Number, default: 0 },
});

const emit = defineEmits(['update:modelValue']);
const tokenId = ref<number>(0);
</script>

<style lang="postcss">
input[type='radio'] + label {
  cursor: pointer;
}
input[type='radio']:checked + label .nft {
  box-shadow: 0 0 1rem 0.25rem rgba(255, 255, 255, 0.5);
}
</style>
