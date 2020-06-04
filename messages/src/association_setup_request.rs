use super::header::*;
use super::*;
use elements::ie_type;
use elements::node_id::*;
use elements::recovery_time_stamp::*;

#[derive(Debug, Default)]
pub struct AssociationSetupRequest {
    pub header: Header,
    pub node_id: NodeID,
    pub recovery_time_stamp: RecoveryTimeStamp,
}

impl AssociationSetupRequest {
    pub fn parse(mut buf: &mut [u8], header: Header) -> Message {
        let mut associationsetuprequest = AssociationSetupRequest {
            header,
            ..Default::default()
        };
        if associationsetuprequest.header.s == true {
            buf = &mut buf[12..];
        } else {
            buf = &mut buf[4..];
        }
        //let cursor: u16 = 0;
        //while cursor < associationsetuprequest.header.msg_len {
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                //t if t == IEType::NodeID as u16 => {
                ie_type::NODE_ID => {
                    associationsetuprequest.node_id = NodeID::decode(buf, elen);
                }
                //t if t == IEType::RecoveryTimeStamp as u16 => {
                ie_type::RECOVERY_TIME_STAMP => {
                    associationsetuprequest.recovery_time_stamp =
                        RecoveryTimeStamp::decode(buf, elen);
                }
                _ => println!(""),
            }
            buf = &mut buf[elen.into()..];
            //let cursor = cursor + elen + 4;
        }
        println!("{:#?}", associationsetuprequest);
        Message::ASR(associationsetuprequest)
    }
}
