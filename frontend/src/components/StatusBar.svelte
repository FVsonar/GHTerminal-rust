<script>
  import { radioStatus } from '../lib/store.js';
  import { MODES } from '../lib/radio-types.js';
  let s = $derived($radioStatus);
  let freq = $derived(s.v === 0 ? s.fA : s.fB);
  let mode = $derived(s.v === 0 ? s.mA : s.mB);
  function fmtFreq(f) { const mhz = (f/1_000_000).toFixed(6); const [int,dec] = mhz.split('.'); return {int,dec}; }
  let f = $derived(fmtFreq(freq));
</script>

<div class="flex items-center gap-5 flex-1">
  <div class="flex items-baseline gap-0 font-mono text-2xl font-semibold text-success tracking-wider">
    <span>{f.int}</span><span>.</span>
    <span class="text-lg opacity-50">{f.dec}</span>
    <span class="text-base text-base-content/50 ml-1 font-normal">MHz</span>
  </div>
  <span class="text-sm font-semibold px-3 py-1 rounded tracking-wider {s.tx ? 'bg-error/20 text-error' : 'bg-primary/20 text-primary'}">{MODES[mode]||'USB'}</span>
  <div class="flex items-center gap-1.5">
    <span class="w-2.5 h-2.5 rounded-full {s.tx ? 'bg-error shadow-[0_0_8px_var(--color-error)] animate-pulse' : 'bg-success shadow-[0_0_6px_var(--color-success)]'}"></span>
    <span class="text-base font-semibold {s.tx ? 'text-error' : 'text-success'}">{s.tx?'TX':'RX'}</span>
  </div>
  <div class="ml-auto flex gap-3 font-mono text-base text-base-content/60">
    <span>{s.volt.toFixed(1)}V</span>
    <span>{String(s.utc[0]).padStart(2,'0')}:{String(s.utc[1]).padStart(2,'0')}:{String(s.utc[2]).padStart(2,'0')}</span>
  </div>
</div>
