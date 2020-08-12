use super::super::PFCPError;
use super::ie_type;

// --------------------------------------------
//	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 19 (decimal)
// 3 to 4	    Length = n
// 5	        Cause value
// --------------------------------------------

pub mod cause_type {
    pub const RESERVED: u8 = 0;
    pub const SUCCESS: u8 = 1;
    //2-63,spare
    pub const UNSPECIFIED_REASON: u8 = 64;
    pub const SESSION_CONTEXT_NO_FOUND: u8 = 65; //fseid pfcp session modification/deletion is unknown
    pub const MANDATORY_IE_MISSING: u8 = 66;
    pub const CONDITIONAL_IE_MISSING: u8 = 67;
    pub const INVALID_LENGTH: u8 = 68;
    pub const MANDATORY_IE_INCORRECT: u8 = 69; //eg:is malformed or it carries an invalid oe unexpected value
    pub const INVALID_FORWARDING_POLICY: u8 = 70;
    pub const INVALID_FTEID_ALLOCATION_OPTION: u8 = 71; //same TEID
    pub const NO_ESTABLISHED_PFCP_ASSOCIATION: u8 = 72;
    pub const RULE_CREATION_OR_MODIFICATION_FAILURE: u8 = 73; //failed to be stored
    pub const PFCP_ENTITY_IN_CONGESTION: u8 = 74;
    pub const NO_RESOURCES_AVAILABLE: u8 = 75;
    pub const SERVICE_NOT_SUPPORTED: u8 = 76;
    pub const SYSTEM_FAILURE: u8 = 77;
    pub const REDIRECTION_REQUESTED: u8 = 78;
    //78-255 ,SPARE
}

#[derive(Debug, Default)]
pub struct Cause {
    ie_type: u16,
    ie_len: u16,
    cause: u8,
}

impl Cause {
    pub fn decode(buf: &mut [u8], len: u16) -> Result<Cause, PFCPError> {
        let mut element = Cause {
            ie_type: ie_type::CAUSE,
            ie_len: len,
            ..Default::default()
        };
        element.cause = buf[0];
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.cause);
        element_vec
    }
}
