<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let polls = $state({ status: true, meter: true, params: true, spectrum: true });

  onMount(async () => {
    try { polls = await invoke('get_poll_state'); } catch (_) {}
  });

  async function toggle(key) {
    polls[key] = !polls[key];
    try { await invoke('set_poll_toggle', { poll: key, on: polls[key] }); } catch (_) {}
  }
</script>

<div class="poll-bar">
  <button class="poll-dot-btn" class:on={polls.status} onclick={() => toggle('status')} title="状态轮询">
    <span class="dot"></span>S
  </button>
  <button class="poll-dot-btn" class:on={polls.meter} onclick={() => toggle('meter')} title="仪表轮询">
    <span class="dot"></span>M
  </button>
  <button class="poll-dot-btn" class:on={polls.params} onclick={() => toggle('params')} title="参数轮询">
    <span class="dot"></span>P
  </button>
  <button class="poll-dot-btn" class:on={polls.spectrum} onclick={() => toggle('spectrum')} title="频谱轮询">
    <span class="dot"></span>F
  </button>
</div>

<style>
  .poll-bar {
    display: flex;
    gap: 2px;
    align-items: center;
  }
  .poll-dot-btn {
    display: flex;
    align-items: center;
    gap: 3px;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 10px;
    font-weight: 600;
    padding: 2px 4px;
    cursor: pointer;
    letter-spacing: 0.5px;
  }
  .poll-dot-btn.on { color: var(--text-primary); }
  .poll-dot-btn .dot {
    width: 6px; height: 6px;
    border-radius: 50%;
    background: rgba(255,255,255,0.15);
    transition: all 0.2s;
  }
  .poll-dot-btn.on .dot {
    background: var(--accent-green);
    box-shadow: 0 0 5px rgba(0,229,160,0.5);
  }
</style>
