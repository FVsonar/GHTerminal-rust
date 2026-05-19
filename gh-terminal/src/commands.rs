use std::sync::Arc;
use serde::Serialize;
use serde_json::Value;
use tauri::{Emitter, State};
use tracing::{info, warn};

use gh_protocol::RadioCommand;
use gh_protocol::types::*;

use crate::state::{AppState, RadioStatus, RadioParams, MeterData, SerialConfig};

#[derive(Debug, Serialize)]
pub struct SerialPortInfo {
    pub name: String,
    pub description: String,
}

#[tauri::command]
pub async fn list_serial_ports() -> Result<Vec<SerialPortInfo>, String> {
    match serialport::available_ports() {
        Ok(ports) => Ok(ports
            .into_iter()
            .map(|p| SerialPortInfo {
                name: p.port_name.clone(),
                description: format!("{:?}", p.port_type),
            })
            .collect()),
        Err(e) => Err(format!("无法枚举串口: {e}")),
    }
}

#[tauri::command]
pub async fn connect_serial(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    port: String,
    baud: u32,
) -> Result<(), String> {
    // 先断开已有连接
    if let Some(handle) = state.serial_abort.lock().await.take() {
        handle.abort();
        info!("Disconnected previous serial connection");
    }

    // 用 blocking 模式打开 (兼容性更好, 避免异步IO驱动问题)
    let serial_port = serialport::new(&port, baud)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .parity(serialport::Parity::None)
        .flow_control(serialport::FlowControl::None)
        .timeout(std::time::Duration::from_millis(50))
        .open()
        .map_err(|e| format!("无法打开串口 {}: {}", port, e))?;

    info!("Serial port {} opened successfully", port);

    let config = SerialConfig {
        port: port.clone(),
        baud_rate: baud,
    };

    *state.serial_config.lock().await = Some(config.clone());
    *state.connected.lock().await = true;

    let app_handle = app.clone();
    let shared_state = (*state).clone();

    // 生成后台读写/轮询任务 (传入已打开的端口)
    let abort_handle = tokio::spawn(async move {
        crate::serial_port::run_with_port(app_handle, shared_state, serial_port).await;
    });

    *state.serial_abort.lock().await = Some(abort_handle);

    let _ = app.emit("serial-status", serde_json::json!({"connected": true, "port": &port}));
    Ok(())
}

#[tauri::command]
pub async fn disconnect_serial(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    if let Some(handle) = state.serial_abort.lock().await.take() {
        handle.abort();
        // 等待 spawned tasks 释放串口句柄
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        *state.connected.lock().await = false;
        *state.cmd_tx.lock().await = None;
        info!("Serial disconnected");
    }
    Ok(())
}

fn parse_command(cmd: &str, d: &Value) -> Option<RadioCommand> {
    Some(match cmd {
        "ptt" => RadioCommand::Ptt {
            pressed: d.get("pressed")?.as_bool()?,
        },
        "set_frequency" => RadioCommand::SetFrequency {
            freq: d.get("freq")?.as_u64()? as u32,
        },
        "set_mode" => RadioCommand::SetMode {
            mode: Mode::from_u8(d.get("mode")?.as_u64()? as u8)?,
        },
        "set_speaker_vol" => RadioCommand::SetSpeakerVol {
            vol: d.get("vol")?.as_u64()? as u8,
        },
        "set_headphone_vol" => RadioCommand::SetHeadphoneVol {
            vol: d.get("vol")?.as_u64()? as u8,
        },
        "set_mic_gain" => RadioCommand::SetMicGain {
            gain: d.get("gain")?.as_u64()? as u8,
        },
        "set_compandor" => RadioCommand::SetCompandor {
            ratio: d.get("ratio")?.as_u64()? as u8,
        },
        "set_bass_eq" => RadioCommand::SetBassEq {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_treble_eq" => RadioCommand::SetTrebleEq {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_rfg" => RadioCommand::SetRfg {
            gain: d.get("gain")?.as_u64()? as u8,
        },
        "set_ifg" => RadioCommand::SetIfg {
            gain: d.get("gain")?.as_u64()? as u8,
        },
        "set_sql" => RadioCommand::SetSql {
            level: d.get("level")?.as_u64()? as u8,
        },
        "set_agc" => RadioCommand::SetAgc {
            level: d.get("level")?.as_u64()? as u8,
        },
        "set_amp" => RadioCommand::SetAmp {
            mode: match d.get("mode")?.as_u64()? {
                0 => AmpMode::AmpA,
                1 => AmpMode::AmpB,
                _ => return None,
            },
        },
        "set_nr" => RadioCommand::SetNr {
            on: d.get("on")?.as_bool()?,
        },
        "set_nb" => RadioCommand::SetNb {
            on: d.get("on")?.as_bool()?,
        },
        "set_nr_threshold" => RadioCommand::SetNrThreshold {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_nb_threshold" => RadioCommand::SetNbThreshold {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_peak_threshold" => RadioCommand::SetPeakThreshold {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_tuner" => RadioCommand::SetTuner {
            mode: match d.get("mode")?.as_u64()? {
                0 => TunerMode::Off,
                1 => TunerMode::On,
                2 => TunerMode::Tune,
                _ => return None,
            },
        },
        "set_spectrum_span" => RadioCommand::SetSpectrumSpan {
            span: SpectrumSpan::from_u8(d.get("span")?.as_u64()? as u8)?,
        },
        "set_spectrum_ref" => RadioCommand::SetSpectrumRef {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_spectrum_speed" => RadioCommand::SetSpectrumSpeed {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_display_mode" => RadioCommand::SetDisplayMode {
            mode: match d.get("mode")?.as_u64()? {
                0 => DisplayMode::Both,
                1 => DisplayMode::Spectrum,
                2 => DisplayMode::Waterfall,
                3 => DisplayMode::Off,
                _ => return None,
            },
        },
        "set_key_type" => RadioCommand::SetKeyType {
            key_type: match d.get("key_type")?.as_u64()? {
                0 => KeyType::AutoL,
                1 => KeyType::AutoR,
                2 => KeyType::Key,
                _ => return None,
            },
        },
        "set_sidetone_vol" => RadioCommand::SetSidetoneVol {
            vol: d.get("vol")?.as_u64()? as u8,
        },
        "set_sidetone_freq" => RadioCommand::SetSidetoneFreq {
            freq: d.get("freq")?.as_u64()? as u8,
        },
        "set_txrx_delay" => RadioCommand::SetTxRxDelay {
            delay: d.get("delay")?.as_u64()? as u8,
        },
        "set_key_speed" => RadioCommand::SetKeySpeed {
            speed: d.get("speed")?.as_u64()? as u8,
        },
        "set_cw_training" => RadioCommand::SetCwTraining {
            on: d.get("on")?.as_bool()?,
        },
        "set_cw_decode" => RadioCommand::SetCwDecode {
            on: d.get("on")?.as_bool()?,
        },
        "set_cw_decode_threshold" => RadioCommand::SetCwDecodeThreshold {
            value: d.get("value")?.as_u64()? as u8,
        },
        "set_ab" => RadioCommand::SetAb {
            mode: match d.get("mode")?.as_u64()? {
                0 => AbMode::A,
                1 => AbMode::B,
                2 => AbMode::Equal,
                _ => return None,
            },
        },
        "status_request" => RadioCommand::StatusRequest,
        "params_request" => RadioCommand::ParamsRequest,
        "meter_request" => RadioCommand::MeterRequest,
        _ => {
            warn!("Unknown command: {cmd}");
            return None;
        }
    })
}

#[tauri::command]
pub async fn send_command(cmd: String, data: Value, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let radio_cmd = parse_command(&cmd, &data).ok_or_else(|| format!("Invalid command: {cmd}"))?;
    let bytes = radio_cmd.encode().encode();
    let hex: String = bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ");
    info!("CMD from UI: {cmd} → {hex}");

    let tx_guard = state.cmd_tx.lock().await;
    if let Some(tx) = tx_guard.as_ref() {
        tx.send(bytes).await.map_err(|e| e.to_string())?;
        info!("CMD sent to serial queue");
    } else {
        warn!("No serial connection, dropping command: {cmd}");
    }
    Ok(())
}

#[tauri::command]
pub async fn set_poll_toggle(
    state: State<'_, Arc<AppState>>,
    poll: String,
    on: bool,
) -> Result<(), String> {
    let mut ps = state.poll_state.lock().await;
    match poll.as_str() {
        "status" => ps.status = on,
        "meter" => ps.meter = on,
        "params" => ps.params = on,
        "spectrum" => ps.spectrum = on,
        _ => return Err(format!("Unknown poll: {poll}")),
    }
    info!("Poll {poll}: {}", if on { "ON" } else { "OFF" });
    Ok(())
}

#[tauri::command]
pub async fn get_poll_state(state: State<'_, Arc<AppState>>) -> Result<crate::state::PollState, String> {
    Ok(state.poll_state.lock().await.clone())
}

#[tauri::command]
pub async fn get_status(state: State<'_, Arc<AppState>>) -> Result<RadioStatus, String> {
    Ok(state.status.lock().await.clone())
}

#[tauri::command]
pub async fn get_params(state: State<'_, Arc<AppState>>) -> Result<RadioParams, String> {
    Ok(state.params.lock().await.clone())
}

#[tauri::command]
pub async fn get_meter(state: State<'_, Arc<AppState>>) -> Result<MeterData, String> {
    Ok(state.meter.lock().await.clone())
}
