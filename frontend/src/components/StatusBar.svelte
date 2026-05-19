<script>
  import { radioStatus, currentFreq, currentMode } from '../lib/store.js';
  import { MODES } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
  let freq = $derived($currentFreq);
  let mode = $derived($currentMode);
</script>

<div class="flex-row" style="gap: 16px;">
  <!-- TX/RX 指示灯 -->
  <div class="flex-row" style="gap: 6px;">
    <div class="tx-indicator" class:rx={!s.tx} class:tx={s.tx}></div>
    <span style="font-size:12px; color: var(--text-secondary);">
      {s.tx ? 'TX' : 'RX'}
    </span>
  </div>

  <!-- 频率 -->
  <span class="freq-display">
    {(freq / 1000000).toFixed(3)} <span style="font-size:14px;">MHz</span>
  </span>

  <!-- 模式 -->
  <span style="font-size:18px; color: var(--accent-blue); font-weight: bold;">
    {MODES[mode] || 'USB'}
  </span>

  <!-- 电压 -->
  <span style="font-size:12px; color: var(--text-secondary);">
    {s.volt.toFixed(1)}V
  </span>

  <!-- UTC -->
  <span style="font-size:12px; color: var(--text-secondary); font-family: var(--font-mono);">
    {String(s.utc[0]).padStart(2,'0')}:{String(s.utc[1]).padStart(2,'0')}:{String(s.utc[2]).padStart(2,'0')}z
  </span>
</div>
