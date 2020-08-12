use super::super::PFCPError;
use super::ie_type;

use super::far_id::FARID;
use super::apply_action::ApplyAction;
use super::forwarding_parameters::ForwardingParameters;



#[derive(Debug, Default)]
pub struct CreateFAR {
    ie_type: u16,
    ie_len: u16,

    //This IE shall uniquely identify the FAR among all the FARs configured for that PFCP session.
    far_id: FARID, //M

    //This IE shall indicate the action to apply to the packets.
    apply_action: ApplyAction, //M

    //This IE shall be present when the Apply Action requests the packets to be forwarded. It may
    //be present otherwise.
    //When present, this IE shall contain the forwarding instructions to be applied by the UP
    //function when the Apply Action requests the packets to be forwarded.
    forwarding_parameters: Option<ForwardingParameters>, //C

                                                         //bar_id
}

impl CreateFAR {
    pub fn decode(buf: &[u8], len: u16) -> Result<CreateFAR, PFCPError> {
        let mut element = CreateFAR {
            ie_type: ie_type::CREATE_FAR,
            ie_len: len,
            ..Default::default()
        };
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::FAR_ID => {
                    element.far_id = FARID::decode(buf, elen)?;
                }
                ie_type::APPLY_ACTION => {
                    element.apply_action = ApplyAction::decode(buf, elen)?;
                }
                ie_type::FORWARDING_PARAMETERS => {
                    element.forwarding_parameters = Some(ForwardingParameters::decode(buf, elen)?);
                }
            }
            buf = &mut buf[elen.into()..];
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.far_id.encode());
        element_vec.append(&mut self.apply_action.encode());
        if let Some(forwarding_parameters) = self.forwarding_parameters {
            element_vec.append(&mut forwarding_parameters.encode());
        }
        element_vec
    }
}
