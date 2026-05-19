<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODES } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
</script>

<div>
  <div class="panel-title">VFO</div>
  <div class="flex-row">
    <button
      class:active={s.v === 0}
      onclick={() => sendCommand('set_ab', { mode: 0 })}
    >
      A: {MODES[s.mA] || 'USB'} {(s.fA / 1000000).toFixed(3)}
    </button>
  </div>
  <div class="flex-row" style="margin-top:4px;">
    <button
      class:active={s.v === 1}
      onclick={() => sendCommand('set_ab', { mode: 1 })}
    >
      B: {MODES[s.mB] || 'USB'} {(s.fB / 1000000).toFixed(3)}
    </button>
  </div>
  <div class="flex-row" style="margin-top:4px;">
    <button onclick={() => sendCommand('status_request')}>
      刷新状态
    </button>
  </div>
</div>

<style>
  button {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 220px;
  }
</style>
