<script>
  import { sMeterValue, poMeterValue, meterData, radioStatus } from '../lib/store.js';

  let sMeter = $derived($sMeterValue);
  let poMeter = $derived($poMeterValue);
  let m = $derived($meterData);
  let s = $derived($radioStatus);

  // SWR/ALC/ADU from swr_aud_alc byte
  let swrVal = $derived((m.swr & 0xC0) === 0x00 ? m.swr & 0x3F : 0);
  let alcVal = $derived((m.swr & 0xC0) === 0x40 ? m.swr & 0x3F : 0);
  let aduVal = $derived((m.swr & 0xC0) === 0x80 ? m.swr & 0x3F : 0);

  function pct(v) { return Math.min(100, (v / 34) * 100); }
</script>

<div>
  <div class="panel-title">仪表</div>

  <!-- S 表 -->
  <div class="meter-row">
    <span class="meter-label">S</span>
    <div class="meter-bar">
      <div class="meter-bar-fill" style="width:{pct(sMeter)}%"></div>
    </div>
    <span class="meter-val">{sMeter}</span>
  </div>

  <!-- PO 表 (发射时有效) -->
  <div class="meter-row">
    <span class="meter-label">PO</span>
    <div class="meter-bar">
      <div class="meter-bar-fill" style="width:{pct(poMeter)}%; background: var(--accent-red);"></div>
    </div>
    <span class="meter-val">{poMeter}</span>
  </div>

  <!-- SWR -->
  <div class="meter-row">
    <span class="meter-label">SWR</span>
    <div class="meter-bar">
      <div class="meter-bar-fill" style="width:{pct(swrVal)}%; background: var(--accent-yellow);"></div>
    </div>
    <span class="meter-val">{swrVal}</span>
  </div>

  <!-- ALC -->
  <div class="meter-row">
    <span class="meter-label">ALC</span>
    <div class="meter-bar">
      <div class="meter-bar-fill" style="width:{pct(alcVal)}%; background: var(--accent-blue);"></div>
    </div>
    <span class="meter-val">{alcVal}</span>
  </div>

  <!-- ADU -->
  <div class="meter-row">
    <span class="meter-label">ADU</span>
    <div class="meter-bar">
      <div class="meter-bar-fill" style="width:{pct(aduVal)}%; background: var(--accent);"></div>
    </div>
    <span class="meter-val">{aduVal}</span>
  </div>
</div>

<style>
  .meter-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 4px;
  }
  .meter-label {
    width: 28px;
    font-size: 11px;
    color: var(--text-secondary);
  }
  .meter-bar {
    flex: 1;
    height: 10px;
    background: var(--border);
    border-radius: 5px;
    overflow: hidden;
  }
  .meter-bar-fill {
    height: 100%;
    background: var(--meter-bar);
    border-radius: 5px;
    transition: width 0.15s;
  }
  .meter-val {
    width: 24px;
    font-size: 11px;
    font-family: var(--font-mono);
    text-align: right;
  }
</style>
