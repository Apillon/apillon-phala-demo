<script lang="ts" setup>
defineProps({
  size: { type: String, default: '2rem' },
  color: { type: String, default: '#c23432' },
  width: { type: String, default: '0.25rem' },
});
</script>

<template>
  <span class="inline-flex items-center justify-center">
    <span
      :class="$style.spinner"
      :style="{
        width: size,
        height: size,
        'border-left-color': color,
        'border-width': width,
      }"
    />
  </span>
</template>

<style lang="postcss" module>
.spinner {
  display: inline-block;
  border-radius: 50%;
  position: relative;
  text-indent: -9999rem;
  border: 0.25rem solid rgba(255, 255, 255, 0.35);
  border-left: 0.25rem solid #ffffff;
  transform: translateZ(0);
  animation: load 1.1s infinite linear;

  &:after {
    border-radius: 50%;
    width: 100%;
    height: 100%;
  }

  @keyframes load {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
}
</style>
