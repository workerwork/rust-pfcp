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
const NODE_ID_TYPE_IPV4: u8 = 0;
const NODE_ID_TYPE_IPV6: u8 = 1;
const NODE_ID_TYPE_FQDN: u8 = 2;

#[derive(Debug, Default)]
pub struct NodeID {
    ie_type: u16,
    ie_len: u16,

    //Node ID Type
    node_id_type: u8,

    //Node ID Value
    node_id_value: Vec<u8>,
}

impl NodeID {
    pub fn decode(buf: &[u8], len: u16) -> Result<NodeID, PFCPError> {
        let mut element = NodeID {
            ie_type: ie_type::NODE_ID,
            ie_len: len,
            ..Default::default()
        };
        match buf[0] & 0b0000_1111 {
            NODE_ID_TYPE_IPV4 => {
                element.node_id_type = NODE_ID_TYPE_IPV4;
                element.node_id_value = buf[1..5].to_vec();
            }
            NODE_ID_TYPE_IPV6 => {
                element.node_id_type = NODE_ID_TYPE_IPV6;
                element.node_id_value = buf[1..17].to_vec();
            }
            NODE_ID_TYPE_FQDN => {
                element.node_id_type = NODE_ID_TYPE_FQDN;
                element.node_id_value = buf[1..2].to_vec();
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
        element_vec.append(&mut self.node_id_value);
        element_vec
    }
}
