<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioParams } from '../lib/store.js';
  let p = $derived($radioParams);
</script>

<div class="panel">
  <div class="panel-title">AF 音频</div>
  {@render slider("扬声器", p.sv, 0, 30, (v) => sendCommand('set_speaker_vol', { vol: v }))}
  {@render slider("耳机", p.hv, 0, 80, (v) => sendCommand('set_headphone_vol', { vol: v }))}
  {@render slider("MIC增益", p.mg, 0, 100, (v) => sendCommand('set_mic_gain', { gain: v }))}
  {@render slider("压扩比", p.cmp, 0, 14, (v) => sendCommand('set_compandor', { ratio: v }))}
  {@render slider("低音EQ", p.bass, 0, 40, (v) => sendCommand('set_bass_eq', { value: v }))}
  {@render slider("高音EQ", p.treb, 0, 40, (v) => sendCommand('set_treble_eq', { value: v }))}
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
