/// 工作模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    Usb = 0,
    Lsb = 1,
    Cwr = 2,
    Cwl = 3,
    Am = 4,
    Wfm = 5,
    Nfm = 6,
    Digi = 7,
    Pkt = 8,
    Dmr = 9,
    Dfm = 10,
}

impl Mode {
    pub fn from_u8(v: u8) -> Option<Mode> {
        Some(match v {
            0 => Mode::Usb,
            1 => Mode::Lsb,
            2 => Mode::Cwr,
            3 => Mode::Cwl,
            4 => Mode::Am,
            5 => Mode::Wfm,
            6 => Mode::Nfm,
            7 => Mode::Digi,
            8 => Mode::Pkt,
            9 => Mode::Dmr,
            10 => Mode::Dfm,
            _ => return None,
        })
    }
}

/// VFO 选择
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VfoSelect {
    A = 0,
    B = 1,
}

impl VfoSelect {
    pub fn from_u8(v: u8) -> Option<VfoSelect> {
        Some(match v {
            0 => VfoSelect::A,
            1 => VfoSelect::B,
            _ => return None,
        })
    }
}

/// NR/NB 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NrNbState {
    Off = 0,
    NrOn = 1,
    NbOn = 2,
}

/// 收发状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TxRxState {
    Rx = 0,
    Tx = 1,
}

/// 异频模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SplitMode {
    Off = 0,
    On = 1,
}

/// AB频设置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AbMode {
    A = 0,
    B = 1,
    Equal = 2,
}

/// 前级放大器选择
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AmpMode {
    AmpA = 0,
    AmpB = 1,
}

/// 频谱带宽 (SPAN)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpectrumSpan {
    K48 = 0,
    K24 = 1,
    K12 = 2,
    K6 = 3,
    K3 = 4,
    K1_5 = 5,
}

impl SpectrumSpan {
    pub fn from_u8(v: u8) -> Option<SpectrumSpan> {
        Some(match v {
            0 => SpectrumSpan::K48,
            1 => SpectrumSpan::K24,
            2 => SpectrumSpan::K12,
            3 => SpectrumSpan::K6,
            4 => SpectrumSpan::K3,
            5 => SpectrumSpan::K1_5,
            _ => return None,
        })
    }
}

/// 频谱显示模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DisplayMode {
    Both = 0,
    Spectrum = 1,
    Waterfall = 2,
    Off = 3,
}

/// 天调模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TunerMode {
    Off = 0,
    On = 1,
    Tune = 2,
}

/// 电键类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyType {
    AutoL = 0,
    AutoR = 1,
    Key = 2,
}

/// USB数据格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbDataFormat {
    Audio = 0,
    Iq = 1,
}

/// 信道模式/VFO模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelMode {
    Vfo = 0,
    Channel = 1,
}

/// 功率等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerLevel {
    Low = 0,
    High = 1,
}

/// 设备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DeviceType {
    Q900 = 0,
    Hs2 = 1,
    Hs3 = 2,
    Qr20 = 3,
    Tbr119 = 4,
    Pmr119 = 5,
    Sjr188 = 6,
    Pmr171 = 7,
    Xp100 = 7, // note: XP-100 also maps to 7 in 11.12; PMR-171 is 7 in earlier tables
}

impl DeviceType {
    pub fn from_u8(v: u8) -> Option<DeviceType> {
        Some(match v {
            0 => DeviceType::Q900,
            1 => DeviceType::Hs2,
            2 => DeviceType::Hs3,
            3 => DeviceType::Qr20,
            4 => DeviceType::Tbr119,
            5 => DeviceType::Pmr119,
            6 => DeviceType::Sjr188,
            7 => DeviceType::Pmr171,
            _ => return None,
        })
    }
}

/// ITUR 分区
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IturRegion {
    All = 0,
    Itur1 = 1,
    Itur2 = 2,
    Itur3 = 3,
    AirForce = 4,
}

/// 功放触发模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PaTriggerMode {
    Ptt1 = 0,
    Ptt2 = 1,
    PowerTrigger = 3,
}

/// 功放天线选择
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AntennaSelect {
    Port1 = 1,
    Port2 = 2,
    Port3 = 3,
    Port4 = 4,
}

/// 输入衰减器
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Attenuator {
    Db0 = 0,
    Db1 = 1,
    Db2 = 2,
    Db3 = 3,
}

/// 频段索引
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Band {
    B1_8MHz,
    B3_5MHz,
    B5MHz,
    B7MHz,
    B10MHz,
    B14MHz,
    B18MHz,
    B21MHz,
    B24MHz,
    B28MHz,
    B50MHz,
    B144MHz,
    B430MHz,
}

impl Band {
    /// 按协议中的频段定义顺序返回对应索引 (5.11)
    pub fn to_u8(self) -> u8 {
        match self {
            Band::B1_8MHz => 0,
            Band::B3_5MHz => 1,
            Band::B5MHz => 2,
            Band::B7MHz => 3,
            Band::B10MHz => 4,
            Band::B14MHz => 5,
            Band::B18MHz => 6,
            Band::B21MHz => 7,
            Band::B24MHz => 8,
            Band::B28MHz => 9,
            Band::B50MHz => 10,
            Band::B144MHz => 11,
            Band::B430MHz => 12,
        }
    }

    pub fn from_u8(v: u8) -> Option<Band> {
        Some(match v {
            0 => Band::B1_8MHz,
            1 => Band::B3_5MHz,
            2 => Band::B5MHz,
            3 => Band::B7MHz,
            4 => Band::B10MHz,
            5 => Band::B14MHz,
            6 => Band::B18MHz,
            7 => Band::B21MHz,
            8 => Band::B24MHz,
            9 => Band::B28MHz,
            10 => Band::B50MHz,
            11 => Band::B144MHz,
            12 => Band::B430MHz,
            _ => return None,
        })
    }
}

/// 发射亚音表 (5.20)
pub const CTCSS_TABLE: [f32; 56] = [
    0.0, 67.0, 69.3, 71.9, 74.4, 77.0, 79.7, 82.5, 85.4, 88.5,
    91.5, 94.8, 97.4, 100.0, 103.5, 107.2, 110.9, 114.8, 118.8, 123.0,
    127.3, 131.8, 136.5, 141.3, 146.2, 150.0, 151.4, 156.7, 159.8, 162.2,
    165.5, 167.9, 171.3, 173.8, 177.3, 179.9, 183.5, 186.2, 189.9, 192.8,
    196.6, 199.5, 203.5, 206.5, 210.7, 213.8, 218.1, 221.3, 225.7, 229.1,
    233.6, 237.1, 241.8, 245.5, 250.3, 254.1,
];

/// 前导音定义 (5.20)
pub const LEAD_TONES: [(u8, f32); 3] = [
    (0, 0.0),
    (1, 1750.0),
    (2, 2135.0),
];
