pub mod association_setup_request;
pub mod header;
pub mod msg_type;

pub enum Message {
    ASR(association_setup_request::AssociationSetupRequest),
    AUR(AssociationUpdateRequest),
    ARR(AssociationReleaseRequest),
    //NodeReportResponse,
    SER(SessionEstablishmentRequest),
    SMR(SessionModificationRequest),
    SDR(SessionDeletionRequest),
}

pub struct AssociationUpdateRequest {}

pub struct AssociationReleaseRequest {}

pub struct SessionEstablishmentRequest {}

pub struct SessionModificationRequest {}

pub struct SessionDeletionRequest {}

/*
#[derive(Debug, Default)]
pub struct Header {
    pub version: u8,
    pub mp: bool,
    pub s: bool,
    pub msg_t: u8,
    pub msg_len: u16,
    pub seid: Option<u64>,
    pub sequence: u32,
    pub priority: Option<u8>,
}*/

pub fn parse_header<'a>(buf: &'a [u8], header: &mut header::Header) -> &'a [u8] {
    //let mut header = header::Header::new();
    header.version = buf[0] >> 5;
    header.msg_t = buf[1];
    header.msg_len = (buf[2] * 16 + buf[3]).into();
    match buf[0] & 0b00000010 >> 1 {
        1 => {
            header.mp = true;
            header.seid =
                Some((buf[4] * 16 * 16 * 16 + buf[5] * 16 * 16 + buf[6] * 16 + buf[7]).into());
            header.sequence = (buf[8] * 16 * 16 + buf[9] * 16 + buf[10]).into();
            match buf[0] & 0b00000001 {
                1 => {
                    header.s = true;
                    header.priority = Some(buf[12]);
                    &buf[13..]
                }
                _ => {
                    header.s = false;
                    header.priority = None;
                    &buf[12..]
                }
            }
        }
        _ => {
            header.mp = false;
            header.seid = None;
            header.sequence = (buf[4] * 16 * 16 + buf[5] * 16 + buf[6]).into();
            match buf[0] & 0b00000001 {
                1 => {
                    header.s = true;
                    header.priority = Some(buf[8]);
                    &buf[9..]
                }
                _ => {
                    header.s = false;
                    header.priority = None;
                    &buf[8..]
                }
            }
        }
    }
}
