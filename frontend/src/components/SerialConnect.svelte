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
    <span class="w-2 h-2 rounded-full bg-success shadow-[0_0_5px_var(--color-success)]"></span>
    <span class="font-mono text-sm text-success">{status.port}</span>
    <button class="btn btn-sm btn-ghost text-error text-sm" onclick={disconnect}>断开</button>
  </div>
{:else}
  <div class="fixed inset-0 bg-base-100/95 backdrop-blur-md flex items-center justify-center z-50">
    <div class="card bg-base-200 border border-base-300 shadow-2xl p-12 w-[480px] max-w-[92vw] text-center">
      <span class="text-6xl mb-4 block">📡</span>
      <h2 class="text-2xl font-semibold mb-2">连接电台设备</h2>
      <p class="text-base text-base-content/60 mb-8">选择串口并点击连接</p>
      <div class="flex gap-4 mb-5">
        <div class="flex-1 text-left">
          <label class="label-text text-sm uppercase tracking-wider text-base-content/60 mb-1.5 block">端口</label>
          <div class="flex gap-1">
            <select class="select select-bordered w-full text-base" bind:value={selectedPort}>
              {#if ports.length===0}<option value="">未检测到串口</option>{/if}
              {#each ports as p}<option value={p.name}>{p.name}</option>{/each}
            </select>
            <button class="btn btn-ghost text-xl px-2" onclick={refreshPorts}>↻</button>
          </div>
        </div>
        <div class="flex-1 text-left">
          <label class="label-text text-sm uppercase tracking-wider text-base-content/60 mb-1.5 block">波特率</label>
          <select class="select select-bordered w-full text-base" bind:value={selectedBaud}>
            {#each BAUD_RATES as b}<option value={b}>{b}</option>{/each}
          </select>
        </div>
      </div>
      {#if errorMsg}<div class="alert alert-error alert-soft text-base mb-4">{errorMsg}</div>{/if}
      <button class="btn btn-success btn-wide text-lg font-bold" onclick={connect} disabled={connecting||!selectedPort}>
        {connecting?'连接中...':'连接'}
      </button>
    </div>
  </div>
{/if}
