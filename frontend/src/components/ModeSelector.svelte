<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODE_LIST } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
  let currentMode = $derived(s.v === 0 ? s.mA : s.mB);
</script>

<div class="panel">
  <div class="panel-title">模式</div>
  <div class="mode-grid">
    {#each MODE_LIST as m}
      <button
        class="mode-btn"
        class:active={currentMode === m.v}
        onclick={() => sendCommand('set_mode', { mode: m.v })}
      >{m.n}</button>
    {/each}
  </div>
</div>

<style>
  .mode-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 4px;
  }
  .mode-btn {
    padding: 7px 4px;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.5px;
  }
</style>
