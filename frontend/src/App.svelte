<script>
  import './app.css';
  import { onMount, onDestroy } from 'svelte';
  import { onEvent } from './lib/tauri-bridge.js';
  import { radioStatus, radioParams, meterData, spectrumData, connectionStatus } from './lib/store.js';

  import SerialConnect from './components/SerialConnect.svelte';
  import StatusBar from './components/StatusBar.svelte';
  import FrequencyControl from './components/FrequencyControl.svelte';
  import ModeSelector from './components/ModeSelector.svelte';
  import PttButton from './components/PttButton.svelte';
  import MeterDisplay from './components/MeterDisplay.svelte';
  import VfoPanel from './components/VfoPanel.svelte';
  import AudioControls from './components/AudioControls.svelte';
  import RfControls from './components/RfControls.svelte';
  import NrNbControls from './components/NrNbControls.svelte';
  import SpectrumControls from './components/SpectrumControls.svelte';
  import CwControls from './components/CwControls.svelte';
  import TunerControl from './components/TunerControl.svelte';
  import SpectrumCanvas from './canvas/SpectrumCanvas.svelte';
  import WaterfallCanvas from './canvas/WaterfallCanvas.svelte';

  let unlisteners = [];
  let conn = $derived($connectionStatus);

  onMount(() => {
    unlisteners = [
      onEvent('radio-status', (d) => radioStatus.update(s => ({ ...s, ...d }))),
      onEvent('radio-params', (d) => radioParams.update(s => ({ ...s, ...d }))),
      onEvent('meter-data', (d) => meterData.update(s => ({ ...s, ...d }))),
      onEvent('spectrum-data', (d) => spectrumData.set({ data: new Uint8Array(d), timestamp: Date.now() })),
      onEvent('radio-error', (d) => console.warn('Radio error:', d)),
    ];
  });

  onDestroy(() => {
    unlisteners.forEach(fn => fn());
  });
</script>

<main class="app-layout">
  <!-- 未连接时全屏遮罩 -->
  {#if !conn.connected}
    <SerialConnect />
  {/if}

  <!-- 顶部栏 -->
  <header class="top-bar">
    <h1 class="app-title">GH-Terminal</h1>
    {#if conn.connected}
      <StatusBar />
      <div style="margin-left:auto;"></div>
      <SerialConnect />
    {/if}
  </header>

  {#if conn.connected}
    <div class="main-content">
      <section class="spectrum-area">
        <div class="canvas-container" style="height: 150px;">
          <SpectrumCanvas />
        </div>
        <div class="canvas-container" style="height: 80px;">
          <WaterfallCanvas />
        </div>
        <SpectrumControls />
      </section>

      <section class="controls-area">
        <div class="controls-grid">
          <div class="panel">
            <FrequencyControl />
            <ModeSelector />
            <PttButton />
          </div>
          <div class="panel">
            <VfoPanel />
          </div>
          <div class="panel">
            <MeterDisplay />
          </div>
          <div class="panel">
            <AudioControls />
          </div>
          <div class="panel">
            <RfControls />
          </div>
          <div class="panel">
            <NrNbControls />
          </div>
          <div class="panel">
            <TunerControl />
          </div>
          <div class="panel">
            <CwControls />
          </div>
        </div>
      </section>
    </div>
  {:else}
    <!-- 未连接时的占位界面 -->
    <div class="placeholder">
      <div class="placeholder-icon">🔌</div>
      <p>请先连接电台串口设备</p>
    </div>
  {/if}
</main>

<style>
  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
  .top-bar {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 8px 16px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }
  .app-title {
    font-size: 16px;
    color: var(--accent);
    letter-spacing: 2px;
    white-space: nowrap;
  }
  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .spectrum-area {
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex-shrink: 0;
  }
  .controls-area {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }
  .controls-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 8px;
  }
  .placeholder {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    gap: 16px;
    opacity: 0.5;
  }
  .placeholder-icon {
    font-size: 64px;
  }
  .placeholder p {
    font-size: 16px;
  }
</style>
