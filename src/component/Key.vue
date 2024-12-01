<script setup lang="ts">
const {
  size = 1,
  code = '',
  empty = false,
} = defineProps<{
  size?: number;
  code?: string;
  empty?: boolean;
}>();
</script>

<template>
  <div
    :style="{ width: `${size * 48}px`, height: `${48}px` }"
    class="flex shrink-0 flex-col items-center justify-center rounded border text-xs capitalize"
    :class="[empty ? 'border-transparent bg-transparent' : 'border-black bg-white']"
  >
    <slot> {{ code }}</slot>
  </div>
</template>
