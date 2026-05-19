use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::Extension;
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::broadcast;
use tracing::{debug, error, warn};

use gh_protocol::RadioCommand;

use crate::state::{AppState, StateEvent};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    debug!("WebSocket connected");
    let mut event_rx = state.subscribe();

    // 发送当前状态
    {
        let status = state.status.read().await;
        if let Ok(json) = serde_json::to_string(&serde_json::json!({
            "t": "status",
            "d": {
                "tx": status.is_transmitting,
                "fA": status.vfo_a_freq,
                "fB": status.vfo_b_freq,
                "mA": status.vfo_a_mode,
                "mB": status.vfo_b_mode,
                "v": status.active_vfo,
                "nr": status.nr_nb,
                "rit": status.rit,
                "xit": status.xit,
                "filt": status.filter_bw,
                "span": status.spectrum_span,
                "volt": status.voltage,
                "utc": [status.utc_hour, status.utc_min, status.utc_sec],
                "sb": status.status_bits,
                "sm": status.s_po_value,
                "swr": status.swr_aud_alc,
            }
        })) {
            let _ = socket.send(Message::Text(json.into())).await;
        }
    }

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // 广播事件 → 浏览器
    let _send_state = state.clone();
    let mut send_task = tokio::spawn(async move {
        loop {
            match event_rx.recv().await {
                Ok(event) => {
                    let msg = match event {
                        StateEvent::Status(s) => serialize_status(&s),
                        StateEvent::Params(p) => serialize_params(&p),
                        StateEvent::Spectrum(s) => Message::Binary(s.data.into()),
                        StateEvent::Meter(m) => serialize_meter(&m),
                        StateEvent::Error(e) => {
                            if let Ok(json) = serde_json::to_string(&serde_json::json!({
                                "t": "error",
                                "d": {"msg": e}
                            })) {
                                Message::Text(json.into())
                            } else {
                                continue;
                            }
                        }
                    };
                    if ws_sender.send(msg).await.is_err() {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warn!("WS lagged by {n} messages");
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    // 浏览器命令 → 串口
    let recv_state = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Some(cmd_data) = parse_ws_command(&text) {
                        recv_state.send_command(cmd_data).await;
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    error!("WS receive error: {e}");
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => {}
        _ = &mut recv_task => {}
    }

    send_task.abort();
    recv_task.abort();
    debug!("WebSocket disconnected");
}

fn parse_ws_command(text: &str) -> Option<Vec<u8>> {
    let v: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            warn!("Invalid JSON from WS: {e}");
            return None;
        }
    };

    if v.get("t")?.as_str()? != "cmd" {
        return None;
    }

    let cmd_str = v.get("c")?.as_str()?;
    let d = v.get("d");

    let cmd = match cmd_str {
        "ptt" => {
            let pressed = d?.get("pressed")?.as_bool()?;
            RadioCommand::Ptt { pressed }
        }
        "set_frequency" => {
            let freq = d?.get("freq")?.as_u64()? as u32;
            RadioCommand::SetFrequency { freq }
        }
        "set_mode" => {
            let mode = d?.get("mode")?.as_u64()? as u8;
            RadioCommand::SetMode {
                mode: gh_protocol::types::Mode::from_u8(mode)?,
            }
        }
        "set_speaker_vol" => {
            let vol = d?.get("vol")?.as_u64()? as u8;
            RadioCommand::SetSpeakerVol { vol }
        }
        "set_headphone_vol" => {
            let vol = d?.get("vol")?.as_u64()? as u8;
            RadioCommand::SetHeadphoneVol { vol }
        }
        "set_mic_gain" => {
            let gain = d?.get("gain")?.as_u64()? as u8;
            RadioCommand::SetMicGain { gain }
        }
        "set_rfg" => {
            let gain = d?.get("gain")?.as_u64()? as u8;
            RadioCommand::SetRfg { gain }
        }
        "set_ifg" => {
            let gain = d?.get("gain")?.as_u64()? as u8;
            RadioCommand::SetIfg { gain }
        }
        "set_sql" => {
            let level = d?.get("level")?.as_u64()? as u8;
            RadioCommand::SetSql { level }
        }
        "set_agc" => {
            let level = d?.get("level")?.as_u64()? as u8;
            RadioCommand::SetAgc { level }
        }
        "set_nr" => {
            let on = d?.get("on")?.as_bool()?;
            RadioCommand::SetNr { on }
        }
        "set_nb" => {
            let on = d?.get("on")?.as_bool()?;
            RadioCommand::SetNb { on }
        }
        "set_tuner" => {
            let mode = d?.get("mode")?.as_u64()? as u8;
            let mode = match mode {
                0 => gh_protocol::types::TunerMode::Off,
                1 => gh_protocol::types::TunerMode::On,
                2 => gh_protocol::types::TunerMode::Tune,
                _ => return None,
            };
            RadioCommand::SetTuner { mode }
        }
        "status_request" => RadioCommand::StatusRequest,
        "params_request" => RadioCommand::ParamsRequest,
        "meter_request" => RadioCommand::MeterRequest,
        _ => {
            debug!("Unknown WS command: {cmd_str}");
            return None;
        }
    };

    Some(cmd.encode().encode())
}

fn serialize_status(s: &crate::state::RadioStatus) -> Message {
    let json = serde_json::json!({
        "t": "status",
        "d": {
            "tx": s.is_transmitting,
            "fA": s.vfo_a_freq,
            "fB": s.vfo_b_freq,
            "mA": s.vfo_a_mode,
            "mB": s.vfo_b_mode,
            "v": s.active_vfo,
            "nr": s.nr_nb,
            "rit": s.rit,
            "xit": s.xit,
            "filt": s.filter_bw,
            "span": s.spectrum_span,
            "volt": s.voltage,
            "utc": [s.utc_hour, s.utc_min, s.utc_sec],
            "sb": s.status_bits,
            "sm": s.s_po_value,
            "swr": s.swr_aud_alc,
        }
    });
    Message::Text(serde_json::to_string(&json).unwrap_or_default().into())
}

fn serialize_params(p: &crate::state::RadioParams) -> Message {
    let json = serde_json::json!({
        "t": "params",
        "d": {
            "sv": p.speaker_vol,
            "hv": p.headphone_vol,
            "mg": p.mic_gain,
            "cmp": p.compandor,
            "bass": p.bass_eq,
            "treb": p.treble_eq,
            "rfg": p.rfg,
            "ifg": p.ifg,
            "sql": p.sql,
            "agc": p.agc,
            "amp": p.amp,
            "nr": p.nr,
            "nb": p.nb,
            "pk": p.peak,
            "ref": p.spectrum_ref,
            "spd": p.spectrum_speed,
        }
    });
    Message::Text(serde_json::to_string(&json).unwrap_or_default().into())
}

fn serialize_meter(m: &crate::state::MeterData) -> Message {
    let json = serde_json::json!({
        "t": "meter",
        "d": {
            "sp": m.s_po_value,
            "swr": m.swr_aud_alc,
        }
    });
    Message::Text(serde_json::to_string(&json).unwrap_or_default().into())
}
