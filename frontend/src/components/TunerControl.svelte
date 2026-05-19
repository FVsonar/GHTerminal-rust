<script>
  import { sendCommand } from '../lib/tauri-bridge.js';

  let tunerMode = $state(0);
  const MODES = ['关闭', '开启', '调谐'];

  function setTuner(m) { tunerMode = m; sendCommand('set_tuner', { mode: m }); }
</script>

<div class="panel">
  <div class="panel-title">天调</div>
  <div class="flex-row">
    {#each MODES as name, i}
      <button class:active={tunerMode === i} onclick={() => setTuner(i)} style="flex:1;">{name}</button>
    {/each}
  </div>
</div>
