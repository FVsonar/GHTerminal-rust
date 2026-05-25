<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { invoke } from '@tauri-apps/api/core';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams); let on = $state(true);
  function toggle(v){on=v;invoke('set_poll_toggle',{poll:'params',on:v});}
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-4">
  <div class="flex items-center justify-between mb-3 pb-2 border-b border-base-300"><span class="text-sm font-semibold text-base-content/50 uppercase tracking-widest">降噪 / 阈值</span><input type="checkbox" class="toggle toggle-success" checked={on} onchange={(e)=>toggle(e.target.checked)} /></div>
  <div class="flex gap-1 mb-2.5">
    <button class="btn btn-sm flex-1 text-sm {p.nr===1?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_nr',{on:p.nr!==1})}>NR {p.nr===1?'ON':'OFF'}</button>
    <button class="btn btn-sm flex-1 text-sm {p.nb===1?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_nb',{on:p.nb!==1})}>NB {p.nb===1?'ON':'OFF'}</button>
  </div>
  {@render slider("NR阀值",p.nr||0,1,200,(v)=>sendCommand('set_nr_threshold',{value:v}))}
  {@render slider("NB阀值",p.nb||0,0,15,(v)=>sendCommand('set_nb_threshold',{value:v}))}
  {@render slider("PEAK",p.pk,0,20,(v)=>sendCommand('set_peak_threshold',{value:v}))}
</div>

{#snippet slider(label,value,min,max,onChange)}
<div class="mb-1.5">
  <div class="flex justify-between items-baseline mb-1">
    <span class="text-sm font-medium text-base-content/60">{label}</span>
    <span class="font-mono text-base font-semibold">{value}</span>
  </div>
  <input type="range" class="range range-primary" {min} {max} {value} oninput={(e)=>onChange(parseInt(e.target.value))} />
</div>
{/snippet}
