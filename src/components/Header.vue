<template>
  <nav class="max-w-7xl w-full px-8 py-10 mx-auto flex gap-8 justify-between items-center">
    <div>
      <a href="https://apillon.io/" target="_blank">
        <SvgInclude :name="SvgNames.Logo" />
      </a>
    </div>
    <div>
      <Btn type="builders" @click="showModalInfo"> How it works? </Btn>
    </div>
    <div>
      <Btn v-if="walletConnected" type="secondary" class="in-w-[12rem] bg-bg-dark !text-blue">
        Wallet connected
      </Btn>
      <Btn
        v-else
        type="blue"
        class="min-w-[12rem]"
        :loading="walletLoading"
        @click="$emit('walletConnect')"
        >Wallet connect</Btn
      >
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
import { SvgNames } from '~/components/general/SvgInclude.vue';

const props = defineProps({
  walletConnected: { type: Boolean, default: false },
  walletLoading: { type: Boolean, default: false },
});
defineEmits(['walletConnect']);

const isModalInfoVisible = ref<boolean>(false);
const showModalInfo = () => {
  isModalInfoVisible.value = false;
  setTimeout(() => (isModalInfoVisible.value = true), 1);
};
</script>
