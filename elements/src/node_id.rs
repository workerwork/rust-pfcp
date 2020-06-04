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
    //ie_type: IEType,
    ie_type: u16,
    ie_len: u16,
    //node_id_type: NodeIDType,
    node_id_type: u8,
    node_id_value: Vec<u8>,
}

impl NodeID {
    pub fn decode(buf: &[u8], len: u16) -> NodeID {
        let mut nodeid = NodeID {
            //ie_type: IEType::NodeID,
            ie_type: ie_type::NODE_ID,
            ie_len: len,
            ..Default::default()
        };
        match buf[0] & 0b0000_1111 {
            //t if t == NodeIDType::IPV4 as u8 => {
            NODE_ID_TYPE_IPV4 => {
                //nodeid.node_id_type = NodeIDType::IPV4;
                nodeid.node_id_type = NODE_ID_TYPE_IPV4;
                nodeid.node_id_value = buf[1..5].to_vec();
            }
            //t if t == NodeIDType::IPV6 as u8 => {
            NODE_ID_TYPE_IPV6 => {
                //nodeid.node_id_type = NodeIDType::IPV6;
                nodeid.node_id_type = NODE_ID_TYPE_IPV6;
                nodeid.node_id_value = buf[1..17].to_vec();
            }
            //t if t == NodeIDType::FQDN as u8 => {
            NODE_ID_TYPE_FQDN => {
                //nodeid.node_id_type = NodeIDType::FQDN;
                nodeid.node_id_type = NODE_ID_TYPE_FQDN;
                nodeid.node_id_value = buf[1..2].to_vec();
            }
            _ => println!("err"),
        }
        nodeid
    }
}
