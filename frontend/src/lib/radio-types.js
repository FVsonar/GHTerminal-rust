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

// CTCSS 亚音表 (索引 → 频率Hz)
export const CTCSS_TABLE = [
  '-', '67.0', '69.3', '71.9', '74.4', '77.0', '79.7', '82.5', '85.4', '88.5',
  '91.5', '94.8', '97.4', '100.0', '103.5', '107.2', '110.9', '114.8', '118.8', '123.0',
  '127.3', '131.8', '136.5', '141.3', '146.2', '150.0', '151.4', '156.7', '159.8', '162.2',
  '165.5', '167.9', '171.3', '173.8', '177.3', '179.9', '183.5', '186.2', '189.9', '192.8',
  '196.6', '199.5', '203.5', '206.5', '210.7', '213.8', '218.1', '221.3', '225.7', '229.1',
  '233.6', '237.1', '241.8', '245.5', '250.3', '254.1',
];
