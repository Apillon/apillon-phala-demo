<template>
  <Transition name="fade-in">
    <div v-if="modalVisible" class="fixed top-0 bottom-0 left-0 right-0 z-20">
      <div class="fixed top-0 bottom-0 left-0 right-0 bg-black/40"></div>
      <div class="h-full w-full">
        <div
          class="fixed top-0 bottom-0 left-0 right-0 cursor-pointer"
          @click="modalVisible = false"
        ></div>

        <div
          class="absolute top-1/2 left-1/2 w-full max-w-[90vw] lg:max-w-4xl -translate-x-1/2 -translate-y-1/2"
        >
          <button
            class="absolute top-0 right-0 rounded-full p-[14px] border-1 border-white text-white translate-x-1/2 -translate-y-1/2 hover:bg-white hover:text-bg-dark transition-colors duration-300"
            @click="modalVisible = false"
          >
            <SvgInclude :name="SvgNames.Close" />
          </button>
          <div class="max-h-[85vh] overflow-auto bg-bg p-10 lg:p-16" v-bind="$attrs">
            <div v-if="title" class="mb-8">
              <h3>{{ title }}</h3>
            </div>
            <slot />
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script lang="ts" setup>
import { watch, ref } from 'vue';
import { SvgNames } from './SvgInclude.vue';

const props = defineProps({
  show: { type: Boolean, default: false },
  title: { type: String, default: '' },
});

const modalVisible = ref(props.show);

watch(
  () => props.show,
  show => {
    modalVisible.value = show;
  }
);
watch(
  () => modalVisible.value,
  visible => {
    if (visible) {
      document.documentElement.style.overflowY = 'hidden';
    } else {
      document.documentElement.style.overflowY = 'visible';
    }
  }
);
</script>
