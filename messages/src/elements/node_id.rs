use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	        Type = 60 (decimal)
// 3 to 4	        Length = n
// 5	        Spare	    Node ID Type
// 6 to o	        Node ID Value
// m to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

//The Node ID IE shall contain an FQDN or an IPv4/IPv6 address.
//Bit 1 – V6: If this bit is set to "1", then IPv6 address field shall be present in the F-SEID,
//otherwise the IPv6 address field is not present at all.
//
//Bit 2 – V4: If this bit is set to "1", then IPv4 address field shall be present in the F-SEID,
//otherwise the IPv4 address field is not present at all.
//
//Bit 3 to 8 are spare and reserved for future use.
//
//At least one of V4 and V6 shall be set to "1", and both may be set to "1".

//const NODE_ID_TYPE_IPV4: u8 = 0;
//const NODE_ID_TYPE_IPV6: u8 = 1;
//const NODE_ID_TYPE_FQDN: u8 = 2;

pub mod node_id_type {
    pub const IPV4: u8 = 0;
    pub const IPV6: u8 = 1;
    pub const FQDN: u8 = 2;
}

#[derive(Debug, Default)]
pub struct NodeID {
    ie_type: u16,
    ie_len: u16,

    //Node ID Type
    node_id_type: u8,

    //Node ID Value
    node_id: Vec<u8>,
}

pub type _NodeID = Result<NodeID, PFCPError>;

impl NodeID {
    pub fn decode(buf: &[u8], len: u16) -> _NodeID {
        let mut element = NodeID {
            ie_type: ie_type::NODE_ID,
            ie_len: len,
            ..Default::default()
        };
        element.node_id_type = buf[0] & 0b0000_1111;
        match element.node_id_type {
            node_id_type::IPV4 => {
                element.node_id = buf[1..=4].to_vec();
            }
            node_id_type::IPV6 => {
                element.node_id = buf[1..=16].to_vec();
            }
            node_id_type::FQDN => {
                element.node_id = buf[1..=2].to_vec();
            }
            _ => return Err(PFCPError::Unknown),
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.node_id_type);
        element_vec.append(&mut self.node_id);
        element_vec
    }
}
