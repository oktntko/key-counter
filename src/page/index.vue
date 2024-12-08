<script setup lang="ts">
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import * as r from 'remeda';
import { Tauri, type Heatmap } from '~/middleware/tauri';
import Key from './component/Key.vue';

const modelValue = ref<{
  span: 'total' | 'month' | 'week' | 'today';
  type: ('alpha' | 'num' | 'mod' | 'symbol')[];
}>({
  span: 'total',
  type: ['alpha', 'num', 'mod', 'symbol'],
});

const heatmap = ref<Heatmap>({});
const autostart = ref<boolean>(false);
onMounted(async () => {
  heatmap.value = await Tauri.load_key_heatmap();
  autostart.value = await isEnabled();
});
const doubleQuoteSingleQuote = ['"', "'"];
function p(props: { size?: number; codes?: string[] }) {
  return {
    size: props.size,
    codes: props.codes,
    intensity: r.sum(props.codes?.map((x) => heatmap.value[x] ?? 0) ?? [0]),
  };
}
</script>

<template>
  <div class="container mx-auto flex max-w-[1072px] flex-col gap-8 py-8 text-center">
    <!-- form -->
    <div class="flex justify-between">
      <div class="flex flex-col gap-2">
        <div class="flex items-start gap-2">
          <label
            v-for="span of ['total', 'month', 'week', 'today']"
            :key="span"
            class="inline-flex items-center text-sm font-medium capitalize text-gray-900"
          >
            <input v-model="modelValue.span" type="radio" :value="span" class="mr-1 h-4 w-4" />
            {{ span }}
          </label>
        </div>
        <div class="flex items-start gap-2">
          <label
            v-for="type of ['alpha', 'num', 'mod', 'symbol']"
            :key="type"
            class="inline-flex items-center text-sm font-medium capitalize text-gray-900"
          >
            <input v-model="modelValue.type" type="checkbox" :value="type" class="mr-1 h-4 w-4" />
            {{ type }}
          </label>
        </div>
      </div>

      <label class="inline-flex cursor-pointer items-center">
        <input
          type="checkbox"
          value=""
          class="peer sr-only"
          :checked="autostart"
          @change="
            async () => {
              if (autostart) {
                await disable();
                autostart = false;
              } else {
                await enable();
                autostart = true;
              }
            }
          "
        />
        <div
          class="peer relative h-6 w-11 rounded-full bg-slate-600 after:absolute after:start-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rtl:peer-checked:after:-translate-x-full"
        ></div>
        <span class="ms-3 text-sm font-medium capitalize text-gray-900">auto start</span>
      </label>
    </div>

    <!-- keyboard -->
    <div class="flex flex-col gap-2">
      <div>
        <div class="inline-flex flex-wrap gap-1 rounded bg-gray-900 p-1">
          <div class="flex flex-col gap-1">
            <div class="flex flex-col">
              <div class="flex">
                <!-- R4 -->
                <Key v-bind="p({ codes: ['esc'] })" class="mr-12"> </Key>
                <Key v-bind="p({ codes: ['F1'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F2'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F3'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F4'] })" class="mr-6"> </Key>
                <Key v-bind="p({ codes: ['F5'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F6'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F7'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F8'] })" class="mr-6"> </Key>
                <Key v-bind="p({ codes: ['F9'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F10'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F11'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['F12'] })" class=""> </Key>
              </div>
            </div>
            <div class="flex flex-col">
              <div class="flex">
                <!-- R3 -->
                <Key v-bind="p({ codes: ['~', '`'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['!', '1'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['@', '2'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['#', '3'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['$', '4'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['%', '5'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['^', '6'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['&', '7'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['*', '8'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['(', '9'] })" class=""> </Key>
                <Key v-bind="p({ codes: [')', '0'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['_', '-'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['+', '='] })" class=""> </Key>
                <Key v-bind="p({ codes: ['backspace'], size: 2 })" class=""> bksp </Key>
              </div>
              <div class="flex">
                <!-- R2 -->
                <Key v-bind="p({ codes: ['tab'], size: 1.5 })" class=""> tab </Key>
                <Key v-bind="p({ codes: ['q'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['w'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['e'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['r'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['t'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['y'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['u'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['i'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['o'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['p'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['{', '['] })" class=""> </Key>
                <Key v-bind="p({ codes: ['}', ']'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['|', '\\'], size: 1.5 })" class=""> </Key>
              </div>
              <div class="flex">
                <!-- R2 -->
                <Key v-bind="p({ codes: ['capslock'], size: 1.75 })" class="">
                  <span> caps </span> <span> lock </span>
                </Key>
                <Key v-bind="p({ codes: ['a'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['s'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['d'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['f'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['g'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['h'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['j'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['k'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['l'] })" class=""> </Key>
                <Key v-bind="p({ codes: [':', ';'] })" class=""> </Key>
                <Key v-bind="p({ codes: doubleQuoteSingleQuote })" class=""> </Key>
                <Key v-bind="p({ codes: ['return'], size: 2.25 })" class=""> </Key>
              </div>
              <div class="flex">
                <!-- R1 -->
                <Key v-bind="p({ codes: ['shiftleft'], size: 2.25 })" class="!flex-row">
                  <span> l </span> <span> shift </span>
                </Key>
                <Key v-bind="p({ codes: ['z'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['x'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['c'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['v'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['b'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['n'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['m'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['<', ','] })" class=""> </Key>
                <Key v-bind="p({ codes: ['>', '.'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['?', '/'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['shiftright'], size: 2.75 })" class="!flex-row">
                  <span> r </span> <span> shift </span>
                </Key>
              </div>
              <div class="flex">
                <!-- R1 -->
                <Key v-bind="p({ codes: ['controlleft'], size: 1.25 })" class="!flex-row">
                  <span> l </span> <span> ctrl </span>
                </Key>
                <Key v-bind="p({ codes: ['metaleft'], size: 1.25 })" class="!flex-row">
                  <span> l </span> <span> gui </span>
                </Key>
                <Key v-bind="p({ codes: ['alt'], size: 1.25 })" class="!flex-row">
                  <span> l </span> <span> alt </span>
                </Key>
                <Key v-bind="p({ codes: ['space'], size: 6.25 })" class=""> </Key>
                <Key v-bind="p({ codes: ['alt'], size: 1.25 })" class="!flex-row">
                  <span> r </span> <span> alt </span>
                </Key>
                <Key v-bind="p({ codes: ['metaright'], size: 1.25 })" class="!flex-row">
                  <span> r </span> <span> gui </span>
                </Key>
                <Key v-bind="p({ codes: ['unknown(93)'], size: 1.25 })" class="">
                  <span> menu </span>
                </Key>
                <Key v-bind="p({ codes: ['controlright'], size: 1.25 })" class="!flex-row">
                  <span> r </span> <span> ctrl </span>
                </Key>
              </div>
            </div>
          </div>

          <div class="flex flex-col gap-1">
            <div class="flex flex-col">
              <div class="flex">
                <!-- R4 -->
                <Key v-bind="p({ codes: ['printscreen'] })" class="">
                  <div>print</div>
                  <div>screen</div>
                </Key>
                <Key v-bind="p({ codes: ['scrolllock'] })" class="">
                  <span> scroll </span> <span> lock </span>
                </Key>
                <Key v-bind="p({ codes: ['pause'] })" class=""> <span> pause </span> </Key>
              </div>
            </div>
            <div class="flex flex-col">
              <div class="flex">
                <!-- R3 -->
                <Key v-bind="p({ codes: ['insert'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['home'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['pageup'] })" class="">
                  <span> page </span> <span> up </span>
                </Key>
              </div>
              <div class="flex">
                <!-- R2 -->
                <Key v-bind="p({ codes: ['del'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['end'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['pagedown'] })" class="">
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
                <Key v-bind="p({ codes: ['up'] })" class=""> </Key>
                <Key> </Key>
              </div>
              <div class="flex">
                <!-- R1 -->
                <Key v-bind="p({ codes: ['left'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['down'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['right'] })" class=""> </Key>
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
                <Key v-bind="p({ codes: ['numlock'] })" class="">
                  <span> num </span> <span> lock </span>
                </Key>
                <Key v-bind="p({ codes: ['/'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['*'] })" class=""> </Key>
                <Key v-bind="p({ codes: [','] })" class=""> </Key>
              </div>
              <div class="flex">
                <!-- R2 -->
                <Key v-bind="p({ codes: ['7'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['8'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['9'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['+'] })" class=""> </Key>
              </div>
              <div class="flex">
                <!-- R2 -->
                <Key v-bind="p({ codes: ['4'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['5'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['6'] })" class=""> </Key>
                <Key v-bind="p({ codes: [','] })" class=""> </Key>
              </div>
              <div class="flex">
                <!-- R1 -->
                <Key v-bind="p({ codes: ['1'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['2'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['3'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['='] })" class=""> </Key>
              </div>
              <div class="flex">
                <!-- R1 -->
                <Key v-bind="p({ codes: ['0'], size: 2 })" class=""> </Key>
                <Key v-bind="p({ codes: ['.'] })" class=""> </Key>
                <Key v-bind="p({ codes: ['enter'] })" class=""> </Key>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="flex gap-1">
        <Key :codes="['']" :intensity="1" class=""> max </Key>
        <Key :codes="['']" :intensity="0.5" class=""> half </Key>
        <Key :codes="['']" :intensity="0.25" class=""> quarter </Key>
        <Key :codes="['']" :intensity="0" class=""> zero </Key>
      </div>
    </div>

    <!-- table -->
    <div class="">
      <table
        class="h-full w-full overflow-x-auto rounded border border-gray-300 bg-gray-50 text-sm"
      >
        <thead class="text-gray-900">
          <tr class="divide-x divide-gray-200">
            <th scope="col" class="column capitalize">No</th>
            <th scope="col" class="column capitalize">key</th>
            <th scope="col" class="column capitalize">ratio</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200 bg-white text-gray-900">
          <tr
            v-for="([key, count], i) of Object.entries(heatmap)"
            :key="key"
            class="divide-x divide-gray-200"
          >
            <td class="max-w-48">{{ i + 1 }}</td>
            <td class="max-w-48">
              <div class="line-clamp-2 min-w-28 px-2 py-1" :title="key">
                {{ key }}
              </div>
            </td>
            <td class="max-w-48">
              <div class="line-clamp-1 min-w-32 px-2 py-1">
                {{ count }}
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
