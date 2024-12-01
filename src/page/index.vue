<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

type Counter = {
  date: string;
  time: string;
  key: string;
  count: string;
};
async function load_counter(): Promise<Counter[]> {
  return invoke('load_counter');
}
</script>

<template>
  <div class="flex items-center justify-center gap-1 p-4">
    <button @click="load_counter">load_counter</button>
    <div class="flex flex-col gap-1">
      <div class="flex flex-col">
        <div class="flex">
          <!-- R4 -->
          <Key code="esc" class="mr-12"> </Key>
          <Key code="F1" class=""> </Key>
          <Key code="F2" class=""> </Key>
          <Key code="F3" class=""> </Key>
          <Key code="F4" class="mr-6"> </Key>
          <Key code="F5" class=""> </Key>
          <Key code="F6" class=""> </Key>
          <Key code="F7" class=""> </Key>
          <Key code="F8" class="mr-6"> </Key>
          <Key code="F9" class=""> </Key>
          <Key code="F10" class=""> </Key>
          <Key code="F11" class=""> </Key>
          <Key code="F12" class=""> </Key>
        </div>
      </div>
      <div class="flex flex-col">
        <div class="flex">
          <!-- R3 -->
          <Key code="`" class=""> <span> ~ </span> <span> ` </span></Key>
          <Key code="1" class=""> <span> ! </span> <span> 1 </span></Key>
          <Key code="2" class=""> <span> @ </span> <span> 2 </span></Key>
          <Key code="3" class=""> <span> # </span> <span> 3 </span></Key>
          <Key code="4" class=""> <span> $ </span> <span> 4 </span></Key>
          <Key code="5" class=""> <span> % </span> <span> 5 </span></Key>
          <Key code="6" class=""> <span> ^ </span> <span> 6 </span></Key>
          <Key code="7" class=""> <span> & </span> <span> 7 </span></Key>
          <Key code="8" class=""> <span> * </span> <span> 8 </span></Key>
          <Key code="9" class=""> <span> ( </span> <span> 9 </span></Key>
          <Key code="0" class=""> <span> ) </span> <span> 0 </span></Key>
          <Key code="-" class=""> <span> _ </span> <span> - </span></Key>
          <Key code="=" class=""> <span> + </span> <span> = </span></Key>
          <Key code="backspace" :size="2" class=""> bksp </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key code="tab" :size="1.5" class=""> tab </Key>
          <Key code="q" class=""> </Key>
          <Key code="w" class=""> </Key>
          <Key code="e" class=""> </Key>
          <Key code="r" class=""> </Key>
          <Key code="t" class=""> </Key>
          <Key code="y" class=""> </Key>
          <Key code="u" class=""> </Key>
          <Key code="i" class=""> </Key>
          <Key code="o" class=""> </Key>
          <Key code="p" class=""> </Key>
          <Key code="[" class=""> <span> { </span> <span> [ </span></Key>
          <Key code="]" class=""> <span> } </span> <span> ] </span></Key>
          <Key code="\" :size="1.5" class=""> <span> | </span> <span> \ </span> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key code="capslock" :size="1.75" class=""> <span> caps </span> <span> lock </span> </Key>
          <Key code="a" class=""> </Key>
          <Key code="s" class=""> </Key>
          <Key code="d" class=""> </Key>
          <Key code="f" class=""> </Key>
          <Key code="g" class=""> </Key>
          <Key code="h" class=""> </Key>
          <Key code="j" class=""> </Key>
          <Key code="k" class=""> </Key>
          <Key code="l" class=""> </Key>
          <Key code=";" class=""> <span> : </span> <span> ; </span></Key>
          <Key code="'" class=""> <span> &quot; </span> <span> &#39; </span></Key>
          <Key code="enter" :size="2.25" class=""> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key code="lshift" :size="2.25" class="!flex-row">
            <span> l </span> <span> shift </span>
          </Key>
          <Key code="z" class=""> </Key>
          <Key code="x" class=""> </Key>
          <Key code="c" class=""> </Key>
          <Key code="v" class=""> </Key>
          <Key code="b" class=""> </Key>
          <Key code="n" class=""> </Key>
          <Key code="m" class=""> </Key>
          <Key code="," class=""> <span> &lt; </span> <span> , </span></Key>
          <Key code="." class=""> <span> &gt; </span> <span> . </span></Key>
          <Key code="/" class=""> <span> ? </span> <span> / </span></Key>
          <Key code="rsihft" :size="2.75" class="!flex-row">
            <span> r </span> <span> shift </span>
          </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key code="lctrl" :size="1.25" class="!flex-row">
            <span> l </span> <span> ctrl </span>
          </Key>
          <Key code="lgui" :size="1.25" class="!flex-row">
            <span> l </span> <span> gui </span>
          </Key>
          <Key code="lalt" :size="1.25" class="!flex-row">
            <span> l </span> <span> alt </span>
          </Key>
          <Key code="space" :size="6.25" class=""> </Key>
          <Key code="ralt" :size="1.25" class="!flex-row">
            <span> r </span> <span> alt </span>
          </Key>
          <Key code="rgui" :size="1.25" class="!flex-row">
            <span> r </span> <span> gui </span>
          </Key>
          <Key code="menu" :size="1.25" class=""> </Key>
          <Key code="rctrl" :size="1.25" class="!flex-row">
            <span> r </span> <span> ctrl </span>
          </Key>
        </div>
      </div>
    </div>

    <div class="flex flex-col gap-1">
      <div class="flex flex-col">
        <div class="flex">
          <!-- R4 -->
          <Key code="printscreen" class="">
            <div>print</div>
            <div>screen</div>
          </Key>
          <Key code="scrolllock" class=""> <span> scroll </span> <span> lock </span> </Key>
          <Key code="pause" class=""> <span> pause </span> </Key>
        </div>
      </div>
      <div class="flex flex-col">
        <div class="flex">
          <!-- R3 -->
          <Key code="insert" class=""> </Key>
          <Key code="home" class=""> </Key>
          <Key code="pageup" class=""> <span> page </span> <span> up </span> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key code="del" class=""> </Key>
          <Key code="end" class=""> </Key>
          <Key code="pagedown" class=""> <span> page </span> <span> down </span> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key empty> </Key>
          <Key empty> </Key>
          <Key empty> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key empty> </Key>
          <Key code="up" class=""> </Key>
          <Key empty> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key code="left" class=""> </Key>
          <Key code="down" class=""> </Key>
          <Key code="right" class=""> </Key>
        </div>
      </div>
    </div>

    <div class="flex flex-col gap-1">
      <div class="flex flex-col">
        <div class="flex">
          <!-- R4 -->
          <Key empty> </Key>
          <Key empty> </Key>
          <Key empty> </Key>
          <Key empty> </Key>
        </div>
      </div>
      <div class="flex flex-col">
        <div class="flex">
          <!-- R3 -->
          <Key code="numlock" class=""> <span> num </span> <span> lock </span> </Key>
          <Key code="/" class=""> </Key>
          <Key code="*" class=""> </Key>
          <Key code="-" class=""> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key code="7" class=""> </Key>
          <Key code="8" class=""> </Key>
          <Key code="9" class=""> </Key>
          <Key code="+" class=""> </Key>
        </div>
        <div class="flex">
          <!-- R2 -->
          <Key code="4" class=""> </Key>
          <Key code="5" class=""> </Key>
          <Key code="6" class=""> </Key>
          <Key code="," class=""> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key code="1" class=""> </Key>
          <Key code="2" class=""> </Key>
          <Key code="3" class=""> </Key>
          <Key code="=" class=""> </Key>
        </div>
        <div class="flex">
          <!-- R1 -->
          <Key code="0" :size="2" class=""> </Key>
          <Key code="." class=""> </Key>
          <Key code="enter" class=""> </Key>
        </div>
      </div>
    </div>
  </div>
</template>
