use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
// 	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 160 (decimal)
// 3 to 4	    Length = n
// 5 	        Spare	Interface Type value
// 6 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct _3GPPInterfaceType {
    ie_type: u16,
    ie_len: u16,
    interface_type: u8, //M
}

impl _3GPPInterfaceType {
    pub fn decode(buf: &[u8], len: u16) -> Result<_3GPPInterfaceType, PFCPError> {
        let mut element = _3GPPInterfaceType {
            ie_type: ie_type::_3GPP_INTERFACE_TYPE,
            ie_len: len,
            ..Default::default()
        };
        element.interface_type = buf[0] & 0b0011_1111;
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.interface_type);
        element_vec
    }
}
