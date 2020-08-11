use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 26 (decimal)
// 3 to 4	    Length = n
// 5 to 9	    UL MBR
// 10 to 14	    DL MBR
// 15 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct MBR {
    ie_type: u16,
    ie_len: u16,
    ul_mbr: Vec<u8>, //M 5bytes
    dl_mbr: Vec<u8>, //M 5bytes
}

impl MBR {
    pub fn decode(buf: &mut [u8], len: u16) -> Result<MBR, PFCPError> {
        let mut element = MBR {
            ie_type: ie_type::MBR,
            ie_len: len,
            ..Default::default()
        };
        element.ul_mbr = buf[0..=4].to_vec();
        element.dl_mbr = buf[5..=9].to_vec();
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.ul_mbr);
        element_vec.append(&mut self.dl_mbr);
        element_vec
    }
}
