pub mod association_release_request;
pub mod association_setup_request;
pub mod association_update_request;
pub mod elements;
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

//define error
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PFCPError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown pfcp error")]
    Unknown,
    #[error("unknown CreatePDR element")]
    UnknownCreatePDR,
}

#[derive(Debug)]
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
            msg_type::ASSOCIATION_SETUP_REQUEST => AssociationSetupRequest::parse(buf, header),
            //msg_type::ASSOCIATION_UPDATE_REQUEST => AssociationUpdateRequest::parse(buf, header),
            //msg_type::ASSOCIATION_RELEASE_REQUEST => AssociationReleaseRequest::parse(buf, header),
            //msg_type::SESSION_ESTABLISHMENT_REQUEST => SessionEstablishmentRequest::parse(buf, header),
            //msg_type::SESSION_MODIFICATION_REQUEST => SessionModificationRequest::parse(buf, header),
            //msg_type::SESSION_DELETION_REQUEST => SessionDeletionRequest::parse(buf, header),
            _ => {
                println!("err");
                AssociationSetupRequest::parse(buf, header)
            }
        }
    }

    pub fn pack(self) -> Vec<u8> {
        match self {
            Message::ASR(asr) => asr.pack(),
            //Message::AUR(aur) => aur.pack(),
            //Message::ARR(arr) => arr.pack(),
            //Message::SER(ser) => ser.pack(),
            //Message::SMR(smr) => smr.pack(),
            //Message::SDR(sdr) => sdr.pack(),
            _ => vec![0],
        }
    }
}
