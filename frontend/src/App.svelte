<script>
  import './app.css';
  import { onMount } from 'svelte';
  import { onEvent } from './lib/tauri-bridge.js';
  import { radioStatus, radioParams, meterData, spectrumData, connectionStatus } from './lib/store.js';

  import SerialConnect from './components/SerialConnect.svelte';
  import StatusBar from './components/StatusBar.svelte';
  import MeterDisplay from './components/MeterDisplay.svelte';
  import FrequencyControl from './components/FrequencyControl.svelte';
  import ModeSelector from './components/ModeSelector.svelte';
  import PttButton from './components/PttButton.svelte';
  import VfoPanel from './components/VfoPanel.svelte';
  import AudioControls from './components/AudioControls.svelte';
  import RfControls from './components/RfControls.svelte';
  import NrNbControls from './components/NrNbControls.svelte';
  import SpectrumControls from './components/SpectrumControls.svelte';
  import CwControls from './components/CwControls.svelte';
  import TunerControl from './components/TunerControl.svelte';
  import PollControls from './components/PollControls.svelte';
  import SpectrumCanvas from './canvas/SpectrumCanvas.svelte';
  import WaterfallCanvas from './canvas/WaterfallCanvas.svelte';

  let unlisteners = [];

  onMount(() => {
    Promise.all([
      onEvent('radio-status', (d) => radioStatus.update(s => ({ ...s, ...d }))),
      onEvent('radio-params', (d) => radioParams.update(s => ({ ...s, ...d }))),
      onEvent('meter-data', (d) => meterData.update(s => ({ ...s, ...d }))),
      onEvent('spectrum-data', (d) => spectrumData.set({ data: new Uint8Array(d), timestamp: Date.now() })),
      onEvent('radio-error', () => {}),
    ]).then(fns => { unlisteners = fns; });

    return () => unlisteners.forEach(fn => fn());
  });
</script>

<!-- 未连接遮罩 -->
{#if !$connectionStatus.connected}
  <SerialConnect />
{/if}

<main class="app-shell">
  <!-- 顶部栏 -->
  <header class="header">
    <div class="header-left">
      <div class="logo-dot"></div>
      <span class="logo-text">GH-Terminal</span>
    </div>
    {#if $connectionStatus.connected}
      <StatusBar />
    {/if}
    <div class="header-right">
      {#if $connectionStatus.connected}
        <SerialConnect />
      {/if}
    </div>
  </header>

  {#if $connectionStatus.connected}
    <div class="main-area">
      <!-- 仪表行 -->
      <div class="meter-row">
        <MeterDisplay />
      </div>

      <!-- 频谱 -->
      <div class="spectrum-section">
        <div class="spectrum-canvas-wrap">
          <SpectrumCanvas />
        </div>
        <div class="waterfall-canvas-wrap">
          <WaterfallCanvas />
        </div>
        <div class="spectrum-ctrl-wrap">
          <SpectrumControls />
        </div>
      </div>

      <!-- 控制面板 -->
      <div class="controls-scroll">
        <div class="controls-grid">
          <FrequencyControl />
          <ModeSelector />
          <PttButton />
          <VfoPanel />
          <AudioControls />
          <RfControls />
          <NrNbControls />
          <PollControls />
          <TunerControl />
          <CwControls />
        </div>
      </div>
    </div>
  {:else}
    <div class="placeholder-screen">
      <div class="placeholder-icon">📡</div>
      <p>请连接电台设备</p>
    </div>
  {/if}
</main>

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .header {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 0 16px;
    height: 48px;
    background: var(--bg-panel);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    z-index: 10;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .logo-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--accent-green);
    box-shadow: 0 0 8px rgba(0,229,160,0.5);
  }

  .logo-text {
    font-size: 14px;
    font-weight: 600;
    color: var(--accent-green);
    letter-spacing: 1px;
  }

  .header-right {
    margin-left: auto;
    display: flex;
    align-items: center;
  }

  .main-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* 仪表行 */
  .meter-row {
    padding: 8px 16px 4px;
    flex-shrink: 0;
  }

  /* 频谱区域 */
  .spectrum-section {
    padding: 4px 16px 8px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex-shrink: 0;
  }
  .spectrum-canvas-wrap {
    height: 110px;
    background: #000;
    border-radius: var(--radius-sm);
    overflow: hidden;
    border: 1px solid var(--border);
  }
  .waterfall-canvas-wrap {
    height: 45px;
    background: #000;
    border-radius: var(--radius-sm);
    overflow: hidden;
    border: 1px solid var(--border);
  }
  .spectrum-ctrl-wrap {
    margin-top: 4px;
  }

  /* 控制面板滚动区 */
  .controls-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 4px 16px 12px;
  }
  .controls-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(248px, 1fr));
    gap: 8px;
  }

  .placeholder-screen {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    gap: 14px;
  }
  .placeholder-icon { font-size: 56px; opacity: 0.6; }
  .placeholder-screen p { font-size: 15px; }
</style>
