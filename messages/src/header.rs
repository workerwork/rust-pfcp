#[derive(Debug, Default)]
pub struct Header {
    pub version: u8,
    pub mp: bool,
    pub s: bool,
    pub msg_t: u16,
    pub msg_len: u16,
    pub seid: u64,
    pub sequence: u32,
    pub priority: u8,
}

impl Header {
    pub fn new() -> Header {
        Header {
            version: 1,
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
}
