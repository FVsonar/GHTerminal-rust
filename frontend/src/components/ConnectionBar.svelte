<script>
  import { connectionStatus } from '../lib/store.js';

  let status = $derived($connectionStatus);
</script>

<div class="flex-row" style="margin-left: auto;">
  <span class="conn-dot" class:connected={status.connected}></span>
  <span style="font-size:12px; color: var(--text-secondary);">
    {status.connected ? '已连接' : '未连接'}
  </span>
  {#if status.error}
    <span style="font-size:11px; color: var(--accent-red);">{status.error}</span>
  {/if}
</div>

<style>
  .conn-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent-red);
  }
  .conn-dot.connected {
    background: var(--accent-green);
  }
</style>
