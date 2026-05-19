<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { MODES } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
</script>

<div class="panel">
  <div class="panel-title">VFO</div>
  <div class="vfo-list">
    <button class="vfo-btn" class:active={s.v === 0} onclick={() => sendCommand('set_ab', { mode: 0 })}>
      <span class="vfo-label">A</span>
      <span class="vfo-freq">{(s.fA / 1_000_000).toFixed(3)}</span>
      <span class="vfo-mode">{MODES[s.mA] || 'USB'}</span>
    </button>
    <button class="vfo-btn" class:active={s.v === 1} onclick={() => sendCommand('set_ab', { mode: 1 })}>
      <span class="vfo-label">B</span>
      <span class="vfo-freq">{(s.fB / 1_000_000).toFixed(3)}</span>
      <span class="vfo-mode">{MODES[s.mB] || 'USB'}</span>
    </button>
  </div>
  <button class="refresh-btn" onclick={() => sendCommand('status_request')}>刷新状态</button>
</div>

<style>
  .vfo-list { display: flex; flex-direction: column; gap: 4px; margin-bottom: 8px; }
  .vfo-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    width: 100%;
    text-align: left;
  }
  .vfo-label {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-muted);
    background: var(--bg-input);
    padding: 2px 6px;
    border-radius: 3px;
    min-width: 20px;
    text-align: center;
  }
  .vfo-btn.active .vfo-label {
    background: var(--accent-blue);
    color: #fff;
  }
  .vfo-freq {
    font-family: var(--font-mono);
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
  }
  .vfo-mode {
    font-size: 10px;
    color: var(--text-muted);
    margin-left: auto;
  }
  .refresh-btn {
    width: 100%;
    font-size: 11px;
    padding: 5px;
    color: var(--text-muted);
  }
</style>
