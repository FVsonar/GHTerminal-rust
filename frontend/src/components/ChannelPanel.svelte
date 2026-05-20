<script>
  import { sendCommand } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { onEvent } from '../lib/tauri-bridge.js';
  import { onMount } from 'svelte';
  import { MODE_LIST } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
  let ch = $state(0);
  let chData = $state(null);
  let chMode = $state(0); // 0=VFO, 1=Channel
  let writeName = $state('');
  let writeModeA = $state(0);
  let writeModeB = $state(0);
  let writeFreqA = $state('');
  let writeFreqB = $state('');
  let msg = $state('');

  onMount(() => {
    onEvent('channel-data', (d) => { chData = d; msg = ''; });
  });

  async function readChannel() {
    msg = '读取中...';
    sendCommand('channel_read', { channel: ch });
  }

  async function writeChannel() {
    const fa = parseFloat(writeFreqA) || 0;
    const fb = parseFloat(writeFreqB) || 0;
    if (fa <= 0 || fb <= 0) { msg = '请输入有效频率'; return; }
    msg = '写入中...';
    sendCommand('channel_write', {
      channel: ch,
      vfoa_mode: writeModeA,
      vfob_mode: writeModeB,
      vfoa_freq: Math.round(fa * 1_000_000),
      vfob_freq: Math.round(fb * 1_000_000),
      tx_ctcss: 0,
      rx_ctcss: 0,
      name: writeName,
    });
    setTimeout(() => { msg = '已写入'; }, 500);
  }

  function setMode(m) {
    chMode = m;
    sendCommand('channel_mode', { mode: m });
  }

  function fillFromCurrent() {
    writeFreqA = (s.fA / 1_000_000).toFixed(6);
    writeFreqB = (s.fB / 1_000_000).toFixed(6);
    writeModeA = s.mA;
    writeModeB = s.mB;
  }
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2.5 block">信道管理</span>

  <!-- 模式切换 -->
  <div class="flex gap-1 mb-2.5">
    <button class="btn btn-xs flex-1 {chMode===0?'btn-primary':'btn-ghost'}" onclick={()=>setMode(0)}>VFO模式</button>
    <button class="btn btn-xs flex-1 {chMode===1?'btn-primary':'btn-ghost'}" onclick={()=>setMode(1)}>信道模式</button>
  </div>

  <!-- 信道号 + 读 -->
  <div class="flex items-end gap-1.5 mb-2.5">
    <div class="flex-1">
      <span class="text-[10px] font-medium text-base-content/50 block mb-0.5">信道号 (0-999)</span>
      <input type="number" class="input input-xs input-bordered w-full font-mono text-sm text-center" bind:value={ch} min="0" max="999" />
    </div>
    <button class="btn btn-xs btn-primary" onclick={readChannel}>读取</button>
    <button class="btn btn-xs btn-ghost" onclick={fillFromCurrent} title="从当前VFO填入">↓VFO</button>
  </div>

  <!-- 读出数据 -->
  {#if chData}
    <div class="bg-base-300/50 rounded-md px-2.5 py-2 mb-2.5 text-xs space-y-1">
      <div class="flex justify-between"><span class="text-base-content/50">信道</span><span class="font-mono">{chData.channel}</span></div>
      <div class="flex justify-between"><span class="text-base-content/50">名称</span><span class="font-medium">{chData.name || '-'}</span></div>
      <div class="flex justify-between"><span class="text-base-content/50">VFOA</span><span class="font-mono">{(chData.vfoa_freq/1_000_000).toFixed(3)} MHz {MODE_LIST.find(m=>m.v===chData.vfoa_mode)?.n||'USB'}</span></div>
      <div class="flex justify-between"><span class="text-base-content/50">VFOB</span><span class="font-mono">{(chData.vfob_freq/1_000_000).toFixed(3)} MHz {MODE_LIST.find(m=>m.v===chData.vfob_mode)?.n||'USB'}</span></div>
    </div>
  {/if}

  <!-- 写频 -->
  <div class="border-t border-base-300 pt-2.5">
    <span class="text-[10px] font-medium text-base-content/50 uppercase tracking-wide mb-2 block">写频</span>
    <div class="space-y-1.5">
      <div class="flex gap-1.5">
        <div class="flex-1">
          <span class="text-[9px] text-base-content/50">VFOA频率</span>
          <input type="text" class="input input-xs input-bordered w-full font-mono text-xs" bind:value={writeFreqA} placeholder="MHz" />
        </div>
        <div class="w-16">
          <span class="text-[9px] text-base-content/50">模式</span>
          <select class="select select-xs select-bordered w-full text-[11px]" bind:value={writeModeA}>
            {#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}
          </select>
        </div>
      </div>
      <div class="flex gap-1.5">
        <div class="flex-1">
          <span class="text-[9px] text-base-content/50">VFOB频率</span>
          <input type="text" class="input input-xs input-bordered w-full font-mono text-xs" bind:value={writeFreqB} placeholder="MHz" />
        </div>
        <div class="w-16">
          <span class="text-[9px] text-base-content/50">模式</span>
          <select class="select select-xs select-bordered w-full text-[11px]" bind:value={writeModeB}>
            {#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}
          </select>
        </div>
      </div>
      <div>
        <span class="text-[9px] text-base-content/50">信道名称</span>
        <input type="text" class="input input-xs input-bordered w-full text-xs" bind:value={writeName} maxlength="12" placeholder="最多12字符" />
      </div>
      <button class="btn btn-xs btn-success w-full" onclick={writeChannel}>写入信道 {ch}</button>
      {#if msg}<div class="text-[10px] text-center text-base-content/50">{msg}</div>{/if}
    </div>
  </div>
</div>
