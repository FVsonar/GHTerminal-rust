<script>
  import { radioStatus } from '../lib/store.js';
  import { MODES } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
  let freq = $derived(s.v === 0 ? s.fA : s.fB);
  let mode = $derived(s.v === 0 ? s.mA : s.mB);

  function fmtFreq(f) {
    const mhz = (f / 1_000_000).toFixed(6);
    const [int, dec] = mhz.split('.');
    return { int, dec };
  }
  let f = $derived(fmtFreq(freq));
  let txLabel = $derived(s.tx ? 'TX' : 'RX');
</script>

<div class="status-bar">
  <div class="freq-display">
    <span class="freq-int">{f.int}</span>
    <span class="freq-dot">.</span>
    <span class="freq-dec">{f.dec}</span>
    <span class="freq-unit">MHz</span>
  </div>

  <span class="mode-badge" class:tx-mode={s.tx}>{MODES[mode] || 'USB'}</span>

  <div class="tx-indicator" class:tx-active={s.tx}>
    <div class="tx-dot"></div>
    <span class="tx-label">{txLabel}</span>
  </div>

  <div class="status-info">
    <span class="volt">{s.volt.toFixed(1)}V</span>
    <span class="utc">
      {String(s.utc[0]).padStart(2,'0')}:{String(s.utc[1]).padStart(2,'0')}:{String(s.utc[2]).padStart(2,'0')}
    </span>
  </div>
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    gap: 16px;
    flex: 1;
  }

  .freq-display {
    font-family: var(--font-mono);
    font-size: 22px;
    font-weight: 600;
    color: var(--accent-green);
    letter-spacing: 1px;
    display: flex;
    align-items: baseline;
    gap: 0;
  }
  .freq-int { color: var(--accent-green); }
  .freq-dot { color: var(--accent-green); }
  .freq-dec { color: rgba(0,229,160,0.5); font-size: 16px; }
  .freq-unit {
    font-size: 11px;
    color: var(--text-muted);
    margin-left: 4px;
    font-weight: 400;
  }

  .mode-badge {
    font-size: 12px;
    font-weight: 600;
    background: rgba(59,130,246,0.2);
    color: var(--accent-blue);
    padding: 3px 10px;
    border-radius: 4px;
    letter-spacing: 1px;
  }
  .mode-badge.tx-mode {
    background: rgba(239,68,68,0.2);
    color: var(--accent-red);
  }

  .tx-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .tx-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent-green);
    box-shadow: 0 0 6px rgba(0,229,160,0.5);
    transition: all 0.2s;
  }
  .tx-active .tx-dot {
    background: var(--accent-red);
    box-shadow: 0 0 10px rgba(239,68,68,0.8);
    animation: tx-pulse 0.5s infinite;
  }
  .tx-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--accent-green);
  }
  .tx-active .tx-label { color: var(--accent-red); }

  @keyframes tx-pulse {
    0%, 100% { box-shadow: 0 0 10px rgba(239,68,68,0.8); }
    50% { box-shadow: 0 0 20px rgba(239,68,68,1.2); }
  }

  .status-info {
    display: flex;
    gap: 12px;
    margin-left: auto;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-secondary);
  }
</style>
