<script>
  import './app.css';
  import { onMount, onDestroy } from 'svelte';
  import { radioSocket } from './lib/ws.js';
  import { radioStatus, radioParams, meterData, spectrumData, connectionStatus } from './lib/store.js';

  import ConnectionBar from './components/ConnectionBar.svelte';
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

  let cleanup = null;

  onMount(() => {
    radioSocket.connect();
    radioSocket.emit = (event, data) => {
      switch (event) {
        case 'connected':
          connectionStatus.set({ connected: true, port: '', error: null });
          break;
        case 'disconnected':
          connectionStatus.set({ connected: false, port: '', error: '连接断开' });
          break;
        case 'error':
          connectionStatus.update(s => ({ ...s, error: data }));
          break;
        case 'status':
          radioStatus.update(s => ({ ...s, ...data }));
          break;
        case 'params':
          radioParams.update(s => ({ ...s, ...data }));
          break;
        case 'meter':
          meterData.update(s => ({ ...s, ...data }));
          break;
        case 'spectrum':
          spectrumData.set({ data: new Uint8Array(data), timestamp: Date.now() });
          break;
      }
    };
  });

  onDestroy(() => {
    radioSocket.close();
  });
</script>

<main class="app-layout">
  <header class="top-bar">
    <h1 class="app-title">GH-Terminal</h1>
    <StatusBar />
    <ConnectionBar />
  </header>

  <div class="main-content">
    <!-- 频谱区域 -->
    <section class="spectrum-area">
      <div class="canvas-container" style="height: 150px;">
        <SpectrumCanvas />
      </div>
      <div class="canvas-container" style="height: 80px;">
        <WaterfallCanvas />
      </div>
      <SpectrumControls />
    </section>

    <!-- 控制面板区域 -->
    <section class="controls-area">
      <div class="controls-grid">
        <!-- 频率 & 模式 -->
        <div class="panel">
          <FrequencyControl />
          <ModeSelector />
          <PttButton />
        </div>

        <!-- VFO -->
        <div class="panel">
          <VfoPanel />
        </div>

        <!-- S 表 / 功率表 -->
        <div class="panel">
          <MeterDisplay />
        </div>

        <!-- AF 音频 -->
        <div class="panel">
          <AudioControls />
        </div>

        <!-- RF 射频 -->
        <div class="panel">
          <RfControls />
        </div>

        <!-- NR/NB -->
        <div class="panel">
          <NrNbControls />
        </div>

        <!-- 天调 -->
        <div class="panel">
          <TunerControl />
        </div>

        <!-- CW -->
        <div class="panel">
          <CwControls />
        </div>
      </div>
    </section>
  </div>
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
</style>
