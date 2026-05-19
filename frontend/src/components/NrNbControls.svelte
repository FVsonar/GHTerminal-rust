<script>
  import { radioSocket } from '../lib/ws.js';
  import { radioParams } from '../lib/store.js';

  let p = $derived($radioParams);
</script>

<div>
  <div class="panel-title">降噪</div>

  <div class="flex-row">
    <button class:active={p.nr === 1}
      onclick={() => radioSocket.cmd('set_nr', { on: p.nr !== 1 })}>
      NR {p.nr === 1 ? '开' : '关'}
    </button>
    <button class:active={p.nb === 1}
      onclick={() => radioSocket.cmd('set_nb', { on: p.nb !== 1 })}>
      NB {p.nb === 1 ? '开' : '关'}
    </button>
  </div>

  <label style="margin-top:8px;">NR 阀值: {p.nr || 0}</label>
  <input type="range" min="1" max="200" value={p.nr || 1}
    oninput={(e) => radioSocket.cmd('set_nr_threshold', { value: parseInt(e.target.value) })} />

  <label>NB 阀值: {p.nb || 0}</label>
  <input type="range" min="0" max="15" value={p.nb || 0}
    oninput={(e) => radioSocket.cmd('set_nb_threshold', { value: parseInt(e.target.value) })} />

  <label>PEAK 阀值: {p.pk}</label>
  <input type="range" min="0" max="20" value={p.pk}
    oninput={(e) => radioSocket.cmd('set_peak_threshold', { value: parseInt(e.target.value) })} />
</div>
