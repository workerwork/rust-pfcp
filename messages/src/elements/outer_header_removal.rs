use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 95 (decimal)
// 3 to 4	    Length = n
// 5	        Outer Header Removal Description
// 6	        GTP-U Extension Header Deletion
// 7 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

//#[derive(Debug, Default)]
//pub enum OuterHeaderRemovalDescription {
//    gtpu_udp_ipv4: u8,
//}

#[derive(Debug, Default)]
pub struct OuterHeaderRemoval {
    ie_type: u16,
    ie_len: u16,
    description: u8,                            //M
    gtpu_extension_header_deletion: Option<u8>, //C
}

impl OuterHeaderRemoval {
    pub fn decode(buf: &[u8], len: u16) -> Result<OuterHeaderRemoval, PFCPError> {
        let mut element = OuterHeaderRemoval {
            ie_type: ie_type::OUTER_HEADER_REMOVAL,
            ie_len: len,
            ..Default::default()
        };
        element.description = buf[0];
        if len > 1 {
            element.gtpu_extension_header_deletion = Some(buf[1]);
        }
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.description);
        if let Some(gtpu_extension_header_deletion) = self.gtpu_extension_header_deletion {
            element_vec.push(gtpu_extension_header_deletion);
        }
        element_vec
    }
}
