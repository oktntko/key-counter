<script setup lang="ts">
import * as r from 'remeda';
import type { Heatmap } from '~/middleware/tauri';

const {
  size = 1,
  codes = [],
  heatmap = {},
} = defineProps<{
  size?: number;
  codes?: string[];
  heatmap?: Heatmap;
}>();

const intensity = computed(() => r.sum(codes.map((x) => heatmap[x] ?? 0)));
</script>

<template>
  <div
    :style="{ width: `${size * 48}px`, height: `${48}px`, '--intensity': intensity }"
    class="flex shrink-0 flex-col items-center justify-center rounded border text-xs capitalize"
    :class="[codes.length > 0 ? 'border-black bg-white' : 'border-transparent bg-transparent']"
  >
    <slot>
      <span v-for="code of codes" :key="code"> {{ code }} </span>
    </slot>
  </div>
</template>

<style lang="postcss" scoped>
.rainbow {
  --hue: calc((1 - var(--intensity)) * 240);
  background: hsl(var(--hue), 100%, 50%);
}
</style>
