use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 20 (decimal)
// 3 to 4	    Length = n
// 5 	        Spare	    Interface value
// 6 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct SourceInterface {
    ie_type: u16,
    ie_len: u16,
    interface: u8, //M
}

impl SourceInterface {
    pub fn decode(buf: &mut [u8], len: u16) -> Result<SourceInterface, PFCPError> {
        let mut element = SourceInterface {
            ie_type: ie_type::SOURCE_INTERFACE,
            ie_len: len,
            ..Default::default()
        };
        element.interface = buf[0] & 0b0000_1111;
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.interface);
        element_vec
    }
}
