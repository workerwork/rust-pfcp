use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
// 	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 113 (decimal)
// 3 to 4	    Length = n
// 5	Spare	PDN Type
// 6 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct PDNType {
    ie_type: u16,
    ie_len: u16,
    pdn_type: u8,
}

impl PDNType {
    pub fn decode(buf: &[u8], len: u16) -> Result<PDNType, PFCPError> {
        let mut element = PDNType {
            ie_type: ie_type::PDN_TYPE,
            ie_len: len,
            ..Default::default()
        };
        element.pdn_type = buf[0] & 0b0000_0111;
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.pdn_type);
        element_vec
    }
}
