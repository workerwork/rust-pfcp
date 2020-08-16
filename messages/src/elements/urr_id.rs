use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//                      Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 81 (decimal)
// 3 to 4	    Length = n
// 5 to 8	    URR ID value
// 9 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct URRID {
    ie_type: u16,
    ie_len: u16,
    urr_id: Vec<u8>, //M 4bytes
}

pub type _URRID = Result<URRID, PFCPError>;

impl URRID {
    pub fn decode(buf: &[u8], len: u16) -> _URRID {
        let mut element = URRID {
            ie_type: ie_type::URR_ID,
            ie_len: len,
            ..Default::default()
        };
        element.urr_id = buf[0..=3].to_vec();
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.urr_id);
        element_vec
    }
}
