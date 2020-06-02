#[derive(Debug, Default)]
pub struct Header {
    pub version: u8,
    pub mp: bool,
    pub s: bool,
    pub msg_t: u8,
    pub msg_len: u16,
    pub seid: Option<u64>,
    pub sequence: u32,
    pub priority: Option<u8>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            ..Default::default()
        }
    }

    pub fn set_version(mut self, version: u8) -> Self {
        self.version = version;
        self
    }

    pub fn set_mp(mut self, mp: bool) -> Self {
        self.mp = mp;
        self
    }

    pub fn set_s(mut self, s: bool) -> Self {
        self.s = s;
        self
    }

    pub fn parse(buf: &[u8]) -> Header {
        let mut header = Header {
            ..Default::default()
        };
        header.version = buf[0] >> 5;
        match buf[0] & 0b00000010 >> 1 {
            1 => {
                header.mp = true;
                header.seid =
                    Some((buf[4] * 16 * 16 * 16 + buf[5] * 16 * 16 + buf[6] * 16 + buf[7]).into());
                header.sequence = (buf[8] * 16 * 16 + buf[9] * 16 + buf[10]).into();
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
        header.msg_t = buf[1];
        header.msg_len = (buf[2] * 16 + buf[3]).into();
        header
    }
}
