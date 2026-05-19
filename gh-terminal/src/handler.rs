use std::sync::Arc;
use tracing::{debug, trace, warn};

use gh_protocol::Frame;

use crate::state::{AppState, MeterData, RadioParams, RadioStatus, SpectrumData, StateEvent};

pub fn handle_frame(frame: &Frame, state: &Arc<AppState>) {
    match frame {
        Frame::Command(cmd) => match cmd.cmd {
            0x0B => handle_status_response(&cmd.data, state),
            0x27 => handle_device_type(&cmd.data, state),
            0x2D => handle_meter_response(&cmd.data, state),
            0x2E => todo_handle_params_response(&cmd.data, state),
            // 命令回显 (设备对设置命令的确认)
            0x07..=0x0A
            | 0x0D..=0x26
            | 0x28..=0x2C
            | 0x2F..=0x38
            | 0x40..=0x45
            | 0x49..=0x50
            | 0x52
            | 0x54..=0x64
            | 0x67..=0x6C => {
                trace!("command echo: 0x{:02X}", cmd.cmd);
            }
            _ => {
                debug!("unhandled command: 0x{:02X}", cmd.cmd);
            }
        },
        Frame::Spectrum(spec) => {
            let mut spectrum = state.spectrum.blocking_write();
            spectrum.latest = spec.data.clone();
            if spectrum.frames.len() >= 256 {
                spectrum.frames.pop_front();
            }
            spectrum.frames.push_back(spec.data.clone());

            let _ = state.event_tx.send(StateEvent::Spectrum(SpectrumData {
                data: spec.data.clone(),
            }));
        }
    }
}

fn handle_status_response(data: &[u8], state: &Arc<AppState>) {
    if data.len() < 22 {
        warn!("status response too short: {} bytes", data.len());
        return;
    }

    let status = RadioStatus {
        is_transmitting: data[0] == 1,
        vfo_a_mode: data[1],
        vfo_b_mode: data[2],
        vfo_a_freq: u32::from_be_bytes([data[3], data[4], data[5], data[6]]),
        vfo_b_freq: u32::from_be_bytes([data[7], data[8], data[9], data[10]]),
        active_vfo: data[11],
        nr_nb: data[12],
        rit: data[13],
        xit: data[14],
        filter_bw: data[15],
        spectrum_span: data[16],
        voltage: data[17] as f32 / 10.0,
        utc_hour: data[18],
        utc_min: data[19],
        utc_sec: data[20],
        status_bits: data[21],
        s_po_value: data[22],
        swr_aud_alc: data[23],
    };

    {
        let mut s = state.status.blocking_write();
        *s = status.clone();
    }

    let _ = state.event_tx.send(StateEvent::Status(status));
}

fn handle_meter_response(data: &[u8], state: &Arc<AppState>) {
    if data.len() < 2 {
        warn!("meter response too short");
        return;
    }

    let meter = MeterData {
        s_po_value: data[0],
        swr_aud_alc: data[1],
    };

    {
        let mut m = state.meter.blocking_write();
        *m = meter.clone();
    }

    let _ = state.event_tx.send(StateEvent::Meter(meter));
}

fn handle_device_type(data: &[u8], state: &Arc<AppState>) {
    if let Some(&device_type) = data.first() {
        debug!("device type: {}", device_type);
        let _ = state.event_tx.send(StateEvent::Error(format!(
            "device_type: {}",
            device_type
        )));
    }
}

fn todo_handle_params_response(data: &[u8], state: &Arc<AppState>) {
    // 5.28 参数同步回复: 共 30 个字段
    if data.len() < 30 {
        warn!("params response too short: {} bytes", data.len());
        return;
    }

    let params = RadioParams {
        speaker_vol: data[0],
        headphone_vol: data[1],
        mic_gain: data[2],
        compandor: data[3],
        bass_eq: data[4],
        treble_eq: data[5],
        rfg: data[6],
        ifg: data[7],
        sql: data[8],
        agc: data[9],
        amp: data[10],
        nr: data[11],
        nb: data[12],
        peak: data[13],
        spectrum_ref: data[15],
        spectrum_speed: data[16],
    };

    {
        let mut p = state.params.blocking_write();
        *p = params.clone();
    }

    let _ = state.event_tx.send(StateEvent::Params(params));
}
