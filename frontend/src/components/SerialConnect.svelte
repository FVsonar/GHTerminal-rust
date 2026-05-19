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

  onDestroy(() => {
    if (unlisten) unlisten();
  });

  async function refreshPorts() {
    try {
      ports = await invoke('list_serial_ports');
      if (ports.length > 0 && !selectedPort) {
        selectedPort = ports[0].name;
      }
      errorMsg = '';
    } catch (e) {
      errorMsg = '无法枚举串口: ' + e;
    }
  }

  async function connect() {
    if (!selectedPort) return;
    connecting = true;
    errorMsg = '';
    try {
      await invoke('connect_serial', { port: selectedPort, baud: selectedBaud });
    } catch (e) {
      errorMsg = '连接失败: ' + e;
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

{#if status.connected}
  <!-- 已连接：顶部紧凑状态条 -->
  <div class="connected-bar">
    <span class="conn-dot on"></span>
    <span class="port-name">{status.port}</span>
    <button class="btn-sm disconnect-btn" onclick={disconnect}>断开</button>
  </div>
{:else}
  <!-- 未连接：全屏遮罩对话框 -->
  <div class="overlay">
    <div class="dialog">
      <div class="dialog-icon">📡</div>
      <h2>连接电台设备</h2>
      <p class="dialog-sub">请选择串口并点击连接</p>

      <div class="form-row">
        <div class="field">
          <label>串口端口</label>
          <div class="input-row">
            <select bind:value={selectedPort} class="port-select">
              {#if ports.length === 0}
                <option value="">未检测到串口</option>
              {/if}
              {#each ports as p}
                <option value={p.name}>{p.name}</option>
              {/each}
            </select>
            <button class="btn-icon" onclick={refreshPorts} title="刷新">↻</button>
          </div>
        </div>

        <div class="field">
          <label>波特率</label>
          <select bind:value={selectedBaud} class="baud-select">
            {#each BAUD_RATES as b}
              <option value={b}>{b}</option>
            {/each}
          </select>
        </div>
      </div>

      {#if errorMsg}
        <div class="error-msg">{errorMsg}</div>
      {/if}

      <button
        class="connect-btn"
        onclick={connect}
        disabled={connecting || !selectedPort}
      >
        {connecting ? '连接中...' : '连接'}
      </button>
    </div>
  </div>
{/if}

<style>
  /* === 已连接状态条 === */
  .connected-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-left: auto;
    flex-shrink: 0;
  }
  .conn-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent-red);
    flex-shrink: 0;
  }
  .conn-dot.on {
    background: var(--accent-green);
  }
  .port-name {
    font-size: 12px;
    color: var(--accent-green);
    font-family: var(--font-mono);
  }
  .btn-sm {
    font-size: 11px;
    padding: 2px 8px;
  }
  .disconnect-btn {
    background: transparent;
    border-color: var(--accent-red);
    color: var(--accent-red);
  }
  .disconnect-btn:hover {
    background: var(--accent-red);
    color: white;
  }

  /* === 未连接遮罩 === */
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(10, 10, 30, 0.92);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }
  .dialog {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 40px;
    width: 420px;
    max-width: 90vw;
    text-align: center;
    box-shadow: 0 20px 60px rgba(0,0,0,0.5);
  }
  .dialog-icon {
    font-size: 48px;
    margin-bottom: 12px;
  }
  .dialog h2 {
    font-size: 20px;
    color: var(--text-primary);
    margin-bottom: 4px;
  }
  .dialog-sub {
    font-size: 13px;
    color: var(--text-secondary);
    margin-bottom: 24px;
  }

  .form-row {
    display: flex;
    gap: 12px;
    margin-bottom: 16px;
  }
  .field {
    flex: 1;
    text-align: left;
  }
  .field label {
    font-size: 11px;
    color: var(--text-secondary);
    display: block;
    margin-bottom: 4px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .input-row {
    display: flex;
    gap: 4px;
  }

  select {
    width: 100%;
    background: var(--bg-input);
    border: 1px solid var(--border);
    color: var(--text-primary);
    border-radius: 6px;
    padding: 10px 12px;
    font-size: 14px;
    outline: none;
  }
  select:focus {
    border-color: var(--accent-blue);
  }

  .btn-icon {
    background: var(--bg-panel);
    border: 1px solid var(--border);
    color: var(--text-primary);
    border-radius: 6px;
    width: 40px;
    font-size: 18px;
    cursor: pointer;
    flex-shrink: 0;
  }
  .btn-icon:hover {
    background: var(--border);
  }

  .connect-btn {
    width: 100%;
    padding: 12px;
    font-size: 16px;
    font-weight: bold;
    background: var(--accent-green);
    color: #000;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: opacity 0.2s;
  }
  .connect-btn:hover {
    opacity: 0.9;
  }
  .connect-btn:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .error-msg {
    background: rgba(231, 76, 60, 0.15);
    border: 1px solid var(--accent-red);
    border-radius: 6px;
    padding: 8px 12px;
    margin-bottom: 12px;
    font-size: 12px;
    color: var(--accent-red);
  }
</style>
