use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, oneshot};
use tokio::task::JoinHandle;
use serde::Serialize;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SerialConfig {
    pub port: String,
    pub baud_rate: u32,
}

/// 前端事件数据结构
#[derive(Debug, Clone, Serialize)]
pub struct RadioStatus {
    pub tx: bool,
    #[serde(rename = "fA")]
    pub vfo_a_freq: u32,
    #[serde(rename = "fB")]
    pub vfo_b_freq: u32,
    #[serde(rename = "mA")]
    pub vfo_a_mode: u8,
    #[serde(rename = "mB")]
    pub vfo_b_mode: u8,
    pub v: u8,
    pub nr: u8,
    pub rit: u8,
    pub xit: u8,
    pub filt: u8,
    pub span: u8,
    pub volt: f32,
    pub utc: [u8; 3],
    pub sb: u8,
    pub sm: u8,
    pub swr: u8,
}

impl Default for RadioStatus {
    fn default() -> Self {
        Self {
            tx: false,
            vfo_a_freq: 14074000,
            vfo_b_freq: 7100000,
            vfo_a_mode: 0,
            vfo_b_mode: 0,
            v: 0,
            nr: 0,
            rit: 0,
            xit: 0,
            filt: 10,
            span: 2,
            volt: 13.8,
            utc: [8, 0, 0],
            sb: 0,
            sm: 0,
            swr: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RadioParams {
    pub sv: u8,
    pub hv: u8,
    pub mg: u8,
    pub cmp: u8,
    pub bass: u8,
    pub treb: u8,
    pub rfg: u8,
    pub ifg: u8,
    pub sql: u8,
    pub agc: u8,
    pub amp: u8,
    pub nr: u8,
    pub nb: u8,
    pub pk: u8,
    #[serde(rename = "ref")]
    pub spectrum_ref: u8,
    pub spd: u8,
}

impl Default for RadioParams {
    fn default() -> Self {
        Self {
            sv: 20, hv: 40, mg: 50, cmp: 7,
            bass: 20, treb: 20, rfg: 60, ifg: 40,
            sql: 5, agc: 3, amp: 0, nr: 0, nb: 0,
            pk: 10, spectrum_ref: 10, spd: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MeterData {
    pub sp: u8,
    pub swr: u8,
}

impl Default for MeterData {
    fn default() -> Self {
        Self { sp: 0, swr: 0 }
    }
}

/// 轮询开关
#[derive(Debug, Clone, Serialize)]
pub struct PollState {
    pub status: bool,
    pub meter: bool,
    pub params: bool,
    pub spectrum: bool,
    pub cw: bool,
    pub channel: bool,
}

impl Default for PollState {
    fn default() -> Self {
        Self { status: true, meter: true, params: true, spectrum: true, cw: true, channel: true }
    }
}

pub struct AppState {
    pub status: Mutex<RadioStatus>,
    pub params: Mutex<RadioParams>,
    pub meter: Mutex<MeterData>,
    pub spectrum: Mutex<SpectrumBuffer>,
    pub connected: Mutex<bool>,
    pub cmd_tx: Mutex<Option<mpsc::Sender<Vec<u8>>>>,
    pub serial_config: Mutex<Option<SerialConfig>>,
    pub serial_abort: Mutex<Option<JoinHandle<()>>>,
    pub poll_state: Mutex<PollState>,
    pub pending_cmds: std::sync::Mutex<Vec<(u8, oneshot::Sender<bool>)>>,
}

pub struct SpectrumBuffer {
    pub frames: VecDeque<Vec<u8>>,
    pub latest: Vec<u8>,
}

impl Default for SpectrumBuffer {
    fn default() -> Self {
        Self {
            frames: VecDeque::with_capacity(256),
            latest: Vec::new(),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            status: Mutex::new(RadioStatus::default()),
            params: Mutex::new(RadioParams::default()),
            meter: Mutex::new(MeterData::default()),
            spectrum: Mutex::new(SpectrumBuffer::default()),
            connected: Mutex::new(false),
            cmd_tx: Mutex::new(None),
            serial_config: Mutex::new(None),
            serial_abort: Mutex::new(None),
            poll_state: Mutex::new(PollState::default()),
            pending_cmds: std::sync::Mutex::new(Vec::new()),
        }
    }
}

pub type SharedState = Arc<AppState>;
