<script>
  import { invoke } from '@tauri-apps/api/core';
  import { sendCommand, onEvent } from '../lib/tauri-bridge.js';
  import { onMount } from 'svelte';
  import { MODE_LIST, CTCSS_TABLE } from '../lib/radio-types.js';

  const PAGE_SIZE = 50, TOTAL_CH = 1000;
  let tab = $state('analog'); // 'analog' | 'dmr'
  let channels = $state({});      // analog channel cache
  let dmrChannels = $state({});   // DMR channel cache
  let page = $state(0), pageInput = $state('0');
  let editing = $state(null), editVal = $state('');
  let reading = $state(false), readProgress = $state(0);
  let chMode = $state(0); let chOn = $state(true);
  function toggleCh(v){chOn=v;invoke('set_poll_toggle',{poll:'channel',on:v});}
  let start = $derived(page * PAGE_SIZE), end = $derived(Math.min(start + PAGE_SIZE, TOTAL_CH));
  let totalPages = $derived(Math.ceil(TOTAL_CH / PAGE_SIZE));

  onMount(() => {
    onEvent('channel-data', (d) => { channels = { ...channels, [d.channel]: d }; });
    onEvent('dmr-channel-data', (d) => { dmrChannels = { ...dmrChannels, [d.channel]: d }; });
  });

  function modeName(v) { return MODE_LIST.find(m => m.v === v)?.n || 'USB'; }
  function ctcssName(idx) { return CTCSS_TABLE[idx] || String(idx); }

  function readChannel(n) {
    if (tab === 'dmr') sendCommand('dmr_channel_read', { channel: n });
    else sendCommand('channel_read', { channel: n });
  }

  async function readPage() {
    reading = true; readProgress = 0;
    const cmd = tab === 'dmr' ? 'dmr_channel_read' : 'channel_read';
    for (let i = 0; i < end - start; i++) {
      sendCommand(cmd, { channel: start + i });
      readProgress = i + 1;
      await new Promise(r => setTimeout(r, 220));
    }
    reading = false;
  }

  function startEdit(ch, field, current) {
    editing = { ch, field };
    editVal = String(current || '');
  }

  function commitEdit() {
    if (!editing) return;
    const { ch, field } = editing;
    const cache = tab === 'dmr' ? dmrChannels : channels;
    const cur = cache[ch] || {};
    const updated = { ...cur, channel: ch };
    const v = parseInt(editVal) || 0;

    if (tab === 'dmr') {
      if (field === 'call_id') updated.call_id = parseInt(editVal) || 0;
      else if (field === 'own_id') updated.own_id = parseInt(editVal) || 0;
      else if (field === 'call_format') updated.call_format = v;
      else if (field === 'tx_cc') updated.tx_cc = v;
      else if (field === 'rx_cc') updated.rx_cc = v;
      else if (field === 'slot') updated.slot = v;
      else if (field === 'ch_type') updated.ch_type = v;
      else if (field === 'tx_ctcss') updated.tx_ctcss = v;
      else if (field === 'rx_ctcss') updated.rx_ctcss = v;
      else if (field === 'sqlevel') updated.sqlevel = v;
      else if (field === 'spkgain') updated.spkgain = v;
      else if (field === 'dmod_gain') updated.dmod_gain = v;
      else if (field === 'scr_seed') updated.scr_seed = v;
      else if (field === 'ch_bs_mode') updated.ch_bs_mode = v;
      if (tab === 'dmr') dmrChannels = { ...dmrChannels, [ch]: updated };
    } else {
      if (field === 'freq_a') updated.vfoa_freq = Math.round(parseFloat(editVal) * 1_000_000) || 0;
      else if (field === 'freq_b') updated.vfob_freq = Math.round(parseFloat(editVal) * 1_000_000) || 0;
      else if (field === 'mode_a') updated.vfoa_mode = v;
      else if (field === 'mode_b') updated.vfob_mode = v;
      else if (field === 'name') updated.name = editVal.slice(0, 12);
      else if (field === 'ctcss_tx') updated.tx_ctcss = v;
      else if (field === 'ctcss_rx') updated.rx_ctcss = v;
      channels = { ...channels, [ch]: updated };
    }
    editing = null;
  }
  function cancelEdit() { editing = null; }
  function handleEditKey(e) { if (e.key === 'Enter') commitEdit(); if (e.key === 'Escape') cancelEdit(); }

  function writeChannel(n) {
    const c = tab === 'dmr' ? dmrChannels[n] : channels[n];
    if (!c) return;
    if (tab === 'dmr') {
      sendCommand('dmr_channel_write', { channel: n, call_format: c.call_format||0, tx_cc: c.tx_cc||0, rx_cc: c.rx_cc||0, slot: c.slot||1, call_id: c.call_id||0, own_id: c.own_id||0, ch_type: c.ch_type||0, rx_ctcss: c.rx_ctcss||0, tx_ctcss: c.tx_ctcss||0, sqlevel: c.sqlevel||0, spkgain: c.spkgain||0, dmrexist: c.dmrexist||0, dmod_gain: c.dmod_gain||0, scr_en: c.scr_en||0, scr_seed: c.scr_seed||0, ch_bs_mode: c.ch_bs_mode||0, validat: c.validat||0 });
    } else {
      sendCommand('channel_write', { channel: n, vfoa_freq: c.vfoa_freq||0, vfob_freq: c.vfob_freq||0, vfoa_mode: c.vfoa_mode||0, vfob_mode: c.vfob_mode||0, tx_ctcss: c.tx_ctcss||0, rx_ctcss: c.rx_ctcss||0, name: c.name||'' });
    }
  }

  async function writePage() {
    for (let i = 0; i < end - start; i++) {
      const n = start + i;
      const c = tab === 'dmr' ? dmrChannels[n] : channels[n];
      if (c) { writeChannel(n); await new Promise(r => setTimeout(r, 220)); }
    }
  }

  function goPage(p) { const pg = Math.max(0, Math.min(p, totalPages - 1)); page = pg; pageInput = String(pg); }
  function setMode(m) { chMode = m; sendCommand('channel_mode', { mode: m }); }
  function fmtFreq(hz) { return hz ? (hz / 1_000_000).toFixed(5) : ''; }
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <div class="flex items-center justify-between mb-2.5"><span class="text-[12px] font-semibold text-base-content/50 uppercase tracking-widest">信道管理 (0-999)</span><input type="checkbox" class="toggle toggle-sm toggle-success" checked={chOn} onchange={(e)=>toggleCh(e.target.checked)} /></div>

  <!-- Tab + 模式 -->
  <div class="flex items-end gap-2 mb-2.5">
    <div class="flex gap-1">
      <button class="btn btn-sm text-[13px] {tab==='analog'?'btn-primary':'btn-ghost'}" onclick={()=>{tab='analog';page=0;pageInput='0'}}>模拟</button>
      <button class="btn btn-sm text-[13px] {tab==='dmr'?'btn-primary':'btn-ghost'}" onclick={()=>{tab='dmr';page=0;pageInput='0'}}>DMR</button>
    </div>
    <div class="flex items-end gap-1.5 ml-auto">
      <div>
        <span class="text-[11px] text-base-content/50 block mb-0.5">模式</span>
        <div class="flex gap-1">
          <button class="btn btn-sm text-[13px] {chMode===0?'btn-primary':'btn-ghost'}" onclick={()=>setMode(0)}>VFO</button>
          <button class="btn btn-sm text-[13px] {chMode===1?'btn-primary':'btn-ghost'}" onclick={()=>setMode(1)}>信道</button>
        </div>
      </div>
    </div>
    <button class="btn btn-sm btn-primary" onclick={readPage} disabled={reading}>
      {reading ? `读取中 ${readProgress}/${end-start}` : `读第${page}页`}
    </button>
    <button class="btn btn-sm btn-success btn-ghost" onclick={writePage}>写本页</button>
  </div>

  <!-- 分页 -->
  <div class="flex items-center justify-center gap-1.5 mb-2">
    <button class="btn btn-sm btn-ghost" onclick={()=>goPage(0)} disabled={page===0}>«</button>
    <button class="btn btn-sm btn-ghost" onclick={()=>goPage(page-1)} disabled={page===0}>‹</button>
    <input type="number" class="input input-sm input-bordered w-12 text-center font-mono text-xs" bind:value={pageInput} min="0" max={totalPages-1} onkeydown={(e)=>{if(e.key==='Enter')goPage(parseInt(pageInput)||0)}} />
    <span class="text-[13px] text-base-content/50">/ {totalPages - 1}</span>
    <button class="btn btn-sm btn-ghost" onclick={()=>goPage(page+1)} disabled={page>=totalPages-1}>›</button>
    <button class="btn btn-sm btn-ghost" onclick={()=>goPage(totalPages-1)} disabled={page>=totalPages-1}>»</button>
    <span class="text-[13px] text-base-content/40 ml-2">#{start}-#{end-1}</span>
  </div>

  <!-- 表格 -->
  <div class="overflow-auto max-h-[360px] rounded-md border border-base-300">
    {#if tab === 'analog'}
    <table class="w-full text-[13px] border-collapse">
      <thead class="sticky top-0 z-10">
        <tr class="bg-base-300 text-base-content/50 text-[11px] uppercase tracking-wide">
          <th class="py-1.5 px-1 text-center font-medium w-7">#</th>
          <th class="py-1.5 px-1 text-center font-medium w-16">名称</th>
          <th class="py-1.5 px-1 text-center font-medium w-[72px]">VFOA</th>
          <th class="py-1.5 px-1 text-center font-medium w-8">M</th>
          <th class="py-1.5 px-1 text-center font-medium w-[72px]">VFOB</th>
          <th class="py-1.5 px-1 text-center font-medium w-8">M</th>
          <th class="py-1.5 px-1 text-center font-medium w-[44px]">TX亚音</th>
          <th class="py-1.5 px-1 text-center font-medium w-[44px]">RX亚音</th>
          <th class="py-1.5 px-1 text-center font-medium w-12">操作</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-base-300">
        {#each Array(end - start) as _, i}
          {@const chNum = start + i}
          {@const c = channels[chNum]}
          {@const isEdit = editing?.ch === chNum}
          <tr class="hover:bg-base-300/30 transition-colors {c ? '' : 'opacity-50'}">
            <td class="py-0.5 px-1 text-center font-mono text-base-content/40 text-[12px]">{chNum}</td>
            <td class="py-0.5 px-1" onclick={()=>startEdit(chNum, 'name', c?.name)}>
              {#if isEdit && editing.field === 'name'}<input type="text" class="input input-sm input-bordered w-full text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} maxlength="12" autofocus />
              {:else}<span class="cursor-pointer truncate block max-w-[72px] {c?.name?'font-medium':''}">{c?.name || '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'freq_a', fmtFreq(c?.vfoa_freq))}>
              {#if isEdit && editing.field === 'freq_a'}<input type="text" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} autofocus />
              {:else}<span class="cursor-pointer font-mono {c?'text-success':''}">{c ? fmtFreq(c.vfoa_freq) : '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'mode_a', c?.vfoa_mode)}>
              {#if isEdit && editing.field === 'mode_a'}<select class="select select-sm select-bordered w-full text-[12px]" bind:value={editVal} onchange={commitEdit}>{#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}</select>
              {:else}<span class="cursor-pointer">{c ? modeName(c.vfoa_mode) : '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'freq_b', fmtFreq(c?.vfob_freq))}>
              {#if isEdit && editing.field === 'freq_b'}<input type="text" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} autofocus />
              {:else}<span class="cursor-pointer font-mono {c?'text-success':''}">{c ? fmtFreq(c.vfob_freq) : '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'mode_b', c?.vfob_mode)}>
              {#if isEdit && editing.field === 'mode_b'}<select class="select select-sm select-bordered w-full text-[12px]" bind:value={editVal} onchange={commitEdit}>{#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}</select>
              {:else}<span class="cursor-pointer">{c ? modeName(c.vfob_mode) : '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center font-mono" onclick={()=>startEdit(chNum, 'ctcss_tx', c?.tx_ctcss)}>
              {#if isEdit && editing.field === 'ctcss_tx'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="55" autofocus />
              {:else}<span class="cursor-pointer text-[12px]" title={ctcssName(c?.tx_ctcss||0)}>{c ? ctcssName(c.tx_ctcss) : '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center font-mono" onclick={()=>startEdit(chNum, 'ctcss_rx', c?.rx_ctcss)}>
              {#if isEdit && editing.field === 'ctcss_rx'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="55" autofocus />
              {:else}<span class="cursor-pointer text-[12px]" title={ctcssName(c?.rx_ctcss||0)}>{c ? ctcssName(c.rx_ctcss) : '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center">
              <button class="btn btn-sm btn-ghost text-[12px] px-1 h-6" onclick={()=>readChannel(chNum)}>R</button>
              <button class="btn btn-sm btn-ghost text-[12px] px-1 h-6 {c?'text-success':''}" onclick={()=>writeChannel(chNum)} disabled={!c}>W</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
    {:else}
    <!-- DMR 表格 -->
    <table class="w-full text-[13px] border-collapse">
      <thead class="sticky top-0 z-10">
        <tr class="bg-base-300 text-base-content/50 text-[11px] uppercase tracking-wide">
          <th class="py-1.5 px-1 text-center font-medium w-7">#</th>
          <th class="py-1.5 px-1 text-center font-medium">fmt</th>
          <th class="py-1.5 px-1 text-center font-medium">TxCC</th>
          <th class="py-1.5 px-1 text-center font-medium">RxCC</th>
          <th class="py-1.5 px-1 text-center font-medium">slot</th>
          <th class="py-1.5 px-1 text-center font-medium">Call ID</th>
          <th class="py-1.5 px-1 text-center font-medium">Own ID</th>
          <th class="py-1.5 px-1 text-center font-medium">typ</th>
          <th class="py-1.5 px-1 text-center font-medium">SQL</th>
          <th class="py-1.5 px-1 text-center font-medium">Gain</th>
          <th class="py-1.5 px-1 text-center font-medium">BS</th>
          <th class="py-1.5 px-1 text-center font-medium w-12">操作</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-base-300">
        {#each Array(end - start) as _, i}
          {@const chNum = start + i}
          {@const c = dmrChannels[chNum]}
          {@const isEdit = editing?.ch === chNum}
          <tr class="hover:bg-base-300/30 transition-colors {c ? '' : 'opacity-50'}">
            <td class="py-0.5 px-1 text-center font-mono text-base-content/40 text-[12px]">{chNum}</td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'call_format', c?.call_format)}>
              {#if isEdit && editing.field==='call_format'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="2" autofocus />
              {:else}<span class="cursor-pointer">{c?.call_format ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'tx_cc', c?.tx_cc)}>
              {#if isEdit && editing.field==='tx_cc'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="15" autofocus />
              {:else}<span class="cursor-pointer">{c?.tx_cc ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'rx_cc', c?.rx_cc)}>
              {#if isEdit && editing.field==='rx_cc'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="15" autofocus />
              {:else}<span class="cursor-pointer">{c?.rx_cc ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'slot', c?.slot)}>
              {#if isEdit && editing.field==='slot'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="1" max="2" autofocus />
              {:else}<span class="cursor-pointer">{c?.slot ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center font-mono" onclick={()=>startEdit(chNum, 'call_id', c?.call_id)}>
              {#if isEdit && editing.field==='call_id'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} autofocus />
              {:else}<span class="cursor-pointer">{c?.call_id ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center font-mono" onclick={()=>startEdit(chNum, 'own_id', c?.own_id)}>
              {#if isEdit && editing.field==='own_id'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} autofocus />
              {:else}<span class="cursor-pointer">{c?.own_id ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'ch_type', c?.ch_type)}>
              {#if isEdit && editing.field==='ch_type'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="1" autofocus />
              {:else}<span class="cursor-pointer">{c?.ch_type ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'sqlevel', c?.sqlevel)}>
              {#if isEdit && editing.field==='sqlevel'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="1" max="5" autofocus />
              {:else}<span class="cursor-pointer">{c?.sqlevel ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'spkgain', c?.spkgain)}>
              {#if isEdit && editing.field==='spkgain'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="10" autofocus />
              {:else}<span class="cursor-pointer">{c?.spkgain ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'ch_bs_mode', c?.ch_bs_mode)}>
              {#if isEdit && editing.field==='ch_bs_mode'}<input type="number" class="input input-sm input-bordered w-full font-mono text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="1" autofocus />
              {:else}<span class="cursor-pointer">{c?.ch_bs_mode ?? '—'}</span>{/if}
            </td>
            <td class="py-0.5 px-1 text-center">
              <button class="btn btn-sm btn-ghost text-[12px] px-1 h-6" onclick={()=>readChannel(chNum)}>R</button>
              <button class="btn btn-sm btn-ghost text-[12px] px-1 h-6 {c?'text-success':''}" onclick={()=>writeChannel(chNum)} disabled={!c}>W</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
    {/if}
  </div>
</div>
