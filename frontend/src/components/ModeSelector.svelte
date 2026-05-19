<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODE_LIST } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
  let currentMode = $derived(s.v === 0 ? s.mA : s.mB);
</script>

<div>
  <div class="panel-title">模式</div>
  <div class="flex-row" style="flex-wrap: wrap;">
    {#each MODE_LIST as m}
      <button
        class:active={currentMode === m.v}
        onclick={() => sendCommand('set_mode', { mode: m.v })}
      >
        {m.n}
      </button>
    {/each}
  </div>
</div>
