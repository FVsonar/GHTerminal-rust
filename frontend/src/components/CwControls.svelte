<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { KEY_TYPES } from '../lib/radio-types.js';

  let sidetoneVol = $state(8);
  let sidetoneFreq = $state(60);
  let txDelay = $state(10);
  let keySpeed = $state(20);
  let cwTraining = $state(false);
  let cwDecode = $state(false);
  let cwDecodeThreshold = $state(10);
</script>

<div>
  <div class="panel-title">CW</div>

  <label>电键类型</label>
  <div class="flex-row">
    {#each KEY_TYPES as name, i}
      <button onclick={() => sendCommand('set_key_type', { key_type: i })}>
        {name}
      </button>
    {/each}
  </div>

  <label>侧音音量: {sidetoneVol}</label>
  <input type="range" min="0" max="15" value={sidetoneVol}
    oninput={(e) => { sidetoneVol = parseInt(e.target.value); sendCommand('set_sidetone_vol', { vol: sidetoneVol }); }} />

  <label>侧音频率: {sidetoneFreq * 10}Hz</label>
  <input type="range" min="40" max="200" step="2" value={sidetoneFreq}
    oninput={(e) => { sidetoneFreq = parseInt(e.target.value); sendCommand('set_sidetone_freq', { freq: sidetoneFreq }); }} />

  <label>收发延时: {txDelay * 40}ms</label>
  <input type="range" min="0" max="50" value={txDelay}
    oninput={(e) => { txDelay = parseInt(e.target.value); sendCommand('set_txrx_delay', { delay: txDelay }); }} />

  <label>自动键速度: {keySpeed}wpm</label>
  <input type="range" min="5" max="48" value={keySpeed}
    oninput={(e) => { keySpeed = parseInt(e.target.value); sendCommand('set_key_speed', { speed: keySpeed }); }} />

  <div class="flex-row" style="margin-top:4px;">
    <button class:active={cwTraining}
      onclick={() => { cwTraining = !cwTraining; sendCommand('set_cw_training', { on: cwTraining }); }}>
      练习 {cwTraining ? '开' : '关'}
    </button>
    <button class:active={cwDecode}
      onclick={() => { cwDecode = !cwDecode; sendCommand('set_cw_decode', { on: cwDecode }); }}>
      解码 {cwDecode ? '开' : '关'}
    </button>
  </div>

  <label style="margin-top:4px;">解码阈值: {cwDecodeThreshold}</label>
  <input type="range" min="1" max="50" value={cwDecodeThreshold}
    oninput={(e) => { cwDecodeThreshold = parseInt(e.target.value); sendCommand('set_cw_decode_threshold', { value: cwDecodeThreshold }); }} />
</div>
