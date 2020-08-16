use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 29 (decimal)
// 3 to 4	    Length = n
// 5 to 8	    Precedence value
// 9 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Precedence {
    ie_type: u16,
    ie_len: u16,
    precedence: Vec<u8>, //M 4bytes
}

pub type _Precedence = Result<Precedence, PFCPError>;

impl Precedence {
    pub fn decode(buf: &[u8], len: u16) -> _Precedence {
        let mut element = Precedence {
            ie_type: ie_type::PRECEDENCE,
            ie_len: len,
            ..Default::default()
        };
        element.precedence = buf[0..=3].to_vec();
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.precedence);
        element_vec
    }
}
