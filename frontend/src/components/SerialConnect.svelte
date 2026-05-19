<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { connectionStatus } from '../lib/store.js';

  let ports = $state([]);
  let selectedPort = $state('');
  let selectedBaud = $state(115200);
  let connecting = $state(false);
  let errorMsg = $state('');
  let unlisten = null;

  const BAUD_RATES = [9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600];

  onMount(async () => {
    await refreshPorts();
    unlisten = await listen('serial-status', (event) => {
      connectionStatus.set(event.payload);
    });
  });

  onDestroy(() => { if (unlisten) unlisten(); });

  async function refreshPorts() {
    try {
      ports = await invoke('list_serial_ports');
      if (ports.length > 0 && !selectedPort) selectedPort = ports[0].name;
      errorMsg = '';
    } catch (e) { errorMsg = '无法枚举串口: ' + e; }
  }

  async function connect() {
    if (!selectedPort) return;
    connecting = true;
    errorMsg = '';
    try {
      await invoke('connect_serial', { port: selectedPort, baud: selectedBaud });
      connectionStatus.set({ connected: true, port: selectedPort, error: null });
    } catch (e) {
      errorMsg = '连接失败: ' + e;
      connectionStatus.set({ connected: false, port: selectedPort, error: String(e) });
    } finally { connecting = false; }
  }

  async function disconnect() {
    try { await invoke('disconnect_serial'); } catch (e) {}
    connectionStatus.set({ connected: false, port: '', error: null });
  }

  let status = $derived($connectionStatus);
</script>

{#if status.connected}
  <div class="conn-bar">
    <span class="conn-dot"></span>
    <span class="conn-port">{status.port}</span>
    <button class="disconnect-btn" onclick={disconnect}>断开</button>
  </div>
{:else}
  <div class="overlay">
    <div class="dialog">
      <div class="dialog-icon">📡</div>
      <h2>连接电台设备</h2>
      <p class="dialog-sub">选择串口并点击连接</p>

      <div class="form-row">
        <div class="field">
          <label>端口</label>
          <div class="input-row">
            <select bind:value={selectedPort}>
              {#if ports.length === 0}<option value="">未检测到串口</option>{/if}
              {#each ports as p}<option value={p.name}>{p.name}</option>{/each}
            </select>
            <button class="refresh-btn" onclick={refreshPorts}>↻</button>
          </div>
        </div>
        <div class="field">
          <label>波特率</label>
          <select bind:value={selectedBaud}>
            {#each BAUD_RATES as b}<option value={b}>{b}</option>{/each}
          </select>
        </div>
      </div>

      {#if errorMsg}<div class="error-msg">{errorMsg}</div>{/if}

      <button class="connect-btn" onclick={connect} disabled={connecting || !selectedPort}>
        {connecting ? '连接中...' : '连接'}
      </button>
    </div>
  </div>
{/if}

<style>
  .conn-bar {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .conn-dot {
    width: 7px; height: 7px;
    border-radius: 50%;
    background: var(--accent-green);
    box-shadow: 0 0 6px rgba(0,229,160,0.5);
  }
  .conn-port {
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--accent-green);
  }
  .disconnect-btn {
    font-size: 10px;
    padding: 3px 10px;
    background: transparent;
    border: 1px solid rgba(239,68,68,0.4);
    color: var(--accent-red);
  }
  .disconnect-btn:hover {
    background: var(--accent-red);
    color: #fff;
  }

  .overlay {
    position: fixed; inset: 0;
    background: rgba(10,14,23,0.94);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(12px);
  }
  .dialog {
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 40px;
    width: 420px;
    max-width: 92vw;
    text-align: center;
    box-shadow: var(--shadow-lg);
  }
  .dialog-icon { font-size: 48px; margin-bottom: 12px; }
  .dialog h2 { font-size: 20px; font-weight: 600; margin-bottom: 4px; }
  .dialog-sub { font-size: 13px; color: var(--text-secondary); margin-bottom: 24px; }

  .form-row { display: flex; gap: 12px; margin-bottom: 16px; }
  .field { flex: 1; text-align: left; }
  .input-row { display: flex; gap: 4px; }
  .field select { width: 100%; }
  .refresh-btn {
    width: 38px;
    font-size: 18px;
    padding: 0;
    flex-shrink: 0;
  }

  .connect-btn {
    width: 100%;
    padding: 12px;
    font-size: 16px;
    font-weight: 700;
    background: var(--accent-green);
    color: #000;
    border: none;
    border-radius: var(--radius-sm);
    transition: all 0.2s;
  }
  .connect-btn:hover { opacity: 0.9; box-shadow: 0 0 20px rgba(0,229,160,0.4); }
  .connect-btn:disabled { opacity: 0.3; cursor: default; box-shadow: none; }

  .error-msg {
    background: rgba(239,68,68,0.1);
    border: 1px solid rgba(239,68,68,0.3);
    border-radius: var(--radius-sm);
    padding: 8px 12px;
    margin-bottom: 12px;
    font-size: 12px;
    color: var(--accent-red);
  }
</style>
