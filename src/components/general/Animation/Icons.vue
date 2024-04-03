<template>
  <div class="flex items-center">
    <NuxtIcon
      v-for="icon in numOfIcons"
      v-show="icon === iconIndex"
      :name="`animation/${animationName}/${icon}`"
      :key="icon"
      :class="btnClass"
      filled
    />
  </div>
</template>

<script lang="ts" setup>
const props = defineProps({
  numOfIcons: { type: Number, default: 0 },
  animationName: { type: String, required: true },
  size: {
    type: String,
    default: 'md',
    validator: (type: string) => ['xs', 'sm', 'md', 'lg', 'xl', 'xxl'].includes(type),
  },
});

/** Animation style and classes */
const $style = useCssModule();
const btnClass = computed(() => {
  return [
    $style.animation,
    `animation-${props.animationName}`,
    { 'text-[40px]': props.size === 'xs' },
    { 'text-[40px]': props.size === 'sm' },
    { 'text-[140px]': props.size === 'md' },
    { 'text-[240px]': props.size === 'lg' },
    { 'text-[340px]': props.size === 'xl' },
    { 'text-[540px]': props.size === 'xxl' },
  ];
});

/** Index of currently visibile icon */
const iconIndex = ref<number>(1);

/** Name of currently visibile icon */
const iconName = computed(() => {
  return `animation/${props.animationName}/${iconIndex.value}`;
});

setInterval(() => {
  iconIndex.value = iconIndex.value === props.numOfIcons ? 1 : iconIndex.value + 1;
}, 300);
</script>

<style lang="postcss" module>
.animation > svg {
  width: 100%;
}
</style>

<style lang="postcss">
.animation-Labirynth > svg rect {
  width: 100%;
  height: 100%;
}
</style>
