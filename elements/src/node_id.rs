//use super::ie_type::IEType;
use super::ie_type;

/*
#[derive(Debug)]
pub enum NodeIDType {
    IPV4 = 0,
    IPV6 = 1,
    FQDN = 2,
    Null = 255,
}

impl Default for NodeIDType {
    fn default() -> Self {
        NodeIDType::Null
    }
}*/

const NODE_ID_TYPE_IPV4: u8 = 0;
const NODE_ID_TYPE_IPV6: u8 = 1;
const NODE_ID_TYPE_FQDN: u8 = 2;

#[derive(Debug, Default)]
pub struct NodeID {
    ie_type: u16,
    ie_len: u16,
    node_id_type: u8,
    node_id_value: Vec<u8>,
}

impl NodeID {
    pub fn decode(buf: &[u8], len: u16) -> NodeID {
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
            _ => println!("err"),
        }
        element
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
