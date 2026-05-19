use serde_json::Value;
use tauri::State;
use tracing::warn;

use gh_protocol::RadioCommand;
use gh_protocol::types::*;

use crate::state::{AppState, RadioStatus, RadioParams, MeterData};

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
pub async fn send_command(cmd: String, data: Value, state: State<'_, AppState>) -> Result<(), String> {
    let radio_cmd = parse_command(&cmd, &data).ok_or_else(|| format!("Invalid command: {cmd}"))?;
    let bytes = radio_cmd.encode().encode();

    let tx_guard = state.cmd_tx.lock().await;
    if let Some(tx) = tx_guard.as_ref() {
        tx.send(bytes).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_status(state: State<'_, AppState>) -> Result<RadioStatus, String> {
    Ok(state.status.lock().await.clone())
}

#[tauri::command]
pub async fn get_params(state: State<'_, AppState>) -> Result<RadioParams, String> {
    Ok(state.params.lock().await.clone())
}

#[tauri::command]
pub async fn get_meter(state: State<'_, AppState>) -> Result<MeterData, String> {
    Ok(state.meter.lock().await.clone())
}
