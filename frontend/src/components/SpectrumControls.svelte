<script>
  import { radioSocket } from '../lib/ws.js';
  import { radioParams, radioStatus } from '../lib/store.js';
  import { SPECTRUM_SPANS, DISPLAY_MODES } from '../lib/radio-types.js';

  let p = $derived($radioParams);
  let s = $derived($radioStatus);
</script>

<div>
  <div class="panel-title">频谱</div>
  <div class="flex-row" style="flex-wrap: wrap;">

    <label style="flex:1;">
      SPAN: {SPECTRUM_SPANS[s.span] || '?'}
      <select value={s.span}
        onchange={(e) => radioSocket.cmd('set_spectrum_span', { span: parseInt(e.target.value) })}>
        {#each Object.entries(SPECTRUM_SPANS) as [k, v]}
          <option value={k}>{v}</option>
        {/each}
      </select>
    </label>

    <label style="flex:1;">REF: {p.ref || 10}
      <input type="range" min="1" max="20" value={p.ref || 10}
        oninput={(e) => radioSocket.cmd('set_spectrum_ref', { value: parseInt(e.target.value) })} />
    </label>

    <label style="flex:1;">速率: {p.spd || 5}
      <input type="range" min="1" max="30" value={p.spd || 5}
        oninput={(e) => radioSocket.cmd('set_spectrum_speed', { value: parseInt(e.target.value) })} />
    </label>
  </div>

  <label style="margin-top:4px;">显示模式</label>
  <select onchange={(e) => radioSocket.cmd('set_display_mode', { mode: parseInt(e.target.value) })}>
    {#each DISPLAY_MODES as m, i}
      <option value={i}>{m}</option>
    {/each}
  </select>
</div>
