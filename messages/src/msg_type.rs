//define message type
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
}


