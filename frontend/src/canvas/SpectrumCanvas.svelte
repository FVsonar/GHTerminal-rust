<script>
  import { spectrumData } from '../lib/store.js';

  let canvas;
  let ctx;
  let width = 600;
  let height = 150;

  $effect(() => {
    ctx = canvas?.getContext('2d');
  });

  let spec = $derived($spectrumData);

  $effect(() => {
    // 收到新频谱数据时重绘
    if (ctx && spec.data.length > 0) {
      drawSpectrum(spec.data);
    }
  });

  function drawSpectrum(data) {
    const w = canvas.width;
    const h = canvas.height;

    ctx.clearRect(0, 0, w, h);

    // 背景
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, w, h);

    // 网格
    ctx.strokeStyle = '#111';
    ctx.lineWidth = 1;
    for (let i = 0; i <= 10; i++) {
      const x = (w / 10) * i;
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, h);
      ctx.stroke();
      const y = (h / 5) * i;
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(w, y);
      ctx.stroke();
    }

    // 频谱线
    const len = data.length;
    if (len === 0) return;

    ctx.strokeStyle = '#00cc66';
    ctx.lineWidth = 1;
    ctx.beginPath();

    for (let i = 0; i < len; i++) {
      const x = (w / len) * i;
      const y = h - (data[i] / 255) * h;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // 填充
    ctx.globalAlpha = 0.15;
    ctx.fillStyle = '#00cc66';
    ctx.lineTo(w, h);
    ctx.lineTo(0, h);
    ctx.closePath();
    ctx.fill();
    ctx.globalAlpha = 1;
  }
</script>

<canvas bind:this={canvas} {width} {height}>
</canvas>

<style>
  canvas {
    width: 100%;
    height: 100%;
    display: block;
  }
</style>
