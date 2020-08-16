use super::super::PFCPError;
use super::ie_type;

// ----------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 22 (decimal)
// 3 to 4	    Length = n
// 5 to (n+4)	Network Instance
// ----------------------------------------------

#[derive(Debug, Default)]
pub struct NetworkInstance {
    ie_type: u16,
    ie_len: u16,
    network_instance: Vec<u8>, //M
}

pub type _NetworkInstance = Result<NetworkInstance, PFCPError>;

impl NetworkInstance {
    pub fn decode(buf: &[u8], len: u16) -> _NetworkInstance {
        let mut element = NetworkInstance {
            ie_type: ie_type::NETWORK_INSTANCE,
            ie_len: len,
            ..Default::default()
        };
        element.network_instance = buf.to_vec();
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.network_instance);
        element_vec
    }
}
