<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';

  let s = $derived($radioStatus);
  let pttActive = $state(false);

  function pttDown() { pttActive = true; sendCommand('ptt', { pressed: true }); }
  function pttUp() { pttActive = false; sendCommand('ptt', { pressed: false }); }
</script>

<div class="ptt-panel panel">
  <div class="panel-title">PTT 发射</div>
  <button
    class="ptt-btn"
    class:ptt-pressed={pttActive || s.tx}
    onmousedown={pttDown}
    onmouseup={pttUp}
    onmouseleave={pttUp}
  >
    <span class="ptt-icon">{pttActive || s.tx ? '⏺' : '⏺'}</span>
    <span class="ptt-text">{pttActive || s.tx ? '发射中' : 'PTT'}</span>
  </button>
</div>

<style>
  .ptt-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .ptt-btn {
    width: 100%;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    border-radius: var(--radius-md);
    border: 2px solid var(--border);
    background: var(--bg-elevated);
    transition: all 0.15s ease;
  }
  .ptt-btn:hover {
    border-color: rgba(239,68,68,0.4);
    background: rgba(239,68,68,0.08);
  }

  .ptt-pressed {
    background: var(--accent-red) !important;
    border-color: var(--accent-red) !important;
    box-shadow: 0 0 24px rgba(239,68,68,0.6);
    animation: ptt-glow 0.6s infinite;
  }

  @keyframes ptt-glow {
    0%, 100% { box-shadow: 0 0 24px rgba(239,68,68,0.6); }
    50% { box-shadow: 0 0 40px rgba(239,68,68,0.9); }
  }

  .ptt-icon {
    font-size: 22px;
    color: var(--text-secondary);
    transition: color 0.15s;
  }
  .ptt-pressed .ptt-icon { color: #fff; }

  .ptt-text {
    font-size: 16px;
    font-weight: 700;
    color: var(--text-secondary);
    letter-spacing: 2px;
    transition: color 0.15s;
  }
  .ptt-pressed .ptt-text { color: #fff; }
</style>
