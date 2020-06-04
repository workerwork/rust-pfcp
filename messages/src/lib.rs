pub mod association_release_request;
pub mod association_setup_request;
pub mod association_update_request;
pub mod header;
pub mod msg_type;
pub mod session_deletion_request;
pub mod session_establishment_request;
pub mod session_modification_request;

use association_release_request::*;
use association_setup_request::*;
use association_update_request::*;
use header::*;
//use msg_type::*;
use session_deletion_request::*;
use session_establishment_request::*;
use session_modification_request::*;

pub enum Message {
    ASR(AssociationSetupRequest),
    AUR(AssociationUpdateRequest),
    ARR(AssociationReleaseRequest),
    //NodeReportResponse,
    SER(SessionEstablishmentRequest),
    SMR(SessionModificationRequest),
    SDR(SessionDeletionRequest),
}

impl Message {
    pub fn parse(mut buf: &mut [u8]) -> Message {
        let header = Header::parse(&buf);
        if header.s == true {
            buf = &mut buf[12..];
        } else {
            buf = &mut buf[4..];
        }
        match header.msg_t {
            //t if t == MsgType::AssociationSetupRequest as u8 => {
            msg_type::ASSOCIATION_SETUP_REQUEST => AssociationSetupRequest::parse(buf, header),
            _ => {
                println!("err");
                AssociationSetupRequest::parse(buf, header)
            }
        }
    }

    pub fn pack(self) -> Vec<u8> {
        let mut resp_vec: Vec<u8> = Vec::new();
        match self {
            Message::ASR(asr) => {
                //resp_vec.push(asr.header.pack()[0]);
                resp_vec.append(&mut asr.header.pack());
                println!("{:?}", resp_vec);
            }
            _ => (),
        }
        resp_vec
    }
}
