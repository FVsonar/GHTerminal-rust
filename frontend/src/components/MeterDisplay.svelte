<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sMeterValue, poMeterValue, meterData } from '../lib/store.js';
  let sM = $derived($sMeterValue); let pM = $derived($poMeterValue); let m = $derived($meterData);
  let swr = $derived((m.swr&0xC0)===0?m.swr&0x3F:0);
  let alc = $derived((m.swr&0xC0)===0x40?m.swr&0x3F:0);
  let meterOn = $state(true);
  function pct(v) { return Math.min(100,(v/34)*100); }
  function toggle(on) { meterOn=on; invoke('set_poll_toggle',{poll:'meter',on}); }
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm px-3.5 py-2.5">
  <div class="flex items-center justify-between mb-1.5">
    <span class="text-[12px] font-semibold text-base-content/50 uppercase tracking-widest">仪表</span>
    <input type="checkbox" class="toggle toggle-sm toggle-success" checked={meterOn} onchange={(e)=>toggle(e.target.checked)} />
  </div>
  <div class="grid grid-cols-[2fr_1fr_1fr_1fr] gap-2">
    {#each [{label:'S',val:sM,cls:'from-success via-warning to-error'},{label:'PO',val:pM,cls:'bg-error'},{label:'SWR',val:swr,cls:'bg-warning'},{label:'ALC',val:alc,cls:'bg-secondary'}] as it}
    <div class="min-w-0">
      <div class="flex justify-between items-baseline mb-0.5">
        <span class="text-[12px] font-semibold text-base-content/50 uppercase">{it.label}</span>
        <span class="font-mono text-xs font-semibold">{it.val}</span>
      </div>
      <div class="relative h-3.5 bg-base-300 rounded-sm overflow-hidden border border-base-300">
        <div class="h-full rounded-sm transition-[width] duration-200 ease-out {it.cls}" style="width:{pct(it.val)}%; {it.label==='S'?'background-image:linear-gradient(90deg,var(--color-success),var(--color-warning),var(--color-error))':'background-color:var(--color-'+it.cls.split('-')[1]+')'}"></div>
        {#if it.label==='S'}{#each [3,6,9,12,15,18,21,24,27,30] as t}<div class="absolute top-0 w-px h-full bg-white/5" style="left:{(t/34)*100}%"></div>{/each}{/if}
      </div>
    </div>
    {/each}
  </div>
</div>
