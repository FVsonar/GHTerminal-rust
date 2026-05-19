use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_serial::SerialPortBuilderExt;
use tracing::{error, info, warn};

use gh_protocol::{Codec, RadioCommand};

use crate::config::Cli;
use crate::handler;
use crate::state::AppState;

pub async fn run(state: Arc<AppState>, config: &Cli) {
    info!("Opening serial port {} at {} baud", config.port, config.baud);

    let port = match tokio_serial::new(&config.port, config.baud)
        .data_bits(tokio_serial::DataBits::Eight)
        .stop_bits(tokio_serial::StopBits::One)
        .parity(tokio_serial::Parity::None)
        .flow_control(tokio_serial::FlowControl::None)
        .open_native_async()
    {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to open serial port {}: {}", config.port, e);
            state.send_event(crate::state::StateEvent::Error(format!(
                "串口打开失败: {e}"
            )));
            return;
        }
    };

    info!("Serial port opened successfully");
    state.set_connected(true);

    let (reader, writer) = tokio::io::split(port);

    // 命令发送通道
    let (cmd_tx, cmd_rx) = mpsc::channel::<Vec<u8>>(32);
    {
        let mut tx = state.serial_cmd_tx.write().await;
        *tx = Some(cmd_tx.clone());
    }

    let poll_interval = config.poll_interval_ms;

    // 读任务
    let state_r = state.clone();
    let reader_handle = tokio::spawn(async move {
        if let Err(e) = reader_task(reader, state_r).await {
            error!("Reader task error: {e}");
        }
    });

    // 写任务
    let writer_handle = tokio::spawn(async move {
        if let Err(e) = writer_task(writer, cmd_rx).await {
            error!("Writer task error: {e}");
        }
    });

    // 轮询任务
    let state_p = state.clone();
    let cmd_tx_p = cmd_tx.clone();
    let poller_handle = tokio::spawn(async move {
        poller_task(state_p, cmd_tx_p, poll_interval).await;
    });

    // 等待任一任务退出
    tokio::select! {
        res = reader_handle => {
            if let Err(e) = res { error!("Reader panic: {e}"); }
        }
        res = writer_handle => {
            if let Err(e) = res { error!("Writer panic: {e}"); }
        }
        res = poller_handle => {
            if let Err(e) = res { error!("Poller panic: {e}"); }
        }
    }

    state.set_connected(false);
    let _ = state.event_tx.send(crate::state::StateEvent::Error(
        "串口已断开".to_string(),
    ));
    warn!("Serial port disconnected");
}

async fn reader_task(
    reader: tokio::io::ReadHalf<tokio_serial::SerialStream>,
    state: Arc<AppState>,
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

        let frames = codec.feed(&buf[..n]);
        for frame_result in frames {
            match frame_result {
                Ok(frame) => handler::handle_frame(&frame, &state),
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
        if let Err(e) = writer.write_all(&data).await {
            error!("Write error: {e}");
            return Err(e);
        }
    }
    Ok(())
}

async fn poller_task(
    state: Arc<AppState>,
    cmd_tx: mpsc::Sender<Vec<u8>>,
    interval_ms: u64,
) {
    let mut tick = tokio::time::interval(tokio::time::Duration::from_millis(interval_ms));
    let mut meter_tick = tokio::time::interval(tokio::time::Duration::from_millis(1000));
    let mut params_tick = tokio::time::interval(tokio::time::Duration::from_millis(2000));

    loop {
        tokio::select! {
            _ = tick.tick() => {
                if state.is_connected() {
                    let status_cmd = RadioCommand::StatusRequest.encode().encode();
                    let _ = cmd_tx.send(status_cmd).await;
                }
            }
            _ = meter_tick.tick() => {
                if state.is_connected() {
                    let meter_cmd = RadioCommand::MeterRequest.encode().encode();
                    let _ = cmd_tx.send(meter_cmd).await;
                }
            }
            _ = params_tick.tick() => {
                if state.is_connected() {
                    let params_cmd = RadioCommand::ParamsRequest.encode().encode();
                    let _ = cmd_tx.send(params_cmd).await;
                }
            }
        }
    }
}
