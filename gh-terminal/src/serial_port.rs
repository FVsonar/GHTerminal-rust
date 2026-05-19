use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Emitter;
use tokio::sync::mpsc;
use tracing::{error, warn};

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
                    Err(e) => {
                        error!("Serial read error: {e}");
                        let _ = read_handle.emit("serial-status", serde_json::json!({"connected": false, "port": ""}));
                        return;
                    }
                }
            };
            if n == 0 {
                let _ = read_handle.emit("serial-status", serde_json::json!({"connected": false, "port": ""}));
                error!("Serial port closed (EOF)");
                return;
            }
            for frame_result in codec.feed(&buf[..n]) {
                match frame_result {
                    Ok(frame) => handler::handle_frame(&frame, &read_handle, &read_state),
                    Err(e) => warn!("Protocol decode error: {e}"),
                }
            }
        }
    });

    // 写桥接: async cmd_rx → blocking write
    let write_port = shared_port.clone();
    tokio::spawn(async move {
        while let Some(data) = cmd_rx.recv().await {
            let mut port = write_port.lock().unwrap();
            if let Err(e) = port.write_all(&data) {
                error!("Serial write error: {e}");
                break;
            }
        }
    });

    // 轮询任务
    tokio::spawn(async move {
        poller_task(handle, cmd_tx).await;
    });
}

async fn poller_task(_handle: tauri::AppHandle, cmd_tx: mpsc::Sender<Vec<u8>>) {
    let mut tick = tokio::time::interval(Duration::from_millis(250));
    let mut meter_tick = tokio::time::interval(Duration::from_millis(1000));
    let mut params_tick = tokio::time::interval(Duration::from_millis(2000));

    loop {
        tokio::select! {
            _ = tick.tick() => {
                let cmd = RadioCommand::StatusRequest.encode().encode();
                let _ = cmd_tx.send(cmd).await;
            }
            _ = meter_tick.tick() => {
                let cmd = RadioCommand::MeterRequest.encode().encode();
                let _ = cmd_tx.send(cmd).await;
            }
            _ = params_tick.tick() => {
                let cmd = RadioCommand::ParamsRequest.encode().encode();
                let _ = cmd_tx.send(cmd).await;
            }
        }
    }
}
