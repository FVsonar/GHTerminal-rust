<script>
  import { sendCommand, onEvent } from '../lib/tauri-bridge.js';
  import { radioStatus } from '../lib/store.js';
  import { onMount } from 'svelte';
  import { MODE_LIST } from '../lib/radio-types.js';

  let s = $derived($radioStatus);
  let ch = $state(0);
  let chData = $state(null);
  let chMode = $state(0);
  let msg = $state('');

  // 写频表单
  let wName = $state('');
  let wMA = $state(0); let wMB = $state(0);
  let wFA = $state(''); let wFB = $state('');
  let wTxCtcss = $state(0); let wRxCtcss = $state(0);

  onMount(() => {
    onEvent('channel-data', (d) => { chData = d; msg = ''; });
  });

  async function readChannel() { msg = '读取中...'; sendCommand('channel_read', { channel: ch }); }

  async function writeChannel() {
    const fa = parseFloat(wFA) || 0, fb = parseFloat(wFB) || 0;
    if (fa <= 0 || fb <= 0) { msg = '无效频率'; return; }
    msg = '写入中...';
    sendCommand('channel_write', {
      channel: ch, vfoa_mode: wMA, vfob_mode: wMB,
      vfoa_freq: Math.round(fa * 1_000_000), vfob_freq: Math.round(fb * 1_000_000),
      tx_ctcss: wTxCtcss, rx_ctcss: wRxCtcss, name: wName,
    });
    setTimeout(() => { msg = '已写入'; }, 500);
  }

  function setMode(m) { chMode = m; sendCommand('channel_mode', { mode: m }); }

  function fillFromCurrent() {
    wFA = (s.fA / 1_000_000).toFixed(6); wFB = (s.fB / 1_000_000).toFixed(6);
    wMA = s.mA; wMB = s.mB;
  }

  function fillFromRead() {
    if (!chData) return;
    wFA = (chData.vfoa_freq / 1_000_000).toFixed(6);
    wFB = (chData.vfob_freq / 1_000_000).toFixed(6);
    wMA = chData.vfoa_mode; wMB = chData.vfob_mode;
    wName = chData.name || '';
    wTxCtcss = chData.tx_ctcss || 0;
    wRxCtcss = chData.rx_ctcss || 0;
  }

  function modeName(v) { return MODE_LIST.find(m => m.v === v)?.n || 'USB'; }
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2.5 block">信道管理</span>

  <!-- 模式 + 信道号 -->
  <div class="flex gap-1.5 mb-2.5 items-end">
    <div class="flex-1">
      <span class="text-[9px] text-base-content/50 block mb-0.5">工作模式</span>
      <div class="flex gap-1">
        <button class="btn btn-xs flex-1 text-[11px] {chMode===0?'btn-primary':'btn-ghost'}" onclick={()=>setMode(0)}>VFO</button>
        <button class="btn btn-xs flex-1 text-[11px] {chMode===1?'btn-primary':'btn-ghost'}" onclick={()=>setMode(1)}>信道</button>
      </div>
    </div>
    <div class="w-[72px]">
      <span class="text-[9px] text-base-content/50 block mb-0.5">信道号</span>
      <input type="number" class="input input-xs input-bordered w-full font-mono text-center text-sm" bind:value={ch} min="0" max="999" />
    </div>
    <button class="btn btn-xs btn-primary" onclick={readChannel}>读取</button>
  </div>

  <!-- 信道数据表格 -->
  <div class="overflow-hidden rounded-md border border-base-300 mb-2.5">
    <table class="w-full text-xs">
      <thead>
        <tr class="bg-base-300/50 text-base-content/50 text-[10px] uppercase tracking-wide">
          <th class="py-1.5 px-2 text-left font-medium">项目</th>
          <th class="py-1.5 px-2 text-right font-medium">VFO A</th>
          <th class="py-1.5 px-2 text-right font-medium">VFO B</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-base-300">
        <tr>
          <td class="py-1 px-2 text-base-content/50">频率</td>
          <td class="py-1 px-2 text-right font-mono text-success">
            {chData ? (chData.vfoa_freq / 1_000_000).toFixed(3) : '—'}
          </td>
          <td class="py-1 px-2 text-right font-mono text-success">
            {chData ? (chData.vfob_freq / 1_000_000).toFixed(3) : '—'}
          </td>
        </tr>
        <tr>
          <td class="py-1 px-2 text-base-content/50">模式</td>
          <td class="py-1 px-2 text-right font-semibold">{chData ? modeName(chData.vfoa_mode) : '—'}</td>
          <td class="py-1 px-2 text-right font-semibold">{chData ? modeName(chData.vfob_mode) : '—'}</td>
        </tr>
        <tr>
          <td class="py-1 px-2 text-base-content/50">亚音</td>
          <td class="py-1 px-2 text-right font-mono">{chData?.tx_ctcss || '—'}</td>
          <td class="py-1 px-2 text-right font-mono">{chData?.rx_ctcss || '—'}</td>
        </tr>
        <tr>
          <td class="py-1 px-2 text-base-content/50">名称</td>
          <td class="py-1 px-2 text-right font-medium" colspan="2">{chData?.name || '—'}</td>
        </tr>
      </tbody>
    </table>
  </div>

  {#if chData}
    <button class="btn btn-xs btn-ghost w-full mb-2.5" onclick={fillFromRead}>↓ 填入写频表单</button>
  {/if}

  <!-- 写频表单 -->
  <div class="border-t border-base-300 pt-2.5">
    <div class="flex items-center justify-between mb-2">
      <span class="text-[10px] font-medium text-base-content/50 uppercase tracking-wide">写频</span>
      <button class="btn btn-xs btn-ghost" onclick={fillFromCurrent}>从当前VFO</button>
    </div>

    <div class="overflow-hidden rounded-md border border-base-300 mb-2">
      <table class="w-full text-xs">
        <thead>
          <tr class="bg-base-300/50 text-base-content/50 text-[10px] uppercase tracking-wide">
            <th class="py-1 px-1.5 text-left font-medium w-10">项目</th>
            <th class="py-1 px-1.5 text-right font-medium">VFO A</th>
            <th class="py-1 px-1.5 text-right font-medium">VFO B</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-base-300">
          <tr>
            <td class="py-0.5 px-1.5 text-base-content/50 align-middle">频率</td>
            <td class="py-0.5 px-1.5"><input type="text" class="input input-xs input-bordered w-full font-mono text-[11px] text-right" bind:value={wFA} placeholder="MHz" /></td>
            <td class="py-0.5 px-1.5"><input type="text" class="input input-xs input-bordered w-full font-mono text-[11px] text-right" bind:value={wFB} placeholder="MHz" /></td>
          </tr>
          <tr>
            <td class="py-0.5 px-1.5 text-base-content/50 align-middle">模式</td>
            <td class="py-0.5 px-1.5">
              <select class="select select-xs select-bordered w-full text-[11px]" bind:value={wMA}>
                {#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}
              </select>
            </td>
            <td class="py-0.5 px-1.5">
              <select class="select select-xs select-bordered w-full text-[11px]" bind:value={wMB}>
                {#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}
              </select>
            </td>
          </tr>
          <tr>
            <td class="py-0.5 px-1.5 text-base-content/50 align-middle">亚音</td>
            <td class="py-0.5 px-1.5"><input type="number" class="input input-xs input-bordered w-full font-mono text-[11px] text-right" bind:value={wTxCtcss} min="0" max="55" placeholder="TX" /></td>
            <td class="py-0.5 px-1.5"><input type="number" class="input input-xs input-bordered w-full font-mono text-[11px] text-right" bind:value={wRxCtcss} min="0" max="55" placeholder="RX" /></td>
          </tr>
          <tr>
            <td class="py-0.5 px-1.5 text-base-content/50 align-middle">名称</td>
            <td class="py-0.5 px-1.5" colspan="2">
              <input type="text" class="input input-xs input-bordered w-full text-[11px]" bind:value={wName} maxlength="12" placeholder="最多12字符" />
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <button class="btn btn-xs btn-success w-full" onclick={writeChannel}>写入信道 {ch}</button>
    {#if msg}<div class="text-[10px] text-center text-base-content/50 mt-1">{msg}</div>{/if}
  </div>
</div>
