<template>
  <div class="animation-loader">
    <svg
      width="220"
      height="220"
      viewBox="0 0 220 220"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path :d="dotRight" fill="#F0F2DA" />
      <path :d="dotCenter" fill="#F0F2DA" />
      <path :d="dotLeft" fill="#F0F2DA" />
      <path :d="triangleLeft" fill="#F0F2DA" />
      <path :d="triangleRight" fill="#F0F2DA" />
    </svg>
  </div>
</template>

<script lang="ts" setup>
const i = ref<number>(0);

const dotLeft = computed(() => {
  return i.value < 30
    ? 'M110 140C126.569 140 140 126.569 140 110C140 93.4315 126.569 80 110 80C93.4315 80 80 93.4315 80 110C80 126.569 93.4315 140 110 140Z'
    : i.value < 40
    ? 'M50 140C66.5685 140 80 126.569 80 110C80 93.4315 66.5685 80 50 80C33.4315 80 20 93.4315 20 110C20 126.569 33.4315 140 50 140Z'
    : i.value < 95
    ? 'M50 140C66.5685 140 80 126.569 80 110C80 93.4315 66.5685 80 50 80C33.4315 80 20 93.4315 20 110C20 126.569 33.4315 140 50 140Z'
    : '';
});
const dotCenter = computed(() => {
  return i.value < 95
    ? 'M110 140C126.569 140 140 126.569 140 110C140 93.4315 126.569 80 110 80C93.4315 80 80 93.4315 80 110C80 126.569 93.4315 140 110 140Z'
    : '';
});
const dotRight = computed(() => {
  return i.value < 30
    ? 'M110 140C126.569 140 140 126.569 140 110C140 93.4315 126.569 80 110 80C93.4315 80 80 93.4315 80 110C80 126.569 93.4315 140 110 140Z'
    : i.value < 40
    ? 'M170 140C186.569 140 200 126.569 200 110C200 93.4315 186.569 80 170 80C153.431 80 140 93.4315 140 110C140 126.569 153.431 140 170 140Z'
    : i.value < 95
    ? 'M170 140C186.569 140 200 126.569 200 110C200 93.4315 186.569 80 170 80C153.431 80 140 93.4315 140 110C140 126.569 153.431 140 170 140Z'
    : '';
});
const triangleLeft = computed(() => {
  return i.value < 98
    ? 'M110 88V133L92 110.5L110 88Z'
    : i.value < 120
    ? 'M110 35V185L50 110L110 35Z'
    : i.value < 130
    ? 'M110 50V170L50 110L110 50Z'
    : i.value < 140
    ? 'M110 35V185L50 110L110 35Z'
    : 'M190 50V170L110 110L190 50Z';
});
const triangleRight = computed(() => {
  return i.value < 98
    ? 'M110 133V88L128 110.5L110 133Z'
    : i.value < 120
    ? 'M110 185V35L170 110L110 185Z'
    : i.value < 130
    ? 'M110 170V50L170 110L110 170Z'
    : i.value < 140
    ? 'M110 185V35L170 110L110 185Z'
    : 'M30 170V50L110 110L30 170Z';
});

setInterval(() => {
  i.value = i.value > 200 ? 0 : i.value + 1;
}, 10);
</script>

<style lang="postcss">
.animation-loader > svg {
  width: 100%;
  path {
    transition: all 0.3s;

    &.dot-left {
      transform: translate(60px);
      animation: dotLeft 4s ease-in-out infinite;
    }
    &.dot-center {
      animation: dotCenter 4s ease-in-out infinite;
    }
    &.dot-right {
      transform: translate(-60px);
      animation: dotRight 4s ease-in-out infinite;
    }
    &.triangle-left {
      animation: triangleLeft 4s ease-in-out infinite;
    }
    &.triangle-right {
      animation: triangleRight 4s ease-in-out infinite;
    }
  }
}

@keyframes dotLeft {
  0% {
    transform: translate(60px);
    opacity: 1;
  }
  20% {
    transform: translate(0px);
    opacity: 1;
  }
  25% {
    opacity: 0;
  }
  100% {
    transform: translate(0px);
    opacity: 0;
  }
}
@keyframes dotCenter {
  0% {
    opacity: 1;
  }
  20% {
    opacity: 1;
  }
  25% {
    opacity: 0;
  }
  100% {
    opacity: 0;
  }
}
@keyframes dotRight {
  0% {
    transform: translate(-60px);
    opacity: 1;
  }
  20% {
    transform: translate(0px);
    opacity: 1;
  }
  25% {
    opacity: 0;
  }
  100% {
    transform: translate(0px);
    opacity: 0;
  }
}
@keyframes triangleLeft {
  0% {
    transform: scale(1) translate(0, 0);
    opacity: 0;
  }
  25% {
    opacity: 1;
  }
  29% {
    transform: scale(1) translate(0, 0);
  }
  30% {
    transform: scale(3.3333) translate(-35%, -35%);
  }
  35% {
    transform: scale(2.6666) translate(-31%, -31%);
  }
  50% {
    transform: scale(4.3333, 2.6666) translate(-30%, -31%);
  }
  80% {
    transform: scale(4.3333, 2.6666) translate(-30%, -31%);
  }
  99% {
    transform: scale(4.3333, 2.6666) translate(-30%, -31%);
    opacity: 1;
  }
  100% {
    transform: scale(1) translate(0, 0);
    opacity: 0;
  }
}
@keyframes triangleRight {
  0% {
    transform: scale(1) translate(0, 0);
    opacity: 0;
  }
  25% {
    opacity: 1;
  }
  29% {
    transform: scale(1) translate(0, 0);
  }
  30% {
    transform: scale(3.3333) translate(-35%, -35%);
  }
  35% {
    transform: scale(2.6666) translate(-31%, -31%);
  }
  50% {
    transform: scale(4.3333, 2.6666) translate(-46%, -31%);
  }
  80% {
    transform: scale(4.3333, 2.6666) translate(-46%, -31%);
  }
  99% {
    transform: scale(4.3333, 2.6666) translate(-46%, -31%);
    opacity: 1;
  }
  100% {
    transform: scale(1) translate(0, 0);
    opacity: 0;
  }
}
</style>
