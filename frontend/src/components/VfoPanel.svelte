<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODES } from '../lib/radio-types.js';
  let s = $derived($radioStatus); let on = $state(true); let tm = $state(0); let pttActive = $state(false);
  function toggle(v){on=v;invoke('set_poll_toggle',{poll:'status',on:v});}
  function pttDown(){pttActive=true;sendCommand('ptt',{pressed:true});}
  function pttUp(){pttActive=false;sendCommand('ptt',{pressed:false});}
  let pttOn = $derived(pttActive||s.tx);
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3 flex flex-col gap-3">
  <div class="flex items-center justify-between pb-1.5 border-b border-base-300">
    <span class="text-[12px] font-semibold text-base-content/50 uppercase tracking-widest">操作</span>
    <input type="checkbox" class="toggle toggle-sm toggle-success" checked={on} onchange={(e)=>toggle(e.target.checked)} />
  </div>

  {#if on}
  <!-- VFO -->
  <div>
    <span class="text-[12px] font-medium text-base-content/50 uppercase tracking-wide mb-1.5 block">VFO 通道</span>
    <div class="flex gap-1">
      {#each [{l:'A',f:s.fA,m:s.mA,v:0},{l:'B',f:s.fB,m:s.mB,v:1}] as vfo}
        <button class="flex-1 flex items-center gap-1.5 px-2 py-2 rounded-md text-left transition-all {s.v===vfo.v?'bg-primary/20 border-primary/30':'bg-base-300/50'} border border-transparent hover:border-base-300" onclick={()=>sendCommand('set_ab',{mode:vfo.v})}>
          <span class="text-[12px] font-bold px-1.5 py-0.5 rounded text-base-content/50 bg-base-300 min-w-[18px] text-center">{vfo.l}</span>
          <span class="font-mono text-[13px] font-semibold leading-tight">{(vfo.f/1_000_000).toFixed(3)}</span>
          <span class="text-[13px] text-base-content/50 ml-auto">{MODES[vfo.m]||'USB'}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- PTT -->
  <div>
    <span class="text-[12px] font-medium text-base-content/50 uppercase tracking-wide mb-1.5 block">发射控制</span>
    <button
      class="w-full h-12 flex items-center justify-center gap-2.5 rounded-lg border-2 transition-all duration-150 {pttOn?'bg-error border-error shadow-[0_0_24px_rgba(239,68,68,.6)] animate-[pulse_.6s_infinite]':'bg-base-300 border-base-300 hover:border-error/40 hover:bg-error/10'}"
      onmousedown={pttDown} onmouseup={pttUp} onmouseleave={pttUp}
    >
      <span class="text-xl {pttOn?'text-white':'text-base-content/50'}">⏺</span>
      <span class="text-base font-bold tracking-[2px] {pttOn?'text-white':'text-base-content/50'}">{pttOn?'发射中':'PTT'}</span>
    </button>
  </div>

  <!-- 天调 -->
  <div>
    <span class="text-[12px] font-medium text-base-content/50 uppercase tracking-wide mb-1.5 block">天线调谐</span>
    <div class="flex gap-1">
      {#each [{label:'关闭',desc:'直通'},{label:'开启',desc:'待机'},{label:'调谐',desc:'自动'}] as item,i}
        <button class="btn btn-sm flex-1 text-[13px] {tm===i?'btn-primary':'btn-ghost'}" onclick={()=>{tm=i;sendCommand('set_tuner',{mode:i})}}>
          <span>{item.label}</span>
        </button>
      {/each}
    </div>
  </div>
  {/if}
</div>
