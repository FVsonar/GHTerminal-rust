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

  <Slider label="NR阀值" value={p.nr || 0} min={1} max={200} onchange={(v) => sendCommand('set_nr_threshold', { value: v })} />
  <Slider label="NB阀值" value={p.nb || 0} min={0} max={15} onchange={(v) => sendCommand('set_nb_threshold', { value: v })} />
  <Slider label="PEAK" value={p.pk} min={0} max={20} onchange={(v) => sendCommand('set_peak_threshold', { value: v })} />
</div>

{#snippet Slider(s)}<div class="slider-row"><label>{s.label}</label><span class="slider-val">{s.value}</span><input type="range" min={s.min} max={s.max} value={s.value} oninput={(e) => s.onchange(parseInt(e.target.value))} /></div>{/snippet}

<style>
  .slider-row { margin-bottom: 2px; }
  .slider-row label { margin-bottom: 0; font-size: 10px; display: inline; }
  .slider-val { float: right; font-family: var(--font-mono); font-size: 11px; color: var(--text-primary); }
  input[type="range"] { margin: 2px 0 6px 0; }
</style>
