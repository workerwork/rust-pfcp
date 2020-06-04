use super::ie_type::IEType;

#[derive(Debug, Default)]
pub struct RecoveryTimeStamp {
    ie_type: IEType,
    ie_len: u16,
    recovery_time_stamp: Vec<u8>, //4bytes
}

impl RecoveryTimeStamp {
    pub fn decode(buf: &[u8], len: u16) -> RecoveryTimeStamp {
        RecoveryTimeStamp {
            ie_type: IEType::RecoveryTimeStamp,
            ie_len: len,
            recovery_time_stamp: buf.to_vec(),
        }
    }
}
