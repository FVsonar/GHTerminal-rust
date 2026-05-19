<script>
  import { sMeterValue, poMeterValue, meterData } from '../lib/store.js';

  let sMeter = $derived($sMeterValue);
  let poMeter = $derived($poMeterValue);
  let m = $derived($meterData);

  let swrVal = $derived((m.swr & 0xC0) === 0x00 ? m.swr & 0x3F : 0);
  let alcVal = $derived((m.swr & 0xC0) === 0x40 ? m.swr & 0x3F : 0);

  function pct(v, max) { return Math.min(100, (v / max) * 100); }

  const S_LEVELS = ['S0','S1','S2','S3','S4','S5','S6','S7','S8','S9','+10','+20','+30'];
</script>

<div class="meter-row">
  <!-- S 表 -->
  <div class="meter-block">
    <div class="meter-label-row">
      <span class="meter-name">S</span>
      <span class="meter-value">{sMeter}</span>
    </div>
    <div class="meter-track">
      <div class="meter-fill s-fill" style="width:{pct(sMeter, 34)}%"></div>
      {#each [3,6,9,12,15,18,21,24,27,30] as t}
        <div class="meter-tick" style="left:{(t/34)*100}%"></div>
      {/each}
    </div>
  </div>

  <!-- PO 表 -->
  <div class="meter-block">
    <div class="meter-label-row">
      <span class="meter-name">PO</span>
      <span class="meter-value">{poMeter}</span>
    </div>
    <div class="meter-track">
      <div class="meter-fill po-fill" style="width:{pct(poMeter, 34)}%"></div>
    </div>
  </div>

  <!-- SWR -->
  <div class="meter-block">
    <div class="meter-label-row">
      <span class="meter-name">SWR</span>
      <span class="meter-value">{swrVal}</span>
    </div>
    <div class="meter-track">
      <div class="meter-fill swr-fill" style="width:{pct(swrVal, 34)}%"></div>
    </div>
  </div>

  <!-- ALC -->
  <div class="meter-block">
    <div class="meter-label-row">
      <span class="meter-name">ALC</span>
      <span class="meter-value">{alcVal}</span>
    </div>
    <div class="meter-track">
      <div class="meter-fill alc-fill" style="width:{pct(alcVal, 34)}%"></div>
    </div>
  </div>
</div>

<style>
  .meter-row {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr;
    gap: 8px;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 10px 14px;
    box-shadow: var(--shadow-panel);
  }

  .meter-block { min-width: 0; }

  .meter-label-row {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin-bottom: 5px;
  }
  .meter-name {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 1px;
  }
  .meter-value {
    font-family: var(--font-mono);
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .meter-track {
    position: relative;
    height: 14px;
    background: var(--bg-input);
    border-radius: 3px;
    overflow: hidden;
    border: 1px solid var(--border);
  }

  .meter-fill {
    height: 100%;
    border-radius: 2px;
    transition: width 0.2s ease;
  }
  .s-fill { background: linear-gradient(90deg, var(--meter-green), var(--meter-yellow), var(--meter-red)); }
  .po-fill { background: var(--accent-red); }
  .swr-fill { background: var(--accent-amber); }
  .alc-fill { background: var(--accent-purple); }

  .meter-tick {
    position: absolute;
    top: 0;
    width: 1px;
    height: 100%;
    background: rgba(255,255,255,0.08);
  }
</style>
