use super::super::PFCPError;
use super::ie_type;

// -----------------------------------------------------------------------
// 	                    Bits
// Octets	8	7	6	5	4	3	2	1
// 1 to 2	    Type = 84 (decimal)
// 3 to 4	    Length = n
// 5 to 6	    Outer Header Creation Description
// m to (m+3)	TEID
// p to (p+3)	IPv4 Address
// q to (q+15)	IPv6 Address
// r to (r+1)	Port Number
// t to (t+2)	C-TAG
// u to (u+2)	S-TAG
// s to (n+4)	These octet(s) is/are present only if explicitly specified
// -----------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct OuterHeaderCreation {
    ie_type: u16,
    ie_len: u16,
    description: Vec<u8>,         //M 2bytes
    teid: Option<Vec<u8>>,        //C 4bytes
    ipv4_addr: Option<Vec<u8>>,   //C 4bytes
    ipv6_addr: Option<Vec<u8>>,   //C 16bytes
    port_number: Option<Vec<u8>>, //C 2bytes
    c_tag: Option<Vec<u8>>,       //C 3bytes
    s_tag: Option<Vec<u8>>,       //C 3bytes
}

impl OuterHeaderCreation {
    pub fn decode(mut buf: &mut [u8], len: u16) -> Result<OuterHeaderCreation, PFCPError> {
        let mut element = OuterHeaderCreation {
            ie_type: ie_type::OUTER_HEADER_CREATION,
            ie_len: len,
            ..Default::default()
        };
        element.description = buf[0..=1].to_vec();
        buf = &mut buf[2..];
        if element.description[0] & 0b0000_0011 != 0 {
            element.teid = Some(buf[0..=3].to_vec());
            buf = &mut buf[4..];
        }
        if element.description[0] & 0b0001_0101 != 0 {
            element.ipv4_addr = Some(buf[0..=3].to_vec());
            buf = &mut buf[4..];
        }
        if element.description[0] & 0b0010_1010 != 0 {
            element.ipv6_addr = Some(buf[0..=15].to_vec());
            buf = &mut buf[16..];
        }
        if element.description[0] & 0b0000_1111 != 0 {
            element.port_number = Some(buf[0..=1].to_vec());
            buf = &mut buf[2..];
        }
        if element.description[0] & 0b0100_0000 != 0 {
            element.c_tag = Some(buf[0..=2].to_vec());
            buf = &mut buf[3..];
        }
        if element.description[0] & 0b1000_0000 != 0 {
            element.s_tag = Some(buf[0..=2].to_vec());
        }
        Ok(element)
    }

    pub fn encode(mut self) -> Vec<u8> {
        let mut element_vec: Vec<u8> = Vec::new();
        element_vec.append(&mut self.ie_type.to_be_bytes().to_vec());
        element_vec.append(&mut self.ie_len.to_be_bytes().to_vec());
        element_vec.append(&mut self.description);
        /*if let Some(mut teid) = self.teid {
            element_vec.append(&mut teid);
        }
        if let Some(mut ipv4_addr) = self.ipv4_addr {
            element_vec.append(&mut ipv4_addr);
        }
        if let Some(mut ipv6_addr) = self.ipv6_addr {
            element_vec.append(&mut ipv6_addr);
        }
        if let Some(mut port_number) = self.port_number {
            element_vec.append(&mut port_number);
        }
        if let Some(mut c_tag) = self.c_tag {
            element_vec.append(&mut c_tag);
        }
        if let Some(mut s_tag) = self.s_tag {
            element_vec.append(&mut s_tag);
        }*/
        self.teid.map(|mut teid| element_vec.append(&mut teid));
        self.ipv4_addr
            .map(|mut ipv4_addr| element_vec.append(&mut ipv4_addr));
        self.ipv6_addr
            .map(|mut ipv6_addr| element_vec.append(&mut ipv6_addr));
        self.port_number
            .map(|mut port_number| element_vec.append(&mut port_number));
        self.c_tag.map(|mut c_tag| element_vec.append(&mut c_tag));
        self.s_tag.map(|mut s_tag| element_vec.append(&mut s_tag));
        element_vec
    }
}
