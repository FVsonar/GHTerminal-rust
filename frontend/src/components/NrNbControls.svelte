<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams);
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2.5 block">降噪 / 阈值</span>
  <div class="flex gap-1 mb-2">
    <button class="btn btn-xs flex-1 {p.nr===1?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_nr',{on:p.nr!==1})}>NR {p.nr===1?'ON':'OFF'}</button>
    <button class="btn btn-xs flex-1 {p.nb===1?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_nb',{on:p.nb!==1})}>NB {p.nb===1?'ON':'OFF'}</button>
  </div>
  {@render slider("NR阀值",p.nr||0,1,200,(v)=>sendCommand('set_nr_threshold',{value:v}))}
  {@render slider("NB阀值",p.nb||0,0,15,(v)=>sendCommand('set_nb_threshold',{value:v}))}
  {@render slider("PEAK",p.pk,0,20,(v)=>sendCommand('set_peak_threshold',{value:v}))}
</div>

{#snippet slider(label,value,min,max,onChange)}
<div class="mb-1">
  <div class="flex justify-between items-baseline mb-1">
    <span class="text-[10px] font-medium text-base-content/60">{label}</span>
    <span class="font-mono text-[11px] font-semibold">{value}</span>
  </div>
  <input type="range" class="range range-xs range-primary" {min} {max} {value} oninput={(e)=>onChange(parseInt(e.target.value))} />
</div>
{/snippet}
