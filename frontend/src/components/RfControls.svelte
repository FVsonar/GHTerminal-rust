<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams);
</script>

<div class="panel">
  <div class="panel-title">RF 射频</div>
  <Slider label="射频增益" value={p.rfg} min={0} max={100} onchange={(v) => sendCommand('set_rfg', { gain: v })} />
  <Slider label="中频增益" value={p.ifg} min={0} max={80} onchange={(v) => sendCommand('set_ifg', { gain: v })} />
  <Slider label="禁噪SQL" value={p.sql} min={0} max={20} onchange={(v) => sendCommand('set_sql', { level: v })} />
  <Slider label="自动增益" value={p.agc} min={0} max={5} onchange={(v) => sendCommand('set_agc', { level: v })} />
  <label style="margin-top:4px;">前级放大器</label>
  <div class="flex-row">
    <button class:active={p.amp === 0} onclick={() => sendCommand('set_amp', { mode: 0 })}>AMP-A</button>
    <button class:active={p.amp === 1} onclick={() => sendCommand('set_amp', { mode: 1 })}>AMP-B</button>
  </div>
</div>

{#snippet Slider(s)}<div class="slider-row"><label>{s.label}</label><span class="slider-val">{s.value}</span><input type="range" min={s.min} max={s.max} value={s.value} oninput={(e) => s.onchange(parseInt(e.target.value))} /></div>{/snippet}

<style>
  .slider-row { margin-bottom: 2px; }
  .slider-row label { margin-bottom: 0; font-size: 10px; display: inline; }
  .slider-val { float: right; font-family: var(--font-mono); font-size: 11px; color: var(--text-primary); }
  input[type="range"] { margin: 2px 0 6px 0; }
</style>
