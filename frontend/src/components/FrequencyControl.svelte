<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { BANDS } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
  let freqInput = $state('');
  let activeFreq = $derived(s.v === 0 ? s.fA : s.fB);

  $effect(() => {
    freqInput = (activeFreq / 1_000_000).toFixed(6);
  });

  function setFreq() {
    const mhz = parseFloat(freqInput);
    if (isNaN(mhz) || mhz <= 0 || mhz > 2000) return;
    sendCommand('set_frequency', { freq: Math.round(mhz * 1_000_000) });
  }

  function step(khz) {
    const cur = parseFloat(freqInput) || 0;
    freqInput = (cur + khz / 1000).toFixed(6);
    setFreq();
  }

  function onKeydown(e) { if (e.key === 'Enter') setFreq(); }

  function setBand(v) {
    const freqMap = [1.8, 3.5, 5, 7, 10, 14, 18, 21, 24, 28, 50, 144, 430];
    freqInput = freqMap[v].toFixed(6);
    sendCommand('set_frequency', { freq: Math.round(freqMap[v] * 1_000_000) });
  }
</script>

<div class="panel freq-panel">
  <div class="panel-title">频率控制</div>

  <div class="freq-input-row">
    <input type="text" bind:value={freqInput} onkeydown={onKeydown} class="freq-input" />
    <span class="freq-mhz">MHz</span>
  </div>

  <div class="step-grid">
    <button onclick={() => step(1000)}>+1M</button>
    <button onclick={() => step(100)}>+100K</button>
    <button onclick={() => step(10)}>+10K</button>
    <button onclick={() => step(1)}>+1K</button>
    <button onclick={() => step(-1000)}>-1M</button>
    <button onclick={() => step(-100)}>-100K</button>
    <button onclick={() => step(-10)}>-10K</button>
    <button onclick={() => step(-1)}>-1K</button>
  </div>

  <div class="band-grid">
    {#each BANDS as b}
      <button class="band-btn" onclick={() => setBand(b.v)}>{b.n}</button>
    {/each}
  </div>
</div>

<style>
  .freq-input-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }
  .freq-input {
    flex: 1;
    font-family: var(--font-mono) !important;
    font-size: 18px !important;
    font-weight: 600;
    text-align: center;
    letter-spacing: 1px;
    color: var(--accent-green) !important;
  }
  .freq-mhz {
    font-size: 12px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .step-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 4px;
    margin-bottom: 8px;
  }
  .step-grid button {
    padding: 5px 2px;
    font-size: 11px;
    font-family: var(--font-mono);
  }

  .band-grid {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 3px;
  }
  .band-btn {
    padding: 5px 2px;
    font-size: 10px;
    font-weight: 500;
  }
</style>
