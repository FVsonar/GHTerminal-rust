<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';

  let s = $derived($radioStatus);
  let pttActive = $state(false);

  function pttDown() {
    pttActive = true;
    sendCommand('ptt', { pressed: true });
  }

  function pttUp() {
    pttActive = false;
    sendCommand('ptt', { pressed: false });
  }
</script>

<div>
  <div class="panel-title">PTT</div>
  <button
    onmousedown={pttDown}
    onmouseup={pttUp}
    onmouseleave={pttUp}
    style="width:100%; height:40px; font-size:18px; font-weight:bold;
      background: {pttActive ? 'var(--accent-red)' : 'var(--bg-panel)'};"
  >
    {pttActive ? '发射中...' : s.tx ? 'TX' : 'PTT'}
  </button>
</div>
