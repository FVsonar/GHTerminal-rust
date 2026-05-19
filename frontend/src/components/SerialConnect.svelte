<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { connectionStatus } from '../lib/store.js';

  let ports = $state([]);
  let selectedPort = $state('');
  let selectedBaud = $state(115200);
  let connecting = $state(false);
  let unlisten = null;

  const BAUD_RATES = [9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600];

  onMount(async () => {
    await refreshPorts();
    unlisten = await listen('serial-status', (event) => {
      connectionStatus.set(event.payload);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });

  async function refreshPorts() {
    try {
      ports = await invoke('list_serial_ports');
      if (ports.length > 0 && !selectedPort) {
        selectedPort = ports[0].name;
      }
    } catch (e) {
      console.error('Failed to list ports:', e);
    }
  }

  async function connect() {
    if (!selectedPort) return;
    connecting = true;
    try {
      await invoke('connect_serial', { port: selectedPort, baud: selectedBaud });
    } catch (e) {
      console.error('Connect failed:', e);
      connectionStatus.set({ connected: false, port: '', error: String(e) });
    } finally {
      connecting = false;
    }
  }

  async function disconnect() {
    try {
      await invoke('disconnect_serial');
    } catch (e) {
      console.error('Disconnect failed:', e);
    }
  }

  let status = $derived($connectionStatus);
</script>

<div class="panel">
  <div class="panel-title">串口连接</div>

  <div class="flex-row" style="align-items: flex-end; gap: 6px;">
    <div class="flex-col" style="flex:1;">
      <label>端口</label>
      <select bind:value={selectedPort} style="width:100%;">
        {#if ports.length === 0}
          <option value="">无可用端口</option>
        {/if}
        {#each ports as p}
          <option value={p.name}>{p.name}</option>
        {/each}
      </select>
    </div>

    <div class="flex-col" style="width:80px;">
      <label>波特率</label>
      <select bind:value={selectedBaud} style="width:100%;">
        {#each BAUD_RATES as b}
          <option value={b}>{b}</option>
        {/each}
      </select>
    </div>

    <div class="flex-col">
      <button onclick={refreshPorts} style="font-size:11px;" title="刷新端口列表">
        ↻
      </button>
    </div>
  </div>

  <div class="flex-row" style="margin-top: 8px; gap: 6px;">
    {#if status.connected}
      <div class="flex-row" style="flex:1; gap:6px;">
        <span class="conn-dot connected"></span>
        <span style="font-size:12px; color: var(--accent-green);">{status.port}</span>
      </div>
      <button onclick={disconnect} style="background: var(--accent-red); border-color: var(--accent-red);">
        断开
      </button>
    {:else}
      <span style="font-size:12px; color: var(--text-secondary); flex:1;">
        {ports.length === 0 ? '未检测到串口' : '未连接'}
      </span>
      <button onclick={connect} disabled={connecting || !selectedPort}>
        {connecting ? '连接中...' : '连接'}
      </button>
    {/if}
  </div>

  {#if status.error}
    <div style="margin-top: 4px; font-size: 11px; color: var(--accent-red);">{status.error}</div>
  {/if}
</div>

<style>
  .conn-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent-red);
    flex-shrink: 0;
  }
  .conn-dot.connected {
    background: var(--accent-green);
  }
</style>
