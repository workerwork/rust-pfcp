use super::header::*;
use super::*;
use elements::ie_type;
use elements::node_id::*;
use elements::f_seid::*;
use elements::pdn_type::*;
use elements::create_pdr::*;
use elements::create_far::*;
use elements::create_urr::*;
use elements::create_qer::*;

#[derive(Debug, Default)]
pub struct SessionEstablishmentRequest {
    pub header: Header,
    pub node_id: NodeID,
    pub f_seid: FSEID,
    pub pdn_type: PDNType,
    pub create_pdrs: Vec<CreatePDR>,
    pub create_fars: Vec<CreateFAR>,
    pub create_urrs: Vec<CreateURR>,
    pub create_qers: Vec<CreateQER>,
}

impl SessionEstablishmentRequest {
    pub fn parse(mut buf: &mut [u8], header: Header) -> Message {
        let mut message = SessionEstablishmentRequest {
            header,
            ..Default::default()
        };
        if message.header.s == true {
            buf = &mut buf[12..];
        } else {
            buf = &mut buf[4..];
        }
        while buf != [] {
            let etype: u16 = (buf[0] * 16 + buf[1]).into();
            let elen: u16 = (buf[2] * 16 + buf[3]).into();
            buf = &mut buf[4..];
            match etype {
                ie_type::NODE_ID => {
                    message.node_id = NodeID::decode(buf, elen).unwrap();
                }
                ie_type::F_SEID => {
                    message.f_seid = FSEID::decode(buf, elen).unwrap();
                }
                ie_type::PDN_TYPE => {
                    message.pdn_type = PDNType::decode(buf, elen).unwrap();
                }
                ie_type::CREATE_PDR => {
                    message.create_pdrs.append(CreatePDR::decode(buf, elen).unwrap());
                }
                ie_type::CREATE_FAR => {
                    message.create_fars.append(CreateFAR::decode(buf, elen).unwrap());
                }
                ie_type::CREATE_URR => {
                    message.create_urrs.append(CreateURR::decode(buf, elen).unwrap());
                }
                ie_type::CREATE_QER => {
                    message.create_qers.append(CreateQER::decode(buf, elen).unwrap());
                }
                _ => println!(""),
            }
            buf = &mut buf[elen.into()..];
        }
        println!("{:#?}", message);
        Message::SER(message)
    }

    pub fn pack(self) -> Vec<u8> {
        let mut message_vec: Vec<u8> = Vec::new();
        message_vec.append(&mut self.header.pack());
        message_vec.append(&mut self.node_id.encode());
        message_vec.append(&mut self.f_seid.encode());
        message_vec.append(&mut self.pdn_type.encode());
        message_vec.append(&mut self.create_pdrs.encode());
        message_vec.append(&mut self.create_fars.encode());
        message_vec.append(&mut self.create_urrs.encode());
        message_vec.append(&mut self.create_qers.encode());
        println!("{:?}", message_vec);
        message_vec
    }
}


