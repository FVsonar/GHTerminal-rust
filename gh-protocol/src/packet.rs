use crate::crc::crc16_ccitt_false;
use crate::error::ProtocolError;

/// 命令包 (0xA5 包头)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandPacket {
    pub cmd: u8,
    pub data: Vec<u8>,
}

/// 频谱数据包 (0x7E 包头)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpectrumPacket {
    pub data: Vec<u8>,
}

/// 解码后的帧
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    Command(CommandPacket),
    Spectrum(SpectrumPacket),
}

pub const HEADER_COMMAND: u8 = 0xA5;
pub const HEADER_SPECTRUM: u8 = 0x7E;
pub const HEADER_LEN: usize = 4;
pub const CRC_LEN: usize = 2;

impl CommandPacket {
    pub fn new(cmd: u8, data: Vec<u8>) -> Self {
        Self { cmd, data }
    }

    /// 编码为完整的数据包字节
    ///
    /// 格式: [0xA5;4] + 包长 + 命令类型 + DATA + CRC高 + CRC低
    /// 包长 = 1(cmd) + data.len()
    pub fn encode(&self) -> Vec<u8> {
        let payload_len = 1u8 + self.data.len() as u8;

        let mut buf = Vec::with_capacity(HEADER_LEN + 1 + 1 + self.data.len() + CRC_LEN);
        buf.extend([HEADER_COMMAND; 4]);
        buf.push(payload_len);
        buf.push(self.cmd);
        buf.extend(&self.data);

        // CRC 从包长字节开始到 data 末尾
        let crc = crc16_ccitt_false(&buf[HEADER_LEN..]);
        buf.push((crc >> 8) as u8);
        buf.push((crc & 0xFF) as u8);

        buf
    }

    /// 从完整的数据包解码 (假设已通过 codec 或外部逻辑找到帧头)
    ///
    /// 输入应包含从第一个 0xA5 开始到帧尾的完整字节
    pub fn decode(buf: &[u8]) -> Result<Self, ProtocolError> {
        if buf.len() < HEADER_LEN + 1 + CRC_LEN {
            return Err(ProtocolError::TruncatedPacket);
        }

        if buf[0..HEADER_LEN] != [HEADER_COMMAND; 4] {
            return Err(ProtocolError::InvalidHeader);
        }

        let payload_len = buf[HEADER_LEN] as usize;

        // 包长指 cmd + data, 总长度还需加上 header + 包长本身 + crc
        let expected = HEADER_LEN + 1 + payload_len + CRC_LEN;
        if buf.len() < expected {
            return Err(ProtocolError::TruncatedPacket);
        }

        let crc_data = &buf[HEADER_LEN..HEADER_LEN + 1 + payload_len];
        let crc_hi = buf[HEADER_LEN + 1 + payload_len];
        let crc_lo = buf[HEADER_LEN + 1 + payload_len + 1];
        let expected_crc = ((crc_hi as u16) << 8) | (crc_lo as u16);
        let actual_crc = crc16_ccitt_false(crc_data);

        if expected_crc != actual_crc {
            return Err(ProtocolError::CrcMismatch {
                expected: expected_crc,
                got: actual_crc,
            });
        }

        let cmd = buf[HEADER_LEN + 1];
        let data = buf[HEADER_LEN + 2..HEADER_LEN + 1 + payload_len].to_vec();

        Ok(Self { cmd, data })
    }

    /// 返回完整数据包长度 (用于 codec 确定何时帧结束)
    pub fn total_len(buf: &[u8]) -> Option<usize> {
        if buf.len() < HEADER_LEN + 1 {
            return None;
        }
        let payload_len = buf[HEADER_LEN] as usize;
        Some(HEADER_LEN + 1 + payload_len + CRC_LEN)
    }
}

impl SpectrumPacket {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// 解码频谱数据包
    ///
    /// 输入应包含从第一个 0x7E 开始到帧尾的完整字节
    /// V1.0 硬件: 256字节数据, V2.0 硬件: 80字节数据
    pub fn decode(buf: &[u8]) -> Result<Self, ProtocolError> {
        if buf.len() < HEADER_LEN {
            return Err(ProtocolError::TruncatedPacket);
        }

        if buf[0..HEADER_LEN] != [HEADER_SPECTRUM; 4] {
            return Err(ProtocolError::InvalidHeader);
        }

        let data = buf[HEADER_LEN..].to_vec();
        Ok(Self { data })
    }

    /// 尝试确定频谱帧的数据长度 (V1=256, V2=80)
    /// 返回 (spectrum_data_len, hardware_version)
    pub fn guess_version(buf: &[u8]) -> Option<(usize, u8)> {
        if buf.len() < HEADER_LEN {
            return None;
        }
        if buf[0..HEADER_LEN] != [HEADER_SPECTRUM; 4] {
            return None;
        }
        let remaining = buf.len() - HEADER_LEN;
        if remaining >= 256 {
            Some((256, 1))
        } else if remaining >= 80 {
            Some((80, 2))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let pkt = CommandPacket::new(0x07, vec![0x00]);
        let encoded = pkt.encode();

        assert_eq!(encoded[0..4], [0xA5, 0xA5, 0xA5, 0xA5]);
        assert_eq!(encoded[4], 0x02); // 包长: cmd(1) + data(1) = 2
        assert_eq!(encoded[5], 0x07); // cmd
        assert_eq!(encoded[6], 0x00); // data

        let decoded = CommandPacket::decode(&encoded).unwrap();
        assert_eq!(decoded, pkt);
    }

    #[test]
    fn decode_bad_header() {
        let buf = [0x00, 0x00, 0x00, 0x00, 0x02, 0x07, 0x00, 0x00, 0x00];
        assert_eq!(
            CommandPacket::decode(&buf).unwrap_err(),
            ProtocolError::InvalidHeader
        );
    }

    #[test]
    fn decode_bad_crc() {
        let mut buf = CommandPacket::new(0x07, vec![0x00]).encode();
        // 篡改 CRC
        let len = buf.len();
        buf[len - 1] ^= 0xFF;
        assert!(matches!(
            CommandPacket::decode(&buf).unwrap_err(),
            ProtocolError::CrcMismatch { .. }
        ));
    }

    #[test]
    fn spectrum_decode() {
        let data = vec![0x00; 256];
        let mut buf = vec![0x7E; 4];
        buf.extend(&data);
        let decoded = SpectrumPacket::decode(&buf).unwrap();
        assert_eq!(decoded.data.len(), 256);
    }

    #[test]
    fn status_request_encoding() {
        // 状态同步命令: 0x0B, 无数据
        let pkt = CommandPacket::new(0x0B, vec![]);
        let encoded = pkt.encode();
        assert_eq!(encoded.len(), 4 + 1 + 1 + 2); // header + len + cmd + crc
        assert_eq!(encoded[4], 0x01); // 包长: cmd(1) + data(0) = 1
    }

    #[test]
    fn frequency_set_encoding() {
        // 频率设置: 0x09, 数据=4字节频率 (14.074MHz = 14074000 = 0x00D6BE88)
        let freq: u32 = 14074000;
        let data = freq.to_be_bytes().to_vec();
        let pkt = CommandPacket::new(0x09, data);
        let encoded = pkt.encode();
        assert_eq!(encoded[4], 0x05); // 包长: cmd(1) + data(4) = 5
        assert_eq!(encoded[5], 0x09);
        assert_eq!(&encoded[6..10], &[0x00, 0xD6, 0xC0, 0x90]);
    }
}
