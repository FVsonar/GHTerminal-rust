// Tauri 前后端通信桥接 — 替代 WebSocket
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// 发送控制命令 → Rust 后端
export async function sendCommand(cmd, data = {}) {
  try {
    await invoke('send_command', { cmd, data });
  } catch (e) {
    console.error('Command failed:', cmd, e);
  }
}

// 获取当前状态
export async function getStatus() {
  return invoke('get_status');
}

export async function getParams() {
  return invoke('get_params');
}

export async function getMeter() {
  return invoke('get_meter');
}

// 监听后端事件
export function onEvent(event, callback) {
  return listen(event, (e) => callback(e.payload));
}
