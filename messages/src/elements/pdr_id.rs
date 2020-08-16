use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 56 (decimal)
// 3 to 4	    Length = n
// 5 to 6	    Rule ID
// 7to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct PDRID {
    ie_type: u16,
    ie_len: u16,
    rule_id: Vec<u8>, //M 2bytes
}

pub type _PDRID = Result<PDRID, PFCPError>;

impl PDRID {
    pub fn decode(buf: &[u8], len: u16) -> _PDRID {
        let mut element = PDRID {
            ie_type: ie_type::PDR_ID,
            ie_len: len,
            ..Default::default()
        };
        element.rule_id = buf[0..=1].to_vec();
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.rule_id);
        element_vec
    }
}
