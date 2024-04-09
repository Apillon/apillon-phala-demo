<template>
  <nav
    class="max-w-7xl w-full px-8 py-10 mx-auto flex flex-wrap md:flex-nowrap gap-8 justify-between items-center"
  >
    <div class="md:w-1/3">
      <a href="https://apillon.io/" target="_blank">
        <SvgInclude :name="SvgNames.Logo" />
      </a>
    </div>
    <div class="md:w-1/3 flex justify-center">
      <Btn type="builders" @click="showModalInfo"> How it works? </Btn>
    </div>
    <div class="md:w-1/3 flex justify-end">
      <ConnectWallet />
    </div>
  </nav>

  <Modal :show="isModalInfoVisible" title="How does it work?">
    <img
      src="/images/how-does-it-work.svg"
      class="object-contain w-full"
      alt="how does it work"
      width="691"
      height="464"
  /></Modal>
</template>
<script lang="ts" setup>
import { SvgNames } from '../general/SvgInclude.vue';

const isModalInfoVisible = ref<boolean>(false);
const showModalInfo = () => {
  isModalInfoVisible.value = false;
  setTimeout(() => (isModalInfoVisible.value = true), 1);
};
</script>
