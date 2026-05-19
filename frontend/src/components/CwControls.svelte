<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  const KEY_TYPES = ['AUTO-L', 'AUTO-R', 'KEY'];

  let sidetoneVol = $state(8);
  let sidetoneFreq = $state(60);
  let txDelay = $state(10);
  let keySpeed = $state(20);
  let cwTraining = $state(false);
  let cwDecode = $state(false);
  let cwDecodeThr = $state(10);
</script>

<div class="panel">
  <div class="panel-title">CW</div>

  <label>电键</label>
  <div class="flex-row" style="margin-bottom:6px;">
    {#each KEY_TYPES as name, i}
      <button onclick={() => sendCommand('set_key_type', { key_type: i })} style="flex:1;font-size:10px;">{name}</button>
    {/each}
  </div>

  <Slider label="侧音音量" value={sidetoneVol} min={0} max={15}
    onchange={(v) => { sidetoneVol = v; sendCommand('set_sidetone_vol', { vol: v }); }} />
  <Slider label="侧音频率" value={sidetoneFreq} min={40} max={200}
    onchange={(v) => { sidetoneFreq = v; sendCommand('set_sidetone_freq', { freq: v }); }} />
  <Slider label="收发延时" value={txDelay} min={0} max={50}
    onchange={(v) => { txDelay = v; sendCommand('set_txrx_delay', { delay: v }); }} />
  <Slider label="自动键速" value={keySpeed} min={5} max={48}
    onchange={(v) => { keySpeed = v; sendCommand('set_key_speed', { speed: v }); }} />
  <Slider label="解码阈值" value={cwDecodeThr} min={1} max={50}
    onchange={(v) => { cwDecodeThr = v; sendCommand('set_cw_decode_threshold', { value: v }); }} />

  <div class="flex-row" style="margin-top:4px;">
    <button class:active={cwTraining} onclick={() => { cwTraining = !cwTraining; sendCommand('set_cw_training', { on: cwTraining }); }} style="flex:1;">
      {cwTraining ? '练习ON' : '练习OFF'}
    </button>
    <button class:active={cwDecode} onclick={() => { cwDecode = !cwDecode; sendCommand('set_cw_decode', { on: cwDecode }); }} style="flex:1;">
      {cwDecode ? '解码ON' : '解码OFF'}
    </button>
  </div>
</div>

{#snippet Slider(s)}<div class="slider-row"><label>{s.label}</label><span class="slider-val">{s.value}</span><input type="range" min={s.min} max={s.max} value={s.value} oninput={(e) => s.onchange(parseInt(e.target.value))} /></div>{/snippet}

<style>
  .slider-row { margin-bottom: 2px; }
  .slider-row label { margin-bottom: 0; font-size: 10px; display: inline; }
  .slider-val { float: right; font-family: var(--font-mono); font-size: 11px; color: var(--text-primary); }
  input[type="range"] { margin: 2px 0 6px 0; }
</style>
