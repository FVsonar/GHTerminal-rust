use tauri::Emitter;
use tokio::sync::mpsc;
use tokio_serial::SerialPortBuilderExt;
use tracing::{error, info, warn};

use gh_protocol::{Codec, RadioCommand};

use crate::handler;
use crate::state::SharedState;

pub async fn run(handle: tauri::AppHandle, state: SharedState) {
    // 从 Tauri 资源或环境读取端口配置，此处提供默认值
    let port_name = "COM3";
    let baud_rate = 115200;

    info!("Opening serial port {port_name} at {baud_rate} baud");

    let port = match tokio_serial::new(port_name, baud_rate)
        .data_bits(tokio_serial::DataBits::Eight)
        .stop_bits(tokio_serial::StopBits::One)
        .parity(tokio_serial::Parity::None)
        .flow_control(tokio_serial::FlowControl::None)
        .open_native_async()
    {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to open serial port {port_name}: {e}");
            let _ = handle.emit("radio-error", format!("串口打开失败: {e}"));
            return;
        }
    };

    info!("Serial port opened");
    {
        let mut c = state.connected.lock().await;
        *c = true;
    }

    let (reader, writer) = tokio::io::split(port);
    let (cmd_tx, cmd_rx) = mpsc::channel::<Vec<u8>>(32);

    {
        let mut tx = state.cmd_tx.lock().await;
        *tx = Some(cmd_tx.clone());
    }

    // 读任务
    let h1 = handle.clone();
    let s1 = state.clone();
    tokio::spawn(async move {
        if let Err(e) = reader_task(reader, h1, s1).await {
            error!("Reader task error: {e}");
        }
    });

    // 写任务
    tokio::spawn(async move {
        if let Err(e) = writer_task(writer, cmd_rx).await {
            error!("Writer task error: {e}");
        }
    });

    // 轮询任务
    let h3 = handle.clone();
    tokio::spawn(async move {
        poller_task(h3, cmd_tx).await;
    });
}

async fn reader_task(
    reader: tokio::io::ReadHalf<tokio_serial::SerialStream>,
    handle: tauri::AppHandle,
    state: SharedState,
) -> std::io::Result<()> {
    use tokio::io::AsyncReadExt;

    let mut reader = reader;
    let mut codec = Codec::new();
    let mut buf = [0u8; 512];

    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "serial port closed",
            ));
        }

        for frame_result in codec.feed(&buf[..n]) {
            match frame_result {
                Ok(frame) => handler::handle_frame(&frame, &handle, &state),
                Err(e) => {
                    warn!("Protocol decode error: {e}");
                }
            }
        }
    }
}

async fn writer_task(
    writer: tokio::io::WriteHalf<tokio_serial::SerialStream>,
    mut cmd_rx: mpsc::Receiver<Vec<u8>>,
) -> std::io::Result<()> {
    use tokio::io::AsyncWriteExt;

    let mut writer = writer;
    while let Some(data) = cmd_rx.recv().await {
        writer.write_all(&data).await?;
    }
    Ok(())
}

async fn poller_task(_handle: tauri::AppHandle, cmd_tx: mpsc::Sender<Vec<u8>>) {
    let mut tick = tokio::time::interval(tokio::time::Duration::from_millis(250));
    let mut meter_tick = tokio::time::interval(tokio::time::Duration::from_millis(1000));
    let mut params_tick = tokio::time::interval(tokio::time::Duration::from_millis(2000));

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
