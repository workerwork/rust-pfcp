use std::convert::TryInto;

#[derive(Debug, Default)]
pub struct Header {
    pub version: u8,
    pub mp: bool,
    pub s: bool,
    pub msg_t: u8,
    pub msg_len: u16,
    pub seid: Option<u32>,
    pub sequence: u32,
    pub priority: Option<u8>,
}

impl Header {
    pub fn parse(buf: &[u8]) -> Header {
        let mut header = Header {
            version: buf[0] >> 5,
            msg_t: buf[1],
            msg_len: (buf[2] * 16 + buf[3]).into(),
            //msg_len: buf[2..4].to_vec(),
            ..Default::default()
        };
        match buf[0] & 0b00000010 >> 1 {
            1 => {
                header.mp = true;
                header.seid =
                    Some((buf[4] * 16 * 16 * 16 + buf[5] * 16 * 16 + buf[6] * 16 + buf[7]).into());
                    //Some(buf[4..8].to_vec());
                header.sequence = (buf[8] * 16 * 16 + buf[9] * 16 + buf[10]).into();
                //header.sequence = buf[8..11].to_vec();
                match buf[0] & 0b00000001 {
                    1 => {
                        header.s = true;
                        header.priority = Some(buf[12]);
                    }
                    _ => {
                        header.s = false;
                        header.priority = None;
                    }
                }
            }
            _ => {
                header.mp = false;
                header.seid = None;
                header.sequence = (buf[4] * 16 * 16 + buf[5] * 16 + buf[6]).into();
                //header.sequence = buf[4..7].to_vec();
                match buf[0] & 0b00000001 {
                    1 => {
                        header.s = true;
                        header.priority = Some(buf[8]);
                    }
                    _ => {
                        header.s = false;
                        header.priority = None;
                    }
                }
            }
        }
        header
    }

    pub fn pack(self) -> Vec<u8> {
        let mut header_vec: Vec<u8> = Vec::new();
        let mut b0: u8 = self.version << 5;
        if self.mp {
            b0 = b0 | 0b0000_0010;
        }
        if self.s {
            b0 = b0 | 0b0000_0001;
        }
        header_vec.push(b0);
        header_vec.push(self.msg_t);
        header_vec.append(&mut self.msg_len.to_be_bytes().to_vec());
        /*if self.s {
            header_vec.append(&mut self.seid);
        }*/
        if let Some(seid) = self.seid {
            header_vec.append(&mut seid.to_be_bytes().to_vec());
        }
        header_vec.push(((self.sequence >> 16) & 0xFF).try_into().unwrap());
        header_vec.push(((self.sequence >> 8) & 0xFF).try_into().unwrap());
        header_vec.push((self.sequence & 0xFF).try_into().unwrap());
        if let Some(priority) = self.priority {
            header_vec.push(priority);
        } else {
            header_vec.push(0x00);
        }
        header_vec
    }
}
