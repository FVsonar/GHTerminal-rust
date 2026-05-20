<script>
  let { on = true, ontoggle } = $props();
</script>

<button class="switch" class:active={on} onclick={() => ontoggle(!on)} title={on ? '开启' : '关闭'}>
  <span class="switch-track">
    <span class="switch-thumb"></span>
  </span>
</button>

<style>
  .switch {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }
  .switch-track {
    width: 28px;
    height: 15px;
    border-radius: 8px;
    background: var(--border);
    transition: background 0.2s;
    position: relative;
    display: flex;
    align-items: center;
  }
  .switch.active .switch-track {
    background: rgba(0,229,160,0.3);
  }
  .switch-thumb {
    width: 11px;
    height: 11px;
    border-radius: 50%;
    background: var(--text-muted);
    position: absolute;
    left: 2px;
    transition: all 0.2s;
  }
  .switch.active .switch-thumb {
    background: var(--accent-green);
    left: 15px;
    box-shadow: 0 0 6px rgba(0,229,160,0.5);
  }
</style>
