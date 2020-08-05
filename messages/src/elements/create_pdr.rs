//use std::error::Error;
//use anyhow::Result;
use super::PFCPError;
use super::ie_type;


#[derive(Debug, Default)]
pub struct CreatePDR {
    
    ie_type: u16,
    ie_len: u16,

    //This IE shall contain the PDI against which incoming packets will be matched.
    pdi: PDI,   //M

    //This IE shall indicate the PDR's precedence to be applied by the UP function 
    //among all PDRs of the PFCP session, when looking for a PDR matching an incoming packet.
    precedence: Precedence, //M

    //This IE shall uniquely identify the PDR among all the PDRs configured 
    //for that PFCP session.
    pdr_id: PDRID,  //M

    //This IE shall be present if the UP function is required to remove 
    //one or more outer header(s) from the packets matching this PDR.
    outer_header_removal: Option<OuterHeaderRemoval>,   //C 

    //This IE shall be present if the Activate Predefined Rules IE is not included or 
    //if it is included but it does not result in activating a predefined FAR, 
    //and if the MAR ID is not included.
    //
    //When present this IE shall contain the FAR ID to be associated to the PDR.
    far_id: Option<FARID>,  //C

    //This IE shall be present if a measurement action shall be 
    //applied to packets matching this PDR.
    //
    //When present, this IE shall contain the URR IDs to be associated to the PDR.
    //
    //Several IEs within the same IE type may be present to represent a list of URRs to be
    //associated to the PDR.
    urr_ids: Option<Vec<URRID>>,    //C

    //This IE shall be present if a QoS enforcement or QoS marking action shall be applied to
    //packets matching this PDR.
    //
    //When present, this IE shall contain the QER IDs to be associated to the PDR.
    //
    //Several IEs within the same IE type may be present to represent a list of QERs to be
    //associated to the PDR.
    qer_ids: Option<Vec<QERID>>,    //C

    //TODO
}

impl CreatePDR {
   pub fn decode(buf: &[u8], len: u16) -> Result<CreatePDR, PFCPError> {
        let mut element = CreatePDR {
            ie_type: ie_type::CREATE_PDR,
            ie_len: len,
            ..Default::default()
        };
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::PDI => {
                    element.pdi = PDI::decode(buf, elen)?;
                }
                ie_type::PRECEDENCE => {
                    element.precedence = Precedence::decode(buf, elen)?;
                }
                ie_type::PDR_ID => {
                    element.pdr_id = PDRID::decode(buf, elen)?;
                }
                ie_type::OUTER_HEADER_REMOVAL => {
                    element.outer_header_removal = Some(OuterHeaderRemoval::decode(buf, elen)?);
                }
                ie_type::FAR_ID => {
                    element.far_id = Some(FARID::decode(buf, elen)?);
                }
                ie_type::URR_ID => {
                    let urr_id = URRID::decode(buf, elen)?;
                    if let Some(urr_ids) = element.urr_ids {
                        urr_ids.append(urr_id);
                        element.urr_ids = Some(urr_ids);
                    } else {
                        element.urr_ids = Some(urr_id);
                    }
                }
                ie_type::QER_ID => {
                    let qer_id = QERID::decode(buf, elen)?;
                    if let Some(qer_ids) = element.qer_ids {
                        qer_ids.append(qer_id);
                        element.qer_ids = Some(qer_ids);
                    } else {
                        element.qer_ids = Some(qer_id);
                    }
                }
                _ => return Err(PFCPError::UnknownCreatePDR);
            }
            buf = &mut buf[elen.into()..];
        }
        Ok(element)
   }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.push(self.node_id_type);
        element_vec.append(&mut self.node_id_value);
        element_vec
    }
}
