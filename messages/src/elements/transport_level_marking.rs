use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//                      Bits
//Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 30 (decimal)
// 3 to 4	    Length = n
// 5 to 6	    ToS/Traffic Class
// 7 to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct TransportLevelMarking {
    ie_type: u16,
    ie_len: u16,
    tos_or_traffic_class: Vec<u8>, //M 2bytes
}

pub type _TransportLevelMarking = Result<TransportLevelMarking, PFCPError>;

impl TransportLevelMarking {
    pub fn decode(buf: &[u8], len: u16) -> _TransportLevelMarking {
        let mut element = TransportLevelMarking {
            ie_type: ie_type::TRANSPORT_LEVEL_MARKING,
            ie_len: len,
            ..Default::default()
        };
        element.tos_or_traffic_class = buf[0..=1].to_vec();
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.tos_or_traffic_class);
        element_vec
    }
}
