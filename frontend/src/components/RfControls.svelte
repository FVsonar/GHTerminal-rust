<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams);
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2.5 block">RF 射频</span>
  {@render slider("射频增益",p.rfg,0,100,(v)=>sendCommand('set_rfg',{gain:v}))}
  {@render slider("中频增益",p.ifg,0,80,(v)=>sendCommand('set_ifg',{gain:v}))}
  {@render slider("禁噪SQL",p.sql,0,20,(v)=>sendCommand('set_sql',{level:v}))}
  {@render slider("自动增益",p.agc,0,5,(v)=>sendCommand('set_agc',{level:v}))}
  <span class="text-[10px] font-medium text-base-content/60 mt-1 block">前级放大器</span>
  <div class="flex gap-1 mt-1">
    <button class="btn btn-xs flex-1 {p.amp===0?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_amp',{mode:0})}>AMP-A</button>
    <button class="btn btn-xs flex-1 {p.amp===1?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_amp',{mode:1})}>AMP-B</button>
  </div>
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
