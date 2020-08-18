use super::header::*;
use super::*;

use elements::cause::Cause;
use elements::ie_type;

#[derive(Debug, Default)]
pub struct SessionDeletionRequest {
    pub header: Header,
}

impl SessionDeletionRequest {
    pub fn parse(_buf: &mut [u8], header: Header) -> _Message {
        let message = SessionDeletionRequest {
            header,
            ..Default::default()
        };
        println!("{:#?}", message);
        Ok(Message::SDReq(message))
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        println!("{:?}", message_vec);
        message_vec
    }
}

#[derive(Debug, Default)]
pub struct SessionDeletionResponse {
    pub header: Header,
    pub cause: Cause,
}

impl SessionDeletionResponse {
    pub fn parse(mut buf: &mut [u8], header: Header) -> _Message {
        let mut message = SessionDeletionResponse {
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
                ie_type::CAUSE => {
                    message.cause = Cause::decode(buf, elen)?;
                }
                _ => println!("unknown ie_type!"),
            }
            buf = &mut buf[elen.into()..];
        }
        println!("{:#?}", message);
        Ok(Message::SDResp(message))
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        message_vec.append(&mut self.cause.encode());
        println!("{:?}", message_vec);
        message_vec
    }
}
