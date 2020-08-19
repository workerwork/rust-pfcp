pub mod association_release;
pub mod association_setup;
pub mod association_update;
pub mod elements;
pub mod header;
pub mod heartbeat;
pub mod session_deletion;
pub mod session_establishment;
pub mod session_modification;

use association_release::*;
use association_setup::*;
use association_update::*;
use header::Header;
use heartbeat::*;
use session_deletion::*;
use session_establishment::*;
use session_modification::*;

//define message type
pub mod msg_type {
    pub const HEARTBEAT_REQUEST: u8 = 1;
    pub const HEARTBEAT_RESPONSE: u8 = 2;
    pub const PFD_MANAGEMENT_REQUEST: u8 = 3;
    pub const PFD_MANAGEMENT_RESPONSE: u8 = 4;
    pub const ASSOCIATION_SETUP_REQUEST: u8 = 5;
    pub const ASSOCIATION_SETUP_RESPONSE: u8 = 6;
    pub const ASSOCIATION_UPDATE_REQUEST: u8 = 7;
    pub const ASSOCIATION_UPDATE_RESPONSE: u8 = 8;
    pub const ASSOCIATION_RELEASE_REQUEST: u8 = 9;
    pub const ASSOCIATION_RELEASE_RESPONSE: u8 = 10;
    pub const VERSION_NOT_SUPPORTED: u8 = 11;
    pub const NODE_REPORT_REQUEST: u8 = 12;
    pub const NODE_REPORT_RESPONSE: u8 = 13;
    pub const SESSION_SET_DELETION_REQUEST: u8 = 14;
    pub const SESSION_SET_DELETION_RESPONSE: u8 = 15;
    pub const SESSION_ESTABLISHMENT_REQUEST: u8 = 50;
    pub const SESSION_ESTABLISHMENT_RESPONSE: u8 = 51;
    pub const SESSION_MODIFICATION_REQUEST: u8 = 52;
    pub const SESSION_MODIFICATION_RESPONSE: u8 = 53;
    pub const SESSION_DELETION_REQUEST: u8 = 54;
    pub const SESSION_DELETION_RESPONSE: u8 = 55;
    pub const SESSION_REPORT_REQUEST: u8 = 56;
    pub const SESSION_REPORT_RESPONSE: u8 = 57;
}

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

pub type _Message = Result<Message, PFCPError>;

#[derive(Debug)]
pub enum Message {
    ASReq(AssociationSetupRequest),
    ASResp(AssociationSetupResponse),
    AUReq(AssociationUpdateRequest),
    AUResp(AssociationUpdateResponse),
    ARReq(AssociationReleaseRequest),
    ARResp(AssociationReleaseResponse),
    //NodeReportResponse,
    SEReq(SessionEstablishmentRequest),
    SEResp(SessionEstablishmentResponse),
    SMReq(SessionModificationRequest),
    SMResp(SessionModificationResponse),
    SDReq(SessionDeletionRequest),
    SDResp(SessionDeletionResponse),
    HReq(HeartbeatRequest),
    HResp(HeartbeatResponse),
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
            msg_type::ASSOCIATION_SETUP_REQUEST => {
                AssociationSetupRequest::parse(buf, header).unwrap()
            }
            //msg_type::ASSOCIATION_UPDATE_REQUEST => AssociationUpdateRequest::parse(buf, header),
            msg_type::ASSOCIATION_RELEASE_REQUEST => AssociationReleaseRequest::parse(buf, header).unwrap(),
            msg_type::SESSION_ESTABLISHMENT_REQUEST => {
                SessionEstablishmentRequest::parse(buf, header).unwrap()
            }
            msg_type::SESSION_MODIFICATION_REQUEST => SessionModificationRequest::parse(buf, header).unwrap(),
            msg_type::SESSION_DELETION_REQUEST => SessionDeletionRequest::parse(buf, header).unwrap(),
            _ => {
                println!("err");
                AssociationSetupRequest::parse(buf, header).unwrap()
            }
        }
    }

    pub fn pack(self) -> Vec<u8> {
        match self {
            Message::ASReq(asreq) => asreq.pack(),
            Message::ASResp(asresp) => asresp.pack(),
            //Message::AUReq(aureq) => aureq.pack(),
            //Message::AUResp(aursp) => auresp.pack(),
            Message::ARReq(arreq) => arreq.pack(),
            Message::ARResp(arresp) => arresp.pack(),
            Message::SEReq(sereq) => sereq.pack(),
            Message::SEResp(seresp) => seresp.pack(),
            Message::SMReq(smreq) => smreq.pack(),
            Message::SMResp(smresp) => smresp.pack(),
            Message::SDReq(sdreq) => sdreq.pack(),
            Message::SDResp(sdresp) => sdresp.pack(),
            _ => vec![0],
        }
    }
}
