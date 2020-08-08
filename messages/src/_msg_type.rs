//define message type

/*
pub enum MsgType {
    HeartbeatRequest = 1,
    HeartbeatResponse = 2,
    PFDManagementRequest = 3,
    PFDManagementResponse = 4,
    AssociationSetupRequest = 5,
    AssociationSetupResponse = 6,
    AssociationUpdateRequest = 7,
    AssociationUpdateResponse = 8,
    AssociationReleaseRequest = 9,
    AssociationReleaseResponse = 10,
    VersionNotSupported = 11,
    NodeReportRequest = 12,
    NodeReportResponse = 13,
    SessionSetDeletionRequest = 14,
    SessionSetDeletionResponse = 15,
    SessionEstablishmentRequest = 50,
    SessionEstablishmentResponse = 51,
    SessionModificationRequest = 52,
    SessionModificationResponse = 53,
    SessionDeletionRequest = 54,
    SessionDeletionResponse = 55,
    SessionReportRequest = 56,
    SessionReportResponse = 57,
}*/

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
