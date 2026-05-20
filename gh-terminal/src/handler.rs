use tauri::Emitter;
use tracing::{debug, info, warn};

use gh_protocol::Frame;
use crate::state::SharedState;

use crate::state::*;

fn resolve_pending_cmd(cbyte: u8, state: &SharedState) {
    if let Ok(mut pending) = state.pending_cmds.lock() {
        let mut kept = Vec::new();
        for (cb, tx) in pending.drain(..) {
            if cb == cbyte {
                let _ = tx.send(true);
            } else {
                kept.push((cb, tx));
            }
        }
        *pending = kept;
    }
}

pub fn handle_frame(frame: &Frame, handle: &tauri::AppHandle, state: &SharedState) {
    match frame {
        Frame::Command(cmd) => {
            resolve_pending_cmd(cmd.cmd, state);
            match cmd.cmd {
            0x0B => handle_status_response(&cmd.data, handle, state),
            0x27 => handle_device_type(&cmd.data, handle),
            0x39 => handle_spectrum_response(&cmd.data, handle, state),
            0x41 => handle_channel_read_response(&cmd.data, handle),
            0x2D => handle_meter_response(&cmd.data, handle, state),
            0x2E => handle_params_response(&cmd.data, handle, state),
            0x07..=0x0A
            | 0x0D..=0x26
            | 0x28..=0x2C
            | 0x2F..=0x38
            | 0x40..=0x45
            | 0x49..=0x50
            | 0x52
            | 0x54..=0x64
            | 0x67..=0x6C => {
                debug!("command echo: 0x{:02X}", cmd.cmd);
            }
            _ => {
                debug!("unhandled command: 0x{:02X}", cmd.cmd);
            }
            }
        },
        Frame::Spectrum(spec) => {
            if let Ok(mut spectrum) = state.spectrum.try_lock() {
                spectrum.latest = spec.data.clone();
                if spectrum.frames.len() >= 256 {
                    spectrum.frames.pop_front();
                }
                spectrum.frames.push_back(spec.data.clone());
            }
            let _ = handle.emit("spectrum-data", &spec.data);
        }
    }
}

fn handle_status_response(data: &[u8], handle: &tauri::AppHandle, state: &SharedState) {
    info!("Status response: {} bytes", data.len());
    if data.len() < 24 {
        warn!("status response too short: {} bytes", data.len());
        return;
    }

    let status = RadioStatus {
        tx: data[0] == 1,
        vfo_a_freq: u32::from_be_bytes([data[3], data[4], data[5], data[6]]),
        vfo_b_freq: u32::from_be_bytes([data[7], data[8], data[9], data[10]]),
        vfo_a_mode: data[1],
        vfo_b_mode: data[2],
        v: data[11],
        nr: data[12],
        rit: data[13],
        xit: data[14],
        filt: data[15],
        span: data[16],
        volt: data[17] as f32 / 10.0,
        utc: [data[18], data[19], data[20]],
        sb: data[21],
        sm: data[22],
        swr: data[23],
    };

    if let Ok(mut s) = state.status.try_lock() {
        *s = status.clone();
    }
    let _ = handle.emit("radio-status", &status);
}

fn handle_meter_response(data: &[u8], handle: &tauri::AppHandle, state: &SharedState) {
    if data.len() < 2 {
        return;
    }

    let meter = MeterData {
        sp: data[0],
        swr: data[1],
    };

    if let Ok(mut m) = state.meter.try_lock() {
        *m = meter.clone();
    }
    let _ = handle.emit("meter-data", &meter);
}

fn handle_params_response(data: &[u8], handle: &tauri::AppHandle, state: &SharedState) {
    if data.len() < 30 {
        return;
    }

    let params = RadioParams {
        sv: data[0],
        hv: data[1],
        mg: data[2],
        cmp: data[3],
        bass: data[4],
        treb: data[5],
        rfg: data[6],
        ifg: data[7],
        sql: data[8],
        agc: data[9],
        amp: data[10],
        nr: data[11],
        nb: data[12],
        pk: data[13],
        spectrum_ref: data[15],
        spd: data[16],
    };

    if let Ok(mut p) = state.params.try_lock() {
        *p = params.clone();
    }
    let _ = handle.emit("radio-params", &params);
}

fn handle_channel_read_response(data: &[u8], handle: &tauri::AppHandle) {
    if data.len() < 14 {
        warn!("channel read response too short: {}", data.len());
        return;
    }
    let channel = ((data[0] as u16) << 8) | (data[1] as u16);
    let vfoa_mode = data[2];
    let vfob_mode = data[3];
    let vfoa_freq = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let vfob_freq = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
    let tx_ctcss = data[12];
    let rx_ctcss = data[13];
    let name_end = (14 + 12).min(data.len());
    let name_bytes = &data[14..name_end];
    let name = String::from_utf8_lossy(name_bytes).trim_end_matches('\0').to_string();

    info!("Channel {} read: A={} {} B={} {} name={}", channel, vfoa_freq, vfoa_mode, vfob_freq, vfob_mode, name);
    let _ = handle.emit("channel-data", serde_json::json!({
        "channel": channel,
        "vfoa_mode": vfoa_mode,
        "vfob_mode": vfob_mode,
        "vfoa_freq": vfoa_freq,
        "vfob_freq": vfob_freq,
        "tx_ctcss": tx_ctcss,
        "rx_ctcss": rx_ctcss,
        "name": name,
    }));
}

fn handle_spectrum_response(data: &[u8], handle: &tauri::AppHandle, state: &SharedState) {
    if let Ok(mut spectrum) = state.spectrum.try_lock() {
        spectrum.latest = data.to_vec();
        if spectrum.frames.len() >= 256 {
            spectrum.frames.pop_front();
        }
        spectrum.frames.push_back(data.to_vec());
    }
    let _ = handle.emit("spectrum-data", data);
}

fn handle_device_type(data: &[u8], handle: &tauri::AppHandle) {
    if let Some(&dt) = data.first() {
        debug!("device type: {dt}");
        let _ = handle.emit("radio-error", format!("device_type: {dt}"));
    }
}
