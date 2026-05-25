<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { invoke } from '@tauri-apps/api/core';
  const KT = ['AUTO-L','AUTO-R','KEY'];
  let sv=$state(8),sf=$state(60),td=$state(10),ks=$state(20),tr=$state(false),dc=$state(false),dt=$state(10);
  let on = $state(true);
  function toggle(v){on=v;invoke('set_poll_toggle',{poll:'cw',on:v});}
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-4">
  <div class="flex items-center justify-between mb-3 pb-2 border-b border-base-300"><span class="text-sm font-semibold text-base-content/50 uppercase tracking-widest">CW</span><input type="checkbox" class="toggle toggle-success" checked={on} onchange={(e)=>toggle(e.target.checked)} /></div>
  <span class="text-sm font-medium text-base-content/60 block mb-1.5">电键</span>
  <div class="flex gap-1 mb-2">
    {#each KT as n,i}<button class="btn btn-sm flex-1 text-sm" onclick={()=>sendCommand('set_key_type',{key_type:i})}>{n}</button>{/each}
  </div>
  {@render slider("侧音音量",sv,0,15,(v)=>{sv=v;sendCommand('set_sidetone_vol',{vol:v})})}
  {@render slider("侧音频率",sf,40,200,(v)=>{sf=v;sendCommand('set_sidetone_freq',{freq:v})})}
  {@render slider("收发延时",td,0,50,(v)=>{td=v;sendCommand('set_txrx_delay',{delay:v})})}
  {@render slider("自动键速",ks,5,48,(v)=>{ks=v;sendCommand('set_key_speed',{speed:v})})}
  {@render slider("解码阈值",dt,1,50,(v)=>{dt=v;sendCommand('set_cw_decode_threshold',{value:v})})}
  <div class="flex gap-1">
    <button class="btn btn-sm flex-1 text-sm {tr?'btn-primary':'btn-ghost'}" onclick={()=>{tr=!tr;sendCommand('set_cw_training',{on:tr})}}>{tr?'练习ON':'练习OFF'}</button>
    <button class="btn btn-sm flex-1 text-sm {dc?'btn-primary':'btn-ghost'}" onclick={()=>{dc=!dc;sendCommand('set_cw_decode',{on:dc})}}>{dc?'解码ON':'解码OFF'}</button>
  </div>
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
