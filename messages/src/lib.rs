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
