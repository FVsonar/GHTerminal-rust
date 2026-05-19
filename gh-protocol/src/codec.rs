use crate::error::ProtocolError;
use crate::packet::{self, CommandPacket, Frame, SpectrumPacket, HEADER_COMMAND, HEADER_LEN, HEADER_SPECTRUM};

/// 帧解码状态机 — 处理连续字节流中的粘包/半包
#[derive(Debug, Clone)]
pub struct Codec {
    buf: Vec<u8>,
}

#[derive(Debug, Clone)]
enum HeaderMatch {
    None,
    Command,
    Spectrum,
}

impl Codec {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// 输入原始字节，返回所有已完整解码的帧
    pub fn feed(&mut self, data: &[u8]) -> Vec<Result<Frame, ProtocolError>> {
        self.buf.extend(data);
        let mut frames = Vec::new();

        loop {
            match self.try_decode_one() {
                Ok(Some(frame)) => frames.push(Ok(frame)),
                Err(e) => {
                    frames.push(Err(e));
                    // 错误后清空缓冲区，重新开始搜索帧头
                    self.buf.clear();
                    break;
                }
                Ok(None) => break, // 数据不足，等待更多字节
            }
        }

        // 防止无限增长
        if self.buf.len() > 4096 {
            // 保留最后 1024 字节，丢弃前面积压的字节
            let drain_end = self.buf.len() - 1024;
            self.buf.drain(..drain_end);
        }

        frames
    }

    /// 尝试解码一个帧。返回：
    /// - Ok(Some(frame)): 成功解码
    /// - Ok(None): 数据不足
    /// - Err(e): 解码错误
    fn try_decode_one(&mut self) -> Result<Option<Frame>, ProtocolError> {
        // 1. 查找帧头
        let header_pos = match Self::find_header(&self.buf) {
            (HeaderMatch::None, _) => {
                // 未找到任何帧头 — 清除到最后一个可能的起始位
                if self.buf.len() >= 4 {
                    let keep_start = self.buf.len() - 3;
                    self.buf.drain(..keep_start);
                }
                return Ok(None);
            }
            (kind, pos) => {
                if pos > 0 {
                    self.buf.drain(..pos);
                }
                kind
            }
        };

        match header_pos {
            HeaderMatch::Command => self.decode_command(),
            HeaderMatch::Spectrum => self.decode_spectrum(),
            HeaderMatch::None => Ok(None),
        }
    }

    fn decode_command(&mut self) -> Result<Option<Frame>, ProtocolError> {
        // 需要至少 header + length_byte + cmd + crc
        if self.buf.len() < HEADER_LEN + 1 + 1 + packet::CRC_LEN {
            return Ok(None);
        }

        let payload_len = self.buf[HEADER_LEN] as usize;
        let total = HEADER_LEN + 1 + payload_len + packet::CRC_LEN;

        if self.buf.len() < total {
            return Ok(None);
        }

        let pkt = CommandPacket::decode(&self.buf[..total])?;
        self.buf.drain(..total);
        Ok(Some(Frame::Command(pkt)))
    }

    fn decode_spectrum(&mut self) -> Result<Option<Frame>, ProtocolError> {
        // 需要至少 header + 一些数据
        if self.buf.len() < HEADER_LEN + 1 {
            return Ok(None);
        }

        // 频谱帧无长度字段 — 尝试猜测版本
        // V1: 256 data bytes, V2: 80 data bytes
        let remaining = self.buf.len() - HEADER_LEN;

        let data_len = if remaining >= 256 {
            256
        } else if remaining >= 80 {
            80
        } else {
            return Ok(None); // 等待更多数据
        };

        let total = HEADER_LEN + data_len;
        let pkt = SpectrumPacket::decode(&self.buf[..total])?;
        self.buf.drain(..total);
        Ok(Some(Frame::Spectrum(pkt)))
    }

    /// 搜索帧头，返回 (类型, 位置)
    fn find_header(buf: &[u8]) -> (HeaderMatch, usize) {
        // 至少需要 4 字节才能匹配帧头
        if buf.len() < 4 {
            return (HeaderMatch::None, 0);
        }

        for i in 0..=buf.len() - 4 {
            // 优先匹配命令帧头 (0xA5)
            if buf[i..i + 4] == [HEADER_COMMAND; 4] {
                return (HeaderMatch::Command, i);
            }
            // 频谱帧头 (0x7E)
            if buf[i..i + 4] == [HEADER_SPECTRUM; 4] {
                return (HeaderMatch::Spectrum, i);
            }
        }

        (HeaderMatch::None, 0)
    }
}

impl Default for Codec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_single_command() {
        let mut codec = Codec::new();
        let pkt = CommandPacket::new(0x07, vec![0x00]);
        let encoded = pkt.encode();

        let frames = codec.feed(&encoded);
        assert_eq!(frames.len(), 1);
        match &frames[0] {
            Ok(Frame::Command(cmd)) => assert_eq!(cmd.cmd, 0x07),
            _ => panic!("expected command frame"),
        }
    }

    #[test]
    fn decode_partial_data() {
        let mut codec = Codec::new();
        let pkt = CommandPacket::new(0x0B, vec![]);
        let encoded = pkt.encode();

        // 先喂前半部分
        let frames = codec.feed(&encoded[..3]);
        assert!(frames.is_empty());

        // 喂剩余部分
        let frames = codec.feed(&encoded[3..]);
        assert_eq!(frames.len(), 1);
    }

    #[test]
    fn decode_multiple_packets() {
        let mut codec = Codec::new();
        let pkt1 = CommandPacket::new(0x07, vec![0x00]).encode();
        let pkt2 = CommandPacket::new(0x0B, vec![]).encode();

        let mut combined = Vec::new();
        combined.extend(&pkt1);
        combined.extend(&pkt2);

        let frames = codec.feed(&combined);
        assert_eq!(frames.len(), 2);
    }

    #[test]
    fn decode_spectrum_and_command_interleaved() {
        let mut codec = Codec::new();

        let cmd = CommandPacket::new(0x07, vec![0x00]).encode();
        let mut spec = vec![0x7E; 4];
        spec.extend([0x00; 80]);

        let mut combined = Vec::new();
        combined.extend(&cmd);
        combined.extend(&spec);

        let frames = codec.feed(&combined);
        assert_eq!(frames.len(), 2);

        assert!(matches!(&frames[0], Ok(Frame::Command(_))));
        assert!(matches!(&frames[1], Ok(Frame::Spectrum(_))));
    }

    #[test]
    fn junk_bytes_before_header() {
        let mut codec = Codec::new();
        let pkt = CommandPacket::new(0x07, vec![0x00]).encode();

        let mut input = vec![0xFF, 0xFE, 0x00];
        input.extend(&pkt);

        let frames = codec.feed(&input);
        assert_eq!(frames.len(), 1);
        assert!(matches!(&frames[0], Ok(Frame::Command(_))));
    }
}
