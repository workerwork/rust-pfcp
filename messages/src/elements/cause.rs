enum CauseType {
    CauseReserved = 0,
    CauseSuccess = 1, //2-63,spare
    CauseUnspecifiedReason = 64,
    CauseSessionContextNoFound = 65, //fseid pfcp session modification/deletion is unknown
    CauseMandatoryIEMissing = 66,
    CauseConditionalIEMissing = 67,
    CauseInvalidLength = 68,
    CauseMandatoryIEIncorrect = 69, //eg:is malformed or it carries an invalid oe unexpected value
    CauseInvalidForwardingPolicy = 70,
    CauseInvalidFTEIDAllocation = 71, //same TEID
    CauseNoEstablishedPFCPAssociation = 72,
    CauseRuleCreationOrModificationFailure = 73, //failed to be stored
    CausePFCPEntityInCongestion = 74,
    CauseNoResourcesAvailable = 75,
    CauseServiceNotSupported = 76,
    CauseSystemFailure = 77, //78-255 ,SPARE
}

pub struct Cause {
    ie_type: super::ie_type::IEType,
    ie_len: u16,
    cause: CauseType,
}

impl Cause {}
