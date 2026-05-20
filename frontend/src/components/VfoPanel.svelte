<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODES } from '../lib/radio-types.js';
  let s = $derived($radioStatus); let on = $state(true);
  function toggle(v){on=v;invoke('set_poll_toggle',{poll:'status',on:v});}
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <div class="flex items-center justify-between mb-2.5 pb-1.5 border-b border-base-300">
    <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest">VFO</span>
    <input type="checkbox" class="toggle toggle-xs toggle-success" checked={on} onchange={(e)=>toggle(e.target.checked)} />
  </div>
  <div class="flex flex-col gap-1 mb-2">
    {#each [{l:'A',f:s.fA,m:s.mA,v:0},{l:'B',f:s.fB,m:s.mB,v:1}] as vfo}
      <button class="flex items-center gap-2 px-2.5 py-2 rounded-md w-full text-left transition-all {s.v===vfo.v?'bg-primary/20 border-primary/30':'bg-base-300/50'} border border-transparent hover:border-base-300" onclick={()=>sendCommand('set_ab',{mode:vfo.v})}>
        <span class="text-[11px] font-bold px-1.5 py-0.5 rounded text-base-content/50 bg-base-300 min-w-[20px] text-center">{vfo.l}</span>
        <span class="font-mono text-sm font-semibold">{(vfo.f/1_000_000).toFixed(3)}</span>
        <span class="text-[10px] text-base-content/50 ml-auto">{MODES[vfo.m]||'USB'}</span>
      </button>
    {/each}
  </div>
  <button class="btn btn-ghost btn-xs w-full text-base-content/50" onclick={()=>sendCommand('status_request')}>刷新状态</button>
</div>
