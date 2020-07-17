use super::ie_type;

#[derive(Debug, Default)]
pub struct RecoveryTimeStamp {
    ie_type: u16,
    ie_len: u16,
    recovery_time_stamp: Vec<u8>, //4bytes
    //recovery_time_stamp: u32,
}

impl RecoveryTimeStamp {
    pub fn decode(buf: &[u8], len: u16) -> RecoveryTimeStamp {
        RecoveryTimeStamp {
            ie_type: ie_type::RECOVERY_TIME_STAMP,
            ie_len: len,
            //recovery_time_stamp: (buf[0] * 16 * 16 * 16 + buf[1] * 16 * 16 + buf[2] * 16 + buf[3]).into(),
            recovery_time_stamp: buf.to_vec(),
        }
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.recovery_time_stamp);
        element_vec
    }
}
