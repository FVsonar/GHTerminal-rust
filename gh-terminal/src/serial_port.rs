use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Emitter;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use gh_protocol::{Codec, RadioCommand};

use crate::handler;
use crate::state::SharedState;

pub async fn run_with_port(
    handle: tauri::AppHandle,
    state: SharedState,
    port: Box<dyn serialport::SerialPort>,
) {
    let shared_port = Arc::new(Mutex::new(port));
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Vec<u8>>(32);

    {
        let mut tx = state.cmd_tx.lock().await;
        *tx = Some(cmd_tx.clone());
    }

    // 读任务
    let read_handle = handle.clone();
    let read_state = state.clone();
    let read_port = shared_port.clone();
    tokio::task::spawn_blocking(move || {
        let mut codec = Codec::new();
        let mut buf = [0u8; 512];
        loop {
            let n = {
                let mut port = read_port.lock().unwrap();
                match port.read(&mut buf) {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        // 超时很正常，继续等待数据
                        continue;
                    }
                    Err(e) => {
                        error!("Serial read error: {e}");
                        let _ = read_handle.emit("serial-status", serde_json::json!({"connected": false, "port": ""}));
                        return;
                    }
                }
            };
            if n == 0 {
                continue;
            }
            // 打印收到的原始字节
            let hex: String = buf[..n].iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ");
            info!("RAW [{n}]: {hex}");
            for frame_result in codec.feed(&buf[..n]) {
                match frame_result {
                    Ok(frame) => {
                        info!("Decoded frame: {frame:?}");
                        handler::handle_frame(&frame, &read_handle, &read_state)
                    }
                    Err(e) => warn!("Protocol decode error: {e}"),
                }
            }
        }
    });

    // 写桥接: async cmd_rx → blocking write
    let write_port = shared_port.clone();
    tokio::spawn(async move {
        while let Some(data) = cmd_rx.recv().await {
            let hex: String = data.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ");
            info!("SERIAL WRITE [{len}]: {hex}", len = data.len());
            let mut port = write_port.lock().unwrap();
            if let Err(e) = port.write_all(&data) {
                error!("Serial write error: {e}");
                break;
            }
        }
    });

    // 轮询任务
    tokio::spawn(async move {
        poller_task(handle, state, cmd_tx).await;
    });
}

async fn poller_task(_handle: tauri::AppHandle, state: SharedState, cmd_tx: mpsc::Sender<Vec<u8>>) {
    let mut tick = tokio::time::interval(Duration::from_millis(250));
    let mut meter_tick = tokio::time::interval(Duration::from_millis(1000));
    let mut params_tick = tokio::time::interval(Duration::from_millis(2000));
    let mut spectrum_tick = tokio::time::interval(Duration::from_millis(80));

    loop {
        tokio::select! {
            _ = tick.tick() => {
                if state.poll_state.lock().await.status {
                    let cmd = RadioCommand::StatusRequest.encode().encode();
                    let _ = cmd_tx.send(cmd).await;
                }
            }
            _ = meter_tick.tick() => {
                if state.poll_state.lock().await.meter {
                    let cmd = RadioCommand::MeterRequest.encode().encode();
                    let _ = cmd_tx.send(cmd).await;
                }
            }
            _ = params_tick.tick() => {
                if state.poll_state.lock().await.params {
                    let cmd = RadioCommand::ParamsRequest.encode().encode();
                    let _ = cmd_tx.send(cmd).await;
                }
            }
            _ = spectrum_tick.tick() => {
                if state.poll_state.lock().await.spectrum {
                    let cmd = RadioCommand::SpectrumRequest.encode().encode();
                    let _ = cmd_tx.send(cmd).await;
                }
            }
        }
    }
}
