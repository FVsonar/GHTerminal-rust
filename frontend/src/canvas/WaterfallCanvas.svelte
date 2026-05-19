<script>
  import { spectrumData } from '../lib/store.js';

  let canvas;
  let ctx;
  let width = 600;
  let height = 80;
  let historyRows = [];

  $effect(() => {
    ctx = canvas?.getContext('2d');
    if (ctx) {
      ctx.fillStyle = '#000';
      ctx.fillRect(0, 0, width, height);
    }
  });

  let spec = $derived($spectrumData);

  $effect(() => {
    if (ctx && spec.data.length > 0) {
      addRow(spec.data);
    }
  });

  function addRow(data) {
    // 创建一行图像数据
    const imgRow = ctx.createImageData(width, 1);
    const len = data.length;
    if (len === 0) return;

    for (let x = 0; x < width; x++) {
      const dataIdx = Math.floor((x / width) * len);
      const val = data[dataIdx];

      // 蓝色映射: 低值=黑, 中值=蓝, 高值=青
      const idx = x * 4;
      imgRow.data[idx] = Math.floor(val * 0.3);     // R
      imgRow.data[idx + 1] = Math.floor(val * 0.5);  // G
      imgRow.data[idx + 2] = Math.floor(val * 1.0);  // B
      imgRow.data[idx + 3] = 255;                     // A
    }

    historyRows.push(imgRow);
    if (historyRows.length > height) {
      historyRows.shift();
    }

    drawWaterfall();
  }

  function drawWaterfall() {
    const h = canvas.height;
    const w = canvas.width;

    // 清空
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, w, h);

    // 从底部向上绘制历史行
    for (let i = 0; i < historyRows.length; i++) {
      const y = h - historyRows.length + i;
      ctx.putImageData(historyRows[i], 0, y);
    }
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
