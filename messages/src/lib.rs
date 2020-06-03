pub mod msg_type;
pub mod header;
pub mod association_setup_request;
pub mod association_update_request;
pub mod association_release_request;
pub mod session_establishment_request;
pub mod session_modification_request;
pub mod session_deletion_request;

use header::*;
use association_setup_request::*;
use association_update_request::*;
use association_release_request::*;
use session_establishment_request::*;
use session_modification_request::*;
use session_deletion_request::*;

pub enum Message {
    ASR(AssociationSetupRequest),
    AUR(AssociationUpdateRequest),
    ARR(AssociationReleaseRequest),
    //NodeReportResponse,
    SER(SessionEstablishmentRequest),
    SMR(SessionModificationRequest),
    SDR(SessionDeletionRequest),
}

pub fn parse_header<'a>(buf: &'a [u8], header: &mut Header) -> &'a [u8] {
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
