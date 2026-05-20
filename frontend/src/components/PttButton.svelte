<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  let s = $derived($radioStatus);
  let pttActive = $state(false);
  function pttDown(){pttActive=true;sendCommand('ptt',{pressed:true});}
  function pttUp(){pttActive=false;sendCommand('ptt',{pressed:false});}
  let on = $derived(pttActive||s.tx);
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm flex flex-col items-center p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2">PTT 发射</span>
  <button
    class="w-full h-14 flex items-center justify-center gap-2.5 rounded-lg border-2 transition-all duration-150 {on?'bg-error border-error shadow-[0_0_24px_rgba(239,68,68,.6)] animate-[pulse_.6s_infinite]':'bg-base-300 border-base-300 hover:border-error/40 hover:bg-error/10'}"
    onmousedown={pttDown} onmouseup={pttUp} onmouseleave={pttUp}
  >
    <span class="text-xl {on?'text-white':'text-base-content/50'}">⏺</span>
    <span class="text-base font-bold tracking-[2px] {on?'text-white':'text-base-content/50'}">{on?'发射中':'PTT'}</span>
  </button>
</div>
