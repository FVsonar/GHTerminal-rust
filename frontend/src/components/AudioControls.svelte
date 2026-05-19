<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams);
</script>

<div class="panel">
  <div class="panel-title">AF 音频</div>
  <Slider label="扬声器" value={p.sv} min={0} max={30} onchange={(v) => sendCommand('set_speaker_vol', { vol: v })} />
  <Slider label="耳机" value={p.hv} min={0} max={80} onchange={(v) => sendCommand('set_headphone_vol', { vol: v })} />
  <Slider label="MIC增益" value={p.mg} min={0} max={100} onchange={(v) => sendCommand('set_mic_gain', { gain: v })} />
  <Slider label="压扩比" value={p.cmp} min={0} max={14} onchange={(v) => sendCommand('set_compandor', { ratio: v })} />
  <Slider label="低音EQ" value={p.bass} min={0} max={40} onchange={(v) => sendCommand('set_bass_eq', { value: v })} />
  <Slider label="高音EQ" value={p.treb} min={0} max={40} onchange={(v) => sendCommand('set_treble_eq', { value: v })} />
</div>

{#snippet Slider(s)}<div class="slider-row"><label>{s.label}</label><span class="slider-val">{s.value}</span><input type="range" min={s.min} max={s.max} value={s.value} oninput={(e) => s.onchange(parseInt(e.target.value))} /></div>{/snippet}

<style>
  .slider-row { margin-bottom: 2px; }
  .slider-row label { margin-bottom: 0; font-size: 10px; display: inline; }
  .slider-val { float: right; font-family: var(--font-mono); font-size: 11px; color: var(--text-primary); }
  input[type="range"] { margin: 2px 0 6px 0; }
</style>
