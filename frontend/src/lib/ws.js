// WebSocket 客户端 — 处理与后端的双向通信
export class RadioSocket {
  constructor() {
    this.ws = null;
    this.listeners = new Map();
    this.reconnectTimer = null;
    this.connected = false;
  }

  connect(url) {
    if (this.ws) this.ws.close();

    const wsUrl = url || `ws://${location.host}/ws`;
    this.ws = new WebSocket(wsUrl);
    this.ws.binaryType = 'arraybuffer';

    this.ws.onopen = () => {
      this.connected = true;
      this.emit('connected');
    };

    this.ws.onclose = () => {
      this.connected = false;
      this.emit('disconnected');
      // 自动重连
      this.reconnectTimer = setTimeout(() => this.connect(url), 3000);
    };

    this.ws.onerror = (e) => {
      this.emit('error', 'WebSocket 连接错误');
    };

    this.ws.onmessage = (event) => {
      if (event.data instanceof ArrayBuffer) {
        this.emit('spectrum', new Uint8Array(event.data));
      } else {
        try {
          const msg = JSON.parse(event.data);
          this.emit(msg.t, msg.d);
        } catch (e) {
          console.warn('Failed to parse WS message:', event.data);
        }
      }
    };
  }

  send(type, command, data = {}) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ t: type, c: command, d: data }));
    }
  }

  // 发送控制命令
  cmd(command, data = {}) {
    this.send('cmd', command, data);
  }

  on(event, callback) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event).push(callback);
    return () => this.off(event, callback);
  }

  off(event, callback) {
    const cbs = this.listeners.get(event);
    if (cbs) {
      const idx = cbs.indexOf(callback);
      if (idx >= 0) cbs.splice(idx, 1);
    }
  }

  emit(event, data) {
    const cbs = this.listeners.get(event);
    if (cbs) {
      cbs.forEach((cb) => cb(data));
    }
  }

  close() {
    if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
    if (this.ws) this.ws.close();
  }
}

// 单例
export const radioSocket = new RadioSocket();
