<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  let tm = $state(0);
  const MODES = ['关闭','开启','调谐'];
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2.5 block">天调</span>
  <div class="flex gap-1">
    {#each MODES as n,i}
      <button class="btn btn-xs flex-1 {tm===i?'btn-primary':'btn-ghost'}" onclick={()=>{tm=i;sendCommand('set_tuner',{mode:i})}}>{n}</button>
    {/each}
  </div>
</div>
