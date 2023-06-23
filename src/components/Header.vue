<template>
  <nav class="max-w-7xl w-full px-8 py-4 mx-auto flex gap-8 justify-between items-center">
    <div>
      <a href="https://apillon.io/" target="_blank">
        <SvgInclude :name="SvgNames.Logo" />
      </a>
    </div>
    <div>
      <a href="https://wiki.apillon.io/about/4-how-does-apillon-work.html" target="_blank">
        How it works?
      </a>
    </div>
    <div>
      <Btn v-if="walletConnected" type="secondary" class="bg-bg-dark !text-blue">
        Wallet connected
      </Btn>
      <Btn v-else type="blue" @click="$emit('walletConnect')">Wallet connect</Btn>
    </div>
  </nav>
</template>
<script lang="ts" setup>
import { SvgNames } from './SvgInclude.vue';

const props = defineProps({
  walletConnected: { type: Boolean, default: false },
});
defineEmits(['walletConnect']);
</script>
