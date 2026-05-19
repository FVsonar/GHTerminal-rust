<script>
  import { spectrumData } from '../lib/store.js';

  let canvas;
  let ctx;
  let width = 600;
  let height = 110;

  $effect(() => { ctx = canvas?.getContext('2d'); });

  let spec = $derived($spectrumData);

  $effect(() => {
    if (ctx && spec.data.length > 0) drawSpectrum(spec.data);
  });

  function drawSpectrum(data) {
    const w = canvas.width;
    const h = canvas.height;

    ctx.clearRect(0, 0, w, h);

    // 背景
    ctx.fillStyle = '#000510';
    ctx.fillRect(0, 0, w, h);

    // 水平网格线
    for (let i = 1; i < 5; i++) {
      const y = (h / 5) * i;
      ctx.strokeStyle = 'rgba(30,41,59,0.4)';
      ctx.lineWidth = 0.5;
      ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke();
    }

    // 垂直网格线 (10等分)
    for (let i = 1; i < 10; i++) {
      const x = (w / 10) * i;
      ctx.strokeStyle = 'rgba(30,41,59,0.25)';
      ctx.lineWidth = 0.5;
      ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, h); ctx.stroke();
    }

    const len = data.length;
    if (len === 0) return;

    // 频谱填充
    ctx.beginPath();
    for (let i = 0; i < len; i++) {
      const x = (w / len) * i;
      const y = h - (data[i] / 255) * h * 0.9;
      if (i === 0) ctx.moveTo(x, h);
      ctx.lineTo(x, y);
    }
    ctx.lineTo(w, h);
    ctx.closePath();

    // 渐变填充
    const grad = ctx.createLinearGradient(0, 0, 0, h);
    grad.addColorStop(0, 'rgba(0,229,160,0.35)');
    grad.addColorStop(1, 'rgba(0,229,160,0.02)');
    ctx.fillStyle = grad;
    ctx.fill();

    // 频谱线
    ctx.strokeStyle = 'rgba(0,229,160,0.8)';
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    for (let i = 0; i < len; i++) {
      const x = (w / len) * i;
      const y = h - (data[i] / 255) * h * 0.9;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // 顶部高亮
    ctx.strokeStyle = 'rgba(0,255,180,0.4)';
    ctx.lineWidth = 0.5;
    ctx.beginPath();
    for (let i = 0; i < len; i++) {
      const x = (w / len) * i;
      const y = h - (data[i] / 255) * h * 0.9;
      if (i === 0) ctx.moveTo(x, y - 1);
      else ctx.lineTo(x, y - 1);
    }
    ctx.stroke();
  }
</script>

<canvas bind:this={canvas} {width} {height}></canvas>
<style>canvas { width: 100%; height: 100%; display: block; }</style>
