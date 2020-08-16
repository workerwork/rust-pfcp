use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 124 (decimal)
// 3 to 4	    Length = n
// 5	        Spare	QFI value
// p to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct QFI {
    ie_type: u16,
    ie_len: u16,
    qfi: u8,
}

impl QFI {
    pub fn decode(buf: &[u8], len: u16) -> Result<QFI, PFCPError> {
        let mut element = QFI {
            ie_type: ie_type::QFI,
            ie_len: len,
            ..Default::default()
        };
        element.qfi = buf[0] & 0b0011_1111;
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.qfi);
        element_vec
    }
}
