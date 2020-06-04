use super::ie_type::IEType;

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
}

#[derive(Debug, Default)]
pub struct NodeID {
    ie_type: IEType,
    ie_len: u16,
    node_id_type: NodeIDType,
    node_id_value: Vec<u8>,
}

impl NodeID {
    pub fn decode(buf: &[u8], len: u16) -> NodeID {
        let mut nodeid = NodeID {
            ie_type: IEType::NodeID,
            ie_len: len,
            ..Default::default()
        };
        match buf[4] & 0b0000_1111 {
            t if t == NodeIDType::IPV4 as u8 => {
                nodeid.node_id_type = NodeIDType::IPV4;
                nodeid.node_id_value = buf[5..9].to_vec();
            }
            t if t == NodeIDType::IPV6 as u8 => {
                nodeid.node_id_type = NodeIDType::IPV6;
                nodeid.node_id_value = buf[5..21].to_vec();
            }
            t if t == NodeIDType::FQDN as u8 => {
                nodeid.node_id_type = NodeIDType::FQDN;
                nodeid.node_id_value = buf[5..7].to_vec();
            }
            _ => println!("err"),
        }
        nodeid
    }
}
