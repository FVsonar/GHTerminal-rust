<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODE_LIST, BANDS } from '../lib/radio-types.js';
  let s = $derived($radioStatus); let fi = $state(''); let af = $derived(s.v===0?s.fA:s.fB); let cm = $derived(s.v===0?s.mA:s.mB);
  let on = $state(true);
  $effect(()=>{fi=(af/1_000_000).toFixed(6)});
  function sf(){const m=parseFloat(fi);if(isNaN(m)||m<=0||m>2000)return;sendCommand('set_frequency',{freq:Math.round(m*1_000_000)});}
  function step(k){const c=parseFloat(fi)||0;fi=(c+k/1000).toFixed(6);sf();}
  function band(v){const fm=[1.8,3.5,5,7,10,14,18,21,24,28,50,144,430];fi=fm[v].toFixed(6);sendCommand('set_frequency',{freq:Math.round(fm[v]*1_000_000)});}
  function toggle(v){on=v;invoke('set_poll_toggle',{poll:'status',on:v});}
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <div class="flex items-center justify-between mb-2.5 pb-1.5 border-b border-base-300">
    <span class="text-[12px] font-semibold text-base-content/50 uppercase tracking-widest">频率与模式</span>
    <input type="checkbox" class="toggle toggle-sm toggle-success" checked={on} onchange={(e)=>toggle(e.target.checked)} />
  </div>
  <div class="flex items-center gap-2 mb-2.5">
    <input type="text" class="input input-bordered flex-1 font-mono text-lg font-semibold text-success text-center tracking-wider" bind:value={fi} onkeydown={(e)=>{if(e.key==='Enter')sf()}} />
    <span class="text-xs text-base-content/50 font-medium">MHz</span>
  </div>
  <div class="grid grid-cols-4 gap-1 mb-2">
    {#each [[1,'+1K'],[10,'+10K'],[100,'+100K'],[1000,'+1M'],[-1,'-1K'],[-10,'-10K'],[-100,'-100K'],[-1000,'-1M']] as [k,l]}
      <button class="btn btn-sm font-mono" onclick={()=>step(k)}>{l}</button>
    {/each}
  </div>
  <div class="grid grid-cols-6 gap-0.5 mb-2.5">
    {#each BANDS as b}<button class="btn btn-sm text-[12px] font-medium" onclick={()=>band(b.v)}>{b.n}</button>{/each}
  </div>
  <div class="border-t border-base-300 pt-2.5">
    <div class="grid grid-cols-3 gap-1">
      {#each MODE_LIST as m}
        <button class="btn btn-sm text-xs font-semibold tracking-wide {cm===m.v?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_mode',{mode:m.v})}>{m.n}</button>
      {/each}
    </div>
  </div>
</div>
