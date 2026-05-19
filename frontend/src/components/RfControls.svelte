<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams);
</script>

<div class="panel">
  <div class="panel-title">RF 射频</div>
  {@render slider("射频增益", p.rfg, 0, 100, (v) => sendCommand('set_rfg', { gain: v }))}
  {@render slider("中频增益", p.ifg, 0, 80, (v) => sendCommand('set_ifg', { gain: v }))}
  {@render slider("禁噪SQL", p.sql, 0, 20, (v) => sendCommand('set_sql', { level: v }))}
  {@render slider("自动增益", p.agc, 0, 5, (v) => sendCommand('set_agc', { level: v }))}
  <label style="margin-top:4px;">前级放大器</label>
  <div class="flex-row">
    <button class:active={p.amp === 0} onclick={() => sendCommand('set_amp', { mode: 0 })}>AMP-A</button>
    <button class:active={p.amp === 1} onclick={() => sendCommand('set_amp', { mode: 1 })}>AMP-B</button>
  </div>
</div>

{#snippet slider(label, value, min, max, onChange)}
<div class="slider-row">
  <label>{label}</label>
  <span class="slider-val">{value}</span>
  <input type="range" {min} {max} {value} oninput={(e) => onChange(parseInt(e.target.value))} />
</div>
{/snippet}

<style>
  .slider-row { margin-bottom: 2px; }
  .slider-row label { margin-bottom: 0; font-size: 10px; display: inline; }
  .slider-val { float: right; font-family: var(--font-mono); font-size: 11px; color: var(--text-primary); }
  input[type="range"] { margin: 2px 0 6px 0; }
</style>
