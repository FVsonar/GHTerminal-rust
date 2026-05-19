use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{broadcast, RwLock};


/// 状态广播事件
#[derive(Debug, Clone)]
pub enum StateEvent {
    Status(RadioStatus),
    Params(RadioParams),
    Spectrum(SpectrumData),
    Meter(MeterData),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct SpectrumData {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct RadioStatus {
    pub is_transmitting: bool,
    pub vfo_a_mode: u8,
    pub vfo_b_mode: u8,
    pub vfo_a_freq: u32,
    pub vfo_b_freq: u32,
    pub active_vfo: u8, // 0=A, 1=B
    pub nr_nb: u8,      // 0=off, 1=NR, 2=NB
    pub rit: u8,
    pub xit: u8,
    pub filter_bw: u8,
    pub spectrum_span: u8,
    pub voltage: f32,
    pub utc_hour: u8,
    pub utc_min: u8,
    pub utc_sec: u8,
    pub status_bits: u8,
    pub s_po_value: u8,   // BIT7=0: S表, BIT7=1: PO表
    pub swr_aud_alc: u8,  // BIT7,6: 00=SWR, 01=ALC, 10=ADU
}

#[derive(Debug, Clone, Default)]
pub struct RadioParams {
    pub speaker_vol: u8,
    pub headphone_vol: u8,
    pub mic_gain: u8,
    pub compandor: u8,
    pub bass_eq: u8,
    pub treble_eq: u8,
    pub rfg: u8,
    pub ifg: u8,
    pub sql: u8,
    pub agc: u8,
    pub amp: u8,
    pub nr: u8,
    pub nb: u8,
    pub peak: u8,
    pub spectrum_ref: u8,
    pub spectrum_speed: u8,
}

#[derive(Debug, Clone, Default)]
pub struct MeterData {
    pub s_po_value: u8,
    pub swr_aud_alc: u8,
}

pub struct AppState {
    pub connected: AtomicBool,
    pub status: RwLock<RadioStatus>,
    pub params: RwLock<RadioParams>,
    pub spectrum: RwLock<SpectrumBuffer>,
    pub meter: RwLock<MeterData>,
    pub event_tx: broadcast::Sender<StateEvent>,
    pub serial_cmd_tx: RwLock<Option<tokio::sync::mpsc::Sender<Vec<u8>>>>,
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
        let (event_tx, _) = broadcast::channel(256);
        Self {
            connected: AtomicBool::new(false),
            status: RwLock::new(RadioStatus::default()),
            params: RwLock::new(RadioParams::default()),
            spectrum: RwLock::new(SpectrumBuffer::default()),
            meter: RwLock::new(MeterData::default()),
            event_tx,
            serial_cmd_tx: RwLock::new(None),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<StateEvent> {
        self.event_tx.subscribe()
    }

    pub fn send_event(&self, event: StateEvent) {
        let _ = self.event_tx.send(event);
    }

    pub fn set_connected(&self, connected: bool) {
        self.connected.store(connected, Ordering::SeqCst);
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    pub async fn send_command(&self, data: Vec<u8>) {
        if let Some(tx) = self.serial_cmd_tx.read().await.as_ref() {
            let _ = tx.send(data).await;
        }
    }
}
