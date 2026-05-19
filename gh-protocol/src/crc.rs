/// CRC16/CCITT-FALSE 校验
///
/// 多项式: 0x1021, 初始值: 0xFFFF
/// 算法参见协议附录1
pub fn crc16_ccitt_false(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        let mut cur = (byte as u16) << 8;
        for _ in 0..8 {
            if ((crc ^ cur) as i16) < 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
            cur <<= 1;
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_data_gives_initial_value() {
        assert_eq!(crc16_ccitt_false(&[]), 0xFFFF);
    }

    #[test]
    fn known_vector_check() {
        // "123456789" 的 CRC16/CCITT-FALSE 标准向量
        let data = b"123456789";
        assert_eq!(crc16_ccitt_false(data), 0x29B1);
    }

    #[test]
    fn simple_packet() {
        // 模拟一个简单的命令包 (包长=0x02, 命令=0x0B, 也就是状态同步请求)
        let data = &[0x02, 0x0B];
        let crc = crc16_ccitt_false(data);
        assert_eq!(crc, 0xCA06);
    }
}
