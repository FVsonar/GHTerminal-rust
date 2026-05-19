// 响应式状态存储 — 从 WebSocket 事件更新 Svelte stores
import { writable, derived } from 'svelte/store';

// ---- 电台状态 ----
export const radioStatus = writable({
  tx: false,
  fA: 14074000,
  fB: 7100000,
  mA: 0,
  mB: 0,
  v: 0,
  nr: 0,
  nb: 0,
  rit: 0,
  xit: 0,
  filt: 10,
  span: 2,
  volt: 13.8,
  utc: [8, 0, 0],
  sb: 0,
  sm: 0,
  swr: 0,
});

// ---- 参数 ----
export const radioParams = writable({
  sv: 20,
  hv: 40,
  mg: 50,
  cmp: 7,
  bass: 20,
  treb: 20,
  rfg: 60,
  ifg: 40,
  sql: 5,
  agc: 3,
  amp: 0,
  nr: 0,
  nb: 0,
  pk: 10,
  ref: 10,
  spd: 5,
});

// ---- 仪表 ----
export const meterData = writable({
  sp: 0,
  swr: 0,
});

// ---- 频谱数据 ----
export const spectrumData = writable({
  data: new Uint8Array(0),
  timestamp: 0,
});

// ---- 连接状态 ----
export const connectionStatus = writable({
  connected: false,
  port: '',
  error: null,
});

// ---- 派生状态 ----
export const currentFreq = derived(radioStatus, ($s) =>
  $s.v === 0 ? $s.fA : $s.fB
);

export const currentMode = derived(radioStatus, ($s) =>
  $s.v === 0 ? $s.mA : $s.mB
);

export const freqDisplay = derived(currentFreq, ($f) => {
  const mhz = $f / 1000000;
  return mhz.toFixed(3).replace('.', '.');
});

export const sMeterValue = derived(meterData, ($m) => {
  // S表: BIT7=0, 值0~34
  if (($m.sp & 0x80) === 0) return $m.sp & 0x7F;
  return 0;
});

export const poMeterValue = derived(meterData, ($m) => {
  // PO表: BIT7=1, 值0~34
  if (($m.sp & 0x80) !== 0) return $m.sp & 0x7F;
  return 0;
});
