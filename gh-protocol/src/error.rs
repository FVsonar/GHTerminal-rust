#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    /// 包头无效 (不是 0xA5 × 4)
    InvalidHeader,
    /// CRC 校验失败
    CrcMismatch { expected: u16, got: u16 },
    /// 包长字段不合法
    InvalidLength,
    /// 数据包不完整 (buffer 不够长)
    TruncatedPacket,
    /// 未知命令类型
    UnknownCommand(u8),
    /// 数据字段解析失败
    InvalidData,
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::InvalidHeader => write!(f, "invalid packet header"),
            ProtocolError::CrcMismatch { expected, got } => {
                write!(
                    f,
                    "CRC mismatch: expected 0x{expected:04X}, got 0x{got:04X}"
                )
            }
            ProtocolError::InvalidLength => write!(f, "invalid packet length"),
            ProtocolError::TruncatedPacket => write!(f, "truncated packet"),
            ProtocolError::UnknownCommand(cmd) => write!(f, "unknown command 0x{cmd:02X}"),
            ProtocolError::InvalidData => write!(f, "invalid data field"),
        }
    }
}

impl std::error::Error for ProtocolError {}
