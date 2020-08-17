use super::header::*;
use super::*;

use elements::ie_type;
use elements::node_id::NodeID;
use elements::recovery_time_stamp::RecoveryTimeStamp;

#[derive(Debug, Default)]
pub struct AssociationSetupRequest {
    pub header: Header,
    pub node_id: NodeID,
    pub recovery_time_stamp: RecoveryTimeStamp,
}

impl AssociationSetupRequest {
    pub fn parse(mut buf: &mut [u8], header: Header) -> Message {
        let mut message = AssociationSetupRequest {
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
                    message.node_id = NodeID::decode(buf, elen).unwrap();
                }
                ie_type::RECOVERY_TIME_STAMP => {
                    message.recovery_time_stamp = RecoveryTimeStamp::decode(buf, elen).unwrap();
                }
                _ => println!(""),
            }
            buf = &mut buf[elen.into()..];
        }
        println!("{:#?}", message);
        Message::ASR(message)
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        message_vec.append(&mut self.node_id.encode());
        message_vec.append(&mut self.recovery_time_stamp.encode());
        println!("{:?}", message_vec);
        message_vec
    }
}
