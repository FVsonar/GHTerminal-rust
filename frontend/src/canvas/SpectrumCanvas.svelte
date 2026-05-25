<script>
  import { spectrumData } from '../lib/store.js';

  let canvas;
  let ctx;
  let width = 1024;
  let height = 140;

  $effect(() => { ctx = canvas?.getContext('2d'); });

  let spec = $derived($spectrumData);

  $effect(() => {
    if (ctx && spec.data.length > 0) drawSpectrum(spec.data);
  });

  function drawSpectrum(data) {
    const w = canvas.width;
    const h = canvas.height;

    ctx.clearRect(0, 0, w, h);

    // 深色背景
    ctx.fillStyle = '#020810';
    ctx.fillRect(0, 0, w, h);

    // 水平网格 (dB 参考线)
    for (let i = 1; i < 5; i++) {
      const y = (h / 5) * i;
      ctx.strokeStyle = i === 2 ? 'rgba(255,255,255,0.06)' : 'rgba(255,255,255,0.03)';
      ctx.lineWidth = 0.5;
      ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke();
    }

    // 垂直网格
    for (let i = 1; i < 10; i++) {
      const x = (w / 10) * i;
      ctx.strokeStyle = 'rgba(255,255,255,0.025)';
      ctx.lineWidth = 0.5;
      ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, h); ctx.stroke();
    }

    const len = data.length;
    if (len === 0) return;

    // 频谱填充 (亮青渐变)
    const grad = ctx.createLinearGradient(0, 0, 0, h);
    grad.addColorStop(0, 'rgba(0,240,200,0.45)');
    grad.addColorStop(0.5, 'rgba(0,180,160,0.15)');
    grad.addColorStop(1, 'rgba(0,80,60,0.02)');

    ctx.beginPath();
    for (let i = 0; i < len; i++) {
      const x = (w / len) * i;
      const y = h - (data[i] / 255) * h * 0.92;
      if (i === 0) ctx.moveTo(x, h);
      ctx.lineTo(x, y);
    }
    ctx.lineTo(w, h);
    ctx.closePath();
    ctx.fillStyle = grad;
    ctx.fill();

    // 频谱主线 (亮青)
    ctx.strokeStyle = 'rgba(0,255,200,0.9)';
    ctx.lineWidth = 1.5;
    ctx.shadowColor = 'rgba(0,255,200,0.4)';
    ctx.shadowBlur = 3;
    ctx.beginPath();
    for (let i = 0; i < len; i++) {
      const x = (w / len) * i;
      const y = h - (data[i] / 255) * h * 0.92;
      if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
    }
    ctx.stroke();
    ctx.shadowBlur = 0;

    // 峰值高亮线
    ctx.strokeStyle = 'rgba(255,255,255,0.35)';
    ctx.lineWidth = 0.5;
    ctx.beginPath();
    for (let i = 0; i < len; i++) {
      const x = (w / len) * i;
      const y = h - (data[i] / 255) * h * 0.92;
      if (i === 0) ctx.moveTo(x, y - 1); else ctx.lineTo(x, y - 1);
    }
    ctx.stroke();
  }
</script>

<canvas bind:this={canvas} {width} {height}></canvas>
<style>canvas { width: 100%; height: 100%; display: block; }</style>
