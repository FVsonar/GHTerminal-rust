<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams, radioStatus } from '../lib/store.js';
  import { SPECTRUM_SPANS, DISPLAY_MODES } from '../lib/radio-types.js';
  import ToggleSwitch from './ToggleSwitch.svelte';

  let p = $derived($radioParams);
  let s = $derived($radioStatus);
  let specOn = $state(true);

  function toggleSpectrum(on) {
    specOn = on;
    invoke('set_poll_toggle', { poll: 'spectrum', on });
  }
</script>

<div class="spectrum-ctrl">
  <div class="ctrl-row">
    <div class="ctrl-item" style="flex:0 0 auto; min-width:36px;">
      <label>轮询</label>
      <ToggleSwitch on={specOn} ontoggle={toggleSpectrum} />
    </div>
    <div class="ctrl-item">
      <label>SPAN</label>
      <select value={s.span} onchange={(e) => sendCommand('set_spectrum_span', { span: parseInt(e.target.value) })}>
        {#each Object.entries(SPECTRUM_SPANS) as [k, v]}<option value={k}>{v}</option>{/each}
      </select>
    </div>
    <div class="ctrl-item">
      <label>显示</label>
      <select onchange={(e) => sendCommand('set_display_mode', { mode: parseInt(e.target.value) })}>
        {#each DISPLAY_MODES as m, i}<option value={i}>{m}</option>{/each}
      </select>
    </div>
    <div class="ctrl-item">
      <label>REF {p.ref || 10}</label>
      <input type="range" min="1" max="20" value={p.ref || 10}
        oninput={(e) => sendCommand('set_spectrum_ref', { value: parseInt(e.target.value) })} />
    </div>
    <div class="ctrl-item">
      <label>速率 {p.spd || 5}</label>
      <input type="range" min="1" max="30" value={p.spd || 5}
        oninput={(e) => sendCommand('set_spectrum_speed', { value: parseInt(e.target.value) })} />
    </div>
  </div>
</div>

<style>
  .spectrum-ctrl {
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    padding: 6px 10px;
  }
  .ctrl-row { display: flex; gap: 10px; align-items: flex-end; }
  .ctrl-item { flex: 1; min-width: 0; }
  .ctrl-item label { font-size: 9px; margin-bottom: 2px; }
  .ctrl-item select { width: 100%; padding: 4px 6px; font-size: 11px; }
  .ctrl-item input[type="range"] { margin: 0; height: 4px; }
</style>
