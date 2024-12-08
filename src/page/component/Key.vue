<script setup lang="ts">
const {
  size = 1,
  codes = [],
  intensity = 0,
} = defineProps<{
  size?: number;
  codes?: string[];
  intensity?: number;
}>();
</script>

<template>
  <div
    :style="{ width: `${size * 48}px`, height: `${48}px`, '--intensity': intensity }"
    class="rainbow flex shrink-0 flex-col items-center justify-center rounded border-[0.5px] text-xs capitalize"
    :class="[codes.length > 0 ? 'border-gray-900 bg-white' : '!border-transparent !bg-transparent']"
  >
    <slot>
      <span v-for="code of codes" :key="code"> {{ code }} </span>
    </slot>
  </div>
</template>

<style lang="postcss" scoped>
.rainbow {
  --r-1: 255;
  --g-1: 255;
  --b-1: 255;

  --r-2: 255;
  --g-2: 0;
  --b-2: 0;

  background: rgb(
    calc(var(--r-1) - ((var(--r-1) - var(--r-2)) * var(--intensity))),
    calc(var(--g-1) - ((var(--g-1) - var(--g-2)) * var(--intensity))),
    calc(var(--b-1) - ((var(--b-1) - var(--b-2)) * var(--intensity)))
  );
}
</style>
