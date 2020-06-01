enum NodeIDType {
    IPV4 = 0,
    IPV6 = 1,
    FQDN = 2,
}

enum NodeIDValue {
    IPV4Addr([u8; 4]),
    IPV6Addr([u8; 16]),
    FQDN,
}

pub struct NodeID {
    ie_type: super::ie_type::IEType,
    ie_len: u16,
    node_id_type: NodeIDType,
    node_id_value: NodeIDValue,
}

impl NodeID {
    fn new(nodeidtype: NodeIDType, nodeidvalue: NodeIDValue) -> NodeID {
        let len: u16 = match nodeidtype {
            NodeIDType::IPV4 => 4,
            NodeIDType::IPV6 => 16,
            NodeIDType::FQDN => 2,
        };
        NodeID {
            ie_type: super::ie_type::IEType::NodeID,
            ie_len: len,
            node_id_type: nodeidtype,
            node_id_value: nodeidvalue,
        }
    }
}
