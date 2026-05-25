<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams, radioStatus } from '../lib/store.js';
  import { SPECTRUM_SPANS, DISPLAY_MODES } from '../lib/radio-types.js';
  let p = $derived($radioParams), s = $derived($radioStatus), on = $state(true);
  function toggle(v){on=v;invoke('set_poll_toggle',{poll:'spectrum',on:v});}
</script>

<div class="flex items-end gap-3 px-3 py-2 bg-base-200 border border-base-300 rounded-md">
  <div class="flex flex-col items-center gap-1">
    <span class="text-base text-base-content/50 uppercase">轮询</span>
    <input type="checkbox" class="toggle toggle-success" checked={on} onchange={(e)=>toggle(e.target.checked)} />
  </div>
  <div class="flex-1 min-w-0 flex flex-col">
    <span class="text-base text-base-content/50 mb-1">SPAN</span>
    <select class="select select-bordered w-full text-base" value={s.span} onchange={(e)=>sendCommand('set_spectrum_span',{span:parseInt(e.target.value)})}>
      {#each Object.entries(SPECTRUM_SPANS) as [k,v]}<option value={k}>{v}</option>{/each}
    </select>
  </div>
  <div class="flex-1 min-w-0 flex flex-col">
    <span class="text-base text-base-content/50 mb-1">显示</span>
    <select class="select select-bordered w-full text-base" onchange={(e)=>sendCommand('set_display_mode',{mode:parseInt(e.target.value)})}>
      {#each DISPLAY_MODES as m,i}<option value={i}>{m}</option>{/each}
    </select>
  </div>
  <div class="flex-1 min-w-0 flex flex-col">
    <span class="text-base text-base-content/50">REF {p.ref||10}</span>
    <input type="range" class="range range-primary" min="1" max="20" value={p.ref||10} oninput={(e)=>sendCommand('set_spectrum_ref',{value:parseInt(e.target.value)})} />
  </div>
  <div class="flex-1 min-w-0 flex flex-col">
    <span class="text-base text-base-content/50">速率 {p.spd||5}</span>
    <input type="range" class="range range-primary" min="1" max="30" value={p.spd||5} oninput={(e)=>sendCommand('set_spectrum_speed',{value:parseInt(e.target.value)})} />
  </div>
</div>
