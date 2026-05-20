<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODE_LIST } from '../lib/radio-types.js';
  let s = $derived($radioStatus); let cm = $derived(s.v===0?s.mA:s.mB);
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2.5 block">模式</span>
  <div class="grid grid-cols-3 gap-1">
    {#each MODE_LIST as m}
      <button class="btn btn-sm text-xs font-semibold tracking-wide {cm===m.v?'btn-primary':'btn-ghost'}" onclick={()=>sendCommand('set_mode',{mode:m.v})}>{m.n}</button>
    {/each}
  </div>
</div>
