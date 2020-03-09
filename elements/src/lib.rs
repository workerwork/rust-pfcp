pub mod ie_type;

enum IDType {
    ipv4_addr = 0,
    ipv6_addr = 1,
    fqdn = 2,
}

enum NodeIDType {
    
}

pub struct NodeID {
    ie_type: u16,
    ie_len: u16,
    id_type: IDType,
    node_id: NodeIDType,
}

pub struct RecoveryTimeStamp {}

pub struct Cause {}
