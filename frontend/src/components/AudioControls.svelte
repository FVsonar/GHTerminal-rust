<script>
  import { radioSocket } from '../lib/ws.js';
  import { radioParams } from '../lib/store.js';

  let p = $derived($radioParams);

  function sliderCmd(cmd, e) {
    radioSocket.cmd(cmd, { [cmd.split('_')[1] === 'vol' ? 'vol' : cmd === 'set_mic_gain' ? 'gain' : 'value']: parseInt(e.target.value) });
  }
</script>

<div>
  <div class="panel-title">AF 音频</div>

  <label>扬声器音量: {p.sv}</label>
  <input type="range" min="0" max="30" value={p.sv}
    oninput={(e) => radioSocket.cmd('set_speaker_vol', { vol: parseInt(e.target.value) })} />

  <label>耳机音量: {p.hv}</label>
  <input type="range" min="0" max="80" value={p.hv}
    oninput={(e) => radioSocket.cmd('set_headphone_vol', { vol: parseInt(e.target.value) })} />

  <label>MIC 增益: {p.mg}</label>
  <input type="range" min="0" max="100" value={p.mg}
    oninput={(e) => radioSocket.cmd('set_mic_gain', { gain: parseInt(e.target.value) })} />

  <label>压扩比: {p.cmp}</label>
  <input type="range" min="0" max="14" value={p.cmp}
    oninput={(e) => radioSocket.cmd('set_compandor', { ratio: parseInt(e.target.value) })} />

  <label>低音 EQ: {p.bass}</label>
  <input type="range" min="0" max="40" value={p.bass}
    oninput={(e) => radioSocket.cmd('set_bass_eq', { value: parseInt(e.target.value) })} />

  <label>高音 EQ: {p.treb}</label>
  <input type="range" min="0" max="40" value={p.treb}
    oninput={(e) => radioSocket.cmd('set_treble_eq', { value: parseInt(e.target.value) })} />
</div>

<style>
  input[type="range"] {
    width: 100%;
    accent-color: var(--accent-blue);
    height: 4px;
    margin: 4px 0 8px 0;
  }
</style>
