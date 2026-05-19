<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';

  let p = $derived($radioParams);
</script>

<div>
  <div class="panel-title">RF 射频</div>

  <label>射频增益 (RFG): {p.rfg}</label>
  <input type="range" min="0" max="100" value={p.rfg}
    oninput={(e) => sendCommand('set_rfg', { gain: parseInt(e.target.value) })} />

  <label>中频增益 (IFG): {p.ifg}</label>
  <input type="range" min="0" max="80" value={p.ifg}
    oninput={(e) => sendCommand('set_ifg', { gain: parseInt(e.target.value) })} />

  <label>禁噪 (SQL): {p.sql}</label>
  <input type="range" min="0" max="20" value={p.sql}
    oninput={(e) => sendCommand('set_sql', { level: parseInt(e.target.value) })} />

  <label>自动增益 (AGC): {p.agc}</label>
  <input type="range" min="0" max="5" value={p.agc}
    oninput={(e) => sendCommand('set_agc', { level: parseInt(e.target.value) })} />

  <label>前级放大器</label>
  <div class="flex-row">
    <button class:active={p.amp === 0}
      onclick={() => sendCommand('set_amp', { mode: 0 })}>AMP-A</button>
    <button class:active={p.amp === 1}
      onclick={() => sendCommand('set_amp', { mode: 1 })}>AMP-B</button>
  </div>
</div>
