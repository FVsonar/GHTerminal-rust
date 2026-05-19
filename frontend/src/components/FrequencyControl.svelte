<script>
  import { radioSocket } from '../lib/ws.js';
  import { radioStatus } from '../lib/store.js';

  let s = $derived($radioStatus);
  let freqInput = $state('');
  let activeFreq = $derived(s.v === 0 ? s.fA : s.fB);

  $effect(() => {
    freqInput = (activeFreq / 1000000).toFixed(6);
  });

  function setFrequency() {
    const mhz = parseFloat(freqInput);
    if (isNaN(mhz) || mhz <= 0 || mhz > 2000) return;
    radioSocket.cmd('set_frequency', { freq: Math.round(mhz * 1000000) });
  }

  function onKeydown(e) {
    if (e.key === 'Enter') setFrequency();
  }

  function step(delta) {
    const mhz = parseFloat(freqInput) || 0;
    freqInput = (mhz + delta).toFixed(6);
    setFrequency();
  }
</script>

<div>
  <div class="panel-title">频率</div>
  <div class="flex-row">
    <button onclick={() => step(-0.001)}>-1K</button>
    <button onclick={() => step(-0.01)}>-10K</button>
    <input
      type="text"
      bind:value={freqInput}
      onkeydown={onKeydown}
      style="width:110px; font-family: var(--font-mono); font-size:16px; text-align:center;"
    />
    <button onclick={() => step(0.01)}>+10K</button>
    <button onclick={() => step(0.001)}>+1K</button>
  </div>
  <div class="flex-row" style="margin-top: 4px;">
    <button onclick={() => step(-0.1)}>-100K</button>
    <button onclick={() => step(-1)}>-1M</button>
    <button onclick={() => step(1)}>+1M</button>
    <button onclick={() => step(0.1)}>+100K</button>
  </div>
</div>
