<template>
  <div class="footer fixed left-0 right-0 bottom-0 py-4 lg:py-8 flex justify-center bg-bg">
    <div class="flex gap-2 items-center">
      Powered by
      <img src="/images/phala.png" class="object-contain" alt="phala" width="82" height="16" />
      <img
        src="/images/moonbeam.svg"
        class="object-contain"
        alt="moonbeam"
        width="83"
        height="16"
      />
      <SvgInclude :name="SvgNames.Crust" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { SvgNames } from '../general/SvgInclude.vue';
</script>
