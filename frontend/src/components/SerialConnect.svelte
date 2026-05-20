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
  const BAUD_RATES = [9600,19200,38400,57600,115200,230400,460800,921600];

  onMount(async () => {
    await refreshPorts();
    unlisten = await listen('serial-status', (e) => connectionStatus.set(e.payload));
  });
  onDestroy(() => { if(unlisten) unlisten(); });

  async function refreshPorts() {
    try { ports = await invoke('list_serial_ports'); if(ports.length>0&&!selectedPort) selectedPort=ports[0].name; errorMsg=''; }
    catch(e) { errorMsg = '无法枚举串口: '+e; }
  }
  async function connect() {
    if(!selectedPort) return; connecting=true; errorMsg='';
    try { await invoke('connect_serial',{port:selectedPort,baud:selectedBaud}); connectionStatus.set({connected:true,port:selectedPort,error:null}); }
    catch(e) { errorMsg='连接失败: '+e; connectionStatus.set({connected:false,port:selectedPort,error:String(e)}); }
    finally { connecting=false; }
  }
  async function disconnect() {
    try { await invoke('disconnect_serial'); } catch(e) {}
    connectionStatus.set({connected:false,port:'',error:null});
  }
  let status = $derived($connectionStatus);
</script>

{#if status.connected}
  <div class="flex items-center gap-2">
    <span class="w-1.5 h-1.5 rounded-full bg-success shadow-[0_0_5px_var(--color-success)]"></span>
    <span class="font-mono text-xs text-success">{status.port}</span>
    <button class="btn btn-sm btn-ghost text-error" onclick={disconnect}>断开</button>
  </div>
{:else}
  <div class="fixed inset-0 bg-base-100/95 backdrop-blur-md flex items-center justify-center z-50">
    <div class="card bg-base-200 border border-base-300 shadow-2xl p-10 w-[420px] max-w-[92vw] text-center">
      <span class="text-5xl mb-3 block">📡</span>
      <h2 class="text-xl font-semibold mb-1">连接电台设备</h2>
      <p class="text-sm text-base-content/60 mb-6">选择串口并点击连接</p>
      <div class="flex gap-3 mb-4">
        <div class="flex-1 text-left">
          <label class="label-text text-xs uppercase tracking-wider text-base-content/60 mb-1 block">端口</label>
          <div class="flex gap-1">
            <select class="select select-sm select-bordered w-full" bind:value={selectedPort}>
              {#if ports.length===0}<option value="">未检测到串口</option>{/if}
              {#each ports as p}<option value={p.name}>{p.name}</option>{/each}
            </select>
            <button class="btn btn-sm btn-ghost text-lg px-2" onclick={refreshPorts}>↻</button>
          </div>
        </div>
        <div class="flex-1 text-left">
          <label class="label-text text-xs uppercase tracking-wider text-base-content/60 mb-1 block">波特率</label>
          <select class="select select-sm select-bordered w-full" bind:value={selectedBaud}>
            {#each BAUD_RATES as b}<option value={b}>{b}</option>{/each}
          </select>
        </div>
      </div>
      {#if errorMsg}<div class="alert alert-error alert-soft text-sm mb-3">{errorMsg}</div>{/if}
      <button class="btn btn-success btn-wide text-base font-bold" onclick={connect} disabled={connecting||!selectedPort}>
        {connecting?'连接中...':'连接'}
      </button>
    </div>
  </div>
{/if}
