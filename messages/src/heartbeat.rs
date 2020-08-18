use super::header::*;
use super::*;

use elements::ie_type;
use elements::recovery_time_stamp::RecoveryTimeStamp;

#[derive(Debug, Default)]
pub struct HeartbeatRequest {
    pub header: Header,
    pub recovery_time_stamp: RecoveryTimeStamp,
}

impl HeartbeatRequest {
    pub fn parse(mut buf: &mut [u8], header: Header) -> _Message {
        let mut message = HeartbeatRequest {
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
                ie_type::RECOVERY_TIME_STAMP => {
                    message.recovery_time_stamp = RecoveryTimeStamp::decode(buf, elen)?;
                }
                _ => println!("unknown ie_type!"),
            }
            buf = &mut buf[elen.into()..];
        }
        println!("{:#?}", message);
        Ok(Message::HReq(message))
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        message_vec.append(&mut self.recovery_time_stamp.encode());
        println!("{:?}", message_vec);
        message_vec
    }
}

#[derive(Debug, Default)]
pub struct HeartbeatResponse {
    pub header: Header,
    pub recovery_time_stamp: RecoveryTimeStamp,
}

impl HeartbeatResponse {
    pub fn parse(mut buf: &mut [u8], header: Header) -> _Message {
        let mut message = HeartbeatResponse {
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
                ie_type::RECOVERY_TIME_STAMP => {
                    message.recovery_time_stamp = RecoveryTimeStamp::decode(buf, elen)?;
                }
                _ => println!("unknown ie_type!"),
            }
            buf = &mut buf[elen.into()..];
        }
        println!("{:#?}", message);
        Ok(Message::HResp(message))
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        message_vec.append(&mut self.recovery_time_stamp.encode());
        println!("{:?}", message_vec);
        message_vec
    }
}
