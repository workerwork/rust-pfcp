use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	        Type = 96 (decimal)
// 3 to 4	        Length = n
// 5 to 8	    Recovery Time Stamp value
// 9 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct RecoveryTimeStamp {
    ie_type: u16,
    ie_len: u16,

    //indicates the UTC time when the PFCP entity started
    recovery_time_stamp: Vec<u8>, //4bytes
}

impl RecoveryTimeStamp {
    pub fn decode(buf: &[u8], len: u16) -> Result<RecoveryTimeStamp, PFCPError> {
        let recovery_time_stamp = RecoveryTimeStamp {
            ie_type: ie_type::RECOVERY_TIME_STAMP,
            ie_len: len,
            //recovery_time_stamp: (buf[0] * 16 * 16 * 16 + buf[1] * 16 * 16 + buf[2] * 16 + buf[3]).into(),
            recovery_time_stamp: buf.to_vec(),
        };
        Ok(recovery_time_stamp)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.recovery_time_stamp);
        element_vec
    }
}
