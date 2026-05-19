<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams);
</script>

<div class="panel">
  <div class="panel-title">降噪 / 阈值</div>

  <div class="flex-row" style="margin-bottom:8px;">
    <button class:active={p.nr === 1} onclick={() => sendCommand('set_nr', { on: p.nr !== 1 })}>
      NR {p.nr === 1 ? 'ON' : 'OFF'}
    </button>
    <button class:active={p.nb === 1} onclick={() => sendCommand('set_nb', { on: p.nb !== 1 })}>
      NB {p.nb === 1 ? 'ON' : 'OFF'}
    </button>
  </div>

  {@render slider("NR阀值", p.nr || 0, 1, 200, (v) => sendCommand('set_nr_threshold', { value: v }))}
  {@render slider("NB阀值", p.nb || 0, 0, 15, (v) => sendCommand('set_nb_threshold', { value: v }))}
  {@render slider("PEAK", p.pk, 0, 20, (v) => sendCommand('set_peak_threshold', { value: v }))}
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
