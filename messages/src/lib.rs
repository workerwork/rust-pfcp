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
use msg_type::*;
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
            t if t == MsgType::AssociationSetupRequest as u8 => {
                AssociationSetupRequest::parse(buf, header)
            }
            //t if t == MsgType::AssociationUpdateRequest as u8 => AssociationUpdateRequest::parse(),
            //t if t == MsgType::AssociationReleaseRequest as u8 => AssociationReleaseRequest::parse(),
            //t if t == MsgType::SessionEstablishmentRequest as u8 => {
            //    SessionEstablishmentRequest::parse()
            //}
            //t if t == MsgType::SessionModificationRequest as u8 => SessionModificationRequest::parse(),
            //t if t == MsgType::SessionDeletionRequest as u8 => SessionDeletionRequest::parse(),
            _ => {
                println!("err");
                AssociationSetupRequest::parse(buf, header)
            }
        }
    }
}
