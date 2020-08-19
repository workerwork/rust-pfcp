use super::header::*;
use super::*;

use elements::cause::Cause;
use elements::ie_type;
use elements::node_id::NodeID;

#[derive(Debug, Default)]
pub struct AssociationUpdateRequest {
    pub header: Header,
    pub node_id: NodeID,
}

impl AssociationUpdateRequest {
    pub fn parse(mut buf: &mut [u8], header: Header) -> _Message {
        let mut message = AssociationUpdateRequest {
            header,
            ..Default::default()
        };
        if message.header.s == true {
            buf = &mut buf[12..];
        } else {
            buf = &mut buf[4..];
        }
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::NODE_ID => {
                    message.node_id = NodeID::decode(buf, elen)?;
                }
                _ => println!("unknown ie_type!"),
            }
            buf = &mut buf[elen.into()..];
        }
        println!("{:#?}", message);
        Ok(Message::AUReq(message))
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        message_vec.append(&mut self.node_id.encode());
        println!("{:?}", message_vec);
        message_vec
    }
}

#[derive(Debug, Default)]
pub struct AssociationUpdateResponse {
    pub header: Header,
    pub node_id: NodeID,
    pub cause: Cause,
}

impl AssociationUpdateResponse {
    pub fn parse(mut buf: &mut [u8], header: Header) -> _Message {
        let mut message = AssociationUpdateResponse {
            header,
            ..Default::default()
        };
        if message.header.s == true {
            buf = &mut buf[12..];
        } else {
            buf = &mut buf[4..];
        }
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::NODE_ID => {
                    message.node_id = NodeID::decode(buf, elen)?;
                }
                ie_type::CAUSE => {
                    message.cause = Cause::decode(buf, elen)?;
                }
                _ => println!("unknown ie_type!"),
            }
            buf = &mut buf[elen.into()..];
        }
        println!("{:#?}", message);
        Ok(Message::AUResp(message))
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        message_vec.append(&mut self.node_id.encode());
        println!("{:?}", message_vec);
        message_vec
    }
}
