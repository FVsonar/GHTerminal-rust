<script>
  import { sendCommand, onEvent } from '../lib/tauri-bridge.js';
  import { onMount } from 'svelte';
  import { MODE_LIST, CTCSS_TABLE } from '../lib/radio-types.js';

  const PAGE_SIZE = 50, TOTAL_CH = 1000;
  let channels = $state({});
  let page = $state(0), pageInput = $state('0');
  let editing = $state(null), editVal = $state('');
  let reading = $state(false), readProgress = $state(0);
  let chMode = $state(0);
  let start = $derived(page * PAGE_SIZE), end = $derived(Math.min(start + PAGE_SIZE, TOTAL_CH));
  let totalPages = $derived(Math.ceil(TOTAL_CH / PAGE_SIZE));

  onMount(() => { onEvent('channel-data', (d) => { channels = { ...channels, [d.channel]: d }; }); });

  function modeName(v) { return MODE_LIST.find(m => m.v === v)?.n || 'USB'; }
  function ctcssName(idx) { return CTCSS_TABLE[idx] || String(idx); }

  function readChannel(n) { sendCommand('channel_read', { channel: n }); }

  async function readPage() {
    reading = true; readProgress = 0;
    for (let i = 0; i < end - start; i++) {
      sendCommand('channel_read', { channel: start + i });
      readProgress = i + 1;
      await new Promise(r => setTimeout(r, 220));
    }
    reading = false;
  }

  function startEdit(ch, field, current) {
    editing = { ch, field };
    editVal = field.startsWith('ctcss') ? String(current || 0) : String(current || '');
  }

  function commitEdit() {
    if (!editing) return;
    const { ch, field } = editing;
    const cur = channels[ch] || {};
    const updated = { ...cur, channel: ch };
    if (field === 'freq_a') updated.vfoa_freq = Math.round(parseFloat(editVal) * 1_000_000) || 0;
    else if (field === 'freq_b') updated.vfob_freq = Math.round(parseFloat(editVal) * 1_000_000) || 0;
    else if (field === 'mode_a') updated.vfoa_mode = parseInt(editVal);
    else if (field === 'mode_b') updated.vfob_mode = parseInt(editVal);
    else if (field === 'name') updated.name = editVal.slice(0, 12);
    else if (field === 'ctcss_tx') updated.tx_ctcss = parseInt(editVal) || 0;
    else if (field === 'ctcss_rx') updated.rx_ctcss = parseInt(editVal) || 0;
    channels = { ...channels, [ch]: updated };
    editing = null;
  }
  function cancelEdit() { editing = null; }
  function handleEditKey(e) { if (e.key === 'Enter') commitEdit(); if (e.key === 'Escape') cancelEdit(); }

  function writeChannel(n) {
    const c = channels[n]; if (!c) return;
    sendCommand('channel_write', { channel: n, vfoa_freq: c.vfoa_freq||0, vfob_freq: c.vfob_freq||0, vfoa_mode: c.vfoa_mode||0, vfob_mode: c.vfob_mode||0, tx_ctcss: c.tx_ctcss||0, rx_ctcss: c.rx_ctcss||0, name: c.name||'' });
  }

  async function writePage() {
    for (let i = 0; i < end - start; i++) {
      const n = start + i;
      if (channels[n]) { writeChannel(n); await new Promise(r => setTimeout(r, 220)); }
    }
  }

  function goPage(p) { const pg = Math.max(0, Math.min(p, totalPages - 1)); page = pg; pageInput = String(pg); }
  function setMode(m) { chMode = m; sendCommand('channel_mode', { mode: m }); }
  function fmtFreq(hz) { return hz ? (hz / 1_000_000).toFixed(5) : ''; }
</script>

<div class="card bg-base-200 border border-base-300 shadow-sm p-3">
  <span class="text-[10px] font-semibold text-base-content/50 uppercase tracking-widest mb-2.5 block">信道管理 (0-999)</span>

  <div class="flex items-end gap-2 mb-2.5">
    <div class="flex items-end gap-1.5 flex-1">
      <div>
        <span class="text-[9px] text-base-content/50 block mb-0.5">模式</span>
        <div class="flex gap-1">
          <button class="btn btn-xs text-[11px] {chMode===0?'btn-primary':'btn-ghost'}" onclick={()=>setMode(0)}>VFO</button>
          <button class="btn btn-xs text-[11px] {chMode===1?'btn-primary':'btn-ghost'}" onclick={()=>setMode(1)}>信道</button>
        </div>
      </div>
    </div>
    <button class="btn btn-xs btn-primary" onclick={readPage} disabled={reading}>
      {reading ? `读取中 ${readProgress}/${end-start}` : `读第${page}页`}
    </button>
    <button class="btn btn-xs btn-success btn-ghost" onclick={writePage}>写本页</button>
  </div>

  <!-- 分页 -->
  <div class="flex items-center justify-center gap-1.5 mb-2">
    <button class="btn btn-xs btn-ghost" onclick={()=>goPage(0)} disabled={page===0}>«</button>
    <button class="btn btn-xs btn-ghost" onclick={()=>goPage(page-1)} disabled={page===0}>‹</button>
    <input type="number" class="input input-xs input-bordered w-12 text-center font-mono text-xs" bind:value={pageInput} min="0" max={totalPages-1} onkeydown={(e)=>{if(e.key==='Enter')goPage(parseInt(pageInput)||0)}} />
    <span class="text-[10px] text-base-content/50">/ {totalPages - 1}</span>
    <button class="btn btn-xs btn-ghost" onclick={()=>goPage(page+1)} disabled={page>=totalPages-1}>›</button>
    <button class="btn btn-xs btn-ghost" onclick={()=>goPage(totalPages-1)} disabled={page>=totalPages-1}>»</button>
    <span class="text-[10px] text-base-content/40 ml-2">#{start}-#{end-1}</span>
  </div>

  <!-- 表格 -->
  <div class="overflow-auto max-h-[360px] rounded-md border border-base-300">
    <table class="w-full text-[11px] border-collapse">
      <thead class="sticky top-0 z-10">
        <tr class="bg-base-300 text-base-content/50 text-[9px] uppercase tracking-wide">
          <th class="py-1.5 px-1 text-center font-medium w-7">#</th>
          <th class="py-1.5 px-1 text-left font-medium">名称</th>
          <th class="py-1.5 px-1 text-right font-medium w-[72px]">VFOA</th>
          <th class="py-1.5 px-1 text-center font-medium w-8">M</th>
          <th class="py-1.5 px-1 text-right font-medium w-[72px]">VFOB</th>
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
            <td class="py-0.5 px-1 text-center font-mono text-base-content/40 text-[10px]">{chNum}</td>

            <!-- 名称 -->
            <td class="py-0.5 px-1" onclick={()=>startEdit(chNum, 'name', c?.name)}>
              {#if isEdit && editing.field === 'name'}
                <input type="text" class="input input-xs input-bordered w-full text-[11px]" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} maxlength="12" autofocus />
              {:else}
                <span class="cursor-pointer {c?.name?'font-medium':''}">{c?.name || '—'}</span>
              {/if}
            </td>

            <!-- VFOA 频率 -->
            <td class="py-0.5 px-1 text-right" onclick={()=>startEdit(chNum, 'freq_a', fmtFreq(c?.vfoa_freq))}>
              {#if isEdit && editing.field === 'freq_a'}
                <input type="text" class="input input-xs input-bordered w-full font-mono text-[11px] text-right" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} autofocus />
              {:else}
                <span class="cursor-pointer font-mono {c?'text-success':''}">{c ? fmtFreq(c.vfoa_freq) : '—'}</span>
              {/if}
            </td>

            <!-- VFOA 模式 -->
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'mode_a', c?.vfoa_mode)}>
              {#if isEdit && editing.field === 'mode_a'}
                <select class="select select-xs select-bordered w-full text-[10px]" bind:value={editVal} onchange={commitEdit}>
                  {#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}
                </select>
              {:else}
                <span class="cursor-pointer">{c ? modeName(c.vfoa_mode) : '—'}</span>
              {/if}
            </td>

            <!-- VFOB 频率 -->
            <td class="py-0.5 px-1 text-right" onclick={()=>startEdit(chNum, 'freq_b', fmtFreq(c?.vfob_freq))}>
              {#if isEdit && editing.field === 'freq_b'}
                <input type="text" class="input input-xs input-bordered w-full font-mono text-[11px] text-right" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} autofocus />
              {:else}
                <span class="cursor-pointer font-mono {c?'text-success':''}">{c ? fmtFreq(c.vfob_freq) : '—'}</span>
              {/if}
            </td>

            <!-- VFOB 模式 -->
            <td class="py-0.5 px-1 text-center" onclick={()=>startEdit(chNum, 'mode_b', c?.vfob_mode)}>
              {#if isEdit && editing.field === 'mode_b'}
                <select class="select select-xs select-bordered w-full text-[10px]" bind:value={editVal} onchange={commitEdit}>
                  {#each MODE_LIST as m}<option value={m.v}>{m.n}</option>{/each}
                </select>
              {:else}
                <span class="cursor-pointer">{c ? modeName(c.vfob_mode) : '—'}</span>
              {/if}
            </td>

            <!-- TX CTCSS -->
            <td class="py-0.5 px-1 text-center font-mono" onclick={()=>startEdit(chNum, 'ctcss_tx', c?.tx_ctcss)}>
              {#if isEdit && editing.field === 'ctcss_tx'}
                <input type="number" class="input input-xs input-bordered w-full font-mono text-[11px] text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="55" autofocus />
              {:else}
                <span class="cursor-pointer text-[10px]" title={ctcssName(c?.tx_ctcss || 0)}>{c ? ctcssName(c.tx_ctcss) : '—'}</span>
              {/if}
            </td>

            <!-- RX CTCSS -->
            <td class="py-0.5 px-1 text-center font-mono" onclick={()=>startEdit(chNum, 'ctcss_rx', c?.rx_ctcss)}>
              {#if isEdit && editing.field === 'ctcss_rx'}
                <input type="number" class="input input-xs input-bordered w-full font-mono text-[11px] text-center" bind:value={editVal} onkeydown={handleEditKey} onblur={commitEdit} min="0" max="55" autofocus />
              {:else}
                <span class="cursor-pointer text-[10px]" title={ctcssName(c?.rx_ctcss || 0)}>{c ? ctcssName(c.rx_ctcss) : '—'}</span>
              {/if}
            </td>

            <!-- 操作 -->
            <td class="py-0.5 px-1 text-center">
              <div class="flex gap-0.5 justify-center">
                <button class="btn btn-xs btn-ghost text-[9px] px-1 h-5" onclick={()=>readChannel(chNum)}>R</button>
                <button class="btn btn-xs btn-ghost text-[9px] px-1 h-5 {c?'text-success':''}" onclick={()=>writeChannel(chNum)} disabled={!c}>W</button>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
