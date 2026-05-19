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
    ctx.fillStyle = '#020810';
    ctx.fillRect(0, 0, width, height);
  }

  function addRow(data) {
    const imgRow = ctx.createImageData(width, 1);
    const len = data.length;
    if (len === 0) return;

    for (let x = 0; x < width; x++) {
      const dataIdx = Math.floor((x / width) * len);
      const v = data[dataIdx] / 255; // 0..1
      const idx = x * 4;

      // 热力渐变: 黑→深蓝→青→黄→白
      if (v < 0.25) {
        imgRow.data[idx] = 0;
        imgRow.data[idx + 1] = Math.floor(v * 4 * 60);
        imgRow.data[idx + 2] = Math.floor(40 + v * 4 * 200);
      } else if (v < 0.5) {
        const t = (v - 0.25) * 4;
        imgRow.data[idx] = Math.floor(t * 60);
        imgRow.data[idx + 1] = Math.floor(60 + t * 200);
        imgRow.data[idx + 2] = Math.floor(240 - t * 140);
      } else if (v < 0.75) {
        const t = (v - 0.5) * 4;
        imgRow.data[idx] = Math.floor(60 + t * 195);
        imgRow.data[idx + 1] = Math.floor(255 - t * 60);
        imgRow.data[idx + 2] = Math.floor(100 - t * 100);
      } else {
        const t = (v - 0.75) * 4;
        imgRow.data[idx] = Math.floor(255);
        imgRow.data[idx + 1] = Math.floor(200 + t * 55);
        imgRow.data[idx + 2] = Math.min(255, Math.floor(t * 400));
      }
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
