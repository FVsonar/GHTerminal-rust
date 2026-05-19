<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { TUNER_MODES } from '../lib/radio-types.js';

  let tunerMode = $state(0);
</script>

<div>
  <div class="panel-title">天调</div>
  <div class="flex-row">
    {#each TUNER_MODES as name, i}
      <button
        class:active={tunerMode === i}
        onclick={() => { tunerMode = i; sendCommand('set_tuner', { mode: i }); }}
      >
        {name}
      </button>
    {/each}
  </div>
</div>
