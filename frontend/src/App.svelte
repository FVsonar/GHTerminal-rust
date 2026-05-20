<script>
  import './app.css';
  import { onMount } from 'svelte';
  import { onEvent } from './lib/tauri-bridge.js';
  import { radioStatus, radioParams, meterData, spectrumData, connectionStatus } from './lib/store.js';

  import SerialConnect from './components/SerialConnect.svelte';
  import StatusBar from './components/StatusBar.svelte';
  import MeterDisplay from './components/MeterDisplay.svelte';
  import FrequencyControl from './components/FrequencyControl.svelte';
  import VfoPanel from './components/VfoPanel.svelte';
  import AudioControls from './components/AudioControls.svelte';
  import RfControls from './components/RfControls.svelte';
  import NrNbControls from './components/NrNbControls.svelte';
  import SpectrumControls from './components/SpectrumControls.svelte';
  import CwControls from './components/CwControls.svelte';
  import ChannelPanel from './components/ChannelPanel.svelte';
  import SpectrumCanvas from './canvas/SpectrumCanvas.svelte';
  import WaterfallCanvas from './canvas/WaterfallCanvas.svelte';

  let unlisteners = [];
  let dark = $state(true);
  let toast = $state(null); // {text, ok}

  function toggleTheme() {
    dark = !dark;
    const theme = dark ? 'business' : 'corporate';
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
  }

  onMount(() => {
    const saved = localStorage.getItem('theme');
    dark = saved !== 'corporate';
    document.documentElement.setAttribute('data-theme', dark ? 'business' : 'corporate');

    Promise.all([
      onEvent('radio-status', (d) => radioStatus.update(s => ({ ...s, ...d }))),
      onEvent('radio-params', (d) => radioParams.update(s => ({ ...s, ...d }))),
      onEvent('meter-data', (d) => meterData.update(s => ({ ...s, ...d }))),
      onEvent('spectrum-data', (d) => spectrumData.set({ data: new Uint8Array(d), timestamp: Date.now() })),
      onEvent('radio-error', () => {}),
      onEvent('cmd-result', (d) => {
        toast = { text: d.cmd + (d.ok ? ' ✓' : ' ✗'), ok: d.ok };
        setTimeout(() => { toast = null; }, 1500);
      }),
    ]).then(fns => { unlisteners = fns; });
    return () => unlisteners.forEach(fn => fn());
  });
</script>

{#if !$connectionStatus.connected}
  <SerialConnect />
{/if}

<main class="h-screen flex flex-col overflow-hidden bg-base-100">
  <header class="flex items-center gap-4 px-4 h-12 bg-base-200 border-b border-base-300 shrink-0 z-10">
    <div class="flex items-center gap-2.5">
      <span class="w-2.5 h-2.5 rounded-full bg-success shadow-[0_0_8px_rgba(0,229,160,.5)]"></span>
      <span class="text-sm font-semibold text-success tracking-wider">GH-Terminal</span>
      <button class="btn btn-ghost btn-xs text-base ml-2" onclick={toggleTheme} title="切换主题">
        {dark ? '☀' : '🌙'}
      </button>
    </div>
    {#if $connectionStatus.connected}
      <StatusBar />
    {/if}
    <div class="ml-auto flex items-center">
      {#if $connectionStatus.connected}
        <SerialConnect />
      {/if}
    </div>
  </header>

  {#if $connectionStatus.connected}
    <div class="flex-1 flex flex-col overflow-hidden">
      <div class="px-4 pt-2 pb-0 shrink-0">
        <MeterDisplay />
      </div>
      <div class="px-4 pt-1 pb-2 shrink-0 flex flex-col gap-0.5">
        <div class="h-[110px] bg-black rounded-md overflow-hidden border border-base-300">
          <SpectrumCanvas />
        </div>
        <div class="h-[45px] bg-black rounded-md overflow-hidden border border-base-300">
          <WaterfallCanvas />
        </div>
        <SpectrumControls />
      </div>
      <div class="flex-1 overflow-y-auto px-4 pb-3 pt-1">
        <div class="grid grid-cols-[repeat(auto-fill,minmax(248px,1fr))] gap-2">
          <FrequencyControl />
          <VfoPanel />
          <AudioControls />
          <RfControls />
          <NrNbControls />
          <CwControls />
          <ChannelPanel />
        </div>
      </div>
    </div>
  {:else}
    <div class="flex-1 flex flex-col items-center justify-center text-base-content/50 gap-3">
      <span class="text-6xl opacity-50">📡</span>
      <p class="text-base">请连接电台设备</p>
    </div>
  {/if}

  {#if toast}
    <div class="fixed bottom-4 right-4 z-50 transition-all duration-200 {toast.ok ? 'text-success' : 'text-error'} bg-base-200 border {toast.ok ? 'border-success/30' : 'border-error/30'} rounded-lg px-4 py-2 shadow-lg text-sm font-mono">
      {toast.text}
    </div>
  {/if}
</main>
