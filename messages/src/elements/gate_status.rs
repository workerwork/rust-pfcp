use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 25 (decimal)
// 3 to 4	    Length = n
// 5	        Spare	UL Gate	DL Gate
// 6 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

pub mod gate_status {
    pub const OPEN: u8 = 0;
    pub const CLOSED: u8 = 1;
}

#[derive(Debug, Default)]
pub struct GateStatus {
    ie_type: u16,
    ie_len: u16,
    ul_gate: u8,
    dl_gate: u8,
}

pub type _GateStatus = Result<GateStatus, PFCPError>;

impl GateStatus {
    pub fn decode(buf: &[u8], len: u16) -> _GateStatus {
        let mut element = GateStatus {
            ie_type: ie_type::GATE_STATUS,
            ie_len: len,
            ..Default::default()
        };
        element.ul_gate = buf[0] & 0b0000_1100;
        element.dl_gate = buf[0] & 0b0000_0011;
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.ul_gate | self.dl_gate);
        element_vec
    }
}
