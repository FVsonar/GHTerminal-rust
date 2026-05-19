<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  let polls = $state({ status: true, meter: true, params: true, spectrum: true });

  onMount(async () => {
    try {
      polls = await invoke('get_poll_state');
    } catch (_) {}
  });

  async function toggle(key) {
    const on = !polls[key];
    polls[key] = on;
    try { await invoke('set_poll_toggle', { poll: key, on }); } catch (_) {}
  }

  const ITEMS = [
    { key: 'status', label: '状态', desc: '频率/模式/RX' },
    { key: 'meter', label: '仪表', desc: 'S表/PO/SWR' },
    { key: 'params', label: '参数', desc: 'AF/RF设置' },
    { key: 'spectrum', label: '频谱', desc: '频谱+瀑布' },
  ];
</script>

<div class="panel">
  <div class="panel-title">数据轮询</div>
  <div class="poll-list">
    {#each ITEMS as item}
      <button
        class="poll-btn"
        class:on={polls[item.key]}
        onclick={() => toggle(item.key)}
        title={item.desc}
      >
        <span class="poll-dot"></span>
        <span class="poll-label">{item.label}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .poll-list { display: flex; gap: 4px; }
  .poll-btn {
    display: flex;
    align-items: center;
    gap: 5px;
    flex: 1;
    padding: 5px 8px;
    font-size: 10px;
    font-weight: 500;
    border: 1px solid var(--border);
    background: var(--bg-input);
    color: var(--text-muted);
    border-radius: 4px;
    transition: all 0.15s;
  }
  .poll-btn.on {
    background: var(--bg-elevated);
    color: var(--text-primary);
    border-color: var(--border-light);
  }
  .poll-dot {
    width: 6px; height: 6px;
    border-radius: 50%;
    background: var(--text-muted);
    transition: all 0.2s;
  }
  .poll-btn.on .poll-dot {
    background: var(--accent-green);
    box-shadow: 0 0 5px rgba(0,229,160,0.5);
  }
</style>
