use crate::packet::CommandPacket;
use crate::types::*;

/// 电台控制命令枚举
///
/// 每个变体映射到协议中的一条命令，按协议章节分组。
/// `encode()` 方法将命令序列化为 CommandPacket。
#[derive(Debug, Clone, PartialEq)]
pub enum RadioCommand {
    // === 3.1 PTT 命令 ===
    Ptt { pressed: bool },

    // === 3.2 频率设置命令 ===
    /// freq: 最大 2000000000 (2GHz), 4字节大端
    SetFrequency { freq: u32 },

    // === 3.3 模式设置命令 ===
    SetMode { mode: Mode },

    // === 3.4 频谱数据命令 ===
    SpectrumRequest,               // 0x39

    // === 3.5 状态同步命令 ===
    StatusRequest,

    // === 3.6 关机命令 ===
    PowerOff,
    PowerOn,

    // === 4 AF 菜单 ===
    SetSpeakerVol { vol: u8 },      // 0x0D, 0~30
    SetHeadphoneVol { vol: u8 },    // 0x0E, 0~80
    SetMicGain { gain: u8 },        // 0x0F, 0~100
    SetCompandor { ratio: u8 },     // 0x10, 0~14
    SetBassEq { value: u8 },        // 0x11, 0~40
    SetTrebleEq { value: u8 },      // 0x12, 0~40

    // === 5 RF 菜单 ===
    SetRfg { gain: u8 },            // 0x13, 0~100
    SetIfg { gain: u8 },            // 0x14, 0~80
    SetSql { level: u8 },           // 0x15, 0~20
    SetAgc { level: u8 },           // 0x16, 0~5
    SetAmp { mode: AmpMode },       // 0x17
    SetFilter { index: u8 },        // 0x18, 1~86
    SetNr { on: bool },             // 0x19
    SetNb { on: bool },             // 0x1A
    SetAb { mode: AbMode },         // 0x1B
    SetSplit { on: bool },          // 0x1C
    SetBand { band: Band },         // 0x1D
    SetNrThreshold { value: u8 },   // 0x1E, 1~200
    SetNbThreshold { value: u8 },   // 0x1F, 0~15
    SetPeakThreshold { value: u8 }, // 0x20, 0~20
    SetTuner { mode: TunerMode },   // 0x21
    SetSpectrumSpan { span: SpectrumSpan }, // 0x22
    SetSpectrumRef { value: u8 },   // 0x23, 1~20
    SetSpectrumSpeed { value: u8 }, // 0x24, 1~30
    SetDisplayMode { mode: DisplayMode },   // 0x25
    SetCtcss { tx_ctcss: u8, rx_ctcss: u8, lead_tone: u8 }, // 0x26
    DeviceTypeRequest,              // 0x27
    SetPowerLevel { level: u8 },    // 0x28, 0~100
    SetRit { value: u8 },           // 0x29, 0~120
    SetXit { value: u8 },           // 0x2A, 0~120
    SetLeadToneTime { ms: u8 },     // 0x2B, 50~300
    SetHighLowPower { level: PowerLevel }, // 0x2C
    MeterRequest,                   // 0x2D
    ParamsRequest,                  // 0x2E

    // === 6 CW 命令 ===
    SetKeyType { key_type: KeyType },   // 0x2F
    SetSidetoneVol { vol: u8 },         // 0x30, 0~15
    SetSidetoneFreq { freq: u8 },       // 0x31, 40~200 (×10)
    SetTxRxDelay { delay: u8 },         // 0x32, 0~50 (×40)
    SetUsbDataFormat { format: UsbDataFormat }, // 0x33
    SetCwTraining { on: bool },         // 0x34
    SetKeySpeed { speed: u8 },          // 0x35, 5~48
    SetCwDecode { on: bool },           // 0x36
    SetCwDecodeThreshold { value: u8 }, // 0x37, 1~50

    // === 7 MESH 数传 ===
    /// data 为 MESH 数据包结构 (不包含外层0xA5帧头)
    SendMeshData { data: Vec<u8> },     // 0x38

    // === 8 信道操作 ===
    WriteChannel {
        channel: u16,
        vfoa_mode: Mode,
        vfob_mode: Mode,
        vfoa_freq: u32,
        vfob_freq: u32,
        tx_ctcss: u8,
        rx_ctcss: u8,
        name: [u8; 12],
    },                                  // 0x40
    ReadChannel { channel: u16 },       // 0x41
    SetChannelMode { mode: ChannelMode }, // 0x42
    // DMR信道设置 0x43 (略)
    // DMR信道读取 0x44 (略)
    SetIqBandwidth { bw: u8 },          // 0x45, 0~6

    // === 9 功放控制 ===
    PaParamsRequest,                    // 0x48

    // === 10 电台设置 ===
    SetFreqHopping {
        on: bool,
        hop_count: u16,
        encrypt: bool,
        key: u8,
    },                                  // 0x49
    SetSwrScan {
        on: bool,
        start_freq: u32,
        end_freq: u32,
        step_freq: u32,
    },                                  // 0x50

    SetRadioSettings {
        settings_byte: u8,
        key_vol: u8,
        key_backlight: u8,
        tx_led_brightness: u8,
        fan_temp: u8,
        swr_protect: u8,
        tuner_threshold: u8,
        tx_timeout: u8,
        screen_off_time: u8,
        vox_threshold: u8,
        hw_version: u8,
        fw_main: u8,
        fw_sub: u8,
    },                                  // 0x51

    SetTime {
        year: u8,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    },                                  // 0x52

    ProductTypeRequest,                 // 0x53

    SetIturRegion { write: bool, region: IturRegion }, // 0x54

    // === 11 工程设置 ===
    SetIqCalibration { write: bool, value: u8 },     // 0x55
    SetPoBiasVoltage { write: bool, value: u8 },     // 0x56
    SetP2BiasVoltage { write: bool, value: u8 },     // 0x56 (同命令字)
    SetPdPowerCalibration { write: bool, value: u8 }, // 0x57
    SetSwrCalibration { write: bool, value: u8 },    // 0x58
    SetTxFreqCalibration { write: bool, value: u8 }, // 0x59
    SetRxFreqCalibration { write: bool, value: u8 }, // 0x60
    SetSMeterCalibration { write: bool, value: u8 }, // 0x61
    SetVoltageCalibration { write: bool, value: u8 }, // 0x62
    SetAttenuatorCalibration { write: bool, value: u8 }, // 0x63
    SetEngineeringMode { on: bool },                 // 0x64
    ReadDeviceId,                                    // 0x66
    SetProductType { model: u8 },                    // 0x67

    // === 12 固件升级 ===
    FwInit {
        version: u32,
        crc32: u32,
        size: u32,
        packet_size: u16,
    },                                               // 0x68
    FwData { packet_no: u16, data: Vec<u8> },        // 0x69
    FwGetBitmap,                                      // 0x6A
    FwExecute,                                        // 0x6B
    FwReadVersion,                                    // 0x6C
}

impl RadioCommand {
    /// 命令字节
    pub fn cmd_byte(&self) -> u8 {
        match self {
            RadioCommand::Ptt { .. } => 0x07,
            RadioCommand::SetFrequency { .. } => 0x09,
            RadioCommand::SetMode { .. } => 0x0A,
            RadioCommand::StatusRequest => 0x0B,
            RadioCommand::PowerOff | RadioCommand::PowerOn => 0x0C,
            RadioCommand::SetSpeakerVol { .. } => 0x0D,
            RadioCommand::SetHeadphoneVol { .. } => 0x0E,
            RadioCommand::SetMicGain { .. } => 0x0F,
            RadioCommand::SetCompandor { .. } => 0x10,
            RadioCommand::SetBassEq { .. } => 0x11,
            RadioCommand::SetTrebleEq { .. } => 0x12,
            RadioCommand::SetRfg { .. } => 0x13,
            RadioCommand::SetIfg { .. } => 0x14,
            RadioCommand::SetSql { .. } => 0x15,
            RadioCommand::SetAgc { .. } => 0x16,
            RadioCommand::SetAmp { .. } => 0x17,
            RadioCommand::SetFilter { .. } => 0x18,
            RadioCommand::SetNr { .. } => 0x19,
            RadioCommand::SetNb { .. } => 0x1A,
            RadioCommand::SetAb { .. } => 0x1B,
            RadioCommand::SetSplit { .. } => 0x1C,
            RadioCommand::SetBand { .. } => 0x1D,
            RadioCommand::SetNrThreshold { .. } => 0x1E,
            RadioCommand::SetNbThreshold { .. } => 0x1F,
            RadioCommand::SetPeakThreshold { .. } => 0x20,
            RadioCommand::SetTuner { .. } => 0x21,
            RadioCommand::SetSpectrumSpan { .. } => 0x22,
            RadioCommand::SetSpectrumRef { .. } => 0x23,
            RadioCommand::SetSpectrumSpeed { .. } => 0x24,
            RadioCommand::SpectrumRequest => 0x39,
            RadioCommand::SetDisplayMode { .. } => 0x25,
            RadioCommand::SetCtcss { .. } => 0x26,
            RadioCommand::DeviceTypeRequest => 0x27,
            RadioCommand::SetPowerLevel { .. } => 0x28,
            RadioCommand::SetRit { .. } => 0x29,
            RadioCommand::SetXit { .. } => 0x2A,
            RadioCommand::SetLeadToneTime { .. } => 0x2B,
            RadioCommand::SetHighLowPower { .. } => 0x2C,
            RadioCommand::MeterRequest => 0x2D,
            RadioCommand::ParamsRequest => 0x2E,
            RadioCommand::SetKeyType { .. } => 0x2F,
            RadioCommand::SetSidetoneVol { .. } => 0x30,
            RadioCommand::SetSidetoneFreq { .. } => 0x31,
            RadioCommand::SetTxRxDelay { .. } => 0x32,
            RadioCommand::SetUsbDataFormat { .. } => 0x33,
            RadioCommand::SetCwTraining { .. } => 0x34,
            RadioCommand::SetKeySpeed { .. } => 0x35,
            RadioCommand::SetCwDecode { .. } => 0x36,
            RadioCommand::SetCwDecodeThreshold { .. } => 0x37,
            RadioCommand::SendMeshData { .. } => 0x38,
            RadioCommand::WriteChannel { .. } => 0x40,
            RadioCommand::ReadChannel { .. } => 0x41,
            RadioCommand::SetChannelMode { .. } => 0x42,
            RadioCommand::SetIqBandwidth { .. } => 0x45,
            RadioCommand::PaParamsRequest => 0x48,
            RadioCommand::SetFreqHopping { .. } => 0x49,
            RadioCommand::SetSwrScan { .. } => 0x50,
            RadioCommand::SetRadioSettings { .. } => 0x51,
            RadioCommand::SetTime { .. } => 0x52,
            RadioCommand::ProductTypeRequest => 0x53,
            RadioCommand::SetIturRegion { .. } => 0x54,
            RadioCommand::SetIqCalibration { .. } => 0x55,
            RadioCommand::SetPoBiasVoltage { .. } => 0x56,
            RadioCommand::SetP2BiasVoltage { .. } => 0x56,
            RadioCommand::SetPdPowerCalibration { .. } => 0x57,
            RadioCommand::SetSwrCalibration { .. } => 0x58,
            RadioCommand::SetTxFreqCalibration { .. } => 0x59,
            RadioCommand::SetRxFreqCalibration { .. } => 0x60,
            RadioCommand::SetSMeterCalibration { .. } => 0x61,
            RadioCommand::SetVoltageCalibration { .. } => 0x62,
            RadioCommand::SetAttenuatorCalibration { .. } => 0x63,
            RadioCommand::SetEngineeringMode { .. } => 0x64,
            RadioCommand::ReadDeviceId => 0x66,
            RadioCommand::SetProductType { .. } => 0x67,
            RadioCommand::FwInit { .. } => 0x68,
            RadioCommand::FwData { .. } => 0x69,
            RadioCommand::FwGetBitmap => 0x6A,
            RadioCommand::FwExecute => 0x6B,
            RadioCommand::FwReadVersion => 0x6C,
        }
    }

    /// 将命令编码为可发送的 CommandPacket
    pub fn encode(&self) -> CommandPacket {
        let cmd = self.cmd_byte();
        let mut data = Vec::new();

        match self {
            RadioCommand::Ptt { pressed } => {
                data.push(if *pressed { 0x00 } else { 0x01 });
            }
            RadioCommand::SetFrequency { freq } => {
                data.extend((*freq).to_be_bytes());
            }
            RadioCommand::SetMode { mode } => {
                data.push(*mode as u8);
            }
            RadioCommand::StatusRequest => {}
            RadioCommand::PowerOff => {
                data.push(0x00);
            }
            RadioCommand::PowerOn => {
                data.push(0x01);
            }
            // AF 菜单
            RadioCommand::SetSpeakerVol { vol } => data.push(*vol),
            RadioCommand::SetHeadphoneVol { vol } => data.push(*vol),
            RadioCommand::SetMicGain { gain } => data.push(*gain),
            RadioCommand::SetCompandor { ratio } => data.push(*ratio),
            RadioCommand::SetBassEq { value } => data.push(*value),
            RadioCommand::SetTrebleEq { value } => data.push(*value),
            // RF 菜单
            RadioCommand::SetRfg { gain } => data.push(*gain),
            RadioCommand::SetIfg { gain } => data.push(*gain),
            RadioCommand::SetSql { level } => data.push(*level),
            RadioCommand::SetAgc { level } => data.push(*level),
            RadioCommand::SetAmp { mode } => data.push(*mode as u8),
            RadioCommand::SetFilter { index } => data.push(*index),
            RadioCommand::SetNr { on } => data.push(if *on { 1 } else { 0 }),
            RadioCommand::SetNb { on } => data.push(if *on { 1 } else { 0 }),
            RadioCommand::SetAb { mode } => data.push(*mode as u8),
            RadioCommand::SetSplit { on } => data.push(if *on { 1 } else { 0 }),
            RadioCommand::SetBand { band } => data.push(band.to_u8()),
            RadioCommand::SetNrThreshold { value } => data.push(*value),
            RadioCommand::SetNbThreshold { value } => data.push(*value),
            RadioCommand::SetPeakThreshold { value } => data.push(*value),
            RadioCommand::SetTuner { mode } => data.push(*mode as u8),
            RadioCommand::SetSpectrumSpan { span } => data.push(*span as u8),
            RadioCommand::SetSpectrumRef { value } => data.push(*value),
            RadioCommand::SetSpectrumSpeed { value } => data.push(*value),
            RadioCommand::SpectrumRequest => {}
            RadioCommand::SetDisplayMode { mode } => data.push(*mode as u8),
            RadioCommand::SetCtcss { tx_ctcss, rx_ctcss, lead_tone } => {
                data.push(*tx_ctcss);
                data.push(*rx_ctcss);
                data.push(*lead_tone);
            }
            RadioCommand::DeviceTypeRequest => {}
            RadioCommand::SetPowerLevel { level } => data.push(*level),
            RadioCommand::SetRit { value } => data.push(*value),
            RadioCommand::SetXit { value } => data.push(*value),
            RadioCommand::SetLeadToneTime { ms } => data.push(*ms),
            RadioCommand::SetHighLowPower { level } => data.push(*level as u8),
            RadioCommand::MeterRequest => {}
            RadioCommand::ParamsRequest => {}
            // CW
            RadioCommand::SetKeyType { key_type } => data.push(*key_type as u8),
            RadioCommand::SetSidetoneVol { vol } => data.push(*vol),
            RadioCommand::SetSidetoneFreq { freq } => data.push(*freq),
            RadioCommand::SetTxRxDelay { delay } => data.push(*delay),
            RadioCommand::SetUsbDataFormat { format } => data.push(*format as u8),
            RadioCommand::SetCwTraining { on } => data.push(if *on { 1 } else { 0 }),
            RadioCommand::SetKeySpeed { speed } => data.push(*speed),
            RadioCommand::SetCwDecode { on } => data.push(if *on { 1 } else { 0 }),
            RadioCommand::SetCwDecodeThreshold { value } => data.push(*value),
            // MESH
            RadioCommand::SendMeshData { data: mesh_data } => data.extend(mesh_data),
            // 信道
            RadioCommand::WriteChannel {
                channel,
                vfoa_mode,
                vfob_mode,
                vfoa_freq,
                vfob_freq,
                tx_ctcss,
                rx_ctcss,
                name,
            } => {
                let ch = *channel as u16;
                data.push((ch >> 8) as u8);
                data.push((ch & 0xFF) as u8);
                data.push(*vfoa_mode as u8);
                data.push(*vfob_mode as u8);
                data.extend((*vfoa_freq).to_be_bytes());
                data.extend((*vfob_freq).to_be_bytes());
                data.push(*tx_ctcss);
                data.push(*rx_ctcss);
                data.extend(name);
            }
            RadioCommand::ReadChannel { channel } => {
                let ch = *channel as u16;
                data.push((ch >> 8) as u8);
                data.push((ch & 0xFF) as u8);
            }
            RadioCommand::SetChannelMode { mode } => data.push(*mode as u8),
            RadioCommand::SetIqBandwidth { bw } => data.push(*bw),
            RadioCommand::PaParamsRequest => {}
            // 电台设置
            RadioCommand::SetFreqHopping { on, hop_count, encrypt, key } => {
                data.push(if *on { 1 } else { 0 });
                let hc = *hop_count;
                data.push((hc >> 8) as u8);
                data.push((hc & 0xFF) as u8);
                data.push(if *encrypt { 1 } else { 0 });
                data.push(*key);
            }
            RadioCommand::SetSwrScan { on, start_freq, end_freq, step_freq } => {
                data.push(if *on { 1 } else { 0 });
                data.extend((*start_freq).to_be_bytes());
                data.extend((*end_freq).to_be_bytes());
                data.extend((*step_freq).to_be_bytes());
            }
            RadioCommand::SetRadioSettings {
                settings_byte,
                key_vol,
                key_backlight,
                tx_led_brightness,
                fan_temp,
                swr_protect,
                tuner_threshold,
                tx_timeout,
                screen_off_time,
                vox_threshold,
                hw_version,
                fw_main,
                fw_sub,
            } => {
                data.push(*settings_byte);
                data.push(*key_vol);
                data.push(*key_backlight);
                data.push(*tx_led_brightness);
                data.push(*fan_temp);
                data.push(*swr_protect);
                data.push(*tuner_threshold);
                data.push(*tx_timeout);
                data.push(*screen_off_time);
                data.push(*vox_threshold);
                data.push(*hw_version);
                data.push(*fw_main);
                data.push(*fw_sub);
            }
            RadioCommand::SetTime { year, month, day, hour, minute, second } => {
                data.push(*year);
                data.push(*month);
                data.push(*day);
                data.push(*hour);
                data.push(*minute);
                data.push(*second);
            }
            RadioCommand::ProductTypeRequest => {}
            RadioCommand::SetIturRegion { write, region } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*region as u8);
            }
            // 工程设置
            RadioCommand::SetIqCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetPoBiasVoltage { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetP2BiasVoltage { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetPdPowerCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetSwrCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetTxFreqCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetRxFreqCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetSMeterCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetVoltageCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetAttenuatorCalibration { write, value } => {
                data.push(if *write { 1 } else { 0 });
                data.push(*value);
            }
            RadioCommand::SetEngineeringMode { on } => {
                data.push(if *on { 1 } else { 0 });
            }
            RadioCommand::ReadDeviceId => {}
            RadioCommand::SetProductType { model } => data.push(*model),
            // 固件升级
            RadioCommand::FwInit { version, crc32, size, packet_size } => {
                data.extend((*version).to_be_bytes());
                data.extend((*crc32).to_be_bytes());
                data.extend((*size).to_be_bytes());
                let ps = *packet_size;
                data.push((ps >> 8) as u8);
                data.push((ps & 0xFF) as u8);
            }
            RadioCommand::FwData { packet_no, data: fw_data } => {
                let pn = *packet_no;
                data.push((pn >> 8) as u8);
                data.push((pn & 0xFF) as u8);
                data.extend(fw_data);
            }
            RadioCommand::FwGetBitmap => {}
            RadioCommand::FwExecute => {}
            RadioCommand::FwReadVersion => {}
        }

        CommandPacket::new(cmd, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ptt_press() {
        let cmd = RadioCommand::Ptt { pressed: true };
        let pkt = cmd.encode();
        assert_eq!(pkt.cmd, 0x07);
        assert_eq!(pkt.data, vec![0x00]);
    }

    #[test]
    fn encode_status_request() {
        let cmd = RadioCommand::StatusRequest;
        let pkt = cmd.encode();
        assert_eq!(pkt.cmd, 0x0B);
        assert!(pkt.data.is_empty());
    }

    #[test]
    fn encode_set_frequency() {
        let cmd = RadioCommand::SetFrequency { freq: 14074000 };
        let pkt = cmd.encode();
        assert_eq!(pkt.cmd, 0x09);
        assert_eq!(pkt.data, vec![0x00, 0xD6, 0xC0, 0x90]);
    }

    #[test]
    fn roundtrip_all_simple_commands() {
        let commands: Vec<RadioCommand> = vec![
            RadioCommand::Ptt { pressed: false },
            RadioCommand::StatusRequest,
            RadioCommand::SetMode { mode: Mode::Usb },
            RadioCommand::SetSpeakerVol { vol: 20 },
            RadioCommand::SetRfg { gain: 50 },
            RadioCommand::SetNr { on: true },
            RadioCommand::SetTuner { mode: TunerMode::Tune },
            RadioCommand::MeterRequest,
            RadioCommand::SetKeySpeed { speed: 20 },
            RadioCommand::SetChannelMode { mode: ChannelMode::Vfo },
            RadioCommand::ReadDeviceId,
        ];

        for cmd in &commands {
            let pkt = cmd.encode();
            let decoded = CommandPacket::decode(&pkt.encode()).unwrap();
            assert_eq!(decoded.cmd, cmd.cmd_byte());
            assert_eq!(decoded.data, pkt.data);
        }
    }
}
