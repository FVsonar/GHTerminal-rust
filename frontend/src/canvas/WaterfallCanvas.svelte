<script>
  import { spectrumData } from '../lib/store.js';

  let canvas;
  let ctx;
  let width = 600;
  let height = 45;
  let historyRows = [];

  $effect(() => { ctx = canvas?.getContext('2d'); if (ctx) resetCanvas(); });

  let spec = $derived($spectrumData);

  $effect(() => {
    if (ctx && spec.data.length > 0) addRow(spec.data);
  });

  function resetCanvas() {
    ctx.fillStyle = '#000510';
    ctx.fillRect(0, 0, width, height);
  }

  function addRow(data) {
    const imgRow = ctx.createImageData(width, 1);
    const len = data.length;
    if (len === 0) return;

    for (let x = 0; x < width; x++) {
      const dataIdx = Math.floor((x / width) * len);
      const val = data[dataIdx] / 255;
      const idx = x * 4;
      // 冷色渐变映射: 黑 → 深蓝 → 青蓝 → 青绿
      imgRow.data[idx] = Math.floor(val * val * 80);      // R
      imgRow.data[idx + 1] = Math.floor(val * 120);         // G
      imgRow.data[idx + 2] = Math.floor(20 + val * 160);    // B
      imgRow.data[idx + 3] = 255;
    }

    historyRows.push(imgRow);
    if (historyRows.length > height) historyRows.shift();
    drawWaterfall();
  }

  function drawWaterfall() {
    for (let i = 0; i < historyRows.length; i++) {
      const y = height - historyRows.length + i;
      ctx.putImageData(historyRows[i], 0, y);
    }
  }
</script>

<canvas bind:this={canvas} {width} {height}></canvas>
<style>canvas { width: 100%; height: 100%; display: block; }</style>
