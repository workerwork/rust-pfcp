use super::super::PFCPError;
use super::ie_type;

use super::gate_status::GateStatus;
use super::qer_id::QERID;

#[derive(Debug, Default)]
pub struct CreateQER {
    ie_type: u16,
    ie_len: u16,

    //This IE shall uniquely identify the QER among all the QER configured for that PFCP session.
    qer_id: QERID, //M

    //This IE shall indicate whether the packets are allowed to be forwarded (the gate is open) or
    //shall be discarded (the gate is closed) in the uplink and/or downlink directions.
    gate_status: GateStatus, //M

                             //
}

impl CreateQER {
    pub fn decode(buf: &[u8], len: u16) -> Result<CreateQER, PFCPError> {
        let mut element = CreateQER {
            ie_type: ie_type::CREATE_QER,
            ie_len: len,
            ..Default::default()
        };
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::QER_ID => {
                    element.qer_id = QERID::decode(buf, elen)?;
                }
                ie_type::GATE_STATUS => {
                    element.gate_status = GateStatus::decode(buf, elen)?;
                }
                _ => (),
            }
            buf = &mut buf[elen.into()..];
        }
        Ok(element)
    }

    pub fn encode(self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.qer_id.encode());
        element_vec.append(&mut self.gate_status.encode());
        element_vec
    }
}
