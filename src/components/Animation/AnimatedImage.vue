<script lang="ts" setup>
const props = defineProps({
  imageUrls: { type: Array<string>, default: [] },
  imageUrlBase: { type: String, default: '' },
  imageFileExt: { type: String, default: 'svg' },
  frameCount: { type: Number, default: 0 },
  frameCountStart: { type: Number, default: 1 },
  resetOnFrame: { type: Number, default: 0 },
  frameTime: { type: Number, default: 100 },
  resetTime: { type: Number, default: 800 },
  inline: { type: Boolean, default: false },

  // for extraStoppedFrames, object should be of type {[frameIndex: number]: timeout in ms}
  extraStoppedFrames: { type: Object, default: {} },
});

const currentFrame = ref(-1);
const constructedImageUrls = ref<String[]>([]);

async function startAnimation() {
  currentFrame.value = 0;

  if (props.imageUrls?.length) {
    constructedImageUrls.value = props.imageUrls;
  } else {
    for (let i = 0; i < props.frameCount; i++) {
      constructedImageUrls.value.push(
        `${props.imageUrlBase}${props.frameCountStart + i}.${props.imageFileExt}`
      );
    }
  }

  while (true) {
    if (currentFrame.value === props.resetOnFrame) {
      await new Promise<void>(resolve => setTimeout(() => resolve(), props.resetTime));
    } else if (props.extraStoppedFrames && props.extraStoppedFrames[currentFrame.value]) {
      await new Promise<void>(resolve =>
        setTimeout(() => resolve(), props.extraStoppedFrames[currentFrame.value])
      );
    } else {
      await new Promise<void>(resolve => setTimeout(() => resolve(), props.frameTime));
    }

    if (currentFrame.value < constructedImageUrls.value.length - 1) {
      currentFrame.value = currentFrame.value + 1;
    } else {
      currentFrame.value = 0;
    }
  }
}

onMounted(() => {
  startAnimation();
});
</script>
<template>
  <img
    v-for="(url, index) of constructedImageUrls"
    :key="index"
    class="inline-block"
    :src="`${url}`"
    :class="{ 'w-full': !props.inline, hidden: index !== currentFrame }"
  />
</template>
