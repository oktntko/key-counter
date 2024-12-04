<script setup lang="ts">
import { Tauri, type Heatmap } from '~/middleware/tauri';

const heatmap = ref<Heatmap>({});
onMounted(async () => {
  heatmap.value = await Tauri.load_key_heatmap();
});
const doubleQuoteSingleQuote = ['"', "'"];
</script>

<template>
  <div class="flex items-center justify-center gap-1 p-4">
    <div class="flex flex-col gap-1">
      <div class="flex flex-col">
        <div class="flex">
          <!-- R4 -->
          <Key :codes="['esc']" class="rainbow mr-12" :heatmap="heatmap"> </Key>
          <Key :codes="['F1']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F2']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F3']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F4']" class="rainbow mr-6" :heatmap="heatmap"> </Key>
          <Key :codes="['F5']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F6']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F7']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F8']" class="rainbow mr-6" :heatmap="heatmap"> </Key>
          <Key :codes="['F9']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F10']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F11']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['F12']" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
      </div>
      <div class="flex flex-col">
        <div class="flex">
          <!-- R3 -->
          <Key :codes="['~', '`']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['!', '1']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['@', '2']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['#', '3']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['$', '4']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['%', '5']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['^', '6']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['&', '7']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['*', '8']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['(', '9']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="[')', '0']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['_', '-']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['+', '=']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['backspace']" :size="2" class="rainbow" :heatmap="heatmap"> bksp </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key :codes="['tab']" :size="1.5" class="rainbow" :heatmap="heatmap"> tab </Key>
          <Key :codes="['q']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['w']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['e']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['r']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['t']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['y']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['u']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['i']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['o']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['p']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['{', '[']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['}', ']']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['|', '\\']" :size="1.5" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key :codes="['capslock']" :size="1.75" class="rainbow" :heatmap="heatmap">
            <span> caps </span> <span> lock </span>
          </Key>
          <Key :codes="['a']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['s']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['d']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['f']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['g']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['h']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['j']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['k']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['l']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="[':', ';']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="doubleQuoteSingleQuote" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['return']" :size="2.25" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key :codes="['shiftleft']" :size="2.25" class="rainbow !flex-row" :heatmap="heatmap">
            <span> l </span> <span> shift </span>
          </Key>
          <Key :codes="['z']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['x']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['c']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['v']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['b']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['n']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['m']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['<', ',']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['>', '.']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['?', '/']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['shiftright']" :size="2.75" class="rainbow !flex-row" :heatmap="heatmap">
            <span> r </span> <span> shift </span>
          </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key :codes="['controlleft']" :size="1.25" class="rainbow !flex-row" :heatmap="heatmap">
            <span> l </span> <span> ctrl </span>
          </Key>
          <Key :codes="['metaleft']" :size="1.25" class="rainbow !flex-row" :heatmap="heatmap">
            <span> l </span> <span> gui </span>
          </Key>
          <Key :codes="['alt']" :size="1.25" class="rainbow !flex-row" :heatmap="heatmap">
            <span> l </span> <span> alt </span>
          </Key>
          <Key :codes="['space']" :size="6.25" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['alt']" :size="1.25" class="rainbow !flex-row" :heatmap="heatmap">
            <span> r </span> <span> alt </span>
          </Key>
          <Key :codes="['metaright']" :size="1.25" class="rainbow !flex-row" :heatmap="heatmap">
            <span> r </span> <span> gui </span>
          </Key>
          <Key :codes="['unknown(93)']" :size="1.25" class="rainbow" :heatmap="heatmap">
            <span> menu </span>
          </Key>
          <Key :codes="['controlright']" :size="1.25" class="rainbow !flex-row" :heatmap="heatmap">
            <span> r </span> <span> ctrl </span>
          </Key>
        </div>
      </div>
    </div>

    <div class="flex flex-col gap-1">
      <div class="flex flex-col">
        <div class="flex">
          <!-- R4 -->
          <Key :codes="['printscreen']" class="rainbow" :heatmap="heatmap">
            <div>print</div>
            <div>screen</div>
          </Key>
          <Key :codes="['scrolllock']" class="rainbow" :heatmap="heatmap">
            <span> scroll </span> <span> lock </span>
          </Key>
          <Key :codes="['pause']" class="rainbow" :heatmap="heatmap"> <span> pause </span> </Key>
        </div>
      </div>
      <div class="flex flex-col">
        <div class="flex">
          <!-- R3 -->
          <Key :codes="['insert']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['home']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['pageup']" class="rainbow" :heatmap="heatmap">
            <span> page </span> <span> up </span>
          </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key :codes="['del']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['end']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['pagedown']" class="rainbow" :heatmap="heatmap">
            <span> page </span> <span> down </span>
          </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key> </Key>
          <Key> </Key>
          <Key> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key> </Key>
          <Key :codes="['up']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key :codes="['left']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['down']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['right']" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
      </div>
    </div>

    <div class="flex flex-col gap-1">
      <div class="flex flex-col">
        <div class="flex">
          <!-- R4 -->
          <Key> </Key>
          <Key> </Key>
          <Key> </Key>
          <Key> </Key>
        </div>
      </div>
      <div class="flex flex-col">
        <div class="flex">
          <!-- R3 -->
          <Key :codes="['numlock']" class="rainbow" :heatmap="heatmap">
            <span> num </span> <span> lock </span>
          </Key>
          <Key :codes="['/']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['*']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="[',']" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key :codes="['7']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['8']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['9']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['+']" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key :codes="['4']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['5']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['6']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="[',']" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key :codes="['1']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['2']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['3']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['=']" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key :codes="['0']" :size="2" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['.']" class="rainbow" :heatmap="heatmap"> </Key>
          <Key :codes="['enter']" class="rainbow" :heatmap="heatmap"> </Key>
        </div>
      </div>
    </div>
  </div>
</template>
