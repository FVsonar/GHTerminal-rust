// 电台类型常量 — 与 Rust gh-protocol::types 保持一致

export const MODES = {
  0: 'USB',
  1: 'LSB',
  2: 'CWR',
  3: 'CWL',
  4: 'AM',
  5: 'WFM',
  6: 'NFM',
  7: 'DIGI',
  8: 'PKT',
};

export const MODE_LIST = [
  { v: 0, n: 'USB' },
  { v: 1, n: 'LSB' },
  { v: 2, n: 'CWR' },
  { v: 3, n: 'CWL' },
  { v: 4, n: 'AM' },
  { v: 5, n: 'WFM' },
  { v: 6, n: 'NFM' },
  { v: 7, n: 'DIGI' },
  { v: 8, n: 'PKT' },
];

export const SPECTRUM_SPANS = {
  0: '48K',
  1: '24K',
  2: '12K',
  3: '6K',
  4: '3K',
  5: '1.5K',
};

export const BANDS = [
  { v: 0, n: '1.8 MHz' },
  { v: 1, n: '3.5 MHz' },
  { v: 2, n: '5 MHz' },
  { v: 3, n: '7 MHz' },
  { v: 4, n: '10 MHz' },
  { v: 5, n: '14 MHz' },
  { v: 6, n: '18 MHz' },
  { v: 7, n: '21 MHz' },
  { v: 8, n: '24 MHz' },
  { v: 9, n: '28 MHz' },
  { v: 10, n: '50 MHz' },
  { v: 11, n: '144 MHz' },
  { v: 12, n: '430 MHz' },
];

export const FILTER_NAMES = {
  // FM
  1: '<7.2k', 2: '<10.0k', 3: '<12.0k',
  // CW/SSB filters... (abbreviated)
};

export const AGC_LEVELS = ['AGC-0', 'AGC-1', 'AGC-2', 'AGC-3', 'AGC-4', 'AGC-5'];
export const AMP_OPTIONS = ['AMP-A', 'AMP-B'];
export const DISPLAY_MODES = ['频谱+瀑布', '频谱', '瀑布', '关闭'];
export const TUNER_MODES = ['关闭', '开启', '调谐中'];
export const KEY_TYPES = ['AUTO-L', 'AUTO-R', 'KEY'];
