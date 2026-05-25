<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams); let on = $state(true);
  function toggle(v){on=v;invoke('set_poll_toggle',{poll:'params',on:v});}
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-4">
  <div class="flex items-center justify-between mb-3 pb-2 border-b border-base-300">
    <span class="text-sm font-semibold text-base-content/50 uppercase tracking-widest">AF 音频</span>
    <input type="checkbox" class="toggle toggle-success" checked={on} onchange={(e)=>toggle(e.target.checked)} />
  </div>
  {@render slider("扬声器",p.sv,0,30,(v)=>sendCommand('set_speaker_vol',{vol:v}))}
  {@render slider("耳机",p.hv,0,80,(v)=>sendCommand('set_headphone_vol',{vol:v}))}
  {@render slider("MIC增益",p.mg,0,100,(v)=>sendCommand('set_mic_gain',{gain:v}))}
  {@render slider("压扩比",p.cmp,0,14,(v)=>sendCommand('set_compandor',{ratio:v}))}
  {@render slider("低音EQ",p.bass,0,40,(v)=>sendCommand('set_bass_eq',{value:v}))}
  {@render slider("高音EQ",p.treb,0,40,(v)=>sendCommand('set_treble_eq',{value:v}))}
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
